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
/// L2CP table entry
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct L2CP_ENTRY_CFG(u32);
impl L2CP_ENTRY_CFG {
    /// Enable use of COSID_VAL as COS ID.
    #[inline]
    pub fn cosid_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline]
    pub fn set_cosid_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// COS ID for L2CP frame.
    #[inline]
    pub fn cosid_val(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    #[inline]
    pub fn set_cosid_val(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 6;
        self.0 &= !0x1c0;
        self.0 |= value;
    }
    /// CPU forward configuration for L2CP frame.
    ///
    /// 0: Normal forward 1: Enable redirection to CPU queue 2: Enable copy to CPU queue 3: Discard the frame
    #[inline]
    pub fn cpu_fwd_cfg(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    #[inline]
    pub fn set_cpu_fwd_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 3;
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// CPU extraction queue for L2CP frame copied or redirected to CPU by CPU_FWD_CFG.
    #[inline]
    pub fn cpu_l2cp_qu(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline]
    pub fn set_cpu_l2cp_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
