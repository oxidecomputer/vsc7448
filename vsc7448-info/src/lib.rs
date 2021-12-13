//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

use std::collections::BTreeMap;

use lazy_static::lazy_static;
use vsc7448_types::{Page, OwnedTarget};

pub mod parse;

#[derive(serde::Deserialize)]
pub struct InfoFileContents {
    memory_map: BTreeMap<String, (String, Vec<(Option<u32>, u32)>)>,
    targets: BTreeMap<String, OwnedTarget>,
    phy_map: BTreeMap<String, Page<String>>,
}

impl InfoFileContents {
    /// Loads a register info file. This crate includes a canned info file (see
    /// `MEMORY_MAP`, `TARGETS`, and `PHY_MAP`, but this function also allows
    /// you to load one at runtime, should you want to do that.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

lazy_static! {
    static ref FILE_CONTENTS: InfoFileContents = {
        InfoFileContents::from_bytes(EMBEDDED_TABLE).unwrap()
    };

    /// Maps from user-facing target name to a tuple of
    /// - Name in `TARGETS` map (which may be different!)
    /// - List of instances, as tuples of `(instance id, address)`.  If there is
    ///   only one instance of this target, then this is vec![(None, ADDRESS)].
    pub static ref MEMORY_MAP: &'static BTreeMap<String, (String, Vec<(Option<u32>, u32)>)> = {
        &FILE_CONTENTS.memory_map
    };

    /// Maps from target name to `Target`, which contains a hierarchy
    /// of register groups, registers, and fields.
    pub static ref TARGETS: &'static BTreeMap<String, OwnedTarget> = {
        &FILE_CONTENTS.targets
    };

    /// Maps from PHY page name to `Page`, which contains a hierarchy
    /// of registers containing bit fields.
    pub static ref PHY_MAP: &'static BTreeMap<String, Page<String>> = {
        &FILE_CONTENTS.phy_map
    };
}

static EMBEDDED_TABLE: &[u8] = include_bytes!("vsc7448.postcard");

#[cfg(test)]
mod test {
    #[test]
    fn things_actually_parse() {
        let start = std::time::Instant::now();
        let _ = &*super::FILE_CONTENTS;
        println!("parsing took {:?}", start.elapsed());
    }
}
