//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

//! Types for converting user-provided strings into registers at runtime.
use regex::Regex;
use thiserror::Error;

use std::collections::HashMap;

use crate::{MEMORY_MAP, TARGETS};
use vsc7448_types::Field;

/// One level of hierarchy within a fully qualified register, with a name and
/// optional index (for targets / groups / registers that have multiple
/// instances)
#[derive(Debug, PartialEq, Eq)]
pub struct Indexed<'a> {
    /// Name of the target, register group, or register
    name: &'a str,
    /// Index of the item within an array
    index: Option<usize>,
}
impl<'a> std::fmt::Display for Indexed<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.index {
            None => write!(f, "{}", self.name),
            Some(i) => write!(f, "{}[{}]", self.name, i),
        }
    }
}

/// Fully qualified register
#[derive(Debug, PartialEq, Eq)]
pub struct TargetRegister<'a> {
    target: Indexed<'a>,
    group: Indexed<'a>,
    reg: Indexed<'a>,
}

impl<'a> std::fmt::Display for TargetRegister<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.target, self.group, self.reg)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ParseError {
    #[error("Regex match failed")]
    MatchFailed,
    #[error("Too many items in name (should be TARGET:GROUP:REG)")]
    TooManyItems,
    #[error("No register with that name")]
    NoSuchRegister,
    #[error("Multiple registers with this name; specify target or group")]
    AmbiguousRegister,
    #[error("Invalid target index: {0:?}")]
    InvalidTargetIndex(Option<usize>),
    #[error("Register is part of an array and requires an [index]")]
    RegisterArray,
    #[error("Register index specified for a non-array register")]
    NotRegisterArray,
    #[error("Register index is out of range ({0} >= {1})")]
    InvalidRegisterIndex(usize, usize),
    #[error("Register group is part of an array and requires an [index]")]
    RegisterGroupArray,
    #[error("Register group index specified for a non-array register group")]
    NotRegisterGroupArray,
    #[error("Register group index is out of range ({0} >= {1})")]
    InvalidRegisterGroupIndex(usize, usize),
}

impl<'a> TargetRegister<'a> {
    /// Returns the physical memory address of the given address
    pub fn address(&self) -> u32 {
        let target = &MEMORY_MAP[self.target.name];
        let (_, mut addr) = target.1.iter().find(|t| t.0 == self.target.index)
            .unwrap();

        let target = &TARGETS[target.0];
        let group = &target.groups[self.group.name];
        addr += (group.addr.base + group.addr.width * self.group.index.unwrap_or(0)) * 4;

        let reg = &group.regs[self.reg.name];
        addr += (reg.addr.base + reg.addr.width * self.reg.index.unwrap_or(0)) * 4;

        addr.try_into().expect("Address exceeds 32-bit space?!")
    }
    /// Returns this register's field map.
    pub fn fields(&self) -> &HashMap<&'static str, Field<&'static str>> {
        &TARGETS[MEMORY_MAP[self.target.name].0]
            .groups[self.group.name]
            .regs[self.reg.name]
            .fields
    }
}

impl<'a> std::str::FromStr for TargetRegister<'a> {
    type Err = ParseError;
    /// Parses a string into a qualified register.  This is very flexible,
    /// accepting strings of the format
    /// - `REG_NAME`
    /// - `REG_NAME[index]`
    /// - `TARGET[index]:GROUP[index]:REGISTER[index]`
    /// - And more!
    ///
    /// This will raise an error if the parse is invalid or ambiguous, based
    /// on the target and register maps.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([A-Z_0-9]+)(\[([0-9]+)\])?$").unwrap();
        let words = s.split(':').collect::<Vec<&str>>();

