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

/// Register `AW_MSW`
///
/// VS AN aneg_wait timer msw
#[derive(From, Into)]
pub struct AW_MSW(u32);
impl AW_MSW {
    /// an_wait_timer setting
    pub fn aw_tmr_msw(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_aw_tmr_msw(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `LFLONG_LSW`
///
/// VS AN link_fail_inhibit timer lsw
#[derive(From, Into)]
pub struct LFLONG_LSW(u32);
impl LFLONG_LSW {
    /// 10g link_fail_inhibit_timer setting
    pub fn lflong_tmr_lsw(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_lflong_tmr_lsw(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
