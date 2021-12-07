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

/// Register `VCAP_ACTION_DAT`
///
/// Action cache
#[derive(From, Into)]
pub struct VCAP_ACTION_DAT(u32);
impl VCAP_ACTION_DAT {    ///
    /// The cache register that holds action. The register is replicated; index 0 is the 32 LSBs of the action.
    pub fn action_dat(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_action_dat(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `VCAP_CNT_DAT`
///
/// Counter cache
#[derive(From, Into)]
pub struct VCAP_CNT_DAT(u32);
impl VCAP_CNT_DAT {    ///
    /// The cache register that holds counter. The register is replicated; index 0 is the 32 LSBs of the counter. When the counter is 1 bit wide the counter operates as a 1 bit saturating counter; it is set by VCAP when a rule is matched by a key.
    pub fn cnt_dat(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_cnt_dat(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `VCAP_ENTRY_DAT`
///
/// Entry data cache
#[derive(From, Into)]
pub struct VCAP_ENTRY_DAT(u32);
impl VCAP_ENTRY_DAT {    ///
    /// The cache register that holds entry data. The register is replicated; index 0 is the 32 LSBs of the entry-data. Together with VCAP_MASK_DAT.MASK_DAT this field defines match parameters for TCAM entries. Version 2 VCAPs allows programming of never-match, this is needed when disabling entries. Version 1 VCAPs converts match-off to match-any when reading/writing entries.
    ///
    /// Match-0: Entry=0, Mask=0 Match-1: Entry=1, Mask=0 Match-any (don't care): Entry=0, Mask=1 Match-off (never-match): Entry=1, Mask=1
    pub fn entry_dat(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_entry_dat(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `VCAP_MASK_DAT`
///
/// Entry mask cache
#[derive(From, Into)]
pub struct VCAP_MASK_DAT(u32);
impl VCAP_MASK_DAT {    ///
    /// The cache register that holds entry mask. The register is replicated; index 0 is the 32 LSBs of the entry-mask. See VCAP_MASK_DAT.MASK_DAT for encoding information.
    pub fn mask_dat(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_mask_dat(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `VCAP_MV_CFG`
///
/// Configuration for move/initialization
#[derive(From, Into)]
pub struct VCAP_MV_CFG(u32);
impl VCAP_MV_CFG {    ///
    /// Specifies the distance during move operations. I.e. if this field is set to 4 for a move-down operation, then source address n is moved to destination address n+5.
    ///
    /// 0: Distance is one position 1: Distance is two positions n: Distance is n+1 positions
    pub fn mv_num_pos(&self) -> u32 {
        (self.0 & 0xffff) >> 16
    }
    pub fn set_mv_num_pos(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }    ///
    /// Specifies the number of addresses to move/initialize during	move/init operations.
    ///
    /// 0: Address VCAP_UPDATE_CTRL.UPDATE_ADDR is moved/initialized n: Addresses VCAP_UPDATE_CTRL.UPDATE_ADDR through VCAP_UPDATE_CTRL.UPDATE_ADDR+n are moved/initialized
    pub fn mv_size(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_mv_size(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
