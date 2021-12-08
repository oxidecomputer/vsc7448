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

/// Register `SD10G65_DES_CFG0`
///
/// SD10G65 DES Configuration register 0
///
/// Configuration register 0 for SD10G65 DES.
#[derive(From, Into)]
pub struct SD10G65_DES_CFG0(u32);
impl SD10G65_DES_CFG0 {    ///
    /// Deserializer disable.
    pub fn des_dis(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_des_dis(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Interface width
    ///
    /// 0: 8 1: 10 2: 16 (energy efficient) 3: 20 (energy efficient) 4: 32 5: 40 6: 16 bit (fast) 7: 20 bit (fast)
    pub fn des_if_mode_sel(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    pub fn set_des_if_mode_sel(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x1c);
        self.0 &= !0x1c;
        self.0 |= value;
    }    ///
    /// Invert output of high auxillary deserializer
    pub fn des_inv_h(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_des_inv_h(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }    ///
    /// Invert output of low auxillary deserializer
    pub fn des_inv_l(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_des_inv_l(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }    ///
    /// Invert output of main deserializer
    pub fn des_inv_m(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_des_inv_m(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }    ///
    /// Auxillary deserializer channels disable.
    pub fn des_vsc_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_des_vsc_dis(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}