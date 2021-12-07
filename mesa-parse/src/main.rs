//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::{App, Arg};
use vsc7448_types::{OwnedTarget, Page};

mod doxygen;
mod phy;
mod symregs;

use doxygen::parse_regs_doxygen;
use phy::{parse_phy_pages, parse_phy_registers};
use symregs::{parse_symregs, TargetList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("mesa-parse")
        .version("0.1")
        .author("Matt Keeter <matt@oxide.computer>")
        .about("Parses C headers from MESA to build a register map")
        .arg(Arg::with_name("root")
            .required(true)
            .help("Path to `mesa-v20xx...` folder")
            .takes_value(true))
        .arg(Arg::with_name("pac")
            .long("pac")
            .help("Root directory of the vs7488-pac crate")
            .takes_value(true))
        .arg(Arg::with_name("info")
            .long("info")
            .help("Root directory of the vs7488-info crate")
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
    let mesa_version = path.file_name().unwrap().to_str().unwrap().to_owned();
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
    let mut target_docs = BTreeMap::new();
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
            let docs = parse_regs_doxygen(&contents, &target_data[&base_target]);
            target_docs.insert(base_target, docs);
        }
    }

    // Then, handle the PHY registers
    let mut path = PathBuf::from(mesa_root);
    path.push("include");
    path.push("vtss_phy_api.h");

    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut pages = parse_phy_pages(&contents);

    let mut path = PathBuf::from(mesa_root);
    path.push("base");
    path.push("phy");
    path.push("phy_1g");
    path.push("vtss_phy.h");
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_phy_registers(&contents, &mut pages);

    let header = format!(
        r#"// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Based on {} (https://github.com/microchip-ung/mesa/) which has
// the following copyright and license:
//
// Copyright (c) 2004-2021 Microchip Technology Inc. and its subsidiaries.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// This is an autogenerated file; do not edit by hand!"#,
        mesa_version
    );
    if let Some(pac) = matches.value_of("pac") {
        print_pac_lib(&header, pac, &target_list, &target_docs, &pages)?;
    }

    if let Some(info) = matches.value_of("info") {
        print_info_table(info, &target_list, &target_docs, &pages)?;
    }

    if matches.value_of("info").is_none() && matches.value_of("pac").is_none() {
        eprintln!("--info or --pac required");
    }

    Ok(())
}

