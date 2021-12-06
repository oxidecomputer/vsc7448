//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use regex::Regex;
use std::collections::BTreeMap;

use vsc7448_types::Address;

// Represents a memory map for a particular Target
// This map is indexed as target_map[register_group].1[register_name]
pub type TargetMap = BTreeMap<String, (Address, BTreeMap<String, Address>)>;

// Represents the layout of Targets in memory.
pub type TargetList = BTreeMap<String, (String, Vec<(Option<usize>, usize)>)>;

type MemoryMap = (BTreeMap<String, TargetMap>, TargetList);

pub fn parse_symregs(s: &str) -> MemoryMap {
    let regs_within_re =
        Regex::new(r"^static const vtss_symreg_reg_t (regs_within_[A-Z0-9_]+)\[\] = \{$").unwrap();
    let reggrp_within_re =
        Regex::new(r"^static const vtss_symreg_reggrp_t reggrps_within_([A-Z0-9_]+)\[\] = \{$")
            .unwrap();
    let reg_re = Regex::new(
        r#"^\s*\{\s*([A-Z0-9_"]+)\s*,\s*([0-9xa-zA-Z]+)\s*,\s*([0-9xa-zA-Z]+)\s*,\s*([0-9xa-zA-Z]+)\s*\}"#)
        .unwrap();
    let reggrp_re = Regex::new(
        r#"^\s*\{\s*([A-Z0-9_"]+)\s*,\s*([0-9xa-zA-Z]+)\s*,\s*([0-9xa-zA-Z]+)\s*,\s*([0-9xa-zA-Z]+)\s*,\s*(regs_within_[A-Z0-9_]+)\s*\}"#)
        .unwrap();
    let target_re = Regex::new(
        r#"^\s*\{\s*([A-Z0-9_"]+)\s*,\s*([\-0-9]+)\s*,\s*(0x[[:xdigit:]]+)\s*,\s*VTSS_IO_OFFSET([0-9]+)\(([0-9xa-zA-Z]+)\)\s*,\s*reggrps_within_([A-Z0-9_]+)\s*\}"#)
        .unwrap();

    let offset_re = Regex::new(r"^#define VTSS_IO_ORIGIN([0-9]+)_OFFSET\s+(0x[[:xdigit:]]+)$")
        .expect("failed to build regex");
    let mut offsets = BTreeMap::new();

    // (Register group name, map of register name -> Address)
    let mut active_regs: Option<(String, BTreeMap<String, Address>)> = None;
    let mut known_regs = BTreeMap::new();

    // (Target name, map of register group -> (Address, Group map))
    let mut active_target: Option<(String, TargetMap)> = None;

    // Map from target name to target memory map
    let mut known_targets = BTreeMap::new();
    let mut target_list = BTreeMap::new();

    for s in s.lines() {
        // When a block ends, finalize it
        if s.trim().starts_with("};") {
            if let Some(r) = active_target.take() {
                assert!(active_regs.is_none());
                known_targets.insert(r.0, r.1);
                known_regs.clear();
            }
            if let Some(r) = active_regs.take() {
                known_regs.insert(r.0, r.1);
            }
        }

        if let Some(cap) = offset_re.captures(s) {
            offsets.insert(cap[1].to_owned(), parse_int::parse(&cap[2]).unwrap());
        }

        if let Some(caps) = regs_within_re.captures(s) {
            assert!(active_regs.is_none());
            active_regs = Some((caps[1].to_owned(), BTreeMap::new()));
        }
        if let Some(caps) = reggrp_within_re.captures(s) {
            assert!(active_target.is_none());
            active_target = Some((caps[1].to_owned(), BTreeMap::new()));
        }

        // For each register within a group, insert it into active_regs
        if let Some(caps) = reg_re.captures(s) {
            if &caps[1] == "NULL" {
                continue;
            }
            let name = caps[1].trim_matches('\"');
            let base = parse_int::parse(&caps[2]).unwrap();
            let count = parse_int::parse(&caps[3]).unwrap();
            let width = parse_int::parse(&caps[4]).unwrap();
            active_regs
                .as_mut()
                .unwrap()
                .1
                .insert(name.to_string(), Address { base, count, width });
        }

        // For each register group within a target, insert it into active_target
        if let Some(caps) = reggrp_re.captures(s) {
            if &caps[1] == "NULL" {
                continue;
            }
            let name = caps[1].trim_matches('\"');
            let base = parse_int::parse(&caps[2]).unwrap();
            let count = parse_int::parse(&caps[3]).unwrap();
            let width = parse_int::parse(&caps[4]).unwrap();
            let regs = &caps[5];
            active_target.as_mut().unwrap().1.insert(
                name.to_string(),
                (
                    Address { base, count, width },
                    known_regs.remove(regs).unwrap(),
                ),
            );
        }

        if let Some(caps) = target_re.captures(s) {
            let name = caps[1].trim_matches('\"').to_owned();
            let repl: i32 = caps[2].parse().unwrap();
            let addr = parse_int::parse::<usize>(&caps[5]).unwrap();
            let entry = target_list
                .entry(name)
                .or_insert((caps[6].to_owned(), Vec::new()));
            assert!(entry.0 == caps[6]);
            let repl = if repl == -1 {
                None
            } else {
                Some(repl as usize)
            };
            entry.1.push((repl, addr + offsets.get(&caps[4]).unwrap()));
        }
    }

    (known_targets, target_list)
}
