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
/// Configuration of various ACL PTP features
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_MISC_CTRL(u32);
impl PTP_MISC_CTRL {
    /// If set, and VCAP_IS2 action REW_CMD[5] is set, ANA_ACL rewrites Delay_Req frames. The following PTP fields can be modified in ANA_ACL depending on configuration in PTP_MISC_CTRL and ANA_ACL:PTP_DOM: - messageType (set to Delay_Resp) - messageLength - flagField - sourcePortIdentity - controlField - logMessageInterval - receiveTimestamp - requestingPortIdentity
    #[inline(always)]
    pub fn ptp_allow_acl_rew_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_ptp_allow_acl_rew_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// IP6 PTP operation.
    ///
    /// 0: No IP6 PTP updates. 1: IP6 PTP updates with UDP chksum updates. 2: IP6 PTP updates with UDP chksum updates, redirect frame to CPU using extraction queue PTP_DELAY_REDIR_QU. 3: IP6 PTP updates with UDP chksum clear.
    #[inline(always)]
    pub fn ptp_delay_ip6_sel(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    #[inline(always)]
    pub fn set_ptp_delay_ip6_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 2;
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// CPU extraction queue for PTP Delay_Resp frames redirected to the CPU.
    #[inline(always)]
    pub fn ptp_delay_redir_qu(&self) -> u32 {
        (self.0 & 0x380) >> 7
    }
    #[inline(always)]
    pub fn set_ptp_delay_redir_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 7;
        self.0 &= !0x380;
        self.0 |= value;
    }
    /// If set, PTP Delay_resp frames expanded beyond one cell is redirected to the CPU. If cleared, such frames are discarded.
    #[inline(always)]
    pub fn ptp_delay_redir_too_big_redir(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_ptp_delay_redir_too_big_redir(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Allow length of Delay_resp with requestingPortIdentity If set, Delay_Req frames are made 10 bytes longer to include requestingPortIdentity. Appropriate header lengths (PTP, UDP, IP) are increased with 10 bytes.
    #[inline(always)]
    pub fn ptp_delay_req_chg_len_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_ptp_delay_req_chg_len_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// If set, logMessageInterval in multicast Delay_Resp frames is updated with VCAP_S2 action: SWAP_MAC_ENA & DLB_OFFSET.
    #[inline(always)]
    pub fn ptp_delay_req_mc_upd_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_ptp_delay_req_mc_upd_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Require received UDP length to be 44 before PTP_DELAY_REQ_CHG_LEN_ENA takes action. Frames with UDP len different from 44 are either discarded or optional redirected by means of PTP_DELAY_REDIR_TOO_BIG_REDIR.
    #[inline(always)]
    pub fn ptp_delay_req_udp_len52(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_ptp_delay_req_udp_len52(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Configuration of various swap features
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SWAP_IP_CTRL(u32);
impl SWAP_IP_CTRL {
    /// Number of bits in frame's DMAC counting from LSB which are not replaced if replacing the DMAC (IS2 action ACL_RT_MODE) and reduced DMAC is enabled (IS2 action SAM_SEQ_ENA). New DMAC is provided by IS2 action ACL_MAC. For example, if set to 16, the 16 LSB bits in the frame's DMAC are not replaced while the 32 MSB bits are replaced with corresponding 32 bits from ACL_MAC.
    #[inline(always)]
    pub fn dmac_repl_offset_val(&self) -> u32 {
        (self.0 & 0xfc0000) >> 18
    }
    #[inline(always)]
    pub fn set_dmac_repl_offset_val(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 18;
        self.0 &= !0xfc0000;
        self.0 |= value;
    }
    /// Replace TTL with value configured in IP_SWAP_IP4_TTL_VAL when swapping IPv4 addresses.
    #[inline(always)]
    pub fn ip_swap_ip4_ttl_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_ip_swap_ip4_ttl_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// New TTL value when swapping IPv4 addresses.
    #[inline(always)]
    pub fn ip_swap_ip4_ttl_val(&self) -> u32 {
        (self.0 & 0x3fc) >> 2
    }
    #[inline(always)]
    pub fn set_ip_swap_ip4_ttl_val(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 2;
        self.0 &= !0x3fc;
        self.0 |= value;
    }
    /// Replace hop count with value configured in IP_SWAP_IP6_HOPC_VAL when swapping IPv6 addresses.
    #[inline(always)]
    pub fn ip_swap_ip6_hopc_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_ip_swap_ip6_hopc_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// New hop count value when swapping IPv6 addresses.
    #[inline(always)]
    pub fn ip_swap_ip6_hopc_val(&self) -> u32 {
        (self.0 & 0x3fc00) >> 10
    }
    #[inline(always)]
    pub fn set_ip_swap_ip6_hopc_val(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 10;
        self.0 &= !0x3fc00;
        self.0 |= value;
    }
}
/// Source IP table used for multicast IP address swapping
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SWAP_SIP(u32);
impl SWAP_SIP {
    /// New source IP address used when frame's IP addresses are swapped and the original destination IP address was a multicast address. IP address swapping is enabled in VCAP_IS2 action ACL_RT_MODE. IPv4: Each entry configures one IPv4 address. IPv6: Four consecutive entries configures one IPv6 address. Entries must start at 4 x n, n=0, 1, ..., 7. First entry encodes bits 31:0 of IPv6 address, second entry encoded bits 63:32, and so on.
    #[inline(always)]
    pub fn sip(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_sip(&mut self, value: u32) {
        self.0 = value;
    }
}
/// VCAP S2 configuration
///
/// Configuration of advanced classification per port. For the 2-bit fields of this register the following applies: Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VCAP_S2_CFG(u32);
impl VCAP_S2_CFG {
    /// Enable/disable VCAP_IS2 lookups. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_ena(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_sec_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// For frames to be routed, enable/disable the use of IRLEG VID and ERLEG VID as the value for the VID field in the VCAP_IS2 keys. First VCAP_IS2 lookup uses IRLEG VID, second lookup uses ERLEG VID.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_route_handling_ena(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    #[inline(always)]
    pub fn set_sec_route_handling_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 26;
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    /// For ARP frames (EtherType 0x0806), enable matching against control entries of type ARP in VCAP_IS2. Otherwise, ARP frames are matched against control entries of type MAC_ETYPE. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_arp_ena(&self) -> u32 {
        (self.0 & 0xc0) >> 6
    }
    #[inline(always)]
    pub fn set_sec_type_arp_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 6;
        self.0 &= !0xc0;
        self.0 |= value;
    }
    /// For IPv4 frames, enable matching against control entries of type IP4_OTHER in VCAP_IS2. Otherwise, IPv4 frames are matched against control entries of type MAC_ETYPE. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_ip4_other_ena(&self) -> u32 {
        (self.0 & 0x300) >> 8
    }
    #[inline(always)]
    pub fn set_sec_type_ip4_other_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 8;
        self.0 &= !0x300;
        self.0 |= value;
    }
    /// For IPv4 TCP/UDP frames, enable matching against control entries of type IP_TCP_UDP in VCAP_IS2. Otherwise, IPv4 TCP/UDP frames are handled as IPv4 Other frames, see SEC_TYPE_IP4_OTHER_ENA. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_ip4_tcpudp_ena(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    #[inline(always)]
    pub fn set_sec_type_ip4_tcpudp_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 10;
        self.0 &= !0xc00;
        self.0 |= value;
    }
    /// For IPv4 multicast frames, enable matching against control entries of type IP4_VID in VCAP_IS2. Otherwise, IPv4 multicast frames are handled as either IPv4 TCP/UDP frames or IPv4 Other frames, see SEC_TYPE_IP4_TCPUDP_ENA and SEC_TYPE_IP4_OTHER_ENA. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_ip4_vid_ena(&self) -> u32 {
        (self.0 & 0x3000) >> 12
    }
    #[inline(always)]
    pub fn set_sec_type_ip4_vid_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 12;
        self.0 &= !0x3000;
        self.0 |= value;
    }
    /// For IPv6 frames, enable matching against control entries of type IP6_OTHER in VCAP_IS2. Otherwise, IPv6 Other frames are handled as IPv6 standard frames, SEC_TYPE_IP6_STD_ENA. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_ip6_other_ena(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    #[inline(always)]
    pub fn set_sec_type_ip6_other_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 14;
        self.0 &= !0xc000;
        self.0 |= value;
    }
    /// For IPv6 frames, enable matching against control entries of type IP6_STD in VCAP_IS2. Otherwise, IPv6 frames are handled as IPv4 frames, see SEC_TYPE_IP6_TCPUDP_OTHER_ENA. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_ip6_std_ena(&self) -> u32 {
        (self.0 & 0xc0000) >> 18
    }
    #[inline(always)]
    pub fn set_sec_type_ip6_std_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 18;
        self.0 &= !0xc0000;
        self.0 |= value;
    }
    /// For IPv6 TCP/UDP frames, enable matching against control entries of type IP6_TCP_UDP in VCAP_IS2. Otherwise, IPv6 TCP/UDP frames are handled as IPv6 standard frames, SEC_TYPE_IP6_STD_ENA. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_ip6_tcpudp_ena(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    #[inline(always)]
    pub fn set_sec_type_ip6_tcpudp_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 16;
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// For IPv6 frames, enable matching against control entries of types IP4_TCP_UDP and IP4_OTHER in VCAP_IS2. The SIP and DIP fields of IP4_TCP_UDP and IP4_OTHER control entries are used to match against bits 63:0 of IPv6 SIP. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable the classification. 1: Enable the classification.
    #[inline(always)]
    pub fn sec_type_ip6_tcpudp_other_ena(&self) -> u32 {
        (self.0 & 0xc00000) >> 22
    }
    #[inline(always)]
    pub fn set_sec_type_ip6_tcpudp_other_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 22;
        self.0 &= !0xc00000;
        self.0 |= value;
    }
    /// For IPv6 multicast frames, enable matching against control entries of type IP6_VID in VCAP_IS2. Otherwise, IPv6 multicast frames are handled as either IPv6 TCP/UDP frames or IPv6 Other frames, see SEC_TYPE_IP6_TCPUDP_ENA and SEC_TYPE_IP6_OTHER_ENA. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_ip6_vid_ena(&self) -> u32 {
        (self.0 & 0x300000) >> 20
    }
    #[inline(always)]
    pub fn set_sec_type_ip6_vid_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 20;
        self.0 &= !0x300000;
        self.0 |= value;
    }
    /// For LLC frames, enable matching against control entries of type MAC_LLC in VCAP_IS2. Otherwise, LLC frames are matched against control entries of type MAC_ETYPE. LLC frames are identified as frames with EtherType < 0x0600 that are not SNAP frames. Note that SNAP frames can be handled as LLC frames by disabling SEC_TYPE_MAC_SNAP_ENA. Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_mac_llc_ena(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    #[inline(always)]
    pub fn set_sec_type_mac_llc_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 2;
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// For SNAP frames, enable matching against control entries of type MAC_SNAP in VCAP_IS2. Otherwise SNAP frames frames are handled as LLC frames, see SEC_TYPE_MAC_LLC_ENA. SNAP frames are identified by EtherType < 0x0600 DSAP = 0xAA SSAP = 0xAA CTRL = 0x03 Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_mac_snap_ena(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    #[inline(always)]
    pub fn set_sec_type_mac_snap_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 4;
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// For OAM frames (with up to three VLAN tags), enable matching against control entries of type OAM in VCAP_IS2. Otherwise, OAM frames are matched against control entries of type MAC_ETYPE. OAM frames are identified by the following EtherType values: 0x8902 - ITU-T Y.1731 0x8809 - Link Level OAM 0x88EE - MEF-16 (E-LMI) Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn sec_type_oam_ena(&self) -> u32 {
        (self.0 & 0x3000000) >> 24
    }
    #[inline(always)]
    pub fn set_sec_type_oam_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 24;
        self.0 &= !0x3000000;
        self.0 |= value;
    }
}
/// Configuration of various features
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VCAP_S2_MISC_CTRL(u32);
impl VCAP_S2_MISC_CTRL {
    /// Controls how to update routing statistics events for egress ACL actions.
    ///
    /// 0: If routed frame is dropped by VCAP_IS2 rule, then clear ivmid_ip_uc_received/ivmid_ip_mc_received (as if frame never left the router) 1: If routed frame is dropped by VCAP_IS2 rule, then leave ivmid_ip_uc_received/ivmid_ip_mc_received set (as if frame did leave the router and was dropped in the L2 switch)
    #[inline(always)]
    pub fn acl_rt_egr_rleg_stat_mode(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_acl_rt_egr_rleg_stat_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// If set, force use of VID key type in VCAP_ES0 when routing in ANA_ACL.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn acl_rt_force_es0_vid_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_acl_rt_force_es0_vid_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Controls how to update routing statistics events for ingress ANA_ACL actions.
    ///
    /// 0: UC: If frame is dropped by VCAP_IS2 rule, then clear ivmid_ip_uc_received (as if frame never reached the router) MC: If frame is dropped by VCAP_IS2 rule, then leave ivmid_ip_mc_received unchanged (as if frame may reach the router) 1: If frame is dropped by VCAP_IS2 rule, then clear ivmid_ip_uc_received/ivmid_ip_mc_received (as if frame never reached the router, but was dropped in the L2 switch)
    #[inline(always)]
    pub fn acl_rt_igr_rleg_stat_mode(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_acl_rt_igr_rleg_stat_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Controls routing related frame edits in ANA_ACL block. By default, routing related frame edits are done in REW, but when combining routing with PTP, ANA_ACL must be configured to change DMAC to next-hop DMAC in order to allow other information to be stored in the IFH. ANA_ACL can rewrite the following routing related fields: 1) Change DMAC to next-hop MAC address (as determined by ANA_L3). 2) Set IFH.FWD.DST_MODE=ENCAP to prevent REW from doing routing related frame editting. Decrement of TTL/Hop limit is still performed by REW. When performing routing related frame edits in ANA_ACL, editing of SMAC must be performed by ANA_L3 (ANA_L3::ROUTING_CFG.RT_SMAC_UPDATE_ENA).
    ///
    /// 0: Disable routing in ANA_ACL block. 1: Enable routing related frame edits independently of VCAP_IS2 action ACL_RT_MODE. 2: Enable routing related frame edits if VCAP_IS2 action ACL_RT_MODE allows routing.
    #[inline(always)]
    pub fn acl_rt_sel(&self) -> u32 {
        (self.0 & 0x60) >> 5
    }
    #[inline(always)]
    pub fn set_acl_rt_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 5;
        self.0 &= !0x60;
        self.0 |= value;
    }
    /// If set, classified VID is set to egress VID.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn acl_rt_update_cl_vid_ena(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_acl_rt_update_cl_vid_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// If set, IFH.GEN_IDX is set to egress RLEG.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn acl_rt_update_gen_idx_erleg_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_acl_rt_update_gen_idx_erleg_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// If set, IFH.GEN_IDX is set to egress VID.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn acl_rt_update_gen_idx_evid_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_acl_rt_update_gen_idx_evid_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Enable VCAP_IS2 key field IGR_PORT_MASK_SEL=3 for CPU injected frames.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn cpu_igr_mask_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_cpu_igr_mask_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable VCAP_IS2 key field IGR_PORT_MASK_SEL=3 for frames received with VStaX header.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn fp_vs2_igr_mask_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_fp_vs2_igr_mask_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Force VCAP_IS2 lookup to use IGR_PORT_MASK_SEL=3 for looped frames instead of IGR_PORT_MASK_SEL=1.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn lbk_igr_mask_sel3_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_lbk_igr_mask_sel3_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Enable VCAP_IS2 key field IGR_PORT_MASK_SEL=2 for masqueraded frames.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn masq_igr_mask_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_masq_igr_mask_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Controls if PAG(7) forces VID for service frames (ISDX > 0). Bit[0]: Relates to first lookup in VCAP_IS2 Bit[1]: Relates to second lookup in VCAP_IS2
    ///
    /// 0: Disable 1: Force VID when isdx >0
    #[inline(always)]
    pub fn pag_force_vid_ena(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    #[inline(always)]
    pub fn set_pag_force_vid_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 14;
        self.0 &= !0xc000;
        self.0 |= value;
    }
    /// Enable VCAP_IS2 key field IGR_PORT_MASK_SEL=3 for frames from VD0 or VD1.
    ///
    /// 0: Disable 1: Enable.
    #[inline(always)]
    pub fn vd_igr_mask_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_vd_igr_mask_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// If set, PIPELINE_PT is set to ANA_VLAN and PIPELINE_ACT is set to XTR if the frame is discarded in ANA_L3.
    #[inline(always)]
    pub fn vlan_pipeline_act_ena(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    #[inline(always)]
    pub fn set_vlan_pipeline_act_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
}
/// Configuration of TCP range generation
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VCAP_S2_RNG_CTRL(u32);
impl VCAP_S2_RNG_CTRL {
    /// Selected field matched against the range
    ///
    /// 0: Idle (No match) 1: TCP / UDP dport value is matched against range 2: TCP / UDP sport value is matched against range 3: TCP / UDP dport or sport values are matched against range 4: Classified VIDvalue is matched against range 5: Classified DSCP value is matched against range 6: Selected value from frame is matched against range, see ANA_ACL::VCAP_S2_RNG_OFFSET_CFG for details.
    #[inline(always)]
    pub fn rng_type_sel(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_rng_type_sel(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Configuration of selected range generation
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VCAP_S2_RNG_OFFSET_CFG(u32);
impl VCAP_S2_RNG_OFFSET_CFG {
    /// 16-bit offset position of selectable range matcher input counting from the EtherType (up to three VLAN tags skipped).
    ///
    /// 0: EtherType 1: frame byte 0 and 1 after EtherType ... n: frame byte 2n-2 and 2n-1 after EtherType
    #[inline(always)]
    pub fn rng_offset_pos(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_rng_offset_pos(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}
/// Configuration of  matcher range generation
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VCAP_S2_RNG_VALUE_CFG(u32);
impl VCAP_S2_RNG_VALUE_CFG {
    /// Upper range value
    #[inline(always)]
    pub fn rng_max_value(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    #[inline(always)]
    pub fn set_rng_max_value(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 16;
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    /// Lower range value
    #[inline(always)]
    pub fn rng_min_value(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_rng_min_value(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Configuration of how VOE loopback is performed
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VOE_LOOPBACK_CFG(u32);
impl VOE_LOOPBACK_CFG {
    /// Controls how loopback port is selected.
    ///
    /// 0: Loop to req.port_num 1: Loop to logical port found in ANA_AC:PGID[req.port_num+VOE_LOOP_PGID_OFFSET]
    #[inline(always)]
    pub fn voe_loop_pgid_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_voe_loop_pgid_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
