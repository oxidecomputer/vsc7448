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

use crate::types::RegisterAddress;

// Register groups are stored in the tree as submodules

pub mod common;

/// Common configurations and status for all ports
pub struct COMMON(pub(super) u32);
impl COMMON {
    pub fn AUTOAGE_CFG(&self, index: u32) -> RegisterAddress<common::AUTOAGE_CFG> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x20 + index * 0x4)
    }
    pub fn AUTOAGE_CFG_1(&self) -> RegisterAddress<common::AUTOAGE_CFG_1> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn AUTOAGE_CFG_2(&self) -> RegisterAddress<common::AUTOAGE_CFG_2> {
        RegisterAddress::new(self.0 + 0x34)
    }
    pub fn AUTO_LRN_CFG(&self) -> RegisterAddress<common::AUTO_LRN_CFG> {
        RegisterAddress::new(self.0 + 0x38)
    }
    pub fn COMMON_ACCESS_CTRL(&self) -> RegisterAddress<common::COMMON_ACCESS_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn EVENT_STICKY(&self) -> RegisterAddress<common::EVENT_STICKY> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    pub fn LATEST_POS_STATUS(&self) -> RegisterAddress<common::LATEST_POS_STATUS> {
        RegisterAddress::new(self.0 + 0x40)
    }
    pub fn MAC_ACCESS_CFG_0(&self) -> RegisterAddress<common::MAC_ACCESS_CFG_0> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn MAC_ACCESS_CFG_1(&self) -> RegisterAddress<common::MAC_ACCESS_CFG_1> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn MAC_ACCESS_CFG_2(&self) -> RegisterAddress<common::MAC_ACCESS_CFG_2> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn SCAN_LAST_ROW_CFG(&self) -> RegisterAddress<common::SCAN_LAST_ROW_CFG> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn SCAN_NEXT_CFG(&self) -> RegisterAddress<common::SCAN_NEXT_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn SCAN_NEXT_CFG_1(&self) -> RegisterAddress<common::SCAN_NEXT_CFG_1> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn SCAN_NEXT_CNT(&self) -> RegisterAddress<common::SCAN_NEXT_CNT> {
        RegisterAddress::new(self.0 + 0x1c)
    }
}