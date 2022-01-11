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
/// SERDES1G DFT Status
///
/// Status register of SERDES1G DFT functions
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SERDES1G_DFT_STATUS(u32);
impl SERDES1G_DFT_STATUS {
    /// BIST activity
    ///
    /// 0: BIST inactive 1: BIST active
    #[inline]
    pub fn bist_active(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline]
    pub fn set_bist_active(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// BIST completion state (low-active)
    ///
    /// 0: BIST completed 1: not completed
    #[inline]
    pub fn bist_complete_n(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline]
    pub fn set_bist_complete_n(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// BIST result
    ///
    /// 0: No error found 1: Errors during BIST found
    #[inline]
    pub fn bist_error(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline]
    pub fn set_bist_error(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// BIST sync result
    ///
    /// 0: Synchronization successful 1: Synchronization on BIST data failed
    #[inline]
    pub fn bist_nosync(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline]
    pub fn set_bist_nosync(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// RC-PLL BIST result
    ///
    /// 0: No error found 1: Errors during BIST found
    #[inline]
    pub fn pll_bist_failed(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline]
    pub fn set_pll_bist_failed(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// RC-PLL BIST not done flag
    ///
    /// 0: BIST done 1: BIST not started or active
    #[inline]
    pub fn pll_bist_not_done(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline]
    pub fn set_pll_bist_not_done(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// RC-PLL BIST timeout error flag
    ///
    /// 0: No timeout occured 1: Timeout occured
    #[inline]
    pub fn pll_bist_timeout_err(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline]
    pub fn set_pll_bist_timeout_err(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
}
/// SERDES1G Misc Status
///
/// Status register for miscellaneous functions
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SERDES1G_MISC_STATUS(u32);
impl SERDES1G_MISC_STATUS {
    /// Phase selection of DES in 100fx mode
    ///
    /// 0: CDR locked at bit 9 1: CDR locked at bit 4
    #[inline]
    pub fn des_100fx_phase_sel(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline]
    pub fn set_des_100fx_phase_sel(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
