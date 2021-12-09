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
#[derive(From, Into)]
pub struct CU_PMD_TX_CTRL(pub u16);
#[derive(From, Into)]
pub struct EEE_CONTROL(u16);
impl EEE_CONTROL {
    pub fn enable_1000base_t_force_mode(&self) -> u16 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_enable_1000base_t_force_mode(&mut self, value: u16) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    pub fn enable_10base_te(&self) -> u16 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_enable_10base_te(&mut self, value: u16) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
}
#[derive(From, Into)]
pub struct TESLA_RGMII_CONTROL(pub u16);
#[derive(From, Into)]
pub struct RGMII_CONTROL(u16);
impl RGMII_CONTROL {
    pub fn nano_rgmii_skew_rx(&self) -> u16 {
        (self.0 & 0x8) >> 4
    }
    pub fn set_nano_rgmii_skew_rx(&mut self, value: u16) {
        let value = value << 4;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    pub fn nano_rgmii_skew_tx(&self) -> u16 {
        self.0 & 0x7
    }
    pub fn set_nano_rgmii_skew_tx(&mut self, value: u16) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    pub fn rx_clk_out_disable(&self) -> u16 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_rx_clk_out_disable(&mut self, value: u16) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
}
#[derive(From, Into)]
pub struct WOL_LOWER_MAC_ADDR(pub u16);
#[derive(From, Into)]
pub struct WOL_MID_MAC_ADDR(pub u16);
#[derive(From, Into)]
pub struct WOL_UPPER_MAC_ADDR(pub u16);
#[derive(From, Into)]
pub struct WOL_LOWER_PASSWD(pub u16);
#[derive(From, Into)]
pub struct WOL_MID_PASSWD(pub u16);
#[derive(From, Into)]
pub struct WOL_UPPER_PASSWD(pub u16);
#[derive(From, Into)]
pub struct WOL_CONTROL(u16);
impl WOL_CONTROL {
    pub fn addr_rep_count(&self) -> u16 {
        (self.0 & 0xf0) >> 8
    }
    pub fn set_addr_rep_count(&mut self, value: u16) {
        let value = value << 8;
        assert!(value <= 0xf0);
        self.0 &= !0xf0;
        self.0 |= value;
    }
    pub fn mdint_cmos_drive_en(&self) -> u16 {
        self.0 & 0x1
    }
    pub fn set_mdint_cmos_drive_en(&mut self, value: u16) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    pub fn mdint_signal_sep(&self) -> u16 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_mdint_signal_sep(&mut self, value: u16) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    pub fn reserved_001(&self) -> u16 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_reserved_001(&mut self, value: u16) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    pub fn reserved_002(&self) -> u16 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_reserved_002(&mut self, value: u16) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    pub fn reserved_003(&self) -> u16 {
        (self.0 & 0x3c) >> 2
    }
    pub fn set_reserved_003(&mut self, value: u16) {
        let value = value << 2;
        assert!(value <= 0x3c);
        self.0 &= !0x3c;
        self.0 |= value;
    }
    pub fn secure_on_enable(&self) -> u16 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_secure_on_enable(&mut self, value: u16) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    pub fn secure_on_passwd_len_4(&self) -> u16 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_secure_on_passwd_len_4(&mut self, value: u16) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
}
#[derive(From, Into)]
pub struct EXTENDED_INTERRUPT_MASK(pub u16);
#[derive(From, Into)]
pub struct EXTENDED_INTERRUPT_STATUS(pub u16);
#[derive(From, Into)]
pub struct EXTENDED_RING_RESILIENCY_CTRL(pub u16);
