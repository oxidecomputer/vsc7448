use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{App, Arg};

mod doxygen;
mod symregs;

use doxygen::parse_regs_doxygen;
use symregs::parse_symregs;

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
    path.push(format!("vtss_{}_symregs.c", subfamily));
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (target_data, target_list) = parse_symregs(&contents);
    path.pop();

    let mut seen_targets = HashSet::new();
    let mut out = HashMap::new();
    for target in target_list {
        let base_target = target.1.0;

        if seen_targets.insert(base_target.clone()) {
            path.push(format!(
                "vtss_{}_regs_{}.h",
                subfamily,
                base_target.to_lowercase()
            ));
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
