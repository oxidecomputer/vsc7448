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

pub mod aneg_cfg;
pub mod aneg_status;
pub mod vaui_channel_cfg;

/// ANEG Configuration Registers
pub struct ANEG_CFG(pub(super) u32);
impl ANEG_CFG {
    pub fn ANEG_ADV_ABILITY_0(&self) -> RegisterAddress<aneg_cfg::ANEG_ADV_ABILITY_0> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn ANEG_ADV_ABILITY_1(&self) -> RegisterAddress<aneg_cfg::ANEG_ADV_ABILITY_1> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn ANEG_CFG(&self) -> RegisterAddress<aneg_cfg::ANEG_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn ANEG_NEXT_PAGE_0(&self) -> RegisterAddress<aneg_cfg::ANEG_NEXT_PAGE_0> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn VAUI_CHANNEL_CFG(&self, index: u32) -> RegisterAddress<aneg_cfg::VAUI_CHANNEL_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
}

/// ANEG Status Registers
pub struct ANEG_STATUS(pub(super) u32);
impl ANEG_STATUS {
    pub fn ANEG_LP_ADV_ABILITY_0(&self) -> RegisterAddress<aneg_status::ANEG_LP_ADV_ABILITY_0> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn ANEG_LP_ADV_ABILITY_1(&self) -> RegisterAddress<aneg_status::ANEG_LP_ADV_ABILITY_1> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn ANEG_NEXT_PAGE_1(&self) -> RegisterAddress<aneg_status::ANEG_NEXT_PAGE_1> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn ANEG_STATUS(&self) -> RegisterAddress<aneg_status::ANEG_STATUS> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// VAUI channel Configuration Registers
pub struct VAUI_CHANNEL_CFG(pub(super) u32);
impl VAUI_CHANNEL_CFG {}
