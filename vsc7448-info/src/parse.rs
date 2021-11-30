//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

//! Types for converting user-provided strings into registers at runtime.
use regex::Regex;
use thiserror::Error;

use std::collections::HashMap;

use crate::{MEMORY_MAP, PHY_MAP, TARGETS};
use vsc7448_types::Field;

/// One level of hierarchy within a fully qualified register, with a name and
/// optional index (for targets / groups / registers that have multiple
/// instances)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Indexed {
    /// Name of the target, register group, or register.  This is always
    /// `'static` because it points into `MEMORY_MAP` or `TARGETS`, which
    /// are `lazy_static` constant globals.
    name: &'static str,
    /// Index of the item within an array
    index: Option<usize>,
}
impl std::fmt::Display for Indexed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.index {
            None => write!(f, "{}", self.name),
            Some(i) => write!(f, "{}[{}]", self.name, i),
        }
    }
}

/// Fully qualified register
#[derive(Debug, PartialEq, Eq)]
pub struct TargetRegister {
    target: Indexed,
    group: Indexed,
    reg: Indexed,
}

impl std::fmt::Display for TargetRegister {
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
    #[error("No page with that name")]
    NoSuchPage,
    #[error("Register shows up in multiple mages ({0}, {1}); specify page")]
    AmbiguousPhyRegister(&'static str, &'static str),
    #[error("Multiple registers with this name ({0}, {1}); specify target or group")]
    AmbiguousRegister(TargetRegister, TargetRegister),
    #[error("Invalid target index for target {0}: {1:?}")]
    InvalidTargetIndex(&'static str, Option<usize>),
    #[error("Register {0} is part of an array and requires an [index]")]
    RegisterArray(&'static str),
    #[error("Register index specified for a non-array register {0}")]
    NotRegisterArray(&'static str),
    #[error("Register index for {0} is out of range ({1} >= {2})")]
    InvalidRegisterIndex(&'static str, usize, usize),
    #[error("Register group {0} is part of an array and requires an [index]")]
    RegisterGroupArray(&'static str),
    #[error("Register group index specified for a non-array register group {0}")]
    NotRegisterGroupArray(&'static str),
    #[error("Register group index for {0} is out of range ({0} >= {1})")]
    InvalidRegisterGroupIndex(&'static str, usize, usize),
}

impl TargetRegister {
    /// Returns the physical memory address of the given address
    pub fn address(&self) -> u32 {
        let target = &MEMORY_MAP[self.target.name];
        let (_, mut addr) = target.1.iter().find(|t| t.0 == self.target.index).unwrap();

        let target = &TARGETS[target.0];
        let group = &target.groups[self.group.name];
        addr += (group.addr.base + group.addr.width * self.group.index.unwrap_or(0)) * 4;

        let reg = &group.regs[self.reg.name];
        addr += (reg.addr.base + reg.addr.width * self.reg.index.unwrap_or(0)) * 4;

        addr.try_into().expect("Address exceeds 32-bit space?!")
    }
    /// Returns this register's field map.
    pub fn fields(&self) -> &HashMap<&'static str, Field<&'static str>> {
        &TARGETS[MEMORY_MAP[self.target.name].0].groups[self.group.name].regs[self.reg.name].fields
    }
}

impl std::str::FromStr for TargetRegister {
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

        let (out, ambig) = match words.len() {
            // Just a register name, potentially qualified with an index
            1 => {
                let cap = re.captures(words[0]).ok_or(ParseError::MatchFailed)?;
                let name = &cap[1];
                let index = cap.get(3).map(|i| i.as_str().parse::<usize>().unwrap());

                let mut iter = MEMORY_MAP
                    .iter()
                    .flat_map(|(i, target)| {
                        TARGETS[target.0]
                            .groups
                            .iter()
                            .map(move |(j, group)| (i, j, group))
                    })
                    .filter_map(|(i, j, group)| {
                        group.regs.get_key_value(name).map(|(k, _v)| (i, j, k))
                    });
                let found = iter.next().ok_or(ParseError::NoSuchRegister)?;
                let out = Self {
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
                };
                (out, iter.next())
            }
            2 => {
                let cap = re.captures(words[1]).ok_or(ParseError::MatchFailed)?;
                let reg_name = &cap[1];
                let reg_index = cap.get(3).map(|i| i.as_str().parse::<usize>().unwrap());

                let cap = re.captures(words[0]).ok_or(ParseError::MatchFailed)?;
                let root_name = &cap[1];
                let root_index = cap.get(3).map(|i| i.as_str().parse::<usize>().unwrap());

                // Attempt to decode root_name as either a target or a register
                // group, for maximum ease of parsing.
                let mut iter = MEMORY_MAP
                    .get_key_value(root_name)
                    .into_iter()
                    .flat_map(|(i, target)| {
                        TARGETS[target.0]
                            .groups
                            .iter()
                            .map(move |(j, group)| (i, j, group))
                    })
                    .filter_map(|(i, j, group)| {
                        group
                            .regs
                            .get_key_value(reg_name)
                            .map(|(k, _v)| (i, j, k, true))
                    })
                    .chain(
                        MEMORY_MAP
                            .iter()
                            .flat_map(|(i, target)| {
                                TARGETS[target.0]
                                    .groups
                                    .get_key_value(root_name)
                                    .into_iter()
                                    .map(move |(j, group)| (i, j, group))
                            })
                            .filter_map(|(i, j, group)| {
                                group
                                    .regs
                                    .get_key_value(reg_name)
                                    .map(|(k, _v)| (i, j, k, false))
                            }),
                    );

                let found = iter.next().ok_or(ParseError::NoSuchRegister)?;
                let out = Self {
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
                };
                (out, iter.next().map(|f| (f.0, f.1, f.2)))
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

                let mut iter = MEMORY_MAP
                    .get_key_value(target_name)
                    .into_iter()
                    .flat_map(|(i, target)| {
                        TARGETS[target.0]
                            .groups
                            .get_key_value(group_name)
                            .into_iter()
                            .map(move |(j, group)| (i, j, group))
                    })
                    .filter_map(|(i, j, group)| {
                        group.regs.get_key_value(reg_name).map(|(k, _v)| (i, j, k))
                    });

                let found = iter.next().ok_or(ParseError::NoSuchRegister)?;
                let out = Self {
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
                };
                (out, iter.next())
            }
            _ => return Err(ParseError::TooManyItems),
        };

        if let Some(ambig) = ambig {
            let index = out.reg.index;
            return Err(ParseError::AmbiguousRegister(
                out,
                Self {
                    target: Indexed {
                        name: ambig.0,
                        index: None,
                    },
                    group: Indexed {
                        name: ambig.1,
                        index: None,
                    },
                    reg: Indexed {
                        name: ambig.2,
                        index,
                    },
                },
            ));
        }

        // Check that the target index is valid
        let target = &MEMORY_MAP[out.target.name];
        target
            .1
            .iter()
            .find(|t| t.0 == out.target.index)
            .ok_or(ParseError::InvalidTargetIndex(
                out.target.name,
                out.target.index,
            ))?;

        let target = &TARGETS[target.0];
        let group = &target.groups[out.group.name];
        match (group.addr.count, out.group.index) {
            (1, Some(_)) => Err(ParseError::NotRegisterGroupArray(out.group.name)),
            (i, None) if i != 1 => Err(ParseError::RegisterGroupArray(out.group.name)),
            (i, Some(j)) if j >= i => {
                Err(ParseError::InvalidRegisterGroupIndex(out.group.name, j, i))
            }
            _ => Ok(()),
        }?;

        let reg = &group.regs[out.reg.name];
        match (reg.addr.count, out.reg.index) {
            (1, Some(_)) => Err(ParseError::NotRegisterArray(out.reg.name)),
            (i, None) if i != 1 => Err(ParseError::RegisterArray(out.reg.name)),
            (i, Some(j)) if j >= i => Err(ParseError::InvalidRegisterIndex(out.reg.name, j, i)),
            _ => Ok(()),
        }?;

        Ok(out)
    }
}

/// PHY register
///
/// `page` and `reg` index into [struct@PHY_MAP]
#[derive(Debug, PartialEq, Eq)]
pub struct PhyRegister {
    page: &'static str,
    reg: &'static str,
}

impl std::str::FromStr for PhyRegister {
    type Err = ParseError;
    /// Parses a string into a PHY register.  This is very flexible, accepting
    /// strings of the format
    /// - `REG_NAME`
    /// - `PAGE:REG_NAME`
    ///
    /// This will raise an error if the parse is invalid or ambiguous, based
    /// on the PHY register map
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(':').collect::<Vec<&str>>();
        match words.len() {
            1 => {
                let mut iter = PHY_MAP.iter().filter_map(|(page, p)| {
                    p.regs.get_key_value(words[0]).map(|(reg, _r)| (page, reg))
                });
                let (page, reg) = iter.next().ok_or(ParseError::NoSuchRegister)?;
                if let Some((ambig, _)) = iter.next() {
                    return Err(ParseError::AmbiguousPhyRegister(page, ambig));
                }
                Ok(PhyRegister { page, reg })
            }
            2 => match PHY_MAP.get_key_value(words[0]) {
                None => Err(ParseError::NoSuchPage),
                Some((page, p)) => match p.regs.get_key_value(words[1]) {
                    None => Err(ParseError::NoSuchRegister),
                    Some((reg, _r)) => Ok(PhyRegister { page, reg }),
                },
            },
            _ => Err(ParseError::TooManyItems),
        }
    }
}

impl std::fmt::Display for PhyRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.page, self.reg)
    }
}

impl PhyRegister {
    /// Returns the page address for this PHY register.
    pub fn page_addr(&self) -> u32 {
        PHY_MAP[self.page].base.try_into().expect("Invalid page address")
    }
    /// Looks up the register address within the page, which is a 5-bit value.
    pub fn reg_addr(&self) -> u8 {
        let addr = PHY_MAP[self.page].regs[self.reg].addr.base.try_into().expect("Invalid register address");
        if addr > 31 {
            // This should never happen, because it's checked in the codegen
            panic!("Invalid register address (must be <= 31)");
        }
        addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_register() {
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
        if let Err(ParseError::AmbiguousRegister(_, _)) = "AGGR_CFG".parse::<TargetRegister>() {
            // Okay
        } else {
            panic!("Failed to detect ambiguous register");
        }
        assert_eq!(
            "QMAP_PORT_MODE".parse::<TargetRegister>(),
            Err(ParseError::RegisterArray("QMAP_PORT_MODE"))
        );
        assert_eq!(
            "DEV10G:MAC_CFG_STATUS:MAC_TX_MONITOR_STICKY".parse::<TargetRegister>(),
            Err(ParseError::InvalidTargetIndex("DEV10G", None))
        );
        assert_eq!(
            "DEV10G[4]:MAC_CFG_STATUS:MAC_TX_MONITOR_STICKY".parse::<TargetRegister>(),
            Err(ParseError::InvalidTargetIndex("DEV10G", Some(4)))
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
            Err(ParseError::InvalidRegisterIndex("QMAP_PORT_MODE", 60, 53))
        );
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

    #[test]
    fn test_phy_register() {
        assert_eq!(
            "LOLOL".parse::<PhyRegister>(),
            Err(ParseError::NoSuchRegister)
        );
        assert_eq!(
            "EXTENDED_PHY_CONTROL".parse::<PhyRegister>(),
            Ok(PhyRegister {
                page: "STANDARD",
                reg: "EXTENDED_PHY_CONTROL",
            })
        );
    }
}
