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
use vsc7448_types::{OwnedTarget, Page, Register};

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
        write_pac_lib(&header, pac, &target_list, &target_docs, &pages)?;
    }

    if let Some(info) = matches.value_of("info") {
        write_info_table(info, &target_list, &target_docs, &pages)?;
    }

    if matches.value_of("info").is_none() && matches.value_of("pac").is_none() {
        eprintln!("--info or --pac required");
    }

    Ok(())
}

fn best_integer_fit(i: usize) -> &'static str {
    if i <= usize::from(u8::MAX) {
        "u8"
    } else if i <= usize::from(u16::MAX) {
        "u16"
    } else if i <= usize::try_from(u32::MAX).unwrap() {
        "u32"
    } else {
        panic!("Invalid integer: {}", i);
    }
}

/// Prints `lib.rs` for the vsc7448_pac crate
fn write_pac_lib(
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
pub mod phy;

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
    #[inline(always)]
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
                let t = best_integer_fit(instances.len());
                write!(
                    &mut file,
                    "
    #[inline(always)]
    pub fn {0}(index: {4}) -> {0} {{
        assert!(index < {2});
        {0}(0x{1:x} + {5} * 0x{3:x})
    }}",
                    name,
                    instances[0].1,
                    instances.len(),
                    delta,
                    t,
                    if t == "u32" {
                        "index"
                    } else {
                        "u32::from(index)"
                    },
                )?;
            }
        }
    }
    writeln!(&mut file, "\n}}")?;

    // Write top-level targets
    for (name, (remap, instances)) in target_list.iter() {
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
impl {0} {{
    pub const BASE: u32 = 0x{2:x};",
            name, target_docs[remap].desc, instances[0].1,
        )?;
        if instances.len() > 1 {
            let delta = instances[1].1 - instances[0].1;
            writeln!(&mut file, "    pub const SIZE: u32 = 0x{:x};", delta)?;
        }
        writeln!(
            &mut file,
            "    pub fn from_raw_unchecked_address(a: u32) -> Self {{
        Self(a)
    }}"
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
                let t = best_integer_fit(group.addr.count.try_into().unwrap());
                write!(
                    &mut file,
                    "
    #[inline(always)]
    pub fn {1}(&self, index: {5}) -> {0}::{1} {{
        assert!(index < {3});
        {0}::{1}(self.0 + 0x{2:x} + {6} * 0x{4:x})
    }}",
                    name.to_lowercase(),
                    gname,
                    group.addr.base * 4,
                    group.addr.count,
                    group.addr.width * 4,
                    t,
                    if t == "u32" {
                        "index"
                    } else {
                        "u32::from(index)"
                    }
                )?;
            } else {
                write!(
                    &mut file,
                    "
    #[inline(always)]
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
                    let t = best_integer_fit(reg.addr.count.try_into().unwrap());
                    write!(
                        &mut tfile,
                        "
    #[inline(always)]
    pub fn {0}(&self, index: {5}) -> RegisterAddress<{1}::{0}> {{
        assert!(index < {4});
        RegisterAddress::new(self.0 + 0x{2:x} + {6} * 0x{3:x})
    }}",
                        rname,
                        gname.to_lowercase(),
                        reg.addr.base * 4,
                        reg.addr.width * 4,
                        reg.addr.count,
                        t,
                        if t == "u32" {
                            "index"
                        } else {
                            "u32::from(index)"
                        }
                    )?;
                } else {
                    write!(
                        &mut tfile,
                        "
    #[inline(always)]
    pub fn {0}(&self) -> RegisterAddress<{1}::{0}> {{
        RegisterAddress::new(self.0 + 0x{2:x})
    }}",
                        rname,
                        gname.to_lowercase(),
                        reg.addr.base * 4
                    )?;
                }
                write_reg(&mut gfile, rname, "u32", reg)?;
            }
            writeln!(&mut tfile, "\n}}")?;
        }
        writeln!(&mut file, "\n}}")?;
    }
    writeln!(&mut file)?;

    // Now, write the Phy registers
    let mut path = PathBuf::from(dir);
    path.push("src");
    path.push("phy.rs");
    let mut file = File::create(&path)?;
    path.pop();
    path.push("phy/");
    if !path.exists() {
        std::fs::create_dir(path.clone())?;
    }

    // Special case for 1588, which isn't a valid identifier
    let get_name = |pname: &str| -> String {
        if pname.parse::<usize>().is_ok() {
            format!("PAGE_{}", pname)
        } else {
            pname.to_string()
        }
    };
    writeln!(&mut file, "{}", header)?;
    writeln!(
        &mut file,
        "
use crate::types::PhyRegisterAddress;
"
    )?;
    for pname in pages.keys() {
        writeln!(&mut file, "pub mod {};", get_name(pname).to_lowercase())?;
    }

    let mut pages = pages.iter().collect::<Vec<_>>();
    pages.sort_by_key(|r| r.1.base);
    for (pname, page) in &pages {
        writeln!(&mut file, "\n/// {}", page.desc)?;
        let pname = get_name(pname);
        write!(
            &mut file,
            "pub struct {pname} {{}}
impl {pname} {{",
            pname = pname
        )?;

        path.push(format!("{}.rs", pname.to_lowercase()));
        let mut pfile = File::create(&path)?;
        path.pop();
        writeln!(&mut pfile, "{}", header)?;
        writeln!(&mut pfile, "use derive_more::{{From, Into}};")?;

        let mut regs = page.regs.iter().collect::<Vec<_>>();
        regs.sort_by_key(|r| r.1.addr.base);
        for (rname, reg) in &regs {
            let rname = if rname.chars().next().unwrap().is_numeric() {
                format!("REG_{}", rname)
            } else {
                rname.to_string()
            };
            write_reg(&mut pfile, &rname, "u16", reg)?;

            assert!(reg.addr.count == 1);
            write!(
                &mut file,
                "
    #[inline(always)]
    pub fn {0}() -> PhyRegisterAddress<{1}::{0}> {{
        PhyRegisterAddress::new({2}, {3})
    }}",
                rname,
                pname.to_lowercase(),
                page.base,
                reg.addr.base
            )?;
        }
        writeln!(&mut file, "}}")?; // end of impl block
    }

    // Run rustfmt on the result
    let mut path = PathBuf::from(dir);
    path.push("Cargo.toml");
    std::process::Command::new("cargo")
        .arg("fmt")
        .arg("--manifest-path")
        .arg(path)
        .output()
        .expect("failed to execute process");

    Ok(())
}

fn write_reg<W: std::io::Write>(
    gfile: &mut W,
    rname: &str,
    inttype: &str,
    reg: &Register<String>,
) -> Result<(), std::io::Error> {
    if let Some(brief) = &reg.brief {
        writeln!(gfile, "/// {}", brief.replace("\n", "\n/// "))?;
    }
    if let Some(details) = &reg.details {
        if reg.brief.is_some() {
            writeln!(gfile, "///")?;
        }
        writeln!(gfile, "/// {}", details.replace("\n", "\n/// "))?;
    }
    // If the register has no bitfields, then treat it as a simple wrapper
    // type; otherwise, make the inner u32 private and force the user to
    // interact with it through getter/setter functions.
    write!(
        gfile,
        "#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct {}({}{});",
        rname,
        if reg.fields.is_empty() { "pub " } else { "" },
        inttype,
    )?;
    if !reg.fields.is_empty() {
        write!(gfile, "\nimpl {0} {{", rname)?;
    }
    for (fname, field) in reg.fields.iter() {
        if inttype == "u16" && (field.lo >= 16 || field.hi > 16) {
            eprintln!(
                "Invalid field for PHY {}:{}: {:?}; skipping",
                rname, fname, field
            );
            continue;
        }

        writeln!(gfile)?;
        if let Some(brief) = &field.brief {
            writeln!(gfile, "    /// {}", brief.replace("\n", "\n    /// "))?;
        }
        if let Some(details) = &field.details {
            if field.brief.is_some() {
                writeln!(gfile, "    ///")?;
            }
            writeln!(gfile, "    /// {}", details.replace("\n", "\n    /// "))?;
        }
        // Write out bitfield access, tuned to avoid no-op shifts
        // and mask operations (which would otherwise produce
        // compiler warnings).
        let shift = field.lo;
        let mask = (((1u64 << field.hi) - 1) ^ ((1u64 << field.lo) - 1)) as u32;
        let max = (1u32 << (field.hi - field.lo)) - 1;

        // Special case for fields which aren't valid Rust identifiers
        let get = if fname == "LOOP" || fname.chars().next().unwrap().is_numeric() {
            "get_"
        } else {
            ""
        };
        match (shift, mask) {
            (0, 0xFFFFFFFF) => write!(
                gfile,
                "    #[inline(always)]
    pub fn {get}{field}(&self) -> {t} {{
        self.0
    }}
    #[inline(always)]
    pub fn set_{field}(&mut self, value: {t}) {{
        self.0 = value;
    }}",
                get = get,
                field = fname.to_lowercase(),
                t = inttype
            )?,
            (0, _) => write!(
                gfile,
                "    #[inline(always)]
    pub fn {get}{field}(&self) -> {t} {{
        self.0 & 0x{mask:x}
    }}
    #[inline(always)]
    pub fn set_{field}(&mut self, value: {t}) {{
        assert!(value <= 0x{mask:x});
        self.0 &= !0x{mask:x};
        self.0 |= value;
    }}",
                get = get,
                field = fname.to_lowercase(),
                mask = mask,
                t = inttype,
            )?,
            (_, 0xFFFFFFFF) => panic!("Cannot have a full mask and non-zero shift"),
            _ => write!(
                gfile,
                "    #[inline(always)]
    pub fn {get}{field}(&self) -> {t} {{
        (self.0 & 0x{mask:x}) >> {shift}
    }}
    #[inline(always)]
    pub fn set_{field}(&mut self, value: {t}) {{
        assert!(value <= 0x{max:x});
        let value = value << {shift};
        self.0 &= !0x{mask:x};
        self.0 |= value;
    }}",
                get = get,
                field = fname.to_lowercase(),
                max = max,
                shift = shift,
                mask = mask,
                t = inttype,
            )?,
        }
    }
    if !reg.fields.is_empty() {
        write!(gfile, "\n}}")?;
    }
    writeln!(gfile)?;
    Ok(())
}

fn write_info_table(
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
