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
/// SD10G65 DES Configuration register 0
///
/// Configuration register 0 for SD10G65 DES.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SD10G65_DES_CFG0(u32);
impl SD10G65_DES_CFG0 {
    /// Deserializer disable.
    #[inline(always)]
    pub fn des_dis(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_des_dis(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Interface width
    ///
    /// 0: 8 1: 10 2: 16 (energy efficient) 3: 20 (energy efficient) 4: 32 5: 40 6: 16 bit (fast) 7: 20 bit (fast)
    #[inline(always)]
    pub fn des_if_mode_sel(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    #[inline(always)]
    pub fn set_des_if_mode_sel(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 2;
        self.0 &= !0x1c;
        self.0 |= value;
    }
    /// Invert output of high auxillary deserializer
    #[inline(always)]
    pub fn des_inv_h(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_des_inv_h(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Invert output of low auxillary deserializer
    #[inline(always)]
    pub fn des_inv_l(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_des_inv_l(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Invert output of main deserializer
    #[inline(always)]
    pub fn des_inv_m(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_des_inv_m(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Auxillary deserializer channels disable.
    #[inline(always)]
    pub fn des_vsc_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_des_vsc_dis(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// SD10G65 MOEBDIV Configuration register 0
///
/// Configuration register 0 for SD10G65 MoebiusDivider
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SD10G65_MOEBDIV_CFG0(u32);
impl SD10G65_MOEBDIV_CFG0 {
    /// Bandwidth selection for cp/md of cdr loop when core NOT flags valid data detected
    #[inline(always)]
    pub fn moebdiv_bw_cdr_sel_a(&self) -> u32 {
        (self.0 & 0xe00) >> 9
    }
    #[inline(always)]
    pub fn set_moebdiv_bw_cdr_sel_a(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 9;
        self.0 &= !0xe00;
        self.0 |= value;
    }
    /// Bandwidth selection for cp/md of cdr loop when core flags valid data detected
    #[inline(always)]
    pub fn moebdiv_bw_cdr_sel_b(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    #[inline(always)]
    pub fn set_moebdiv_bw_cdr_sel_b(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 6;
        self.0 &= !0x1c0;
        self.0 |= value;
    }
    /// Bandwidth selection for cp/md signals towards core
    #[inline(always)]
    pub fn moebdiv_bw_core_sel(&self) -> u32 {
        (self.0 & 0x38) >> 3
    }
    #[inline(always)]
    pub fn set_moebdiv_bw_core_sel(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 3;
        self.0 &= !0x38;
        self.0 |= value;
    }
    /// CP/MD swapping
    #[inline(always)]
    pub fn moebdiv_cpmd_swap(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_moebdiv_cpmd_swap(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Divider disable
    #[inline(always)]
    pub fn moebdiv_dis(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_moebdiv_dis(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// MD divider enable
    #[inline(always)]
    pub fn moebdiv_div32_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_moebdiv_div32_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
