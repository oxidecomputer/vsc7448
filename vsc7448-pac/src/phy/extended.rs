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
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct REG_17E(pub u16);
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CU_MEDIA_CRC_GOOD_COUNTER(u16);
impl CU_MEDIA_CRC_GOOD_COUNTER {
    #[inline(always)]
    pub fn contents(&self) -> u16 {
        self.0 & 0x3fff
    }
    #[inline(always)]
    pub fn set_contents(&mut self, value: u16) {
        assert!(value <= 0x3fff);
        self.0 &= !0x3fff;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct EXTENDED_MODE_CONTROL(u16);
impl EXTENDED_MODE_CONTROL {
    #[inline(always)]
    pub fn fast_link_failure(&self) -> u16 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_fast_link_failure(&mut self, value: u16) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn force_mdi_crossover(&self) -> u16 {
        (self.0 & 0xc) >> 2
    }
    #[inline(always)]
    pub fn set_force_mdi_crossover(&mut self, value: u16) {
        assert!(value <= 0x3);
        let value = value << 2;
        self.0 &= !0xc;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct EXTENDED_PHY_CONTROL_3(u16);
impl EXTENDED_PHY_CONTROL_3 {
    #[inline(always)]
    pub fn media_mode_status(&self) -> u16 {
        (self.0 & 0xc0) >> 6
    }
    #[inline(always)]
    pub fn set_media_mode_status(&mut self, value: u16) {
        assert!(value <= 0x3);
        let value = value << 6;
        self.0 &= !0xc0;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RESERVED_1_WS(pub u16);
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RESERVED_2_WS(pub u16);
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct EXTENDED_PHY_CONTROL_4(pub u16);
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VERIPHY_CTRL_REG1(u16);
impl VERIPHY_CTRL_REG1 {
    #[inline(always)]
    pub fn pair_a_distance(&self) -> u16 {
        (self.0 & 0x3f00) >> 8
    }
    #[inline(always)]
    pub fn set_pair_a_distance(&mut self, value: u16) {
        assert!(value <= 0x3f);
        let value = value << 8;
        self.0 &= !0x3f00;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pair_b_distance(&self) -> u16 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_pair_b_distance(&mut self, value: u16) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn trigger(&self) -> u16 {
        (self.0 & 0x8000) >> 15
    }
    #[inline(always)]
    pub fn set_trigger(&mut self, value: u16) {
        assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn valid(&self) -> u16 {
        (self.0 & 0x4000) >> 14
    }
    #[inline(always)]
    pub fn set_valid(&mut self, value: u16) {
        assert!(value <= 0x1);
        let value = value << 14;
        self.0 &= !0x4000;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VERIPHY_CTRL_REG2(u16);
impl VERIPHY_CTRL_REG2 {
    #[inline(always)]
    pub fn pair_c_distance(&self) -> u16 {
        (self.0 & 0x3f00) >> 8
    }
    #[inline(always)]
    pub fn set_pair_c_distance(&mut self, value: u16) {
        assert!(value <= 0x3f);
        let value = value << 8;
        self.0 &= !0x3f00;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pair_d_distance(&self) -> u16 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_pair_d_distance(&mut self, value: u16) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VERIPHY_CTRL_REG3(u16);
impl VERIPHY_CTRL_REG3 {
    #[inline(always)]
    pub fn pair_a_termination_status(&self) -> u16 {
        (self.0 & 0xf000) >> 12
    }
    #[inline(always)]
    pub fn set_pair_a_termination_status(&mut self, value: u16) {
        assert!(value <= 0xf);
        let value = value << 12;
        self.0 &= !0xf000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pair_b_termination_status(&self) -> u16 {
        (self.0 & 0xf00) >> 8
    }
    #[inline(always)]
    pub fn set_pair_b_termination_status(&mut self, value: u16) {
        assert!(value <= 0xf);
        let value = value << 8;
        self.0 &= !0xf00;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pair_c_termination_status(&self) -> u16 {
        (self.0 & 0xf0) >> 4
    }
    #[inline(always)]
    pub fn set_pair_c_termination_status(&mut self, value: u16) {
        assert!(value <= 0xf);
        let value = value << 4;
        self.0 &= !0xf0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pair_d_termination_status(&self) -> u16 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_pair_d_termination_status(&mut self, value: u16) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct EXT_28(u16);
impl EXT_28 {
    #[inline(always)]
    pub fn epg_data_reg_idx(&self) -> u16 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_epg_data_reg_idx(&mut self, value: u16) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn etype_udpdest_ov_ena(&self) -> u16 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_etype_udpdest_ov_ena(&mut self, value: u16) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn ipg_ov_ena(&self) -> u16 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_ipg_ov_ena(&mut self, value: u16) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pkt_sz_ov_ena(&self) -> u16 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_pkt_sz_ov_ena(&mut self, value: u16) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn sig_ctr_ena(&self) -> u16 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_sig_ctr_ena(&mut self, value: u16) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
}
