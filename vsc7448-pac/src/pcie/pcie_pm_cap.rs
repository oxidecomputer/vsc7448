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
/// Word offset 0
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CAP_ID_NXT_PTR(u32);
impl CAP_ID_NXT_PTR {
    pub fn aux_curr(&self) -> u32 {
        (self.0 & 0x1c00000) >> 22
    }
    pub fn set_aux_curr(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x1c00000);
        self.0 &= !0x1c00000;
        self.0 |= value;
    }
    pub fn d1_support(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    pub fn set_d1_support(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x2000000);
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    pub fn d2_support(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    pub fn set_d2_support(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0x4000000);
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    pub fn dsi(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_dsi(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }
    pub fn pme_clk(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_pme_clk(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    pub fn pme_support(&self) -> u32 {
        (self.0 & 0xf8000000) >> 27
    }
    pub fn set_pme_support(&mut self, value: u32) {
        let value = value << 27;
        assert!(value <= 0xf8000000);
        self.0 &= !0xf8000000;
        self.0 |= value;
    }
    pub fn pm_cap_id(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_pm_cap_id(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    pub fn pm_next_pointer(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_pm_next_pointer(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
    pub fn pm_spec_ver(&self) -> u32 {
        (self.0 & 0x70000) >> 16
    }
    pub fn set_pm_spec_ver(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x70000);
        self.0 &= !0x70000;
        self.0 |= value;
    }
}
/// Word offset 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CON_STATUS(u32);
impl CON_STATUS {
    pub fn b2_b3_support(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_b2_b3_support(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
    pub fn bus_pwr_clk_con_en(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_bus_pwr_clk_con_en(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }
    pub fn data_reg_add_info(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_data_reg_add_info(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xff000000);
        self.0 &= !0xff000000;
        self.0 |= value;
    }
    pub fn data_scale(&self) -> u32 {
        (self.0 & 0x6000) >> 13
    }
    pub fn set_data_scale(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x6000);
        self.0 &= !0x6000;
        self.0 |= value;
    }
    pub fn data_select(&self) -> u32 {
        (self.0 & 0x1e00) >> 9
    }
    pub fn set_data_select(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x1e00);
        self.0 &= !0x1e00;
        self.0 |= value;
    }
    pub fn no_soft_rst(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_no_soft_rst(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    pub fn pme_enable(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_pme_enable(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    pub fn pme_status(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_pme_status(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    pub fn power_state(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_power_state(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
