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
/// PCS XAUI Test Pattern Configuration
///
/// Test Pattern Generator/Checker Control Register
#[derive(From, Into)]
pub struct PCS_XAUI_TSTPAT_CFG(u32);
impl PCS_XAUI_TSTPAT_CFG {
    /// Capture current error counter values
    ///
    /// 1: Capture
    pub fn freeze_err_cnt_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_freeze_err_cnt_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Enable Test pattern checker
    ///
    /// 1: Check 0: No checking
    pub fn vt_chk_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_vt_chk_ena(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Check test pattern
    ///
    /// 000: Idle 011: MFPAT 100: CRPAT 101: CJPAT all others: Idle Note: LFPAT and HFPAT can not be checked since sync is impossible
    pub fn vt_chk_sel(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_vt_chk_sel(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }
    /// Enable Test pattern generator
    ///
    /// 1: Generate test vectors 0: Normal operation
    pub fn vt_gen_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_vt_gen_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Generate test pattern
    ///
    /// 000: Idle 001: HFPAT 010: LFPAT 011: MFPAT 100: CRPAT 101: CJPAT All others: Idle
    pub fn vt_gen_sel(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_vt_gen_sel(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Test Pattern Receive Sequence Counter
///
/// Random Sequence Master Counter. The counter is used only if frame based pattern are to be checked (CRPAT, CJPAT) and it is incremented by one every 8th received symbol. The counter is started when a start of frame is detected and the counter stops when the last symbol of the (internally stored) reference frame was compared. The idle phase between two frames is not checked.
#[derive(From, Into)]
pub struct PCS_XAUI_TSTPAT_RX_SEQ_CNT_STATUS(u32);
impl PCS_XAUI_TSTPAT_RX_SEQ_CNT_STATUS {
    /// Random sequence master counter
    pub fn rnd_seq_timer(&self) -> u32 {
        self.0
    }
    pub fn set_rnd_seq_timer(&mut self, value: u32) {
        self.0 = value;
    }
}
