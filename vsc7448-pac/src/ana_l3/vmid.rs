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
/// Maximum length for frames routed to this router leg
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MAX_LEN(u32);
impl MAX_LEN {
    /// Max "Total Length" (ref. RFC791) of IPv4 frames using this egress router leg. Related parameters: ANA_L3:COMMON:ROUTING_CFG.IP4_LEN_REDIR ANA_L3:COMMON:CPU_QU_CFG.CPU_IP_LEN_QU
    pub fn ip4_max_len(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_ip4_max_len(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// Max "Payload Length" (ref. RFC2460) of IPv6 frames using this egress router leg. Related parameters: ANA_L3:COMMON:ROUTING_CFG.IP6_LEN_REDIR ANA_L3:COMMON:CPU_QU_CFG.CPU_IP_LEN_QU
    pub fn ip6_max_len(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_ip6_max_len(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
/// Router leg control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct RLEG_CTRL(u32);
impl RLEG_CTRL {
    /// VID for egress router leg. This must be configured consistently in REW:VMID:RLEG_CTRL.RLEG_EVID.
    pub fn rleg_evid(&self) -> u32 {
        (self.0 & 0xfff0000) >> 16
    }
    pub fn set_rleg_evid(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xfff0000);
        self.0 &= !0xfff0000;
        self.0 |= value;
    }
    /// Enable redirection to CPU of IPv4 packets with IVMID==EVMID (i.e. packets that are to be routed back to the router leg, they were received on). CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_UC_FAIL_QU. Related parameters: ANA_L3:COMMON:CPU_QU_CFG.CPU_UC_FAIL_QU ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.UC_ICMP_REDIR_STICKY
    pub fn rleg_ip4_icmp_redir_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_rleg_ip4_icmp_redir_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable IPv4 multicast routing.
    pub fn rleg_ip4_mc_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rleg_ip4_mc_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// SIP RPF mode for IPv4. If SIP RPF check fails, then frame is not allowed to be L3 forwarded. Disabled: No SIP RPF check. RGID Mode: Verify that SIP belongs to a known subnet and verify that SIP's RGID is enabled in ingress router leg's RLEG_RGID_MASK. Rleg Mode: Verify that SIP belongs to a known subnet, and that the frame is received on the router leg through which that subnet is reached. If SIP is reached through an ECMP path then no SIP RPF check is performed. Combined Mode: Apply Rleg Mode for non ECMP paths and RGID Mode for ECMP paths. Related parameters: ANA_L3:COMMON:SIP_RPF_ENA ANA_L3:COMMON:ROUTING_CFG.RLEG_IP4_SIP_RPF_REDIR_ENA ANA_L3:COMMON:CPU_QU_CFG.CPU_SIP_RPF_QU ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.IP4_SIP_RPF_FILTER_STICKY ANA_L3:VMID:SIP_RPF.RLEG_RGID_MASK
    ///
    /// 0: Disabled 1: RGID Mode 2: Rleg Mode 3: Combined Mode
    pub fn rleg_ip4_sip_rpf_mode(&self) -> u32 {
        (self.0 & 0x300) >> 8
    }
    pub fn set_rleg_ip4_sip_rpf_mode(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x300);
        self.0 &= !0x300;
        self.0 |= value;
    }
    /// Enable IPv4 unicast routing.
    pub fn rleg_ip4_uc_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_rleg_ip4_uc_ena(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Enable IPv4 virtual router leg. If enabled, the router leg can also be addressed using a MAC address constructed using the following fields: Bits 47-32: ANA_L3:COMMON:VRRP_IP4_CFG_1.VRRP_IP4_BASE_MAC_HIGH Bits 31-8: ANA_L3:COMMON:VRRP_IP4_CFG_0.VRRP_IP4_BASE_MAC_MID Bits 7-0: ANA_L3:VMID.VRRP_CFG.RLEG_IP4_VRID
    pub fn rleg_ip4_vrid_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_rleg_ip4_vrid_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable redirection to CPU of IPv6 packets with IVMID==EVMID (i.e. packets that are to be routed back to the router leg, they were received on). CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_UC_FAIL_QU. Related parameters: ANA_L3:COMMON:CPU_QU_CFG.CPU_UC_FAIL_QU ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.UC_ICMP_REDIR_STICKY
    pub fn rleg_ip6_icmp_redir_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_rleg_ip6_icmp_redir_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Enable IPv6 multicast routing.
    pub fn rleg_ip6_mc_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_rleg_ip6_mc_ena(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// SIP RPF mode for IPv6. If SIP RPF check fails, then frame is not allowed to be L3 forwarded. Disabled: No SIP RPF check. RGID Mode: Verify that SIP belongs to a known subnet and verify that SIP's RGID is enabled in ingress router leg's RGID_MASK. Rleg Mode: Verify that SIP belongs to a known subnet, and that the frame is received on the router leg through which that subnet is reached. If SIP is reached through an ECMP path then no SIP RPF check is performed. Combined Mode: Apply Rleg Mode for non ECMP paths and RGID Mode for ECMP paths. Related parameters: ANA_L3:COMMON:SIP_RPF_ENA ANA_L3:COMMON:ROUTING_CFG.RLEG_IP6_SIP_RPF_REDIR_ENA ANA_L3:COMMON:CPU_QU_CFG.CPU_SIP_RPF_QU ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.IP6_SIP_RPF_FILTER_STICKY ANA_L3:VMID:SIP_RPF.RLEG_RGID_MASK
    ///
    /// 0: Disabled 1: RGID Mode 2: Rleg Mode 3: Combined Mode
    pub fn rleg_ip6_sip_rpf_mode(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    pub fn set_rleg_ip6_sip_rpf_mode(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xc00);
        self.0 &= !0xc00;
        self.0 |= value;
    }
    /// Enable IPv6 unicast routing.
    pub fn rleg_ip6_uc_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_rleg_ip6_uc_ena(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Enable IPv6 virtual router leg. If enabled, the router leg can also be addressed using a MAC address constructed using the following fields: Bits 47-32: ANA_L3:COMMON:VRRP_IP6_CFG_1.VRRP_IP6_BASE_MAC_HIGH Bits 31-8: ANA_L3:COMMON:VRRP_IP6_CFG_0.VRRP_IP6_BASE_MAC_MID Bits 7-0: ANA_L3:VMID.VRRP_CFG.RLEG_IP6_VRID
    pub fn rleg_ip6_vrid_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_rleg_ip6_vrid_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// SIP RPF check parameters
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SIP_RPF(u32);
impl SIP_RPF {
    /// Route Group IDs (RGIDs) accepted for this router leg when SIP_RPF_MODE is set to "RGID Mode". See description of RLEG_IP4_SIP_RPF_MODE / RLEG_IP6_SIP_RPF_MODE.
    ///
    /// Bit 0: Accept routes with RGID=0 Bit 1: Accept routes with RGID=1 ...
    pub fn rleg_rgid_mask(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_rleg_rgid_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// IP multicast router leg configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VMID_MC(u32);
impl VMID_MC {
    /// Enable checking of DMAC for IPv4 multicast packets. I.e. verify that DMAC[47:24] == 0x01005e DMAC[23] == 0 If the check fails, the packet is not routed.
    pub fn rleg_ip4_mc_dmac_chk_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_rleg_ip4_mc_dmac_chk_ena(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// IPv4 multicast TTL limit. Packets with TTL below the configured limit are not routed. TTL is checked against RLEG_IP4_MC_TTL before transmission on egress router leg. Note: Regardless of the value configured for this parameter, IPv4 MC packets with TTL<2 are not routed.
    ///
    /// 0: Router leg based MC TTL check disabled. 1-2: N/A - such packets are not routed anyway. 3: If packet's TTL is < 3 then packet is not routed. 4: If packet's TTL is < 4 then packet is not routed. ...
    pub fn rleg_ip4_mc_ttl(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_rleg_ip4_mc_ttl(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// Enable checking of DMAC for IPv6 multicast packets. I.e. verify that DMAC[47:32] == 0x3333 If the check fails, the packet is not routed.
    pub fn rleg_ip6_mc_dmac_chk_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rleg_ip6_mc_dmac_chk_ena(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// IPv6 multicast HL limit. Packets with HL below the configured limit are not routed. HL is checked against RLEG_IP6_MC_TTL before transmission on egress router leg. Note: Regardless of the value configured for this parameter, IPv6 MC packets with HC<2 are not routed.
    ///
    /// 0: Router leg based MC HL check disabled 1-2: N/A - such packets are not routed anyway. 3: If packet's HC is < 3 then packet is not routed. 4: If packet's HC is < 4 then packet is not routed. ...
    pub fn rleg_ip6_mc_ttl(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_rleg_ip6_mc_ttl(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
}
/// Virtual router leg configuration
///
/// Configuration of Virtual Router Interface MAC address
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VRRP_CFG(u32);
impl VRRP_CFG {
    /// Router leg's VRID for IPv4. The configured VRID is used as part of the VRRP router MAC address. Only applicable if VRRP is enabled for router leg. See RLEG_CTRL.RLEG_IP4_VRID_ENA for further details. If only one of the two VRIDs is used, then the unused VRID must be set to the same value as the VRID in use.
    pub fn rleg_ip4_vrid(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_rleg_ip4_vrid(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// Router leg's VRID for IPv6. The configured VRID is used as part of the VRRP router MAC address. Only applicable if VRRP is enabled for router leg. See RLEG_CTRL.RLEG_IP6_VRID_ENA for further details. If only one of the two VRIDs is used, then the unused VRID must be set to the same value as the VRID in use.
    pub fn rleg_ip6_vrid(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_rleg_ip6_vrid(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
}
