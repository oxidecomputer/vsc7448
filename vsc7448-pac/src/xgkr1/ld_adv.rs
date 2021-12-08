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

/// Register `KR_7X0001`
///
/// AN status
#[derive(From, Into)]
pub struct KR_7X0001(u32);
impl KR_7X0001 {
    /// AN ability
    pub fn an_able(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_an_able(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// AN complete
    pub fn an_complete(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_an_complete(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// LP AN ability
    pub fn an_lp_able(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_an_lp_able(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Link status (LL)
    pub fn linkstat(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_linkstat(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Extended next page status
    pub fn npstat(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_npstat(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Parallel detection fault (LH)
    pub fn pardetflt(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_pardetflt(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Page received (LH)
    pub fn pg_rcvd(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_pg_rcvd(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Remote fault (LH)
    pub fn rem_flt(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rem_flt(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}

/// Register `KR_7X0010`
///
/// LD advertised abilities 15-0
#[derive(From, Into)]
pub struct KR_7X0010(u32);
impl KR_7X0010 {
    /// Local advertised abilities D[15:0]
    pub fn adv0(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_adv0(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `KR_7X0011`
///
/// LD advertised abilities 31-16
#[derive(From, Into)]
pub struct KR_7X0011(u32);
impl KR_7X0011 {
    /// Local advertised abilities D[31:16]
    pub fn adv1(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_adv1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
