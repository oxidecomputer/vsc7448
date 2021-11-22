use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{App, Arg};
use regex::Regex;

fn normalize_target(s: &str) -> &str {
    // Apply remappings based on inconsistent file naming
    match s {
        "CFG" => "ICPU_CFG",
        "DEV2G5" => "DEV1G",
        "PCIE_EP" => "PCIE",
        "PCS10G_BR" => "PCS_10GBASE_R",
        "TWI2" => "TWI",
        "UART2" => "UART",
        "VAUI0" | "VAUI1" => "VAUI_CHANNEL",
        "VCAP_ES0" | "VCAP_SUPER" => "VCAP_CORE",
        "XGANA" => "SD10G65",
        "XGDIG" => "SD10G65_DIG",
        "XGKR0" | "XGKR1" => "KR_DEV1",
        "XGXFI" => "XFI_SHELL",
        _ => s,
    }
}

#[derive(Debug)]
struct Field {
    brief: Option<String>,
    details: Option<String>,
    lo: usize,
    hi: usize,
}
#[derive(Debug)]
struct Register {
    brief: Option<String>,
    details: Option<String>,
    fields: HashMap<String, Field>
}
#[derive(Debug)]
struct RegisterGroup {
    desc: String,
    regs: HashMap<String, Register>,
}
#[derive(Debug)]
struct Target {
    desc: String,
    groups: HashMap<String, RegisterGroup>,
}

