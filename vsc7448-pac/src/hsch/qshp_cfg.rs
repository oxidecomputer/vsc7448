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
/// Configuration of queue shaper
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct QSHP_CFG(u32);
impl QSHP_CFG {
    /// Accounting mode for this shaper.
    ///
    /// 0: Line rate. Shape bytes including HSCH_MISC_CFG.FRM_ADJ. 1: Data rate. Shape bytes excluding IPG. 2. Frame rate. Shape frames with rate unit = 100 fps and burst unit = 32.8 frames. 3: Frame rate. Shape framed with rate unit = 1 fps and burst unit = 0.3 frames.
    #[inline]
    pub fn se_frm_mode(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline]
    pub fn set_se_frm_mode(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
/// Queue Shaping configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct QSHP_CIR_CFG(u32);
impl QSHP_CIR_CFG {
    /// Burst capacity of this shaper. Unit is 4096 kilobytes. The shaper is disabled when CIR_BURST=0.
    #[inline]
    pub fn cir_burst(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline]
    pub fn set_cir_burst(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// Leak rate for this shaper. Unit is defined by the leak list period the shaper is attached to (see HSCH_LEAK_CFG.LEAK_TIME).
    #[inline]
    pub fn cir_rate(&self) -> u32 {
        (self.0 & 0x7fffc0) >> 6
    }
    #[inline]
    pub fn set_cir_rate(&mut self, value: u32) {
        assert!(value <= 0x1ffff);
        let value = value << 6;
        self.0 &= !0x7fffc0;
        self.0 |= value;
    }
}
