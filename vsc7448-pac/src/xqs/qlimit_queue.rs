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

/// Register `DROP_STAT_CTRL`
///
/// Per egress port mapping of qgrp to drop stat index
#[derive(From, Into)]
pub struct DROP_STAT_CTRL(u32);
impl DROP_STAT_CTRL {
    /// Drop counter set base address.
    pub fn drop_stat_base_addr(&self) -> u32 {
        (self.0 & 0x1ffc0) >> 6
    }
    pub fn set_drop_stat_base_addr(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x1ffc0);
        self.0 &= !0x1ffc0;
        self.0 |= value;
    }
    /// Select the number of counters for this VPORT.

    ///

    /// 0: 4 counters included. Qos x and x+4 shares counter 1: 8 counters included, and two counter sets will be used
    pub fn drop_stat_cos8_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_drop_stat_cos8_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Configures which classified parameter to use when selecting drop stat index.

    ///

    /// 0: Use IPRIO as COS input selector 1: Use COSID as COS input selector 2: Use TC as COS input selector 3: Use PCP as COS input selector
    pub fn drop_stat_cos_sel(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_drop_stat_cos_sel(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x6);
        self.0 &= !0x6;
        self.0 |= value;
    }
    /// Configures which OAM is counted in the from stat counter. Ref: ANA_L2:ISDX:MISC_CFG.QGRP_OAM_TYPE

    ///

    /// bit0: Enable / disable drop count of EVC MEP OAM bit1: Enable / disable drop count of OVC / PW MEP OAM bit2: Enable / disable drop count of DOWN MEP OAM
    pub fn drop_stat_oam_cnt_sel(&self) -> u32 {
        (self.0 & 0x38) >> 3
    }
    pub fn set_drop_stat_oam_cnt_sel(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x38);
        self.0 &= !0x38;
        self.0 |= value;
    }
}
