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
/// Shaping configuration of the SE
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CIR_CFG(u32);
impl CIR_CFG {
    /// Burst capacity of this shaper. Unit is 4096 bytes. The shaper is disabled when CIR_BURST=0.
    #[inline(always)]
    pub fn cir_burst(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_cir_burst(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// Leak rate for this shaper. Unit is defined by the leak list period the shaper is attached to (see HSCH_LEAK_CFG.LEAK_TIME).
    #[inline(always)]
    pub fn cir_rate(&self) -> u32 {
        (self.0 & 0x7fffc0) >> 6
    }
    #[inline(always)]
    pub fn set_cir_rate(&mut self, value: u32) {
        assert!(value <= 0x1ffff);
        let value = value << 6;
        self.0 &= !0x7fffc0;
        self.0 |= value;
    }
}
/// Excess rate configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct EIR_CFG(u32);
impl EIR_CFG {
    /// Burst capacity of this shaper. Unit is 4096 bytes. The dual leaky bucket shaper operates as a single leaky bucket shaper when EIR_BURST=0.
    #[inline(always)]
    pub fn eir_burst(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_eir_burst(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// Leak rate for this shaper. Same unit as CIR_RATE unit.
    #[inline(always)]
    pub fn eir_rate(&self) -> u32 {
        (self.0 & 0x7fffc0) >> 6
    }
    #[inline(always)]
    pub fn set_eir_rate(&mut self, value: u32) {
        assert!(value <= 0x1ffff);
        let value = value << 6;
        self.0 &= !0x7fffc0;
        self.0 |= value;
    }
}
/// Configuration of shaper and scheduling algorithm
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SE_CFG(u32);
impl SE_CFG {
    /// Enable AVB mode for this shaper, creating burst capacity only when data is available.
    #[inline(always)]
    pub fn se_avb_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_se_avb_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Number of inputs running with DWRR algorithm, otherwise strict. Strict inputs always have the highest input index.
    ///
    /// 0: No inputs uses DWRR 1: 2 lowest inputs used DWRR n: (n+1) lowest inputs uses DWRR
    #[inline(always)]
    pub fn se_dwrr_cnt(&self) -> u32 {
        (self.0 & 0xfc0) >> 6
    }
    #[inline(always)]
    pub fn set_se_dwrr_cnt(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 6;
        self.0 &= !0xfc0;
        self.0 |= value;
    }
    /// Accounting mode for the dwrr distribution.
    ///
    /// 0: Line rate. Cost is frame length including HSCH_MISC_CFG.FRM_ADJ. 1: Data rate. Cost is frame length excluding IPG. 2. Frame rate. Cost is 1. 3: Reserved.
    #[inline(always)]
    pub fn se_dwrr_frm_mode(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_se_dwrr_frm_mode(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
    /// Accounting mode for this shaper.
    ///
    /// 0: Line rate. Shape bytes including HSCH_MISC_CFG.FRM_ADJ. 1: Data rate. Shape bytes excluding IPG. 2. Frame rate. Shape frames with rate unit = 100 fps and burst unit = 32.8 frames. 3: Frame rate. Shape framed with rate unit = 1 fps and burst unit = 0.3 frames.
    #[inline(always)]
    pub fn se_frm_mode(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    #[inline(always)]
    pub fn set_se_frm_mode(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 3;
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// Block traffic through this element. This can be used for transfering element to other locations in the scheduling hierarchy
    ///
    /// 0: Traffic can flow through this element 1: This element will block its output
    #[inline(always)]
    pub fn se_stop(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_se_stop(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Configuration of the connections between SEs
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SE_CONNECT(u32);
impl SE_CONNECT {
    /// Forms the leak chains.
    #[inline(always)]
    pub fn se_leak_link(&self) -> u32 {
        self.0 & 0x7fff
    }
    #[inline(always)]
    pub fn set_se_leak_link(&mut self, value: u32) {
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}
/// Configuration of which fill levels in the queue system that the DLB shapers use to trigger excess information rate
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SE_DLB_SENSE(u32);
impl SE_DLB_SENSE {
    /// Destination port used when SE_DLB_DPORT_ENA is set.
    #[inline(always)]
    pub fn se_dlb_dport(&self) -> u32 {
        (self.0 & 0x3f00) >> 8
    }
    #[inline(always)]
    pub fn set_se_dlb_dport(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 8;
        self.0 &= !0x3f00;
        self.0 |= value;
    }
    /// Enable destination port awareness by the DLB shaper. If set, the DLB shaper allows excess information rate when the egress buffer use for destination port SE_DLB_SPORT has reached threshold BUF_P_RSRV_E[SE_DLB_DPORT] minus 3000 bytes. If multiple awareness functions (SE_DLB_PRIO_ENA, SE_DLB_SPORT_ENA, SE_DLB_DPORT_ENA) are enabled, all relevant thresholds must be exceeded before excess information rate is allowed.
    #[inline(always)]
    pub fn se_dlb_dport_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_se_dlb_dport_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// QoS class used when SE_DLB_PRIO_ENA is set.
    #[inline(always)]
    pub fn se_dlb_prio(&self) -> u32 {
        (self.0 & 0x1c000) >> 14
    }
    #[inline(always)]
    pub fn set_se_dlb_prio(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 14;
        self.0 &= !0x1c000;
        self.0 |= value;
    }
    /// Enable priority awareness by the DLB shaper. If set, the DLB shaper allows excess information rate when the egress buffer use for QoS class SE_DLB_PRIO has reached threshold BUF_PRIO_SHR_E[SE_DLB_PRIO] minus 3000 bytes. If both SE_DLB_PRIO_ENA and SE_DLB_SPORT_ENA are set, the excess information rate is alllowed when the ingress buffer use for QoS class per source port has reached threshold BUF_Q_RSRV_I[SE_DLB_PRIO, SE_DLB_SPORT] minus 3000 bytes. If both SE_DLB_PRIO_ENA and SE_DLB_DPORT_ENA are set, the excess information rate is alllowed when the egress buffer use for QoS class per destination port has reached threshold BUF_Q_RSRV_E[SE_DLB_PRIO, SE_DLB_DPORT] minus 3000 bytes. If multiple awareness functions (SE_DLB_PRIO_ENA, SE_DLB_SPORT_ENA, SE_DLB_DPORT_ENA) are enabled, all relevant thresholds must be exceeded before excess information rate is allowed.
    #[inline(always)]
    pub fn se_dlb_prio_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_se_dlb_prio_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Not connected.
    #[inline(always)]
    pub fn se_dlb_sport_obsolete(&self) -> u32 {
        (self.0 & 0xfc) >> 2
    }
    #[inline(always)]
    pub fn set_se_dlb_sport_obsolete(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 2;
        self.0 &= !0xfc;
        self.0 |= value;
    }
}
