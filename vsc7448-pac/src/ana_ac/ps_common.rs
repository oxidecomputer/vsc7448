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
/// Data register for core memory access.
#[derive(From, Into)]
pub struct CM_DATA(u32);
impl CM_DATA {
    /// Data register for core memory access. Wider memories are big endian mapped into the 32 BIT inspection space.
    pub fn cm_data(&self) -> u32 {
        self.0
    }
    pub fn set_cm_data(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Common stacking parameters
#[derive(From, Into)]
pub struct COMMON_EQUAL_STACK_LINK_TTL_CFG(u32);
impl COMMON_EQUAL_STACK_LINK_TTL_CFG {
    /// Controls flooding in ring topology stacks. When set, frames to be flooded will be forwarded on only stack port A or only stack port B. TTL will be set to VSTAX2_EQUAL_STACK_LINK_TTL_VAL. This feature can be used to forward flooded frames in one or the other way around the ring, In such case VSTAX2_EQUAL_STACK_LINK_TTL_VAL must be set to the number of units in the ring minus 1. Frames with even AC will be forwarded on stack port A. Frames with odd AC will be forwarded on stack port B. Note that this feature is not related to equal cost forwarding. It is applicable to any ring topology stack regardless of whether the number of units is even or odd.
    ///
    /// 0: Disable (flood on both stack ports using the stack port's TTL) 1: Enable (flood on one stack port using VSTAX2_EQUAL_STACK_LINK_TTL_VAL)
    pub fn vstax2_equal_stack_links_flood_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_vstax2_equal_stack_links_flood_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// TTL value used for equal cost path. Only applicable for ring topology stacks with an even number of units. Must be set to number of units in the stack divided by two. Use of equal cost paths is enabled by ANA_AC:UPSID:STACK_LINK_EQUAL_COST_CFG.STACK_LINK_EQUAL_ENA
    pub fn vstax2_equal_stack_link_ttl_val(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_vstax2_equal_stack_link_ttl_val(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}
/// Common stacking parameters
#[derive(From, Into)]
pub struct COMMON_VSTAX_CFG(u32);
impl COMMON_VSTAX_CFG {
    /// Specifies own UPSID This must be configured consistently across the following registers: ANA_CL::UPSID_CFG.UPSID_NUM ANA_AC::COMMON_VSTAX_CFG.OWN_UPSID ANA_L2::VSTAX_CTRL.OWN_UPSID REW::COMMON_CTRL.OWN_UPSID
    pub fn own_upsid(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_own_upsid(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
    /// Allow FWD_LOGICAL entries in MAC table to control internal port number in VStaX header. If MAC table entry for DMAC specifies vstax.general.fwd_mode=FWD_LOGICAL and a destination port number of type port_type_intpn, then this bit controls the outgoing destination port number in the VStaX header. If this bit is clear, then the destination port number in VStaX header is set to intpn=15, regardless of the port number specified in the MAC table entry. If this bit is set, then the destination port number in VStaX header is set to the value of the port number specified in the MAC table entry.
    ///
    /// 0: Disable (set destination port to intpn=15) 1: Enable (set destination port to value in MAC table)
    pub fn vstax2_allow_upsid_cpu_or_int_pn_ena(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_vstax2_allow_upsid_cpu_or_int_pn_ena(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    /// CPU queue for VStaX frames forwarding error.
    ///
    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn vstax2_fwd_err_qu(&self) -> u32 {
        (self.0 & 0x700) >> 8
    }
    pub fn set_vstax2_fwd_err_qu(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x700);
        self.0 &= !0x700;
        self.0 |= value;
    }
    /// This can be used to enable GLAG in a stack If enabled the upper GLAG*8 number of PGID multicast entries will be used for GLAG
    ///
    /// 0: Disable 1: Enable
    pub fn vstax2_glag_ena(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_vstax2_glag_ena(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Disable counting of bytes and frames in ANA_AC:STAT_CNT_CFG_ISDX for frames received on a stack port. Related parameters: REW:COMMON:CNT_CTRL.VSTAX_STAT_ESDX_DIS
    pub fn vstax2_isdx_stat_dis(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    pub fn set_vstax2_isdx_stat_dis(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x2000000);
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    /// Forward unicast frames from front ports as fwd_mode==fwd_llookup (instead of fwd_logical)
    ///
    /// 0: Disable 1: Enable
    pub fn vstax2_logical_llookup_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_vstax2_logical_llookup_ena(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Forward multicast frames as fwd_mode==fwd_llookup (instead of fwd_mc). Note that PTP frames will always be forwarded using fwd_llookup, regardless of the value of VSTAX2_MC_LLOOKUP_ENA.
    ///
    /// 0: Disable 1: Enable
    pub fn vstax2_mc_llookup_ena(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_vstax2_mc_llookup_ena(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Forward non flooded multicast frames w. fwd_mode==fwd_llookup (instead of fwd_mc)
    ///
    /// 0: Disable 1: Enable
    pub fn vstax2_mc_llookup_non_flood_ena(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_vstax2_mc_llookup_non_flood_ena(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Configures handling of IP multicast routing on stack link. If IP multicast routing is performed in egress unit, then routed copies shall not be forwarded on the stack ports and this parameter must thus be set to 0. If IP multicast routing is performed in ingress unit (or centralized), then routed copies shall be forwarded on the stack ports and this parameter must thus be set to 3.
    ///
    /// 0: Disable multicast on both links 1: Enable multicast routing on stack link A 2: Enable multicast routing on stack link B 3: Enable multicast routing on both links
    pub fn vstax2_mc_route_to_stack_link_ena(&self) -> u32 {
        (self.0 & 0x60000) >> 17
    }
    pub fn set_vstax2_mc_route_to_stack_link_ena(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x60000);
        self.0 &= !0x60000;
        self.0 |= value;
    }
    /// Enable putting DSCP into VStaX header in positions 73-68, provided that VStaX header contains AC (and not ISDX) in misc section, i.e. VSTAX2_MISC_ISDX_ENA=0. Related parameters: ANA_AC:PS_COMMON:COMMON_VSTAX_CFG.VSTAX2_MISC_ISDX_ENA ANA_CL:PORT:STACKING_CTRL.VSTAX_ISDX_ENA
    pub fn vstax2_misc_dscp_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_vstax2_misc_dscp_ena(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Enable putting ISDX into VStaX header. Related parameters: ANA_CL:PORT:STACKING_CTRL.VSTAX_ISDX_ENA
    ///
    /// 0: Disable (VSTAX MISC contains Aggregation code) 1: Enable (VSTAX MISC contains ISDX)
    pub fn vstax2_misc_isdx_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_vstax2_misc_isdx_ena(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Allow routing of VStaX frames received on stack port with vstax.fwd_mode==FWD_LOGICAL with vstax.upsid=COMMON_VSTAX_CFG.OWN_UPSID regardless of vstax.dst.dst_port.
    ///
    /// 0: Disable (allow only routing of FWD_LOGICAL if dst_port_type == port_type_intpn and dst_pn == intpn_dlookup or dst_pn == intpn_router) 1: Enable
    pub fn vstax2_rt_all_fwd_logical_ena(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_vstax2_rt_all_fwd_logical_ena(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }
    /// Configures value used for vstax.lrn_mode when forwarding routed IP multicast frames on stack port(s).
    ///
    /// 0: lrn_mode=lrn_normal 1: lrn_mode=lrn_skip
    pub fn vstax2_rt_mc_src_lrn_skip_ena(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_vstax2_rt_mc_src_lrn_skip_ena(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Allow update of vstax.src field for multicast routed frames.
    ///
    /// 0: Disable 1: Enable
    pub fn vstax2_rt_mc_src_update_ena(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_vstax2_rt_mc_src_update_ena(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Configures value used for vstax.lrn_mode when forwarding routed IP unicast frames on stack port(s).
    ///
    /// 0: lrn_mode=lrn_normal 1: lrn_mode=lrn_skip
    pub fn vstax2_rt_uc_src_lrn_skip_ena(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_vstax2_rt_uc_src_lrn_skip_ena(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Allow update of vstax.src field for unicast routed frames
    ///
    /// 0: Disable 1: Enable
    pub fn vstax2_rt_uc_src_update_ena(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_vstax2_rt_uc_src_update_ena(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// This feature is to be used in 48 port switches consisting of two units (i.e. chips) and where the CPU is disabled one of the units. Only the unit with the disabled CPU shall have vstax2_ttl_keep==1
    ///
    /// 0: Disable 1: Enable
    pub fn vstax2_ttl_keep_ena(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_vstax2_ttl_keep_ena(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Forward unicast frames from front ports taking congestion management into account
    ///
    /// 0: Disable 1: Enable
    pub fn vstax2_use_cm_ena(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_vstax2_use_cm_ena(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
}
/// CPU related parameters
#[derive(From, Into)]
pub struct CPU_CFG(u32);
impl CPU_CFG {
    /// Specifies a group of CPU queues which will only receive one frame copy in total
    ///
    /// '00000000' : A frame copy will be generated for all CPU queues applicable for reception ... 'xxxxxx11' : Only one CPU copy will be generated for CPU queue 1 and 0 ... '11111111' : At most one frame copy will be generated for all CPU queues applicable for reception
    pub fn one_cpu_copy_only_mask(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_one_cpu_copy_only_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Miscellaneous control parameters
#[derive(From, Into)]
pub struct MISC_CTRL(u32);
impl MISC_CTRL {
    /// Increase DP level for flooded traffic.
    ///
    /// 0: Disabled 1: Increase DP by 1 for flooded traffic 2: Increase DP by 2 for flooded traffic 3: Increase DP by 3 for flooded traffic
    pub fn cmac_flood_dp_inc(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_cmac_flood_dp_inc(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// Avoid CPU sending back to CPU.
    pub fn cpu_to_cpu_kill_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_cpu_to_cpu_kill_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enable use of VID instead of ISDX_BASE_ADDR as index to statistics in ANA_AC:STAT_CNT_CFG_ISDX. Related parameters: ANA_L2::ISDX_BASE_CFG.ISDX_BASE_ADDR REW:COMMON:CNT_CTRL.STAT_MODE.
    pub fn use_vid_as_isdx_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_use_vid_as_isdx_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
}
/// Enable aggregation using physical source port number
#[derive(From, Into)]
pub struct PHYS_SRC_AGGR_CFG(u32);
impl PHYS_SRC_AGGR_CFG {
    /// Enable use of physical source port number for aggregation towards this port. This is intended to be used when interconnecting two devices using two interconnect ports, which are aggregated in a LLAG, to form a 48 port system.
    ///
    /// 'xxx0xx': Use normal aggregation code when finding aggregation mask for this port 'xxx1xx': Use only physical source port number in aggregation code when finding aggregation mask for this port.
    pub fn phys_src_aggr_mask(&self) -> u32 {
        self.0
    }
    pub fn set_phys_src_aggr_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Enable aggregation using physical source port number
#[derive(From, Into)]
pub struct PHYS_SRC_AGGR_CFG1(u32);
impl PHYS_SRC_AGGR_CFG1 {
    /// Refer to PHYS_SRC_AGGR_CFG.PHYS_SRC_AGGR_MASK description.
    pub fn phys_src_aggr_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_phys_src_aggr_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Various, common configuration parameters
#[derive(From, Into)]
pub struct PS_COMMON_CFG(u32);
impl PS_COMMON_CFG {
    /// Enable sFlow sampling.
    pub fn sflow_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_sflow_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enable sFlow Sampler ID in sFlow Stamp. The "sFlow Stamp" replaces the frame's FCS. With this bit disabled the sFlow Stamp format becomes: Bit 31:8: Frame forwarding count. Bit 7:0: Sample count. With this bit enabled the sFlow Stamp format becomes: Bit 31:26: Sampler ID Bit 25:8: Frame forwarding count. Bit 7:0: Sample count.
    pub fn sflow_smpl_id_in_stamp_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_sflow_smpl_id_in_stamp_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable lookup of logical source port. If disabled, the source mask does not affect the egress port mask.
    pub fn src_lookup_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_src_lookup_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// sFlow configuration
///
/// This register configures the sFlow sampler.
#[derive(From, Into)]
pub struct SFLOW_CFG(u32);
impl SFLOW_CFG {
    /// CPU queue for frames copied to the CPU by an sFlow Sampler.
    pub fn sflow_cpu_qu(&self) -> u32 {
        (self.0 & 0x7000) >> 12
    }
    pub fn set_sflow_cpu_qu(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x7000);
        self.0 &= !0x7000;
        self.0 |= value;
    }
}
/// sFlow counter reset
///
/// This register configures the sFlow Sampler reset.
#[derive(From, Into)]
pub struct SFLOW_RESET_CTRL(u32);
impl SFLOW_RESET_CTRL {
    /// When set to 1, the following counters are reset for all sFlow Samplers: ANA_AC:SFLOW:SFLOW_CNT.SFLOW_SAMPLE_CNT ANA_AC:SFLOW:SFLOW_CNT.SFLOW_SAMPLE_FWD_CNT Upon resetting the counters, the device will set the parameter back to 0.
    ///
    /// 0: No action 1: Reset counters.
    pub fn sflow_frame_reset_shot(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_sflow_frame_reset_shot(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Port mask for Stack Port A
#[derive(From, Into)]
pub struct STACK_A_CFG(u32);
impl STACK_A_CFG {
    /// Port mask with ports of stack port A. This mask is only used when forwarding to UPSIDs w. STACK_LINK_EQUAL_COST_CFG.STACK_LINK_EQUAL_ENA=1. If STACK_LINK_EQUAL_ENA is set, then the stack forwarding mask is calculated as one of the following two, depending on AC: Even AC: UPSID.UPSID_PORT_VEC* & STACK_A_CFG.STACK_A_VEC* Odd AC: UPSID.UPSID_PORT_VEC* & ~STACK_A_CFG.STACK_A_VEC*
    ///
    /// 'XX...XXX': Where X is '0' or '1', representing a stacking link A destination port.
    pub fn stack_a_mask(&self) -> u32 {
        self.0
    }
    pub fn set_stack_a_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Port mask for Stack Port A
#[derive(From, Into)]
pub struct STACK_A_CFG1(u32);
impl STACK_A_CFG1 {
    /// Refer to STACK_A_CFG.STACK_A_MASK description.
    pub fn stack_a_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_stack_a_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Stack port mask configuration
#[derive(From, Into)]
pub struct STACK_CFG(u32);
impl STACK_CFG {
    /// Stack port mask. For ports used for stacking, the corresponding bit must be set in this port mask.
    ///
    /// 'XX...XXX': Where X is '0' or '1', representing a destination port.
    pub fn stack_mask(&self) -> u32 {
        self.0
    }
    pub fn set_stack_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Stack port mask configuration
#[derive(From, Into)]
pub struct STACK_CFG1(u32);
impl STACK_CFG1 {
    /// Refer to STACK_CFG.STACK_MASK description.
    pub fn stack_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_stack_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Per port stack configuration
#[derive(From, Into)]
pub struct VSTAX_CTRL(u32);
impl VSTAX_CTRL {
    /// Specify learn and forwarding operation for the port
    ///
    /// 0: VStaX2/BF mode (Basic Forwarding mode) 1: VStaX2/AF mode (Advanced Forwarding mode)
    pub fn vstax2_stack_port_mode(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vstax2_stack_port_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// VStaX Gmirror destination configuration
#[derive(From, Into)]
pub struct VSTAX_GMIRROR_CFG(u32);
impl VSTAX_GMIRROR_CFG {
    /// The set of port(s) to which frames received on a stack port and with fwd_mode=fwd_gmirror traffic are forwarded. This is only used in VStaX context.
    pub fn gmirror_port_mask(&self) -> u32 {
        self.0
    }
    pub fn set_gmirror_port_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// VStaX Gmirror destination configuration
#[derive(From, Into)]
pub struct VSTAX_GMIRROR_CFG1(u32);
impl VSTAX_GMIRROR_CFG1 {
    /// Refer to VSTAX_GMIRROR_CFG.GMIRROR_PORT_MASK description.
    pub fn gmirror_port_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_gmirror_port_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
