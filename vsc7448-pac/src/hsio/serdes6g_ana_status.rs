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
/// SERDES6G ACJTAG Status
///
/// Status register of (AC)JTAG debug capability
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SERDES6G_ACJTAG_STATUS(u32);
impl SERDES6G_ACJTAG_STATUS {
    /// ACJTAG captured data for n leg
    pub fn acjtag_capt_data_n(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_acjtag_capt_data_n(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// ACJTAG captured data for p leg
    pub fn acjtag_capt_data_p(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_acjtag_capt_data_p(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// JTAG direct input (directly driven)
    pub fn ib_direct(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ib_direct(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// SERDES6G GP CFG
///
/// General purpose register A
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SERDES6G_GP_CFG(u32);
impl SERDES6G_GP_CFG {
    /// Bit 9: SNFBC Select negative feedback center - enable for hysteresis suppression in main sampler FFs Bit 8: SNFBV Select negative feedback Vscope - enable for hysteresis suppression in vscope sampler FFs Bit 1: ERLS (used for debug only, allows for manual stepping through calibration procedure) Bit 0: CRLS (used for debug only, allows for manual stepping through calibration procedure)
    pub fn gp_lsb(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_gp_lsb(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// To be defined
    pub fn gp_msb(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_gp_msb(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
/// SERDES6G IB Status register 0
///
/// Status register for Signal Detect
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SERDES6G_IB_STATUS0(u32);
impl SERDES6G_IB_STATUS0 {
    /// Signals mission mode after calibration was done.
    pub fn ib_cal_done(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_ib_cal_done(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Flag high-pass-gain regulation activity. Caution: currently this signal is generated with a clock of datarate/16 and NOT captured (sticky).
    pub fn ib_hp_gain_act(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_ib_hp_gain_act(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Flag low-pass-gain regulation activity. Caution: currently this signal is generated with a clock of datarate/16 and NOT captured (sticky).
    pub fn ib_lp_gain_act(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_ib_lp_gain_act(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Flag mid-pass-gain regulation activity. Caution: currently this signal is generated with a clock of datarate/16 and NOT captured (sticky).
    pub fn ib_mid_gain_act(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_ib_mid_gain_act(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Detection of offset direction in selected (ib_offsx) sampling channels
    pub fn ib_offsdir(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ib_offsdir(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Flag offset regulation activity. Caution: currently this signal is generated with a clock of datarate/16 and NOT captured (sticky).
    pub fn ib_offset_act(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_ib_offset_act(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Overflow error during calibration process. Value at ib_offset_stat not valid.
    pub fn ib_offset_err(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_ib_offset_err(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Valid average data of calibration process at ib_offset_stat available.
    pub fn ib_offset_vld(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_ib_offset_vld(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Detection of toggling signal at PADP and PADN
    pub fn ib_sig_det(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ib_sig_det(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// SERDES6G IB Status register 1
///
/// Regulation stage status register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SERDES6G_IB_STATUS1(u32);
impl SERDES6G_IB_STATUS1 {
    /// Current high-pass-gain regulation value
    pub fn ib_hp_gain_stat(&self) -> u32 {
        (self.0 & 0xfc0000) >> 18
    }
    pub fn set_ib_hp_gain_stat(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0xfc0000);
        self.0 &= !0xfc0000;
        self.0 |= value;
    }
    /// Current low-pass-gain regulation value
    pub fn ib_lp_gain_stat(&self) -> u32 {
        (self.0 & 0xfc0) >> 6
    }
    pub fn set_ib_lp_gain_stat(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0xfc0);
        self.0 &= !0xfc0;
        self.0 |= value;
    }
    /// Current mid-pass-gain regulation value
    pub fn ib_mid_gain_stat(&self) -> u32 {
        (self.0 & 0x3f000) >> 12
    }
    pub fn set_ib_mid_gain_stat(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x3f000);
        self.0 &= !0x3f000;
        self.0 |= value;
    }
    /// Current offset regulation value
    pub fn ib_offset_stat(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_ib_offset_stat(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}
/// SERDES6G Pll Status
///
/// Status register of SERDES6G RCPLL
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SERDES6G_PLL_STATUS(u32);
impl SERDES6G_PLL_STATUS {
    /// Calibration error
    ///
    /// 0: No error during calibration 1: Errors occured during calibration
    pub fn pll_cal_err(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_pll_cal_err(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Calibration status
    ///
    /// 0: Calibration not started or ongoing 1: Calibration finished
    pub fn pll_cal_not_done(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_pll_cal_not_done(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Out of range error
    ///
    /// 0: No out of range condition detected 1: Out of range condition since last calibration detected
    pub fn pll_out_of_range_err(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_pll_out_of_range_err(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// PLL read-back data, depending on "pll_rb_data_sel" either the calibrated setting or the measured period
    pub fn pll_rb_data(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_pll_rb_data(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
