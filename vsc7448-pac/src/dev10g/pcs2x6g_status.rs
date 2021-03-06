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
/// PCS2X6G Error Counter
///
/// PCS2X6G error counter
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCS2X6G_ERR_CNT_STAT(u32);
impl PCS2X6G_ERR_CNT_STAT {
    /// Number of errors detected in 64B/66B decoder.
    #[inline(always)]
    pub fn err_64b66bdec(&self) -> u32 {
        self.0 & 0xffffff
    }
    #[inline(always)]
    pub fn set_err_64b66bdec(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// PCS2X6G Error Status
///
/// Error indication of 64B/66B PCS2X6G logic
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCS2X6G_ERR_STATUS(u32);
impl PCS2X6G_ERR_STATUS {
    /// Alignment lost in deskew logic
    ///
    /// 0: No misalignment occured 1: A (temporary) misalignment has been detected Bit is cleared by writing a 1 to this position.
    #[inline(always)]
    pub fn alignment_lost_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_alignment_lost_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Coding error detected in received 64B/66B encoded data
    ///
    /// 0: No error found 1: Coding error detected Bit is cleared by writing a 1 to this position.
    #[inline(always)]
    pub fn c64b66b_err_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_c64b66b_err_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Synchronization lost in lane i (i = 0...3, one bit per lane)
    ///
    /// 0: No sync lost occured 1: Synchronization lost in lane i (temporarily) Bit is cleared by writing a 1 to this position.
    #[inline(always)]
    pub fn sync_lost_sticky(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_sync_lost_sticky(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Coding error detected in xgmii data to be transmitted
    ///
    /// 0: No error found 1: Coding error detected Bit is cleared by writing a 1 to this position.
    #[inline(always)]
    pub fn xgmii_err_sticky(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_xgmii_err_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
}
/// PCS2X6G Status
///
/// Status of PCS2X6G logic
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCS2X6G_STATUS(u32);
impl PCS2X6G_STATUS {
    /// Status of deskew logic
    ///
    /// 0: Lanes not aligned 1: All lanes are aligned
    #[inline(always)]
    pub fn rx_alignment_status(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_rx_alignment_status(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Status of synchronization in lane i (i = 0...3, one bit per lane)
    ///
    /// 0: Lane i out of sync 1: Lane i is in sync
    #[inline(always)]
    pub fn rx_sync_status(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_rx_sync_status(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Current status of selected signal_detect input lines
    #[inline(always)]
    pub fn signal_detect(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    #[inline(always)]
    pub fn set_signal_detect(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 12;
        self.0 &= !0xf000;
        self.0 |= value;
    }
}
