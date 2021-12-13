use regex::Regex;
use std::collections::BTreeMap;

use vsc7448_types::{Address, Field, Page, Register};

pub fn parse_phy_pages(s: &str) -> BTreeMap<String, Page<String>> {
    let re =
        Regex::new(r"^#define VTSS_PHY_PAGE_([A-Z0-9_]+)\s+(0x[[:xdigit:]]+) /\*\*< (.*) \*/$")
            .unwrap();
    s.lines()
        .filter_map(|line| re.captures(line))
        .map(|cap| {
            (
                cap[1].to_owned(),
                Page {
                    desc: cap[3].to_owned(),
                    base: parse_int::parse(&cap[2]).unwrap(),
                    regs: BTreeMap::new(),
                },
            )
        })
        .collect()
}

pub fn parse_phy_registers(s: &str, pages: &mut BTreeMap<String, Page<String>>) {
    //#define  VTSS_PHY_MODE_CONTROL VTSS_PHY_PAGE_STANDARD, 0
    let reg_re = Regex::new(
        r"^#define\s+VTSS_PHY_(?:PAGE_)?([A-Z0-9_]+)\s+VTSS_PHY_PAGE_([A-Z0-9_]+),\s*([0-9]+)$",
    )
    .unwrap();
    let field_re = Regex::new(
        r"^#define\s+VTSS_(?:F_)?PHY_(?:F_PAGE_)?([A-Z0-9_]+)\s+VTSS_PHY_BIT\(([0-9]+)\)$",
    )
    .unwrap();
    let mask_re =
        Regex::new(r"^#define\s+VTSS_PHY_([A-Z0-9_]+)\s+VTSS_PHY_MASK\(([0-9]+),\s*([0-9]+)\)$")
            .unwrap();
    let bitmask_re = Regex::new(
        r"^#define\s+VTSS_M_PHY_([A-Z0-9_]+)\s+VTSS_PHY_ENCODE_BITMASK\(([0-9]+),\s*([0-9]+)\)$",
    )
    .unwrap();

    let mut active: Option<(String, String, Register<String>)> = None;
    for line in s.lines() {
        if let Some(cap) = reg_re.captures(line) {
            // If this is a new register, then flush the previous one
            if let Some((page, name, reg)) = active.take() {
                pages.get_mut(&page).unwrap().regs.insert(name, reg);
            }
            active = Some((
                cap[2].to_owned(),
                cap[1].to_owned(),
                Register {
                    addr: Address {
                        base: parse_int::parse(&cap[3]).unwrap(),
                        count: 1,
                        width: 0,
                    },
                    brief: None,
                    details: None,
                    fields: BTreeMap::new(),
                },
            ));
        }
        if let Some(cap) = field_re.captures(line) {
            let field_name = match cap[1].strip_prefix(&format!("{}_", active.as_ref().unwrap().1))
            {
                Some(f) => f,
                None => {
                    eprintln!("Out of order field {}; skipping", &cap[1]);
                    continue;
                }
            };
            let bit = parse_int::parse(&cap[2]).unwrap();
            active.as_mut().unwrap().2.fields.insert(
                field_name.to_owned(),
                Field {
                    brief: None,
                    details: None,
                    lo: bit,
                    hi: bit + 1,
                },
            );
        }
        if let Some(cap) = mask_re.captures(line).or_else(|| bitmask_re.captures(line)) {
            let field_name = match cap[1].strip_prefix(&format!("{}_", active.as_ref().unwrap().1))
            {
                Some(f) => f,
                None => {
                    eprintln!("Out of order field {}; skipping", &cap[1]);
                    continue;
                }
            };
            let lo = parse_int::parse(&cap[2]).unwrap();
            let hi = parse_int::parse(&cap[3]).unwrap();
            active.as_mut().unwrap().2.fields.insert(
                field_name.to_owned(),
                Field {
                    brief: None,
                    details: None,
                    lo,
                    hi,
                },
            );
        }
    }
    // Flush the final register
    if let Some((page, name, reg)) = active.take() {
        pages.get_mut(&page).unwrap().regs.insert(name, reg);
    }
}
