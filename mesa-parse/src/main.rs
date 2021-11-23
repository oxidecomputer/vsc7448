use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{App, Arg};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::Serialize;

mod doxygen;
mod symregs;

use doxygen::parse_regs_doxygen;
use symregs::parse_symregs;

#[derive(Debug, Serialize)]
pub struct Field {
    brief: Option<String>,
    details: Option<String>,
    lo: usize,
    hi: usize,
}
#[derive(Debug, Serialize)]
pub struct Register {
    addr: Address,
    brief: Option<String>,
    details: Option<String>,
    fields: HashMap<String, Field>,
}
#[derive(Debug, Serialize)]
pub struct RegisterGroup {
    addr: Address,
    desc: String,
    regs: HashMap<String, Register>,
}
#[derive(Debug, Serialize)]
pub struct Target {
    desc: String,
    groups: HashMap<String, RegisterGroup>,
}
#[derive(Copy, Clone, Debug, Serialize)]
pub struct Address {
    base: usize,
    count: usize,
    width: usize,
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

    // Parse the symregs file first, since that gives us all of our target file
    // names for doxygen parsing
    let mut path = PathBuf::from(mesa_root);
    path.push("base");
    path.push(family);
    path.push(format!("vtss_{}_symregs.c", subfamily));
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (target_data, target_list) = parse_symregs(&contents);
    path.pop();

    // Then, parse each target-specific file
    let mut seen_targets = HashSet::new();
    let mut out = HashMap::new();
    for target in &target_list {
        let base_target = target.1 .0.clone();

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
            let docs = parse_regs_doxygen(&contents, target_data.get(&base_target).unwrap());
            out.insert(base_target, docs);
        }
    }

    println!(
        "{}",
        to_string_pretty(&(out, target_list), PrettyConfig::new()).unwrap()
    );
    Ok(())
}
