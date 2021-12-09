// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Based on mesa-v2021.09 (https://github.com/microchip-ung/mesa/) which has
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

// This is an autogenerated file; do not edit by hand!
use derive_more::{From, Into};
/// Number of defaults
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ACTION_DEF_CNT(u32);
impl ACTION_DEF_CNT {
    /// The number of default actions. For VCAPs with more than one interface (see VCAP_SUPER::IF_CNT); this field returns the total number of defaults for all interfaces.
    pub fn action_def_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_action_def_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Action width
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ACTION_WIDTH(u32);
impl ACTION_WIDTH {
    /// Width of action. For version 1 VCAPs this is full word width. For version 2 VCAPs this is the width of one action suboword.
    pub fn action_width(&self) -> u32 {
        self.0
    }
    pub fn set_action_width(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Counter width
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CNT_WIDTH(u32);
impl CNT_WIDTH {
    /// The width of the counter memory, this is the complete width of all counter-fields associated with one full-word entry. There is one counter per entry sub-word (see VCAP_SUPER::ENTRY_SWCNT for number of subwords.)
    pub fn cnt_width(&self) -> u32 {
        self.0
    }
    pub fn set_cnt_width(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Number of cores
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CORE_CNT(u32);
impl CORE_CNT {
    /// The number of parallel entry/action cores.
    pub fn core_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_core_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Number of full-word entries
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ENTRY_CNT(u32);
impl ENTRY_CNT {
    /// Number of full-word entries (and actions) per core, see VCAP_SUPER::CORE_CNT for number of cores.
    pub fn entry_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_entry_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Number of subwords per full-word entry
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ENTRY_SWCNT(u32);
impl ENTRY_SWCNT {
    /// The number of supported subwords per full-word entry.
    pub fn entry_swcnt(&self) -> u32 {
        self.0
    }
    pub fn set_entry_swcnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Entry type-group width
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ENTRY_TG_WIDTH(u32);
impl ENTRY_TG_WIDTH {
    /// The width of a single TypeGroup id. For version 2 VCAPs this field return 0, the subword-encoding is configured directly via VCAP_SUPER::VCAP_ENTRY_DAT and VCAP_SUPER::VCAP_MASK_DAT.
    pub fn entry_tg_width(&self) -> u32 {
        self.0
    }
    pub fn set_entry_tg_width(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Entry width
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ENTRY_WIDTH(u32);
impl ENTRY_WIDTH {
    /// Width of entry. For version 1 VCAPs this is full word width including bits for the TypeGroup id(s). For version 2 VCAPs this is the width of one entry suboword.
    pub fn entry_width(&self) -> u32 {
        self.0
    }
    pub fn set_entry_width(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Debug information
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VCAP_STICKY(u32);
impl VCAP_STICKY {
    /// A move operation has resulted in deleting of one or more rules. This field applies only to version 1 VCAPs, for version 2 VCAPs it is not implemented and reading it will return zero.
    pub fn vcap_row_deleted_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vcap_row_deleted_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// VCAP version
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VCAP_VER(u32);
impl VCAP_VER {
    /// Version of the VCAP control logic.
    ///
    /// 0: Version 1 1: Version 2
    pub fn vcap_ver(&self) -> u32 {
        self.0
    }
    pub fn set_vcap_ver(&mut self, value: u32) {
        self.0 = value;
    }
}
