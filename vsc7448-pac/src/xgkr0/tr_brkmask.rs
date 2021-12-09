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
/// VS training break_mask lsw
#[derive(From, Into)]
pub struct BRKMASK_LSW(u32);
impl BRKMASK_LSW {
    /// Select lptrain state machine breakpoints. Each bit correpsonds to a state (see design doc)
    pub fn brkmask_lsw(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_brkmask_lsw(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// VS training LUT selection
#[derive(From, Into)]
pub struct TR_LUTSEL(u32);
impl TR_LUTSEL {
    /// Clears LUT table
    pub fn lut_clr(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_lut_clr(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Selects LUT table entry (0 to 63).
    pub fn lut_row(&self) -> u32 {
        (self.0 & 0x1f8) >> 3
    }
    pub fn set_lut_row(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x1f8);
        self.0 &= !0x1f8;
        self.0 |= value;
    }
    /// Selects LUT for lut_o
    ///
    /// 0: Gain 1: DFE_1 2: DFE_2 3: DFE_avg_1 4: DFE_avg_2 5: BER_1 6: BER_2 7: BER_3
    pub fn lut_sel(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_lut_sel(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
