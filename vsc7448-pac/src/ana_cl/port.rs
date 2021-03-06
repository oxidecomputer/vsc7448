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
/// VCAP_CLM configuration
///
/// Replicated per lookup in VCAP_CLM.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct ADV_CL_CFG(u32);
impl ADV_CL_CFG {
    /// VCAP_CLM key type used for frame types other than MPLS and IPv4/IPv6.
    ///
    /// 0: No Lookup 1: MLL 2: SGL_MLBS 3: DBL_MLBS 4: TRI_MLBS 5: TRI_VID 6: LL_FULL 7: NORMAL with SRC information 8: NORMAL with DST information 9: NORMAL_7TUPLE 10 NORMAL_5TUPLE_IP4 11 PURE_5TUPLE_IP4 15: No Lookup other: reserved
    #[inline(always)]
    pub fn etype_clm_key_sel(&self) -> u32 {
        (self.0 & 0x1e) >> 1
    }
    #[inline(always)]
    pub fn set_etype_clm_key_sel(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 1;
        self.0 &= !0x1e;
        self.0 |= value;
    }
    /// VCAP_CLM key type used for IPv4 frames.
    ///
    /// 0: Follow ETYPE_CLM_KEY_SEL selection 1: MLL 2: SGL_MLBS 3: DBL_MLBS 4: TRI_MLBS 5: TRI_VID 6: LL_FULL 7: NORMAL with SRC information 8: NORMAL with DST information 9: NORMAL_7TUPLE 10 NORMAL_5TUPLE_IP4 11 PURE_5TUPLE_IP4 15: No Lookup other: reserved
    #[inline(always)]
    pub fn ip4_clm_key_sel(&self) -> u32 {
        (self.0 & 0x1e00000) >> 21
    }
    #[inline(always)]
    pub fn set_ip4_clm_key_sel(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 21;
        self.0 &= !0x1e00000;
        self.0 |= value;
    }
    /// VCAP_CLM key type used for IPv6 frames.
    ///
    /// 0: Follow ETYPE_CLM_KEY_SEL selection 1: MLL 2: SGL_MLBS 3: DBL_MLBS 4: TRI_MLBS 5: TRI_VID 6: LL_FULL 7: NORMAL with SRC information 8: NORMAL with DST information 9: NORMAL_7TUPLE 10 NORMAL_5TUPLE_IP4 11 PURE_5TUPLE_IP4 15: No Lookup other: reserved
    #[inline(always)]
    pub fn ip6_clm_key_sel(&self) -> u32 {
        (self.0 & 0x1e0000) >> 17
    }
    #[inline(always)]
    pub fn set_ip6_clm_key_sel(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 17;
        self.0 &= !0x1e0000;
        self.0 |= value;
    }
    /// Enable VCAP_CLM lookup.
    ///
    /// 1: Enable 0: Disable
    #[inline(always)]
    pub fn lookup_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_lookup_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// VCAP_CLM key type used when current protocol layer is MPLS label stack.
    ///
    /// 0: Follow ETYPE_CLM_KEY_SEL selection 2: SGL_MLBS 3: DBL_MLBS 4: TRI_MLBS 15: No Lookup other: reserved
    #[inline(always)]
    pub fn mlbs_clm_key_sel(&self) -> u32 {
        (self.0 & 0x1e0) >> 5
    }
    #[inline(always)]
    pub fn set_mlbs_clm_key_sel(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 5;
        self.0 &= !0x1e0;
        self.0 |= value;
    }
    /// VCAP_CLM key type used for multicast MPLS frames (EtherType = 0x8847).
    ///
    /// 0: Follow ETYPE_CLM_KEY_SEL selection 1: MLL 2: SGL_MLBS 3: DBL_MLBS 4: TRI_MLBS 5: TRI_VID 6: LL_FULL 7: NORMAL with SRC information 8: NORMAL with DST information 9: NORMAL_7TUPLE 10 NORMAL_5TUPLE_IP4 11 PURE_5TUPLE_IP4 15: No Lookup other: reserved
    #[inline(always)]
    pub fn mpls_mc_clm_key_sel(&self) -> u32 {
        (self.0 & 0x1e00) >> 9
    }
    #[inline(always)]
    pub fn set_mpls_mc_clm_key_sel(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 9;
        self.0 &= !0x1e00;
        self.0 |= value;
    }
    /// VCAP_CLM key type used for unicast MPLS frames (EtherType = 0x8847).
    ///
    /// 0: Follow ETYPE_CLM_KEY_SEL selection 1: MLL 2: SGL_MLBS 3: DBL_MLBS 4: TRI_MLBS 5: TRI_VID 6: LL_FULL 7: NORMAL with SRC information 8: NORMAL with DST information 9: NORMAL_7TUPLE 10 NORMAL_5TUPLE_IP4 11 PURE_5TUPLE_IP4 15: No Lookup other: reserved
    #[inline(always)]
    pub fn mpls_uc_clm_key_sel(&self) -> u32 {
        (self.0 & 0x1e000) >> 13
    }
    #[inline(always)]
    pub fn set_mpls_uc_clm_key_sel(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 13;
        self.0 &= !0x1e000;
        self.0 |= value;
    }
    /// If set, the VCAP_CLM lookup uses the basic classified DSCP instead of the value from the frame.
    ///
    /// 1: Enable 0: Disable
    #[inline(always)]
    pub fn use_cl_dscp_ena(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    #[inline(always)]
    pub fn set_use_cl_dscp_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 25;
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    /// If set, the VCAP_CLM lookup uses the basic classified VID, DEI and PCP instead of the values from the frame.
    ///
    /// 1: Enable 0: Disable
    #[inline(always)]
    pub fn use_cl_tci0_ena(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    #[inline(always)]
    pub fn set_use_cl_tci0_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 26;
        self.0 &= !0x4000000;
        self.0 |= value;
    }
}
/// BPDU redirection control
///
/// Configuration of CPU capturing of BPDU frames.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CAPTURE_BPDU_CFG(u32);
impl CAPTURE_BPDU_CFG {
    /// Control CPU redirection, copy or discard of reserved DMAC addresses in the range 01-80-C2-00-00-10 to 01-80-C2-00-00-1F. Each two bits of this fields control a DMAC addres: Bits 0 and 1 control address 01-80-C2-00-00-10, bits 2 and 3 control address 01-80-C2-00-00-11, and so on. Frames are extracted to the CPU extraction queue defined in ANA_CL::CPU_8021_QU_CFG.
    ///
    /// 0: Normal forward 1: Enable redirection to CPU queue 2: Enable copy to CPU queue 3: Discard the frame
    #[inline(always)]
    pub fn cpu_bpdu_redir_sel(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_cpu_bpdu_redir_sel(&mut self, value: u32) {
        self.0 = value;
    }
}
/// CPU forward control
///
/// Configuration of CPU capturing of control frames.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CAPTURE_CFG(u32);
impl CAPTURE_CFG {
    /// This configuration applies to the CPU forwarding function of the basic classifier. Each bit corresponds to one of the known TPIDs. If a bit is set, the basic classifier does not CPU forward a frame if the frame's outer VLAN tag contains the corresponding TPID.
    ///
    /// Bit0: TPID = 0x8100 Bit1: TPID = 0x88A8 Bit2: TPID = ANA_CL::VLAN_STAG_CFG[0] Bit3: TPID = ANA_CL::VLAN_STAG_CFG[1] Bit4: TPID = ANA_CL::VLAN_STAG_CFG[2]
    #[inline(always)]
    pub fn capture_tpid_aware_dis(&self) -> u32 {
        (self.0 & 0xf80) >> 7
    }
    #[inline(always)]
    pub fn set_capture_tpid_aware_dis(&mut self, value: u32) {
        assert!(value <= 0x1f);
        let value = value << 7;
        self.0 &= !0xf80;
        self.0 |= value;
    }
    /// Redirect IGMP frames to the CPU extraction queue given by CPU_PROTO_QU_CFG.CPU_IGMP_QU.
    ///
    /// 0: Disable redirection 1: Enable redirection to CPU queue
    #[inline(always)]
    pub fn cpu_igmp_redir_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_cpu_igmp_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Copy IPv4 multicast control frames to the CPU extraction queue given by CPU_PROTO_QU_CFG.CPU_IP4_MC_CTRL_QU.
    ///
    /// 0: Disable copy 1: Enable copy to CPU queue
    #[inline(always)]
    pub fn cpu_ip4_mc_copy_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_cpu_ip4_mc_copy_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Copy IPv6 multicast control frames (DIP equals FF02::/16) to the CPU extraction queue given by CPU_PROTO_QU_CFG.CPU_IP6_MC_CTRL_QU.
    ///
    /// 0: Disable copy 1: Enable copy to CPU queue
    #[inline(always)]
    pub fn cpu_ip6_mc_copy_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_cpu_ip6_mc_copy_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Redirect MLD frames to the CPU extraction queue given by CPU_PROTO_QU_CFG.CPU_MLD_QU.
    ///
    /// 0: Disable redirection 1: Enable redirection to CPU queue.
    #[inline(always)]
    pub fn cpu_mld_redir_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_cpu_mld_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Redirect VRAP frames to the CPU extraction queue given by CPU_PROTO_QU_CFG.CPU_VRAP_QU.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn cpu_vrap_redir_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_cpu_vrap_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Redirect IPv6 frames with hop by hop options to the CPU extraction queue given by CPU_PROTO_QU_CFG.CPU_HOP_BY_HOP_ICMP_QU.
    ///
    /// 0: Disable redirection 1: Enable redirection to the CPU queue
    #[inline(always)]
    pub fn ip6_hop_by_hop_redir_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_ip6_hop_by_hop_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Redirect ICMPv6 frames to the CPU extraction queue given by CPU_PROTO_QU_CFG.CPU_HOP_BY_HOP_ICMP_QU.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ip6_icmp_hop_by_hop_redir_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_ip6_icmp_hop_by_hop_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
}
/// GXRP redirection control
///
/// Configuration of CPU capturing of GARP frames.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CAPTURE_GXRP_CFG(u32);
impl CAPTURE_GXRP_CFG {
    /// Control CPU redirection, copy or discard of reserved DMAC addresses in the range 01-80-C2-00-00-20 to 01-80-C2-00-00-2F. Each two bits of this fields control a DMAC addres: Bits 0 and 1 control address 01-80-C2-00-00-20, bits 2 and 3 control address 01-80-C2-00-00-21, and so on. Frames are extracted to the CPU extraction queue defined in ANA_CL::CPU_8021_QU_CFG.
    ///
    /// 0: Normal forward 1: Enable redirection to CPU queue 2: Enable copy to CPU queue 3: Discard the frame
    #[inline(always)]
    pub fn cpu_gxrp_redir_sel(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_cpu_gxrp_redir_sel(&mut self, value: u32) {
        self.0 = value;
    }
}
/// IEEE802.1ag / ITU-T Y.1731 OAM frame filtering control
///
/// Configuration of CPU capturing of IEEE802.1ag and Y.1731 control frames.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CAPTURE_Y1731_AG_CFG(u32);
impl CAPTURE_Y1731_AG_CFG {
    /// Control CPU redirection, copy or discard of reserved DMAC addresses in the range 01-80-C2-00-00-30 to 01-80-C2-00-00-3F. Each two bits of this fields control a DMAC addres: Bits 0 and 1 control address 01-80-C2-00-00-30, bits 2 and 3 control address 01-80-C2-00-00-31, and so on. Frames are extracted to the CPU extraction queue defined in ANA_CL::CPU_8021_QU_CFG.
    ///
    /// 0: Normal forward 1: Enable redirection to CPU queue 2: Enable copy to CPU queue 3: Discard the frame
    #[inline(always)]
    pub fn cpu_y1731_ag_redir_sel(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_cpu_y1731_ag_redir_sel(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Filter configuration
///
/// Configuration of filtering of frames not matching expected ingress properties
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct FILTER_CTRL(u32);
impl FILTER_CTRL {
    /// Discard frames with DMAC or SMAC equal to 00-00-00-00-00-00.
    ///
    /// 0: Discard frames with a null MAC address 1: No filter
    #[inline(always)]
    pub fn filter_null_mac_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_filter_null_mac_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Discard frames with a multicast SMAC address.
    ///
    /// 0: Discard frames with multicast SMAC address 1: No filter
    #[inline(always)]
    pub fn filter_smac_mc_dis(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_filter_smac_mc_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable FCS update of all frames received on the port.
    #[inline(always)]
    pub fn force_fcs_update_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_force_fcs_update_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// DEI and PCP mapping table
///
/// Mapping of frame's DEI and PCP to classified QoS class and drop precedence level. Configuration per DEI, PCP.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCP_DEI_MAP_CFG(u32);
impl PCP_DEI_MAP_CFG {
    /// Map DEI and PCP to a DP level: DP level = PCP_DEI_MAP_CFG[8*DEI + PCP].PCP_DEI_DP_VAL.
    #[inline(always)]
    pub fn pcp_dei_dp_val(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    #[inline(always)]
    pub fn set_pcp_dei_dp_val(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 3;
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// Map VLAN PCP and DEI to a QoS class: QOS class = PCP_DEI_MAP_CFG[8*DEI + PCP].PCP_DEI_QOS_VAL.
    #[inline(always)]
    pub fn pcp_dei_qos_val(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_pcp_dei_qos_val(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// DEI and PCP translation table
///
/// Translation of frame's DEI and PCP to classified DEI and PCP. Configuration per DEI and PCP. The use of this table is enabled in VLAN_CTRL.VLAN_PCP_DEI_TRANS_ENA.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCP_DEI_TRANS_CFG(u32);
impl PCP_DEI_TRANS_CFG {
    /// Translate VLAN PCP and DEI to a classified DEI: DEI = PCP_DEI_TRANS_CFG[8*DEI + PCP].DEI_TRANS_VAL.
    #[inline(always)]
    pub fn dei_trans_val(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_dei_trans_val(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Translate VLAN PCP and DEI to a classified PCP: PCP = PCP_DEI_TRANS_CFG[8*DEI + PCP].PCP_TRANS_VAL.
    #[inline(always)]
    pub fn pcp_trans_val(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_pcp_trans_val(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Port ID data
///
/// Configuration of GLAG and logical port number.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PORT_ID_CFG(u32);
impl PORT_ID_CFG {
    /// Global Link Aggregation Group (GLAG) number to be used in relation to learning and forwarding.
    ///
    /// PORT_IS_GLAG_ENA=0: Port does not participate in a GLAG PORT_IS_GLAG_ENA=1: 0: port is member of GLAG 0 1: port is member of GLAG 1 ... N: port is member of GLAG N
    #[inline(always)]
    pub fn glag_num(&self) -> u32 {
        (self.0 & 0x1f00) >> 8
    }
    #[inline(always)]
    pub fn set_glag_num(&mut self, value: u32) {
        assert!(value <= 0x1f);
        let value = value << 8;
        self.0 &= !0x1f00;
        self.0 |= value;
    }
    /// Logical port number to be used in relation to learning, forwarding and policing.
    ///
    /// 0: Logical port 0 1: Logical port 1 ... n: Logical port n.
    #[inline(always)]
    pub fn lport_num(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_lport_num(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// Default PAG value used as input to S2. The PAG value can be changed by CLM actions.
    #[inline(always)]
    pub fn pag_val(&self) -> u32 {
        (self.0 & 0x1fe000) >> 13
    }
    #[inline(always)]
    pub fn set_pag_val(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 13;
        self.0 &= !0x1fe000;
        self.0 |= value;
    }
    /// Port is part of a Global Link Aggregation Gloup (GLAG).
    ///
    /// 0: Port is not globally link aggregated. 1: Port is part of a GLAG.
    #[inline(always)]
    pub fn port_is_glag_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_port_is_glag_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Update IFH.SRC_PORT to LPORT_NUM to ensure frames from link aggregated ports are queued in the same queue.
    ///
    /// 0: IFH.FWD.SRC_PORT = phys_num. 0: IFH.FWD.SRC_PORT = LPORT_NUM.
    #[inline(always)]
    pub fn update_ifh_src_port_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_update_ifh_src_port_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
}
/// QoS configuration
///
/// Configuration of basic QoS classification.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct QOS_CFG(u32);
impl QOS_CFG {
    /// Default port DP level.
    ///
    /// 0: DP 0 (disable) 1: DP 1 ... n: DP n (highest drop probability).
    #[inline(always)]
    pub fn default_dp_val(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    #[inline(always)]
    pub fn set_default_dp_val(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 3;
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// Default port QoS class.
    #[inline(always)]
    pub fn default_qos_val(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_default_qos_val(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Allow DP classification based on DSCP for IP frames.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn dscp_dp_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_dscp_dp_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Setting this bit prevents the rewriter from remapping DSCP values for frames from this port.
    ///
    /// 0: Allow rewriter to remap DSCP field 1: Do not allow rewriter to remap of DSCP field
    #[inline(always)]
    pub fn dscp_keep_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_dscp_keep_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Allow QoS classification based on DSCP for IP frames.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn dscp_qos_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_dscp_qos_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Select which DSCP values to rewrite based on DP level and QoS class. If the DSCP value is to be rewritten, then the new DSCP = ANA_CL::QOS_MAP_CFG[8*DP level + QoS class].DSCP_REWR_VAL.
    ///
    /// 0: Rewrite none 1: Rewrite if DSCP=0 2: Rewrite for selected values configured in ANA_CL::DSCP_CFG[DSCP].DSCP_REWR_ENA. 3: Rewrite all
    #[inline(always)]
    pub fn dscp_rewr_mode_sel(&self) -> u32 {
        (self.0 & 0x3000) >> 12
    }
    #[inline(always)]
    pub fn set_dscp_rewr_mode_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 12;
        self.0 &= !0x3000;
        self.0 |= value;
    }
    /// Set if the DSCP value must be translated before using the DSCP value. If set, the translated DSCP value is given from ANA_CL::DSCP_CFG[DSCP].DSCP_TRANSLATE_VAL.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn dscp_translate_ena(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_dscp_translate_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Setting this bit prevents the rewriter from making any changes to frames from this port. If a frame is CPU injected, this configuration bit is overruled.
    ///
    /// 0: Allow rewriter to change the frame 1: Do not allow rewriter to change the frame
    #[inline(always)]
    pub fn keep_ena(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_keep_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Allow DP classification based on PCP and DEI for tagged frames.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn pcp_dei_dp_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_pcp_dei_dp_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Allow QoS classification based on PCP and DEI from tagged frames.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn pcp_dei_qos_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_pcp_dei_qos_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
}
/// Stacking configuration
///
/// Configure stacking awareness
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct STACKING_CTRL(u32);
impl STACKING_CTRL {
    /// Ingress port drop mode configuration. Applicable for front port only.
    ///
    /// 0 : Disable drop mode for the priority 1 : Enable drop mode for the priority
    #[inline(always)]
    pub fn igr_drop_ena(&self) -> u32 {
        (self.0 & 0xff0) >> 4
    }
    #[inline(always)]
    pub fn set_igr_drop_ena(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 4;
        self.0 &= !0xff0;
        self.0 |= value;
    }
    /// Enable usage of stacking information.
    #[inline(always)]
    pub fn stacking_aware_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_stacking_aware_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable discard of frames received with a stacking header.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn stacking_header_discard_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_stacking_header_discard_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable discard of frames received without a stacking header.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn stacking_non_header_discard_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_stacking_non_header_discard_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// If set, and STACKING_AWARE_ENA=1 then any VStaX header in the frame is assumed to contain an ISDX. Otherwise the VStaX header is assumed to contain an AC. Related parameters: ANA_AC:PS_COMMON:COMMON_VSTAX_CFG.VSTAX2_MISC_ISDX_ENA
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn vstax_isdx_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_vstax_isdx_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// VLAN configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VLAN_CTRL(u32);
impl VLAN_CTRL {
    /// Default DEI bit for the port for untagged frames. Also used if port is VLAN unaware.
    #[inline(always)]
    pub fn port_dei(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_port_dei(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Default PCP value for the port for untagged frames. Also used if port is VLAN unaware.
    #[inline(always)]
    pub fn port_pcp(&self) -> u32 {
        (self.0 & 0xe000) >> 13
    }
    #[inline(always)]
    pub fn set_port_pcp(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 13;
        self.0 &= !0xe000;
        self.0 |= value;
    }
    /// Default tag type for untagged frames. Also used if port is VLAN unaware. The tag type is carried with the frame to the rewriter where the tag type can be used when VLAN tagging the frame.
    ///
    /// 0: Tag type equals C-tag 1: Tag type equals S-tag
    #[inline(always)]
    pub fn port_tag_type(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_port_tag_type(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Default VID value for the port for untagged frames. Also used if port is VLAN unaware.
    #[inline(always)]
    pub fn port_vid(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_port_vid(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    /// Default DEI value used by the port VOE for untagged frames or frames for which the outer tag's TPID is not accepted (see PORT_VOE_TPID_AWARE_DIS).
    #[inline(always)]
    pub fn port_voe_default_dei(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    #[inline(always)]
    pub fn set_port_voe_default_dei(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Default PCP value used by the port VOE for untagged frames or frames for which the outer tag's TPID is not accepted (see PORT_VOE_TPID_AWARE_DIS).
    #[inline(always)]
    pub fn port_voe_default_pcp(&self) -> u32 {
        (self.0 & 0x3800000) >> 23
    }
    #[inline(always)]
    pub fn set_port_voe_default_pcp(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 23;
        self.0 &= !0x3800000;
        self.0 |= value;
    }
    /// Configuration port VOE's VLAN awareness. Each bit corresponds to one of the known TPIDs. If the incoming frame's outer tag contains a TPID for which PORT_VOE_TPID_AWARE_DIS is set, then the port VOE sees the frame as untagged and uses the port VOE's default DEI and PCP (PORT_VOE_DEFAULT_PCP and PORT_VOE_DEFAULT_DEI) for LM updates.
    ///
    /// Bit0: Control TPID = 0x8100. Bit1: Control TPID = 0x88A8 Bit2: Control TPID = ANA_CL::VLAN_STAG_CFG[0] Bit3: Control TPID = ANA_CL::VLAN_STAG_CFG[1] Bit4: Control TPID = ANA_CL::VLAN_STAG_CFG[2]
    #[inline(always)]
    pub fn port_voe_tpid_aware_dis(&self) -> u32 {
        (self.0 & 0x7c000000) >> 26
    }
    #[inline(always)]
    pub fn set_port_voe_tpid_aware_dis(&mut self, value: u32) {
        assert!(value <= 0x1f);
        let value = value << 26;
        self.0 &= !0x7c000000;
        self.0 |= value;
    }
    /// Enable VLAN awareness for port. If VLAN unaware, the frame's VLAN tags are not used for VLAN classification. Valid VLAN tags are defined in ANA_CL::VLAN_STAG_CFG[0-2] and ANA_CL:PORT:VLAN_TPID_CTRL.BASIC_TPID_AWARE_DIS.
    ///
    /// 0: Disable (VLAN unaware) 1: Enable (VLAN aware)
    #[inline(always)]
    pub fn vlan_aware_ena(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    #[inline(always)]
    pub fn set_vlan_aware_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// If set, the PCP_DEI_TRANS_CFG table is used for PCP and DEI classification. Otherwise, the frame's values are used directly.
    #[inline(always)]
    pub fn vlan_pcp_dei_trans_ena(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline(always)]
    pub fn set_vlan_pcp_dei_trans_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// Number of VLAN tag headers to remove from ingress frame. If the frame contains less VLAN tags than specified by register, the VLAN_POP_CNT is reduced to the number of VLAN tags in the frame.
    ///
    /// 0: Keep all tags 1: Pop up to one tag if available 2: Pop up to two tags if available 3: Pop up to three tags if available
    #[inline(always)]
    pub fn vlan_pop_cnt(&self) -> u32 {
        (self.0 & 0x60000) >> 17
    }
    #[inline(always)]
    pub fn set_vlan_pop_cnt(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 17;
        self.0 &= !0x60000;
        self.0 |= value;
    }
    /// Select the VLAN tag used for basic VLAN and QoS classification. For VLAN classification, valid tags are defined by ANA_CL:PORT:VLAN_TPID_CTRL.BASIC_TPID_AWARE_DIS. For QoS classification, all accepted tags (ANA_CL:PORT:VLAN_FILTER_CTRL) are valid.
    ///
    /// 0: Use first tag (outer-most tag). 1: Use second tag if present, otherwise use first tag.
    #[inline(always)]
    pub fn vlan_tag_sel(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_vlan_tag_sel(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
}
/// VLAN acceptance filter
///
/// VLAN_FILTER_CTRL[0] applies to outer VLAN tag (first tag). VLAN_FILTER_CTRL[1] applies to middle VLAN tag (second tag). VLAN_FILTER_CTRL[2] applies to inner VLAN tag (third tag).
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VLAN_FILTER_CTRL(u32);
impl VLAN_FILTER_CTRL {
    /// Discard frame if the investigated VLAN tag is a C-tag (TPID=0x8100) and VID>0.
    #[inline(always)]
    pub fn ctag_dis(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_ctag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Discard frame if the investigated VLAN TPID equals VLAN_STAG_CFG[0].STAG_ETYPE_VAL and VID>0.
    #[inline(always)]
    pub fn cust1_stag_dis(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_cust1_stag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Discard frame if the investigated VLAN TPID equals VLAN_STAG_CFG[1].STAG_ETYPE_VAL and VID>0.
    #[inline(always)]
    pub fn cust2_stag_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_cust2_stag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Discard frame if the investigated VLAN TPID equals VLAN_STAG_CFG[2].STAG_ETYPE_VAL and VID>0.
    #[inline(always)]
    pub fn cust3_stag_dis(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_cust3_stag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Discard frame if the investigated VLAN tag is a priority C-tag (TPID=0x8100 and VID=0).
    #[inline(always)]
    pub fn prio_ctag_dis(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_prio_ctag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Discard frame if the investigated VLAN TPID equals VLAN_STAG_CFG[0].STAG_ETYPE_VAL and VID=0.
    #[inline(always)]
    pub fn prio_cust1_stag_dis(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_prio_cust1_stag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Discard frame if the investigated VLAN TPID equals VLAN_STAG_CFG[1].STAG_ETYPE_VAL and VID=0.
    #[inline(always)]
    pub fn prio_cust2_stag_dis(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_prio_cust2_stag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Discard frame if the investigated VLAN TPID equals VLAN_STAG_CFG[2].STAG_ETYPE_VAL and VID=0.
    #[inline(always)]
    pub fn prio_cust3_stag_dis(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_prio_cust3_stag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Discard frame if the investigated VLAN tag is a priority S-tag (TPID=0x88A8 and VID=0).
    #[inline(always)]
    pub fn prio_stag_dis(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_prio_stag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Discard frame if the investigated VLAN tag is an S-tag (TPID=0x88A8) and VID>0.
    #[inline(always)]
    pub fn stag_dis(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_stag_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Discard frame if VLAN_FILTER_CTRL[n].TAG_REQUIRED_ENA is set and the number of VLAN tags in frame is less than n+1: - If VLAN_FILTER_CTRL[0].TAG_REQUIRED_ENA is set: Discard frame if it is untagged. - If VLAN_FILTER_CTRL[1].TAG_REQUIRED_ENA is set: Discard frame if it is single tagged or untagged. - If VLAN_FILTER_CTRL[2].TAG_REQUIRED_ENA is set: Discard frame if it is single tagged, double tagged, or untagged.
    #[inline(always)]
    pub fn tag_required_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_tag_required_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
}
/// TPID awareness configuration
///
/// Configuration of which TPID values are accepted as valid VLAN tags.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VLAN_TPID_CTRL(u32);
impl VLAN_TPID_CTRL {
    /// Control which TPID values are accepted as valid VLAN tags for L3 routing and basic VLAN classification. If an incoming frame contains a TPID for which BASIC_TPID_AWARE_DIS is set, then the TPID is treated a non-TPID EtherType and no further tags are identified.
    ///
    /// Bit0: First (outermost) TPID = 0x8100. Bit1: First TPID = 0x88A8 Bit2: First TPID = ANA_CL::VLAN_STAG_CFG[0] Bit3: First TPID = ANA_CL::VLAN_STAG_CFG[1] Bit4: First TPID = ANA_CL::VLAN_STAG_CFG[2] Bit5: Second TPID = 0x8100. Bit6: Second TPID = 0x88A8 Bit7: Second TPID = ANA_CL::VLAN_STAG_CFG[0] Bit8: Second TPID = ANA_CL::VLAN_STAG_CFG[1] Bit9: Second TPID = ANA_CL::VLAN_STAG_CFG[2] Bit10: Third TPID = 0x8100. Bit11: Third TPID = 0x88A8 Bit12: Third TPID = ANA_CL::VLAN_STAG_CFG[0] Bit13: Third TPID = ANA_CL::VLAN_STAG_CFG[1] Bit14: Third TPID = ANA_CL::VLAN_STAG_CFG[2]
    #[inline(always)]
    pub fn basic_tpid_aware_dis(&self) -> u32 {
        (self.0 & 0x7fff0) >> 4
    }
    #[inline(always)]
    pub fn set_basic_tpid_aware_dis(&mut self, value: u32) {
        assert!(value <= 0x7fff);
        let value = value << 4;
        self.0 &= !0x7fff0;
        self.0 |= value;
    }
    /// Control the number of VLAN tags, which are accepted for frames to be routed. Valid VLAN tags are defined in ANA_CL:PORT:VLAN_TPID_CTRL.BASIC_TPID_AWARE_DIS and ANA_CL::VLAN_STAG_CFG[0-2].
    ///
    /// Bit0: Route untagged frames. Bit1: Route frames with one accepted tag (TPID is accepted by BASIC_TPID_AWARE_DIS) Bit2: Route frames with two accepted tags (TPID is accepted by BASIC_TPID_AWARE_DIS) Bit3: Route frames with three accepted tags (TPID is accepted by BASIC_TPID_AWARE_DIS)
    #[inline(always)]
    pub fn rt_tag_ctrl(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_rt_tag_ctrl(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
