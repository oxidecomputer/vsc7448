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
    path.push("src");
    path.push("lib.rs");
    let mut file = File::create(&path)?;
    path.pop();

    writeln!(&mut file, "{}", header)?;
    writeln!(
        &mut file,
        "
#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod types;

// Top-level targets are stored in the tree as submodules
"
    )?;
    // Write all of the mods
    for name in target_list.keys() {
        writeln!(&mut file, "pub mod {};", name.to_lowercase())?;
    }

    // Write top-level targets
    write!(
        &mut file,
        "
pub struct Vsc7448 {{}}
impl Vsc7448 {{"
    )?;

    // Create the top-level Vsc7448 struct, which has static functions to
    // construct each kind of target
    for (name, (_, instances)) in target_list.iter() {
        let mut instances = instances.clone();
        instances.sort();
        match instances.len() {
            0 => panic!("Target {} has 0 instances", name),
            1 => write!(
                &mut file,
                "
    pub fn {0}() -> {0} {{
        {0}(0x{1:x})
    }}",
                name, instances[0].1
            )?,
            _ => {
                // Sanity-check that the instances are tightly packed in memory
                let delta = instances[1].1 - instances[0].1;
                for (i, instance) in instances.iter().enumerate() {
                    assert_eq!(instance.0, Some(i as u32));
                    assert_eq!(instance.1, instances[0].1 + delta * i as u32);
                }
                write!(
                    &mut file,
                    "
    pub fn {0}(index: u32) -> {0} {{
        assert!(index < {2});
        {0}(0x{1:x} + index * 0x{3:x})
    }}",
                    name,
                    instances[0].1,
                    instances.len(),
                    delta
                )?;
            }
        }
    }
    writeln!(&mut file, "\n}}")?;

    // Write top-level targets
    for (name, (remap, _instances)) in target_list.iter() {
        // Open and write a header to the target-specific file
        let mut path = PathBuf::from(dir);
        path.push("src");
        path.push(format!("{}.rs", name.to_lowercase()));
        let mut tfile = File::create(&path)?;
        path.pop();
        path.push(name.to_lowercase());
        if !path.exists() {
            std::fs::create_dir(path.clone())?;
        }

        writeln!(&mut tfile, "{}", header)?;
        writeln!(
            &mut tfile,
            "
use crate::types::RegisterAddress;

// Register groups are stored in the tree as submodules
"
        )?;

        for (gname, _group) in target_docs[remap].groups.iter() {
            writeln!(&mut tfile, "pub mod {};", gname.to_lowercase())?;
        }

        // Write the struct header
        writeln!(
            &mut file,
            "
/// {1}
pub struct {0}(u32);
impl {0} {{",
            name, target_docs[remap].desc
        )?;

        for (gname, group) in target_docs[remap].groups.iter() {
            path.push(format!("{}.rs", gname.to_lowercase()));
            let mut gfile = File::create(&path)?;
            path.pop();
            writeln!(&mut gfile, "{}", header)?;
            if !group.regs.is_empty() {
                writeln!(&mut gfile, "use derive_more::{{From, Into}};")?;
            }

            if group.addr.count > 1 {
                write!(
                    &mut file,
                    "
    pub fn {1}(&self, index: u32) -> {0}::{1} {{
        assert!(index < {3});
        {0}::{1}(self.0 + 0x{2:x} + index * 0x{4:x})
    }}",
                    name.to_lowercase(),
                    gname,
                    group.addr.base * 4,
                    group.addr.count,
                    group.addr.width,
                )?;
            } else {
                write!(
                    &mut file,
                    "
    pub fn {1}(&self) -> {0}::{1} {{
        {0}::{1}(self.0 + 0x{2:x})
    }}",
                    name.to_lowercase(),
                    gname,
                    group.addr.base * 4,
                )?;
            }

            write!(
                &mut tfile,
                "
/// {1}
pub struct {0}(pub(super) u32);
impl {0} {{",
                gname,
                group.desc.replace("\n", "\n/// ")
            )?;
            for (rname, reg) in group.regs.iter() {
                if reg.addr.count > 1 {
                    write!(
                        &mut tfile,
                        "
    pub fn {0}(&self, index: u32) -> RegisterAddress<{1}::{0}> {{
        assert!(index < {4});
        RegisterAddress::new(self.0 + 0x{2:x} + index * 0x{3:x})
    }}",
                        rname,
                        gname.to_lowercase(),
                        reg.addr.base * 4,
                        reg.addr.width * 4,
                        reg.addr.count
                    )?;
                } else {
                    write!(
                        &mut tfile,
                        "
    pub fn {0}(&self) -> RegisterAddress<{1}::{0}> {{
        RegisterAddress::new(self.0 + 0x{2:x})
    }}",
                        rname,
                        gname.to_lowercase(),
                        reg.addr.base * 4
                    )?;
                }
                writeln!(&mut gfile, "\n/// Register `{0}`", rname)?;
                if let Some(brief) = &reg.brief {
                    writeln!(&mut gfile, "///\n/// {}", brief.replace("\n", "\n/// "))?;
                }
                if let Some(details) = &reg.details {
                    writeln!(&mut gfile, "///\n/// {}", details.replace("\n", "\n/// "))?;
                }
                write!(
                    &mut gfile,
                    "#[derive(From, Into)]
pub struct {0}(u32);
impl {0} {{",
                    rname
                )?;
                assert!(!reg.fields.is_empty());
                for (fname, field) in reg.fields.iter() {
                    if let Some(brief) = &field.brief {
                        writeln!(
                            &mut gfile,
                            "\n    /// {}",
                            brief.replace("\n", "\n    /// ")
                        )?;
                    }
                    if let Some(details) = &field.details {
                        if field.brief.is_some() {
                            writeln!(&mut gfile, "\n    ///")?;
                        }
                        writeln!(
                            &mut gfile,
                            "\n    /// {}",
                            details.replace("\n", "\n    /// ")
                        )?;
                    }
                    // Write out bitfield access, tuned to avoid no-op shifts
                    // and mask operations (which would otherwise produce
                    // compiler warnings).
                    let shift = field.lo;
                    let mask = (((1u64 << field.hi) - 1) ^ ((1u64 << field.lo) - 1)) as u32;
                    match (shift, mask) {
                        (0, 0xFFFFFFFF) => write!(
                            &mut gfile,
                            "    pub fn {field}(&self) -> u32 {{
        self.0
    }}
    pub fn set_{field}(&mut self, value: u32) {{
        self.0 = value;
    }}",
                            field = fname.to_lowercase(),
                        )?,
                        (0, _) => write!(
                            &mut gfile,
                            "    pub fn {field}(&self) -> u32 {{
        self.0 & 0x{mask:x}
    }}
    pub fn set_{field}(&mut self, value: u32) {{
        assert!(value <= 0x{mask:x});
        self.0 &= !0x{mask:x};
        self.0 |= value;
    }}",
                            field = fname.to_lowercase(),
                            mask = mask,
                        )?,
                        (_, 0xFFFFFFFF) => panic!("Cannot have a full mask and non-zero shift"),
                        _ => write!(
                            &mut gfile,
                            "    pub fn {field}(&self) -> u32 {{
        (self.0 & 0x{mask:x}) >> {shift}
    }}
    pub fn set_{field}(&mut self, value: u32) {{
        let value = value << {shift};
        assert!(value <= 0x{mask:x});
        self.0 &= !0x{mask:x};
        self.0 |= value;
    }}",
                            field = fname.to_lowercase(),
                            shift = shift,
                            mask = mask
                        )?,
                    }
                }
                writeln!(&mut gfile, "\n}}")?;
            }
            writeln!(&mut tfile, "\n}}")?;
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

/// Prints lib.rs for the vsc7448_info crate
fn print_info_table(
    dir: &str,
    target_list: &TargetList,
    target_docs: &BTreeMap<String, OwnedTarget>,
    pages: &BTreeMap<String, Page<String>>,
) -> Result<(), std::io::Error> {
    let mut path = PathBuf::from(dir);
    path.push("src");
    path.push("lib.rs");
    let mut file = File::create(&path)?;

    write!(
        &mut file,
        "//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// This is an autogenerated file; do not edit by hand!
use std::collections::BTreeMap;
use lazy_static::lazy_static;
use vsc7448_types::{{Address, Field, Page, Register, RegisterGroup, Target}};
pub mod parse;
lazy_static! {{
    /// Maps from user-facing target name to a tuple of
    /// - Name in `TARGETS` map (which may be different!)
    /// - List of instances, as tuples of `(instance id, address)`.  If there is
    ///   only one instance of this target, then this is vec![(None, ADDRESS)].
    pub static ref MEMORY_MAP: BTreeMap<&'static str, (&'static str, Vec<(Option<u32>, u32)>)> = {{
        let mut out = BTreeMap::new();"
    )?;
    for (k, t) in target_list.iter() {
        write!(
            &mut file,
            "
        out.insert({:?}, ({:?}, vec![",
            k, t.0
        )?;
        for t in &t.1 {
            write!(&mut file, "({:?},{:#x}),", t.0, t.1)?;
        }
        write!(&mut file, "]));")?;
    }
    writeln!(
        &mut file,
        "
        return out;
    }};"
    )?;

    write!(
        &mut file,
        "
    /// Maps from target name to `Target`, which contains a hierarchy
    /// of register groups, registers, and fields.
    pub static ref TARGETS: BTreeMap<&'static str, Target> = {{
        let mut out = BTreeMap::new();"
    )?;
    // Iteration over targets
    for (k, t) in target_docs.iter() {
        write!(
            &mut file,
            "
        let {}groups = BTreeMap::new();",
            if t.groups.is_empty() { "" } else { "mut " }
        )?;

        // Iteration over register groups
        for (k, t) in t.groups.iter() {
            write!(
                &mut file,
                "
        let {}regs = BTreeMap::new();",
                if t.regs.is_empty() { "" } else { "mut " }
            )?;

            // Iteration over registers
            for (k, t) in t.regs.iter() {
                write!(
                    &mut file,
                    "
        let mut fields = BTreeMap::new();"
                )?;

                // Iteration over fields
                for (k, t) in t.fields.iter() {
                    if t.hi > 32 {
                        panic!("Invalid hi bit for {:?}", t);
                    }
                    write!(
                        &mut file,
                        "
        fields.insert({:?}, {:?});",
                        k, t
                    )?;
                }
                write!(
                    &mut file,
                    "
        regs.insert({:?}, Register {{ addr: {:?}, brief: {:?}, details: {:?}, fields }});",
                    k, t.addr, t.brief, t.details
                )?;
            }
            write!(
                &mut file,
                "
        groups.insert({:?}, RegisterGroup {{ addr: {:?}, desc: {:?}, regs }});",
                k, t.addr, t.desc
            )?;
        }
        write!(
            &mut file,
            "
        out.insert({:?}, Target {{ desc: {:?}, groups }});",
            k, t.desc
        )?;
    }
    writeln!(
        &mut file,
        "
        return out;
    }};"
    )?;

    write!(
        &mut file,
        "
    /// Maps from PHY page name to `Page`, which contains a hierarchy
    /// of registers containing bit fields.
    pub static ref PHY_MAP: BTreeMap<&'static str, Page<&'static str>> = {{
        let mut out = BTreeMap::new();"
    )?;

    // Iteration over pages
    let mut keys = pages.keys().collect::<Vec<_>>();
    keys.sort_by_key(|k| (pages[k.as_str()].base, k.as_str()));
    for k in keys {
        let t = &pages[k];
        write!(
            &mut file,
            "
        let {}regs = BTreeMap::new();",
            if t.regs.is_empty() { "" } else { "mut " }
        )?;
        let mut keys = t.regs.keys().collect::<Vec<_>>();
        keys.sort_by_key(|k| (t.regs[k.as_str()].addr.base, k.as_str()));

        // Iteration over registers
        for k in keys {
            let t = &t.regs[k];
            write!(
                &mut file,
                "
        let {}fields = BTreeMap::new();",
                if t.fields.is_empty() { "" } else { "mut " }
            )?;
            let mut keys = t.fields.keys().collect::<Vec<_>>();
            keys.sort_by_key(|k| (t.fields[k.as_str()].lo, k.as_str()));

            // Iteration over fields
            for k in keys {
                let t = &t.fields[k];
                if t.hi > 32 {
                    panic!("Invalid hi bit for {:?}", t);
                }
                write!(
                    &mut file,
                    "
        fields.insert({:?}, {:?});",
                    k, t
                )?;
            }
            if t.addr.base > 31 {
                panic!("Invalid register address for {:?}", t);
            }
            write!(
                &mut file,
                "
        regs.insert({:?}, Register {{ addr: {:?}, brief: {:?}, details: {:?}, fields }});",
                k, t.addr, t.brief, t.details
            )?;
        }

        write!(
            &mut file,
            "
        out.insert({:?}, Page {{ desc: {:?}, base: {}, regs }});",
            k, t.desc, t.base,
        )?;
    }
    writeln!(
        &mut file,
        "
        return out;
    }};
}}"
    )?;
    Ok(())
}
