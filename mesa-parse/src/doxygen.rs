//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use regex::Regex;
use std::collections::BTreeMap;

use crate::symregs::TargetMap;
use vsc7448_types::{Field, OwnedTarget, Register, RegisterGroup};

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
                    }
                    State::Details => {
                        assert!(out.details.is_none());
                        out.details = Some(prev_accum);
                    }
                    State::Desc => {
                        assert!(out.desc.is_none());
                        out.desc = Some(prev_accum);
                    }
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
            }
            State::Details => {
                assert!(out.details.is_none());
                out.details = Some(accum);
            }
            State::Desc => {
                assert!(out.desc.is_none());
                out.desc = Some(accum);
            }
        };
    }
    out
}

// Horrifying code to parse a vtss_*_regs_*.h file
pub fn parse_regs_doxygen(s: &str, map: &TargetMap) -> OwnedTarget {
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
                target = Some(OwnedTarget {
                    desc: item.desc.unwrap(),
                    groups: BTreeMap::new(),
                });
            }
            DoxygenBlockType::RegisterGroup => {
                if let Some((name, group)) = group.take() {
                    target.as_mut().unwrap().groups.insert(name, group);
                }
                assert!(item.brief.is_none());
                assert!(item.details.is_none());
                let addr = map.get(&item.name).unwrap().0;
                group = Some((
                    item.name,
                    RegisterGroup {
                        addr,
                        desc: item.desc.unwrap(),
                        regs: BTreeMap::new(),
                    },
                ));
            }
            DoxygenBlockType::Register => {
                if let Some((name, reg)) = register.take() {
                    group.as_mut().unwrap().1.regs.insert(name, reg);
                }
                let addr = *map
                    .get(&group.as_ref().unwrap().0)
                    .unwrap()
                    .1
                    .get(&item.name)
                    .unwrap();
                register = Some((
                    item.name,
                    Register {
                        addr,
                        brief: item.brief,
                        details: item.details,
                        fields: BTreeMap::new(),
                    },
                ));
            }
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
                    let size: u8 = itr.next().unwrap().parse().unwrap();
                    (lo, lo + size)
                };
                assert!(item.desc.is_none());
                register.as_mut().unwrap().1.fields.insert(
                    item.name,
                    Field {
                        brief: item.brief,
                        details: item.details,
                        lo,
                        hi,
                    },
                );
            }
            _ => panic!("Invalid block type"),
        }
    }
    // Finalize pending parse
    if let Some((name, reg)) = register.take() {
        group.as_mut().unwrap().1.regs.insert(name, reg);
    }
    if let Some((name, group)) = group.take() {
        target.as_mut().unwrap().groups.insert(name, group);
    }
    target.unwrap()
}
