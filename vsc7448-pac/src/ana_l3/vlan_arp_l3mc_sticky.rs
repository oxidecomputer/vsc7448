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
/// ARP and L3MC table related diagnostic registers
///
/// ARP and L3MC table related diagnostic registers.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct L3_ARP_IPMC_STICKY(u32);
impl L3_ARP_IPMC_STICKY {
    /// Set if the IPv4/IPv6 DIP or (DIP,SIP) lookup failed due to - No match in LPM table - Unsupported action type in LPM table - ARP table lookup returns an invalid entry.
    #[inline(always)]
    pub fn entry_not_found_sticky(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    #[inline(always)]
    pub fn set_entry_not_found_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 23;
        self.0 &= !0x800000;
        self.0 |= value;
    }
    /// Set if IPv4 SIP RPF check results in a frame not being L3 forwarded. Related parameters: ANA_L3:VMID:RLEG_CTRL.RLEG_IP4_SIP_RPF_MODE
    #[inline(always)]
    pub fn ip4_sip_rpf_filter_sticky(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    #[inline(always)]
    pub fn set_ip4_sip_rpf_filter_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 26;
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    /// Set if IPv6 SIP RPF check results in a frame not being L3 forwarded. Related parameters: ANA_L3:VMID:RLEG_CTRL.RLEG_IP6_SIP_RPF_MODE
    #[inline(always)]
    pub fn ip6_sip_rpf_filter_sticky(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    #[inline(always)]
    pub fn set_ip6_sip_rpf_filter_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 27;
        self.0 &= !0x8000000;
        self.0 |= value;
    }
    /// Set if a frame has exceeded IP4_MAX_LEN/IP6_MAX_LEN for an egress router leg. Ref. ANA_L3:VMID:MAX_LEN.IP4_MAX_LEN ANA_L3:VMID:MAX_LEN.IP6_MAX_LEN
    #[inline(always)]
    pub fn ip_max_len_exceeded_sticky(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    #[inline(always)]
    pub fn set_ip_max_len_exceeded_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 24;
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    /// Set when L2 forwarding of IPv4 multicast frame has completed.
    #[inline(always)]
    pub fn l2_mc_fwd_sticky(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_l2_mc_fwd_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Set when IP multicast L3 forwarding has been completed for a frame.
    #[inline(always)]
    pub fn l3_mc_fwd_sticky(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_l3_mc_fwd_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Set if an IP MC frame copy from VD0 does not get L3 forwarded (i.e. gets dropped).
    #[inline(always)]
    pub fn mc_looped_cp_sticky(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    #[inline(always)]
    pub fn set_mc_looped_cp_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Set if an IP MC frame has failed RPF check. Related parameters: ANA_L3:L3MC:L3MC_CTRL.RPF_CHK_ENA
    #[inline(always)]
    pub fn mc_rpf_filter_sticky(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    #[inline(always)]
    pub fn set_mc_rpf_filter_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Set if neither L2 nor L3 forwarding is performed for an IP multicast frame.
    #[inline(always)]
    pub fn no_mc_fwd_sticky(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline(always)]
    pub fn set_no_mc_fwd_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// Set if IPv4/IPv6 multicast forwarding fails because no there is no router leg  in ANA_L3:L3MC:EVMID_MASK_CFG.EVMID_MASK to forward to.
    #[inline(always)]
    pub fn no_mc_vmid_avail_sticky(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    #[inline(always)]
    pub fn set_no_mc_vmid_avail_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 25;
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    /// Set if an ingress router leg match has been found for an IP multicast frame with header errors. Refer to ANA_L3:COMMON:ROUTING_CFG.CPU_RLEG_IP_HDR_FAIL_REDIR_ENA for list of covered errors.
    #[inline(always)]
    pub fn rleg_mc_hdr_err_redir_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_rleg_mc_hdr_err_redir_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set if an ingress router leg match has been found for an IP multicast frame.
    #[inline(always)]
    pub fn rleg_mc_hit_sticky(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_rleg_mc_hit_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Set if ingress router leg match has been found for an IPv4/IPv6 multicast frame with options/hop-by-hop options. Such frames may optionally be redirected to CPU, ref. ANA_L3:COMMON:ROUTING_CFG.CPU_IP4_OPTIONS_REDIR_ENA ANA_L3:COMMON:ROUTING_CFG.CPU_IP6_HOPBYHOP_REDIR_ENA A similar sticky bit is available for IP unicast frames: RLEG_UC_IP_OPT_REDIR_STICKY
    #[inline(always)]
    pub fn rleg_mc_ip_opt_redir_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_rleg_mc_ip_opt_redir_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set if ingress router leg match has been found for an IP multicast frame with a TTL less than 2.
    #[inline(always)]
    pub fn rleg_mc_ttl_sticky(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    #[inline(always)]
    pub fn set_rleg_mc_ttl_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Set if ingress router leg match has been found for a non-IP frame. Such frames are redirected to ANA_L3:COMMON:CPU_QU_CFG.CPU_RLEG_QU
    #[inline(always)]
    pub fn rleg_nonip_uc_redir_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_rleg_nonip_uc_redir_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set if an ingress router leg match has been found for an IP unicast frame with header errors. Refer to ANA_L3:COMMON:ROUTING_CFG.CPU_RLEG_IP_HDR_FAIL_REDIR_ENA for list of covered errors.
    #[inline(always)]
    pub fn rleg_uc_hdr_err_redir_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_rleg_uc_hdr_err_redir_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set if an ingress router leg match has been found for an IP unicast frame.
    #[inline(always)]
    pub fn rleg_uc_hit_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_rleg_uc_hit_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Set if ingress router leg match has been found for an IPv4/IPv6 unicast frame with options/hop-by-hop options. Such frames may optionally be redirected to CPU, ref. ANA_L3:COMMON:ROUTING_CFG.CPU_IP4_OPTIONS_REDIR_ENA ANA_L3:COMMON:ROUTING_CFG.CPU_IP6_HOPBYHOP_REDIR_ENA A similar sticky bit is available for IP multicast frames: RLEG_MC_IP_OPT_REDIR_STICKY
    #[inline(always)]
    pub fn rleg_uc_ip_opt_redir_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_rleg_uc_ip_opt_redir_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set if security lookup fails for DIP, i.e. mismatch for (DMAC,DIP) / (VMID,DIP). Related parameters: ANA_L3:COMMON:DIP_SECURE_ENA.DIP_CMP_ENA
    #[inline(always)]
    pub fn secur_dip_fail_sticky(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_secur_dip_fail_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Set for successful IPv4 DIP security lookup, i.e. matching for (DMAC,DIP) and/or (VMID,DIP). Related parameters: ANA_L3:COMMON:DIP_SECURE_ENA.DIP_CMP_ENA
    #[inline(always)]
    pub fn secur_ip4_dip_match_sticky(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_secur_ip4_dip_match_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Set for successful IPv4 SIP security lookup, i.e. matching for (DMAC,SIP) and/or (VMID,SIP). Related parameters: ANA_L3:COMMON:SIP_SECURE_ENA.SIP_CMP_ENA
    #[inline(always)]
    pub fn secur_ip4_sip_match_sticky(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_secur_ip4_sip_match_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Set for successful IPv6 DIP security lookup, i.e. matching for (DMAC,DIP) and/or (VMID,DIP). Related parameters: ANA_L3:COMMON:DIP_SECURE_ENA.DIP_CMP_ENA
    #[inline(always)]
    pub fn secur_ip6_dip_match_sticky(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_secur_ip6_dip_match_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Set for successful IPv6 SIP security lookup, i.e. matching for (DMAC,SIP) and/or (VMID,SIP). Related parameters: ANA_L3:COMMON:SIP_SECURE_ENA.SIP_CMP_ENA
    #[inline(always)]
    pub fn secur_ip6_sip_match_sticky(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_secur_ip6_sip_match_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Set if security lookup fails for SIP, i.e. mismatch for (DMAC,DIP) / (VMID,DIP). Related parameters: ANA_L3:COMMON:SIP_SECURE_ENA.SIP_CMP_ENA
    #[inline(always)]
    pub fn secur_sip_fail_sticky(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_secur_sip_fail_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Set if IP unicast routing lookup results in a valid ARP entry allowing the frame to be routed.
    #[inline(always)]
    pub fn uc_entry_found_sticky(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    #[inline(always)]
    pub fn set_uc_entry_found_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 14;
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Set if a frame has been redirected to CPU due to ICMP Redirect check. Related parameters: ANA_L3:VMID.RLEG_CTRL.RLEG_IP4_ICMP_REDIR_ENA ANA_L3:VMID.RLEG_CTRL.RLEG_IP6_ICMP_REDIR_ENA
    #[inline(always)]
    pub fn uc_icmp_redir_sticky(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline(always)]
    pub fn set_uc_icmp_redir_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Set in case of an IP unicast frame being filtered due to TTL. I.e. a frame with TTL<2 which hits a valid ARP table entry with non-zero MAC address.
    #[inline(always)]
    pub fn uc_ttl_filtering_sticky(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    #[inline(always)]
    pub fn set_uc_ttl_filtering_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Set if a frame has been redirected to CPU due to a zero DMAC address in the ARP entry.
    #[inline(always)]
    pub fn uc_zero_dmac_found_sticky(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_uc_zero_dmac_found_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// DIP or (DIP,SIP) LPM lookup has resulted in match with unsupported action type.
    #[inline(always)]
    pub fn wrong_dip_lpm_action_type_sticky(&self) -> u32 {
        (self.0 & 0x10000000) >> 28
    }
    #[inline(always)]
    pub fn set_wrong_dip_lpm_action_type_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 28;
        self.0 &= !0x10000000;
        self.0 |= value;
    }
    /// SIP LPM lookup has resulted in match with unsupported action type.
    #[inline(always)]
    pub fn wrong_sip_lpm_action_type_sticky(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    #[inline(always)]
    pub fn set_wrong_sip_lpm_action_type_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 29;
        self.0 &= !0x20000000;
        self.0 |= value;
    }
}
/// VLAN Diagnostic
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_STICKY(u32);
impl VLAN_STICKY {
    /// Set if a frame has been filtered due to MSTP_FWD_MASK. Related parameters: ANA_L3:MSTP:MSTP_FWD_CFG.MSTP_FWD_MASK
    #[inline(always)]
    pub fn mstp_discard_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_mstp_discard_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Set if MSTP_FWD_MASK has allowed forwarding of a frame. Related parameters: ANA_L3:MSTP:MSTP_FWD_CFG.MSTP_FWD_MASK
    #[inline(always)]
    pub fn mstp_fwd_allowed_sticky(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_mstp_fwd_allowed_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Set if MSTP_LRN_MASK has allowed learning for a frame. Related parameters: ANA_L3:MSTP:MSTP_LRN_CFG.MSTP_LRN_MASK
    #[inline(always)]
    pub fn mstp_lrn_allowed_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_mstp_lrn_allowed_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set if MSTP_LRN_MASK has denied learning for a frame. Related parameters: ANA_L3:MSTP:MSTP_LRN_CFG.MSTP_LRN_MASK
    #[inline(always)]
    pub fn mstp_lrn_deny_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_mstp_lrn_deny_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set if a frame has been denied forwarding due to ANA_L3:COMMON:PORT_FWD_CTRL.PORT_FWD_ENA = 0
    #[inline(always)]
    pub fn port_fwd_deny_sticky(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    #[inline(always)]
    pub fn set_port_fwd_deny_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Set if learning has been denied for a frame due to ANA_L3:COMMON:PORT_LRN_CTRL.PORT_LRN_ENA = 0
    #[inline(always)]
    pub fn port_lrn_deny_sticky(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_port_lrn_deny_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Set if a frame has been filtered due to VLAN ingress filtering. Related parameters: ANA_L3:COMMON:VLAN_FILTER_CTRL.VLAN_IGR_FILTER_ENA
    #[inline(always)]
    pub fn vlan_igr_filter_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_vlan_igr_filter_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set if a frame has been classified to a VLAN with an empty port mask. Related parameters: ANA_L3:VLAN:VLAN_MASK_CFG.VLAN_PORT_MASK
    #[inline(always)]
    pub fn vlan_lookup_invld_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_vlan_lookup_invld_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set if VLAN_LRN_DIS has denied learning for a frame. Related parameters: ANA_L3:VLAN:VLAN_CFG.VLAN_LRN_DIS
    #[inline(always)]
    pub fn vlan_lrn_deny_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_vlan_lrn_deny_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
}
