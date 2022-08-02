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
/// PCS1G TSTPAT MODE CFG
///
/// PCS1G testpattern mode configuration register (Frame based pattern 4 and 5 might be not available depending on chip type)
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCS1G_TSTPAT_MODE_CFG(u32);
impl PCS1G_TSTPAT_MODE_CFG {
    /// Jitter Test Pattern Select: Enables and selects the jitter test pattern to be transmitted. The jitter test patterns are according to the IEEE 802.3, Annex 36A
    ///
    /// 0: Disable transmission of test patterns 1: High frequency test pattern - repeated transmission of D21.5 code group 2: Low frequency test pattern - repeated transmission of K28.7 code group 3: Mixed frequency test pattern - repeated transmission of K28.5 code group 4: Long continuous random test pattern (packet length is 1524 bytes) 5: Short continuous random test pattern (packet length is 360 bytes)
    #[inline(always)]
    pub fn jtp_sel(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_jtp_sel(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// PCS1G TSTPAT STATUS
///
/// PCS1G testpattern status register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCS1G_TSTPAT_STATUS(u32);
impl PCS1G_TSTPAT_STATUS {
    /// Jitter Test Pattern Error
    ///
    /// 0: Jitter pattern checker has found no error 1: Jitter pattern checker has found an error
    #[inline(always)]
    pub fn jtp_err(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_jtp_err(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Jitter Test Pattern Error Counter. Due to re-sync measures it might happen that single errors are not counted (applies for 2.5gpbs mode). The counter saturates at 255 and is only cleared when writing 0 to the register
    #[inline(always)]
    pub fn jtp_err_cnt(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline(always)]
    pub fn set_jtp_err_cnt(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// Jitter Test Pattern Lock
    ///
    /// 0: Jitter pattern checker has not locked 1: Jitter pattern checker has locked
    #[inline(always)]
    pub fn jtp_lock(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_jtp_lock(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
