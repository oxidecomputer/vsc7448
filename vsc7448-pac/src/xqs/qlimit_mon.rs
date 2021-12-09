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
/// Maximum amount of congested scheduling elements in the share
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct QLIMIT_CONG_CNT_MAX_STAT(u32);
impl QLIMIT_CONG_CNT_MAX_STAT {
    pub fn qlimit_cong_cnt_max(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_qlimit_cong_cnt_max(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// Monitor configuration
///
/// Replicated for each classified priority
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct QLIMIT_MON_CFG(u32);
impl QLIMIT_MON_CFG {
    /// Clear shared memory pool monitoring statistics.
    pub fn qlimit_shr_mon_clr(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_qlimit_shr_mon_clr(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Current per SE watermark
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct QLIMIT_SHR_WM_STAT(u32);
impl QLIMIT_SHR_WM_STAT {
    pub fn qlimit_shr_wm(&self) -> u32 {
        self.0 & 0x7fff
    }
    pub fn set_qlimit_shr_wm(&mut self, value: u32) {
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}
