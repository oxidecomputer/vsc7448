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

/// Register `CIR_CFG`
///
/// Shaping configuration of the SE
#[derive(From, Into)]
pub struct CIR_CFG(u32);
impl CIR_CFG {    ///
    /// Burst capacity of this shaper. Unit is 4096 bytes. The shaper is disabled when CIR_BURST=0.
    pub fn cir_burst(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_cir_burst(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }    ///
    /// Leak rate for this shaper. Unit is defined by the leak list period the shaper is attached to (see HSCH_LEAK_CFG.LEAK_TIME).
    pub fn cir_rate(&self) -> u32 {
        (self.0 & 0x7fffc0) >> 6
    }
    pub fn set_cir_rate(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x7fffc0);
        self.0 &= !0x7fffc0;
        self.0 |= value;
    }
}

/// Register `EIR_CFG`
///
/// Excess rate configuration
#[derive(From, Into)]
pub struct EIR_CFG(u32);
impl EIR_CFG {    ///
    /// Burst capacity of this shaper. Unit is 4096 bytes. The dual leaky bucket shaper operates as a single leaky bucket shaper when EIR_BURST=0.
    pub fn eir_burst(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_eir_burst(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }    ///
    /// Leak rate for this shaper. Same unit as CIR_RATE unit.
    pub fn eir_rate(&self) -> u32 {
        (self.0 & 0x7fffc0) >> 6
    }
    pub fn set_eir_rate(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x7fffc0);
        self.0 &= !0x7fffc0;
        self.0 |= value;
    }
}

/// Register `HSCH_L1W`
///
/// Winner table
#[derive(From, Into)]
pub struct HSCH_L1W(u32);
impl HSCH_L1W {    ///
    /// Contains the layer 0 scheduling history descriptor for a frames passing through layer-1 element A, input B. Group replication index is A and register replication is B.
    pub fn win_shist(&self) -> u32 {
        (self.0 & 0x7fffff) >> 0
    }
    pub fn set_win_shist(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fffff);
        self.0 &= !0x7fffff;
        self.0 |= value;
    }
}

/// Register `SE_CFG`
///
/// Configuration of shaper and scheduling algorithm
#[derive(From, Into)]
pub struct SE_CFG(u32);
impl SE_CFG {    ///
    /// Enable AVB mode for this shaper, creating burst capacity only when data is available.
    pub fn se_avb_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_se_avb_ena(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }    ///
    /// Number of inputs running with DWRR algorithm, otherwise strict. Strict inputs always have the highest input index.
    ///
    /// 0: No inputs uses DWRR 1: 2 lowest inputs used DWRR n: (n+1) lowest inputs uses DWRR
    pub fn se_dwrr_cnt(&self) -> u32 {
        (self.0 & 0xfc0) >> 6
    }
    pub fn set_se_dwrr_cnt(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0xfc0);
        self.0 &= !0xfc0;
        self.0 |= value;
    }    ///
    /// Accounting mode for the dwrr distribution.
    ///
    /// 0: Line rate. Cost is frame length including HSCH_MISC_CFG.FRM_ADJ. 1: Data rate. Cost is frame length excluding IPG. 2. Frame rate. Cost is 1. 3: Reserved.
    pub fn se_dwrr_frm_mode(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_se_dwrr_frm_mode(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x6);
        self.0 &= !0x6;
        self.0 |= value;
    }    ///
    /// Accounting mode for this shaper.
    ///
    /// 0: Line rate. Shape bytes including HSCH_MISC_CFG.FRM_ADJ. 1: Data rate. Shape bytes excluding IPG. 2. Frame rate. Shape frames with rate unit = 100 fps and burst unit = 32.8 frames. 3: Frame rate. Shape framed with rate unit = 1 fps and burst unit = 0.3 frames.
    pub fn se_frm_mode(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    pub fn set_se_frm_mode(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x18);
        self.0 &= !0x18;
        self.0 |= value;
    }    ///
    /// Block traffic through this element. This can be used for transfering element to other locations in the scheduling hierarchy
    ///
    /// 0: Traffic can flow through this element 1: This element will block its output
    pub fn se_stop(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_se_stop(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `SE_CONNECT`
///
/// Configuration of the connections between SEs
#[derive(From, Into)]
pub struct SE_CONNECT(u32);
impl SE_CONNECT {    ///
    /// Forms the leak chains.
    pub fn se_leak_link(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_se_leak_link(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}