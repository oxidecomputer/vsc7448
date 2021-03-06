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
/// Counter for number of frames discarded towards the cpu
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct FWD_CPU_DROP_CNT(u32);
impl FWD_CPU_DROP_CNT {
    /// Counts number of frames discarded towards the cpu, since queue system reset.
    #[inline(always)]
    pub fn fwd_cpu_drop_cnt(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_fwd_cpu_drop_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Forwarder mischeleaneous configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct FWD_CTRL(u32);
impl FWD_CTRL {
    /// The maximum number of clock cycles between guaranteed CSR access to res_stat counters.
    #[inline(always)]
    pub fn fwd_idle_cnt(&self) -> u32 {
        self.0 & 0x1fff
    }
    #[inline(always)]
    pub fn set_fwd_idle_cnt(&mut self, value: u32) {
        assert!(value <= 0x1fff);
        self.0 &= !0x1fff;
        self.0 |= value;
    }
}
/// QS drop events per port and copy type
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct FWD_DROP_EVENTS(u32);
impl FWD_DROP_EVENTS {
    /// A CPU directed frame copy was canceled.
    #[inline(always)]
    pub fn fwd_drop_cpu_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_fwd_drop_cpu_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// A learnall frame copy was canceled.
    #[inline(always)]
    pub fn fwd_drop_learn_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_fwd_drop_learn_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// A mirror copy was canceled.
    #[inline(always)]
    pub fn fwd_drop_mirr_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_fwd_drop_mirr_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// A switched frame copy was canceled to one or more egress ports.
    #[inline(always)]
    pub fn fwd_drop_norm_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_fwd_drop_norm_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// A frame copy was discarded due to a queuemapping violation.
    #[inline(always)]
    pub fn fwd_drop_qmap_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_fwd_drop_qmap_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Counters showing forwarding runs. Debugging purposes only
///
/// Three counters exists in the forwarder. Replication 0 counts each executed queue head processing. Replication 1 counts each occurance of a delayed processing due to egress queue system pressure, and replication 2 counts occurances of delayed process due to statistics event pressure.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct FWD_STAT_CNT(u32);
impl FWD_STAT_CNT {
    /// Counts number of forwarding events since chip reset.
    #[inline(always)]
    pub fn fwd_stat_cnt(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_fwd_stat_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Index into large tables
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MAP_CFG_CFG(u32);
impl MAP_CFG_CFG {
    /// This value is used for indexing into the QMAP_QOS_TBL, QMAP_SE_TBL, and QLIMIT_QUEUE tables.
    #[inline(always)]
    pub fn map_cfg_cfg(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_map_cfg_cfg(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Basic port mode for scheduling hierarchy
///
/// All ports have their own scheduling hierarchy defined, consisting of a part for nonservice frames defined as frames having queing group classified to 0, and a part for the rest. The two parts can be identical if needed, in which case the modes for service and non service must have the same setting.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct QMAP_PORT_MODE(u32);
impl QMAP_PORT_MODE {
    /// Same function as for QMAP_MODE_SERVICE, except this mode is for qgrp=0 frames.
    #[inline(always)]
    pub fn qmap_mode_nonservice(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_qmap_mode_nonservice(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Scheduling mode for frames classified to queing group /=0 (service frames). In NORMAL mode, the queue number is looked up in the map tables at index 1024+qos_value. In GROUP mode, a scheduling element is assigned per queueing group, with 8 queues attached. The queue number is looked up in the tables at index qgrp. In MBH mode, the qos is looked up at index qgrp, and the scheduling index is looked up at index "(grp and not 0xf) + qos_value". A minimum hierachy also exist. It corresponds to GROUP mode, but the qgrp is regarded zero in the mapping.
    ///
    /// 0: Normal mode 1: Hier mode 2: Mbh mode 3: Mini mode
    #[inline(always)]
    pub fn qmap_mode_service(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    #[inline(always)]
    pub fn set_qmap_mode_service(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 2;
        self.0 &= !0xc;
        self.0 |= value;
    }
}
/// Statistics configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct STAT_CFG(u32);
impl STAT_CFG {
    /// Set STAT_CLEAR_SHOT to clear counters for the port or service index selected by STAT_VIEW. Auto-cleared when complete (1us). Multiple counters can be cleared at the same time by setting multiple bits in STAT_CLEAR_SHOT.
    ///
    /// Bit 0: Clear Rx port counters (Packet, LS byte and MS byte) Bit 1: Clear Tx port counters (Packet, LS byte and MS byte) Bit 2: Clear ingress service counters (Packet, LS byte and MS byte) Bit 3: Clear egress service counters (Packet, LS byte and MS byte) When bits 0-1 are used a port number must be configured in STAT_VIEW. When bits 2 is used an ingress service index must be configured in STAT_VIEW. When bits 3 is used an egress service index must be configured in STAT_VIEW.
    #[inline(always)]
    pub fn stat_clear_shot(&self) -> u32 {
        (self.0 & 0x3c0000) >> 18
    }
    #[inline(always)]
    pub fn set_stat_clear_shot(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 18;
        self.0 &= !0x3c0000;
        self.0 |= value;
    }
    /// Set to enable use of all of the service counter memory for packet counting.
    #[inline(always)]
    pub fn stat_srv_pkt_only(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_stat_srv_pkt_only(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Selects the port or service for which counters can be accessed using QSYS:STAT. Also used to select the port or service index for which to clear statistics counters, ref. STAT_CFG.STAT_CLEAR_SHOT.
    #[inline(always)]
    pub fn stat_view(&self) -> u32 {
        (self.0 & 0x3ffe0) >> 5
    }
    #[inline(always)]
    pub fn set_stat_view(&mut self, value: u32) {
        assert!(value <= 0x1fff);
        let value = value << 5;
        self.0 &= !0x3ffe0;
        self.0 |= value;
    }
    /// Counters are by default wrapping when exceeding their maximum value, and software must thus do a subtraction with the previous readen value to see how much the total count has changed. If wrapping is disabled, the counters will clear on read, and saturate at their maximum value. Software can thus detect that a counter overflow has happened, and do not need storing the previous read values. The configuration exists replicated per statistics group as the STAT_CLEAR_SHOT describes.
    #[inline(always)]
    pub fn stat_wrap_dis(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_stat_wrap_dis(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// Statistics configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct STAT_CNT_CFG(u32);
impl STAT_CNT_CFG {
    /// When set, a frame discarded due to lack of resources is counted on the egress port instead of the ingress. Side effect is a slower processing of multiple drops on the same frame, causing potential head-of-line blocking.
    #[inline(always)]
    pub fn drop_count_egress(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_drop_count_egress(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
