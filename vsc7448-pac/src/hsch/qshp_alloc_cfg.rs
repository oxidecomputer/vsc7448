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

/// Register `QSHP_ALLOC_CFG`
///
/// Assign queue shapers to queues
#[derive(From, Into)]
pub struct QSHP_ALLOC_CFG(u32);
impl QSHP_ALLOC_CFG {    ///
    /// First input using queue shapers are using this queue shaper index.
    pub fn qshp_base(&self) -> u32 {
        (self.0 & 0x1fff) >> 0
    }
    pub fn set_qshp_base(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fff);
        self.0 &= !0x1fff;
        self.0 |= value;
    }    ///
    /// Last input using queue shapers. A value lower than the SHP_MIN input disables queue shaping.
    pub fn qshp_max(&self) -> u32 {
        (self.0 & 0x7e000) >> 13
    }
    pub fn set_qshp_max(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x7e000);
        self.0 &= !0x7e000;
        self.0 |= value;
    }    ///
    /// First input using queue shapers.
    pub fn qshp_min(&self) -> u32 {
        (self.0 & 0x1f80000) >> 19
    }
    pub fn set_qshp_min(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x1f80000);
        self.0 &= !0x1f80000;
        self.0 |= value;
    }
}

/// Register `QSHP_CFG`
///
/// Configuration of queue shaper
#[derive(From, Into)]
pub struct QSHP_CFG(u32);
impl QSHP_CFG {    ///
    /// Accounting mode for this shaper.
    ///
    /// 0: Line rate. Shape bytes including HSCH_MISC_CFG.FRM_ADJ. 1: Data rate. Shape bytes excluding IPG. 2. Frame rate. Shape frames with rate unit = 100 fps and burst unit = 32.8 frames. 3: Frame rate. Shape framed with rate unit = 1 fps and burst unit = 0.3 frames.
    pub fn se_frm_mode(&self) -> u32 {
        (self.0 & 0x3) >> 0
    }
    pub fn set_se_frm_mode(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}