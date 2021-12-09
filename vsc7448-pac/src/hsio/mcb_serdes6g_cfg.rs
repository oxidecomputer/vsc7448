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

/// Register `SERDES6G_REVID`
///
/// SERDES6G REVID
///
/// Revision ID register
#[derive(From, Into)]
pub struct SERDES6G_REVID(u32);
impl SERDES6G_REVID {
    /// DES revision
    pub fn des_rev(&self) -> u32 {
        (self.0 & 0xfc00) >> 10
    }
    pub fn set_des_rev(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xfc00);
        self.0 &= !0xfc00;
        self.0 |= value;
    }
    /// IB revision
    pub fn ib_rev(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_ib_rev(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
    /// OB revision
    pub fn ob_rev(&self) -> u32 {
        (self.0 & 0x3e0) >> 5
    }
    pub fn set_ob_rev(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x3e0);
        self.0 &= !0x3e0;
        self.0 |= value;
    }
    /// RCPLL revision
    pub fn rcpll_rev(&self) -> u32 {
        (self.0 & 0x3e00000) >> 21
    }
    pub fn set_rcpll_rev(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x3e00000);
        self.0 &= !0x3e00000;
        self.0 |= value;
    }
    /// Serdes revision
    pub fn serdes_rev(&self) -> u32 {
        (self.0 & 0xfc000000) >> 26
    }
    pub fn set_serdes_rev(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0xfc000000);
        self.0 &= !0xfc000000;
        self.0 |= value;
    }
    /// SER revision
    pub fn ser_rev(&self) -> u32 {
        (self.0 & 0x1f0000) >> 16
    }
    pub fn set_ser_rev(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x1f0000);
        self.0 &= !0x1f0000;
        self.0 |= value;
    }
}
