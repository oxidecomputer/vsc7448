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
/// VS training config 0
#[derive(From, Into)]
pub struct TR_CFG0(u32);
impl TR_CFG0 {
    /// Set local taps starting point
    ///
    /// 0: Set to INITIALIZE 1: Set to PRESET
    pub fn ld_pre_init(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_ld_pre_init(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Send first LP request
    ///
    /// 0: Send   INITIALIZE 1: Send   PRESET
    pub fn lp_pre_init(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_lp_pre_init(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Update taps regardless of v2,vp sum.
    pub fn nosum(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_nosum(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable partial OB tap configuration.
    pub fn part_cfg_en(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_part_cfg_en(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Invert recieved prbs11 within training frame
    pub fn rx_inv(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_rx_inv(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Clear all state machine history
    pub fn sm_hist_clr(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_sm_hist_clr(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Allow LP to to control tap settings.
    pub fn tapctl_en(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_tapctl_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Clock divider value for timer clocks.
    pub fn tmr_dvdr(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    pub fn set_tmr_dvdr(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xf000);
        self.0 &= !0xf000;
        self.0 |= value;
    }
    /// Invert transmitted prbs11 within training frame
    pub fn tx_inv(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_tx_inv(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
}