/// Prints `lib.rs` for the vsc7448_pac crate
fn print_pac_lib(
    header: &str,
    dir: &str,
    target_list: &TargetList,
    target_docs: &BTreeMap<String, OwnedTarget>,
    pages: &BTreeMap<String, Page<String>>,
) -> Result<(), std::io::Error> {
    let mut path = PathBuf::from(dir);
    let folder = path.file_name().unwrap().to_str().unwrap().to_owned();
    path.push("src");
    path.push("lib.rs");
    let mut file = File::create(&path)?;

    write!(&mut file, "{}", header)?;
    writeln!(&mut file, "use crate::RegisterAddress;")?;

    // Write top-level targets
    for (name, (remap, instances)) in target_list.iter() {
        write!(
            &mut file,
            "
#[allow(non_camel_case_types)]
#[derive(Default)]
pub struct {0}();
impl {0} {{
    pub fn addr(&self{1}) -> u32 {{
",
            name,
            if instances.len() > 1 {
                ", index: u32"
            } else {
                ""
            }
        )?;
        let mut instances = instances.clone();
        instances.sort();
        match instances.len() {
            0 => panic!("Target with no instances"),
            1 => {
                assert!(instances[0].0.is_none());
                writeln!(&mut file, "        0x{:x}", instances[0].1)?;
            }
            _ => {
                // Sanity-check that the instances are tightly packed in memory
                let delta = instances[1].1 - instances[0].1;
                for (i, instance) in instances.iter().enumerate() {
                    assert_eq!(instance.0, Some(i as u32));
                    assert_eq!(instance.1, instances[0].1 + delta * i as u32);
                }
                writeln!(
                    &mut file,
                    "        0x{:x} + 0x{:x} * index",
                    instances[0].1, delta
                )?;
            }
        }
        write!(
            &mut file,
            "    }}
}}"
        )?;
    }
    writeln!(&mut file)?;

    Ok(())
}

fn print_info_table(
    dir: &str,
    target_list: &TargetList,
    target_docs: &BTreeMap<String, OwnedTarget>,
    pages: &BTreeMap<String, Page<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(dir);
    path.push("src");
    path.push("vsc7448.postcard");
    let mut out_memory_map = BTreeMap::new();
    for (k, t) in target_list {
        out_memory_map.insert(k.clone(), (t.0.clone(), t.1.clone()));
    }

    let mut out_targets = BTreeMap::new();
    // Iteration over targets
    for (k, t) in target_docs {
        let mut groups = BTreeMap::new();

        // Iteration over register groups
        for (k, t) in &t.groups {
            let mut regs = BTreeMap::new();
            // Iteration over registers
            for (k, t) in &t.regs {
                let mut fields = BTreeMap::new();
                // Iteration over fields
                for (k, t) in &t.fields {
                    if t.hi > 32 {
                        panic!("Invalid hi bit for {:?}", t);
                    }
                    fields.insert(k.clone(), t.clone());
                }
                regs.insert(
                    k.clone(),
                    vsc7448_types::Register {
                        addr: t.addr,
                        brief: t.brief.clone(),
                        details: t.details.clone(),
                        fields,
                    },
                );
            }
            groups.insert(
                k.clone(),
                vsc7448_types::RegisterGroup {
                    addr: t.addr,
                    desc: t.desc.clone(),
                    regs,
                },
            );
        }
        out_targets.insert(
            k.clone(),
            vsc7448_types::OwnedTarget {
                desc: t.desc.clone(),
                groups,
            },
        );
    }

    let mut out_phy_map = BTreeMap::new();

    // Iteration over pages
    let mut keys = pages.keys().collect::<Vec<_>>();
    keys.sort_by_key(|k| (pages[k.as_str()].base, k.as_str()));
    for k in keys {
        let t = &pages[k];
        let mut regs = BTreeMap::new();
        let mut keys = t.regs.keys().collect::<Vec<_>>();
        keys.sort_by_key(|k| (t.regs[k.as_str()].addr.base, k.as_str()));

        // Iteration over registers
        for k in keys {
            let t = &t.regs[k];
            let mut fields = BTreeMap::new();
            let mut keys = t.fields.keys().collect::<Vec<_>>();
            keys.sort_by_key(|k| (t.fields[k.as_str()].lo, k.as_str()));

            // Iteration over fields
            for k in keys {
                let t = &t.fields[k];
                if t.hi > 32 {
                    panic!("Invalid hi bit for {:?}", t);
                }
                fields.insert(k.clone(), t.clone());
            }
            if t.addr.base > 31 {
                panic!("Invalid register address for {:?}", t);
            }
            regs.insert(
                k.clone(),
                vsc7448_types::Register {
                    addr: t.addr,
                    brief: t.brief.clone(),
                    details: t.details.clone(),
                    fields,
                },
            );
        }

        out_phy_map.insert(
            k.clone(),
            vsc7448_types::Page {
                desc: t.desc.clone(),
                base: t.base,
                regs,
            },
        );
    }

    #[derive(serde::Serialize)]
    struct Contents {
        memory_map: BTreeMap<String, (String, Vec<(Option<u32>, u32)>)>,
        targets: BTreeMap<String, vsc7448_types::OwnedTarget>,
        phy_map: BTreeMap<String, vsc7448_types::Page<String>>,
    }
    let contents = Contents {
        memory_map: out_memory_map,
        targets: out_targets,
        phy_map: out_phy_map,
    };

    let mut file = File::create(&path)?;
    let v = postcard::to_stdvec(&contents)?;
    file.write_all(&v)?;
    Ok(())
}
