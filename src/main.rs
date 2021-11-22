use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{App, Arg};
use regex::Regex;

mod doxygen;
mod common;

use common::parse_regs_common;
use doxygen::parse_regs_doxygen;

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
pub struct Field {
    brief: Option<String>,
    details: Option<String>,
    lo: usize,
    hi: usize,
}
#[derive(Debug)]
pub struct Register {
    brief: Option<String>,
    details: Option<String>,
    fields: HashMap<String, Field>
}
#[derive(Debug)]
pub struct RegisterGroup {
    desc: String,
    regs: HashMap<String, Register>,
}
#[derive(Debug)]
pub struct Target {
    desc: String,
    groups: HashMap<String, RegisterGroup>,
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
            out.insert(base_target, parse_regs_doxygen(&contents));
        }
    }
    println!("{:#x?}", out);
    Ok(())
}
