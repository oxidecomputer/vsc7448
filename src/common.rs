use std::collections::HashMap;

use regex::Regex;

/// Parses `vtss_*_regs_common.h`, returning a map of target name to
/// absolute address (with offsets applied)
pub fn parse_regs_common(s: &str) -> HashMap<String, usize> {
    let offset = Regex::new(r"^#define VTSS_IO_ORIGIN([[:xdigit:]]+)_OFFSET 0x([[:xdigit:]]+)$")
        .expect("failed to build regex");
    let reg =
        Regex::new(r"^#define VTSS_TO_([A-Z_0-9]+) +VTSS_IO_OFFSET(\d+)\(0x([[:xdigit:]]+)\)")
            .expect("failed to build regex");
    let mut offsets = HashMap::new();
    let mut out = HashMap::new();
    for s in s.lines() {
        if let Some(cap) = offset.captures(s) {
            offsets.insert(
                (&cap[1]).parse::<usize>().unwrap(),
                usize::from_str_radix(&cap[2], 16).unwrap(),
            );
        }
        if let Some(cap) = reg.captures(s) {
            let i = (&cap[2]).parse::<usize>().unwrap();
            let d = usize::from_str_radix(&cap[3], 16).unwrap();
            out.insert(cap[1].to_owned(), d + offsets[&i]);
        }
    }
    out
}

