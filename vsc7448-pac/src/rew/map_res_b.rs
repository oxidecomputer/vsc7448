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
/// Configuration of mapping tables 2 and 3. MPLS label
///
/// Lookup 2 and 3 values
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MAP_LBL_B(u32);
impl MAP_LBL_B {
    /// Mapped MPLS label value
    ///
    /// n: Label value
    #[inline(always)]
    pub fn label_val(&self) -> u32 {
        self.0 & 0xfffff
    }
    #[inline(always)]
    pub fn set_label_val(&mut self, value: u32) {
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }
}
/// Configuration of mapping tables 2 and 3
///
/// Lookup 2 and 3 values
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MAP_VAL_B(u32);
impl MAP_VAL_B {
    /// Mapped DEI value
    ///
    /// n: New DEI value
    #[inline(always)]
    pub fn dei_val(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_dei_val(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Mapped DSCP value
    ///
    /// n: New DSCP value
    #[inline(always)]
    pub fn dscp_val(&self) -> u32 {
        (self.0 & 0x3f0) >> 4
    }
    #[inline(always)]
    pub fn set_dscp_val(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 4;
        self.0 &= !0x3f0;
        self.0 |= value;
    }
    /// Mapped OAM COLOR value
    ///
    /// n: New OAM COLOR value
    #[inline(always)]
    pub fn oam_color(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_oam_color(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Mapped OAM COSID value
    ///
    /// n: New OAM COSID value
    #[inline(always)]
    pub fn oam_cosid(&self) -> u32 {
        (self.0 & 0xe000) >> 13
    }
    #[inline(always)]
    pub fn set_oam_cosid(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 13;
        self.0 &= !0xe000;
        self.0 |= value;
    }
    /// Mapped PCP value
    ///
    /// n: New PCP value
    #[inline(always)]
    pub fn pcp_val(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_pcp_val(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Mapped TC value
    ///
    /// n: New TC value
    #[inline(always)]
    pub fn tc_val(&self) -> u32 {
        (self.0 & 0x1c00) >> 10
    }
    #[inline(always)]
    pub fn set_tc_val(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 10;
        self.0 &= !0x1c00;
        self.0 |= value;
    }
}
