//! Types for converting user-provided strings into registers at runtime.
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum ParseError {}

impl<'a> std::str::FromStr for TargetRegister<'a> {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            "CHIP_ID".parse::<TargetRegister>().unwrap(),
            TargetRegister {
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
            }
        )
    }
}