        let out = match words.len() {
            // Just a register name, potentially qualified with an index
            1 => {
                let cap = re.captures(words[0]).ok_or(ParseError::MatchFailed)?;
                let name = &cap[1];
                let index = cap.get(3).map(|i| i.as_str().parse::<usize>().unwrap());

                let mut iter = MEMORY_MAP
                    .iter()
                    .flat_map(|(i, target)|
                        TARGETS[target.0]
                            .groups
                            .iter()
                            .map(move |(j, group)| (i, j, group))
                    )
                    .filter_map(|(i, j, group)| {
                        group.regs.get_key_value(name).map(|(k, _v)| (i, j, k))
                    });
                let found = iter.next().ok_or(ParseError::NoSuchRegister)?;
                if iter.next().is_some() {
                    return Err(ParseError::AmbiguousRegister);
                }
                Self {
                    target: Indexed {
                        name: found.0,
                        index: None,
                    },
                    group: Indexed {
                        name: found.1,
                        index: None,
                    },
                    reg: Indexed {
                        name: found.2,
                        index,
                    },
                }
            },
            2 => {
                let cap = re.captures(words[1]).ok_or(ParseError::MatchFailed)?;
                let reg_name = &cap[1];
                let reg_index = cap.get(3).map(|i| i.as_str().parse::<usize>().unwrap());

                let cap = re.captures(words[0]).ok_or(ParseError::MatchFailed)?;
                let root_name = &cap[1];
                let root_index = cap.get(3).map(|i| i.as_str().parse::<usize>().unwrap());

                // Attempt to decode root_name as either a target or a register
                // group, for maximum ease of parsing.
                let mut iter = MEMORY_MAP.get_key_value(root_name)
                    .into_iter()
                    .flat_map(|(i, target)|
                        TARGETS[target.0]
                            .groups
                            .iter()
                            .map(move |(j, group)| (i, j, group)))
                    .filter_map(|(i, j, group)| {
                        group.regs.get_key_value(reg_name).map(|(k, _v)| (i, j, k, true))
                    }).chain(
                        MEMORY_MAP
                        .iter()
                        .flat_map(|(i, target)|
                            TARGETS[target.0]
                                .groups
                                .get_key_value(root_name)
                                .into_iter()
                                .map(move |(j, group)| (i, j, group))
                        )
                        .filter_map(|(i, j, group)| {
                            group.regs.get_key_value(reg_name).map(|(k, _v)| (i, j, k, false))
                        }));

                let found = iter.next().ok_or(ParseError::NoSuchRegister)?;
                if iter.next().is_some() {
                    return Err(ParseError::AmbiguousRegister);
                }
                Self {
                    target: Indexed {
                        name: found.0,
                        index: if found.3 { root_index } else { None },
                    },
                    group: Indexed {
                        name: found.1,
                        index: if !found.3 { root_index } else { None },
                    },
                    reg: Indexed {
                        name: found.2,
                        index: reg_index,
                    },
                }
            }
            3 => {
                let cap = re.captures(words[2]).ok_or(ParseError::MatchFailed)?;
                let reg_name = &cap[1];
                let reg_index = cap.get(3).map(|i| i.as_str().parse::<usize>().unwrap());

                let cap = re.captures(words[1]).ok_or(ParseError::MatchFailed)?;
                let group_name = &cap[1];
                let group_index = cap.get(3).map(|i| i.as_str().parse::<usize>().unwrap());

                let cap = re.captures(words[0]).ok_or(ParseError::MatchFailed)?;
                let target_name = &cap[1];
                let target_index = cap.get(3).map(|i| i.as_str().parse::<usize>().unwrap());

                let mut iter = MEMORY_MAP.get_key_value(target_name)
                    .into_iter()
                    .flat_map(|(i, target)|
                        TARGETS[target.0]
                            .groups
                            .get_key_value(group_name)
                            .into_iter()
                            .map(move |(j, group)| (i, j, group)))
                    .filter_map(|(i, j, group)| {
                        group.regs.get_key_value(reg_name).map(|(k, _v)| (i, j, k, true))
                    });

                let found = iter.next().ok_or(ParseError::NoSuchRegister)?;
                if iter.next().is_some() {
                    return Err(ParseError::AmbiguousRegister);
                }
                Self {
                    target: Indexed {
                        name: found.0,
                        index: target_index,
                    },
                    group: Indexed {
                        name: found.1,
                        index: group_index,
                    },
                    reg: Indexed {
                        name: found.2,
                        index: reg_index,
                    },
                }
            }
            _ => return Err(ParseError::TooManyItems),
        };

        // Check that the target index is valid
        let target = &MEMORY_MAP[out.target.name];
        target
            .1
            .iter()
            .find(|t| t.0 == out.target.index)
            .ok_or(ParseError::InvalidTargetIndex(out.target.index))?;

        let target = &TARGETS[target.0];
        let group = &target.groups[out.group.name];
        match (group.addr.count, out.group.index) {
            (1, Some(_)) => Err(ParseError::NotRegisterGroupArray),
            (i, None) if i != 1 => Err(ParseError::RegisterGroupArray),
            (i, Some(j)) if j >= i => Err(ParseError::InvalidRegisterGroupIndex(j, i)),
            _ => Ok(()),
        }?;

        let reg = &group.regs[out.reg.name];
        match (reg.addr.count, out.reg.index) {
            (1, Some(_)) => Err(ParseError::NotRegisterArray),
            (i, None) if i != 1 => Err(ParseError::RegisterArray),
            (i, Some(j)) if j >= i => Err(ParseError::InvalidRegisterIndex(j, i)),
            _ => Ok(()),
        }?;

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            "CHIP_ID".parse::<TargetRegister>(),
            Ok(TargetRegister {
                target: Indexed {
                    name: "DEVCPU_GCB",
                    index: None
                },
                group: Indexed {
                    name: "CHIP_REGS",
                    index: None
                },
                reg: Indexed {
                    name: "CHIP_ID",
                    index: None
                },
            })
        );
        assert_eq!(
            "AGGR_CFG".parse::<TargetRegister>(),
            Err(ParseError::AmbiguousRegister)
        );
        assert_eq!(
            "QMAP_PORT_MODE".parse::<TargetRegister>(),
            Err(ParseError::RegisterArray)
        );
        assert_eq!(
            "DEV10G:MAC_CFG_STATUS:MAC_TX_MONITOR_STICKY".parse::<TargetRegister>(),
            Err(ParseError::InvalidTargetIndex(None))
        );
        assert_eq!(
            "DEV10G[4]:MAC_CFG_STATUS:MAC_TX_MONITOR_STICKY".parse::<TargetRegister>(),
            Err(ParseError::InvalidTargetIndex(Some(4)))
        );
        assert_eq!(
            "QMAP_PORT_MODE[50]".parse::<TargetRegister>(),
            Ok(TargetRegister {
                target: Indexed {
                    name: "XQS",
                    index: None,
                },
                group: Indexed {
                    name: "SYSTEM",
                    index: None
                },
                reg: Indexed {
                    name: "QMAP_PORT_MODE",
                    index: Some(50)
                },
            })
        );
        assert_eq!(
            "QMAP_PORT_MODE[60]".parse::<TargetRegister>(),
            Err(ParseError::InvalidRegisterIndex(60, 53)));
        assert_eq!(
            "DEV10G[3]:MAC_CFG_STATUS:MAC_TX_MONITOR_STICKY".parse::<TargetRegister>(),
            Ok(TargetRegister {
                target: Indexed {
                    name: "DEV10G",
                    index: Some(3)
                },
                group: Indexed {
                    name: "MAC_CFG_STATUS",
                    index: None
                },
                reg: Indexed {
                    name: "MAC_TX_MONITOR_STICKY",
                    index: None
                },
            })
        );
    }
}
