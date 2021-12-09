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

/// Register `TR_MTHD`
///
/// VS training method
#[derive(From, Into)]
pub struct TR_MTHD(u32);
impl TR_MTHD {
    /// Training method for remote C(0)
    pub fn mthd_c0(&self) -> u32 {
        (self.0 & 0x300) >> 8
    }
    pub fn set_mthd_c0(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x300);
        self.0 &= !0x300;
        self.0 |= value;
    }
    /// Training method for remote C(-1)
    pub fn mthd_cm(&self) -> u32 {
        (self.0 & 0xc0) >> 6
    }
    pub fn set_mthd_cm(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0xc0);
        self.0 &= !0xc0;
        self.0 |= value;
    }
    /// Training method for remote C(+1)

    ///

    /// 0 : BER method 1 : Gain method 2 : DFE method
    pub fn mthd_cp(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    pub fn set_mthd_cp(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xc00);
        self.0 &= !0xc00;
        self.0 |= value;
    }
    /// remote tap to optimize first

    ///

    /// 0 : C(-1) 1 : C(0) 2 : C(+1)
    pub fn ord1(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_ord1(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// remote tap to optimize second
    pub fn ord2(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_ord2(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// remote tap to optimize third
    pub fn ord3(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_ord3(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