/// Parses `vtss_*_regs_common.h`, returning a map of target name to
/// absolute address (with offsets applied)
fn parse_regs_common(s: &str) -> HashMap<String, usize> {
    let offset = Regex::new(r"^#define VTSS_IO_ORIGIN([[:xdigit:]]+)_OFFSET 0x([[:xdigit:]]+)$")
        .expect("failed to build regex");
    let reg =
        Regex::new(r"^#define VTSS_TO_([A-Z_0-9]+) +VTSS_IO_OFFSET(\d+)\(0x([[:xdigit:]]+)\)")
            .expect("failed to build regex");
    let mut offsets = HashMap::new();
    let mut out = HashMap::new();
    for s in s.lines() {
        if let Some(cap) = offset.captures(s) {
            offsets.insert(
                (&cap[1]).parse::<usize>().unwrap(),
                usize::from_str_radix(&cap[2], 16).unwrap(),
            );
        }
        if let Some(cap) = reg.captures(s) {
            let i = (&cap[2]).parse::<usize>().unwrap();
            let d = usize::from_str_radix(&cap[3], 16).unwrap();
            out.insert(cap[1].to_owned(), d + offsets[&i]);
        }
    }
    out
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DoxygenBlockType {
    Target,
    Register,
    RegisterGroup,
    Field,
    Unknown,
}

#[derive(Debug)]
struct DoxygenBlock {
    block_type: DoxygenBlockType,
    name: String,
    desc: Option<String>,
    brief: Option<String>,
    details: Option<String>,
}

// Parses a single Doxygen comment in a `vtss_*_regs_*.h` file
fn parse_doxygen_block(s: &str) -> DoxygenBlock {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum State {
        Brief,
        Details,
        Desc,
    }
    let mut state = State::Desc;
    let mut out = DoxygenBlock {
        block_type: DoxygenBlockType::Unknown,
        name: "".to_string(),
        desc: None,
        brief: None,
        details: None,
    };

    let mut accum = String::new();

    // All the best parsing is done with regexes
    let reg_group_re = Regex::new(r"Register Group: \\a (.*):(.*)$").unwrap();
    let target_re = Regex::new(r"Target: \\a (.*)$").unwrap();
    let reg_re = Regex::new(r"Register: \\a (.*):(.*):(.*)$").unwrap();
    let field_re = Regex::new(r"Field: ::(.*) . (.*)$").unwrap();
    let brief_re = Regex::new(r"\\brief(.*)$").unwrap();
    let details_re = Regex::new(r"\\details(.*)$").unwrap();
    for s in s.lines() {
        let s = s.trim_start_matches(" *").trim();

        let prev_state = (out.block_type, state);
        let mut prev_accum = String::new();
        std::mem::swap(&mut accum, &mut prev_accum);

        if let Some(cap) = reg_group_re.captures(s) {
            assert!(out.block_type == DoxygenBlockType::Unknown);
            out.block_type = DoxygenBlockType::RegisterGroup;
            out.name = cap[2].to_owned();
            state = State::Desc;
        }
        if let Some(cap) = target_re.captures(s) {
            assert!(out.block_type == DoxygenBlockType::Unknown);
            out.block_type = DoxygenBlockType::Target;
            out.name = cap[1].to_owned();
            state = State::Desc;
        }
        if let Some(cap) = reg_re.captures(s) {
            assert!(out.block_type == DoxygenBlockType::Unknown);
            out.block_type = DoxygenBlockType::Register;
            out.name = cap[3].to_owned();
            state = State::Desc;
        }
        if let Some(cap) = field_re.captures(s) {
            assert!(out.block_type == DoxygenBlockType::Unknown);
            out.block_type = DoxygenBlockType::Field;
            out.name = cap[2].to_owned();
            state = State::Desc;
        }
        if let Some(cap) = brief_re.captures(s) {
            state = State::Brief;
            accum = cap[1].to_owned();
        }
        if let Some(cap) = details_re.captures(s) {
            state = State::Details;
            accum = cap[1].to_owned();
        }

        // If our state has changed, then flush the previous accumulated text
        // to the appropriate output location
        if (out.block_type, state) != prev_state {
            if !prev_accum.is_empty() {
                let prev_accum = prev_accum.trim().to_owned();
                match prev_state.1 {
                    State::Brief => {
                        assert!(out.brief.is_none());
                        out.brief = Some(prev_accum);
                    },
                    State::Details => {
                        assert!(out.details.is_none());
                        out.details = Some(prev_accum);
                    },
                    State::Desc => {
                        assert!(out.desc.is_none());
                        out.desc = Some(prev_accum);
                    },
                }
            }
        } else {
            std::mem::swap(&mut accum, &mut prev_accum);
            if !s.is_empty() {
                accum += s;
                accum += " ";
            }
        }
    }
    // Handle any accumulated text at the end of the parse
    let accum = accum.trim().to_owned();
    if !accum.is_empty() {
        match state {
            State::Brief => {
                assert!(out.brief.is_none());
                out.brief = Some(accum);
            },
            State::Details => {
                assert!(out.details.is_none());
                out.details = Some(accum);
            },
            State::Desc => {
                assert!(out.desc.is_none());
                out.desc = Some(accum);
            },
        };
    }
    out
}

// Horrifying code to parse a vtss_*_regs_*.h file
fn parse_regs_specific(s: &str) -> Target {
    let mut itr = s.lines().peekable();
    let field_re = Regex::new(r"#define\s+VTSS_F[A-Z_0-9]*\(x\)\s+(\w*)\((.+)\)$").unwrap();
    let mut target = None;
    let mut group = None;
    let mut register = None;

    while let Some(s) = itr.next() {
        let mut item = None;
        if s.starts_with("/**") {
            let mut block = String::new();
            for s in &mut itr {
                if s.trim().ends_with("*/") {
                    item = Some(parse_doxygen_block(&block));
                    break;
                } else {
                    block += s;
                    block += "\n";
                }
            }
        }
        if item.is_none() {
            continue;
        }
        let item = item.unwrap();
        match item.block_type {
            DoxygenBlockType::Target => {
                assert!(target.is_none());
                assert!(item.brief.is_none());
                assert!(item.details.is_none());
                target = Some(Target {
                    desc: item.desc.unwrap(),
                    groups: HashMap::new(),
                });
            },
            DoxygenBlockType::RegisterGroup => {
                if let Some((name, group)) = group.take() {
                    target.as_mut().unwrap().groups.insert(name, group);
                }
                assert!(item.brief.is_none());
                assert!(item.details.is_none());
                group = Some((item.name, RegisterGroup {
                    desc: item.desc.unwrap(),
                    regs: HashMap::new(),
                }));
            },
            DoxygenBlockType::Register => {
                if let Some((name, reg)) = register.take() {
                    group.as_mut().unwrap().1.regs.insert(name, reg);
                }
                register = Some((item.name, Register {
                    brief: item.brief,
                    details: item.details,
                    fields: HashMap::new(),
                }));
            },
            DoxygenBlockType::Field => {
                let s = itr.next().unwrap();
                let cap = field_re.captures(s).unwrap();

                // Reserved block
                let (lo, hi) = if cap[1].is_empty() {
                    (0, 32)
                } else {
                    let mut itr = cap[2].split(',');
                    itr.next().unwrap(); // Skip first term
                    let lo = itr.next().unwrap().parse().unwrap();
                    let size: usize = itr.next().unwrap().parse().unwrap();
                    (lo, lo + size)
                };
                assert!(item.desc.is_none());
                register.as_mut().unwrap().1.fields.insert(
                    item.name, Field {
                        brief: item.brief,
                        details: item.details,
                        lo, hi,
                    });
            }
            _ => panic!("Invalid block type"),
        }
    }
    target.unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("mesa-parse")
        .version("0.1")
        .author("Matt Keeter <matt@oxide.computer>")
        .about("Parses C headers from MESA to build a register map")
        .arg(Arg::with_name("root")
            .required(true)
            .help("Path to `mesa-v20xx...` folder")
            .takes_value(true))
        .arg(Arg::with_name("family")
            .required(true)
            .help("Chip family (e.g. `jaguar2`).  This may include a subfamily separated by `:`, e.g. `jaguar2:servalt`")
            .takes_value(true))
        .get_matches();

    let mesa_root = matches.value_of("root").unwrap();
    let family = matches.value_of("family").unwrap();

    let (family, subfamily) = if family.contains(':') {
        let mut iter = family.split(':');
        (iter.next().unwrap(), iter.next().unwrap())
    } else {
        (family, family)
    };
    let mut path = PathBuf::from(mesa_root);
    path.push("base");
    path.push(family);
    path.push(format!("vtss_{}_regs_common.h", subfamily));

    // Read the top-level target list (where a "target" is a collection of
    // register groups)
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let targets = parse_regs_common(&contents);
    println!("{:#x?}", targets);
    path.pop();

    let re = Regex::new(r"^(.*)(_[0-9]+)$").unwrap();
    let mut seen_targets = HashSet::new();
    let mut out = HashMap::new();
    for target in targets {
        // Strip trailing "_[0-9]+" from targets with multiple instances
        let base_target = if let Some(cap) = re.captures(&target.0) {
            cap[1].to_owned()
        } else {
            target.0.clone()
        };

        // Apply remappings based on inconsistent file naming
        let base_target = normalize_target(&base_target).to_owned();

        if seen_targets.insert(base_target.clone()) {
            path.push(format!(
                "vtss_{}_regs_{}.h",
                subfamily,
                base_target.to_lowercase()
            ));
            println!("Opening {:?}", path);
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            path.pop();
            out.insert(base_target, parse_regs_specific(&contents));
        }
    }
    println!("{:#x?}", out);
    Ok(())
}
