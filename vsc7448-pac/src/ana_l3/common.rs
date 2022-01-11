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
/// CPU Queue
///
/// Configuration of CPU queues relevant for routing
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CPU_QU_CFG(u32);
impl CPU_QU_CFG {
    /// CPU queue for IPv4/IPv6 frames failing MTU check. Related parameters: ANA_L3:COMMON:ROUTING_CFG.IP4_LEN_REDIR ANA_L3:COMMON:ROUTING_CFG.IP6_LEN_REDIR ANA_L3:VMID:MAX_LEN.IP4_MAX_LEN ANA_L3:VMID:MAX_LEN.IP6_MAX_LEN
    ///
    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn cpu_ip_len_qu(&self) -> u32 {
        (self.0 & 0x7000) >> 12
    }
    pub fn set_cpu_ip_len_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 12;
        self.0 &= !0x7000;
        self.0 |= value;
    }
    /// CPU queue for IP unicast frames with a TTL/HL <2 and with successful ARP lookup yielding a non-zero DMAC. CPU queue for IP multicast frames with a TTL/HL <2 and with successful VCAP lookup. Related enable bits: ANA_L3:COMMON:ROUTING_CFG.IP4_TTL_REDIR_ENA ANA_L3:COMMON:ROUTING_CFG.IP6_HC_REDIR_ENA ANA_L3:L3MC:L3MC_CTRL.IPMC_TTL_COPY_ENA
    ///
    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn cpu_ip_ttl_fail_qu(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_cpu_ip_ttl_fail_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// CPU queue for failed IPv4/IPv6 multicast lookup or failed RPF check. Related enable bits: ANA_L3:L3MC:L3MC_CTRL.RPF_CHK_ENA
    ///
    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn cpu_mc_fail_qu(&self) -> u32 {
        (self.0 & 0x700) >> 8
    }
    pub fn set_cpu_mc_fail_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 8;
        self.0 &= !0x700;
        self.0 |= value;
    }
    /// CPU queue number for IPv4 frames with IP header errors. Redirection of such frames is enabled using ROUTING_CFG.CPU_RLEG_IP_HDR_FAIL_REDIR_ENA
    ///
    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn cpu_rleg_ip_hdr_fail_qu(&self) -> u32 {
        (self.0 & 0x700000) >> 20
    }
    pub fn set_cpu_rleg_ip_hdr_fail_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 20;
        self.0 &= !0x700000;
        self.0 |= value;
    }
    /// CPU queue number for IPv4 frames with options and IPv6 frames with Hop-by-Hop option. Redirection of such frames is enabled using ROUTING_CFG.CPU_IP4_OPTIONS_REDIR_ENA ROUTING_CFG.CPU_IP6_HOPBYHOP_REDIR_ENA
    ///
    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn cpu_rleg_ip_opt_qu(&self) -> u32 {
        (self.0 & 0x7000000) >> 24
    }
    pub fn set_cpu_rleg_ip_opt_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 24;
        self.0 &= !0x7000000;
        self.0 |= value;
    }
    /// CPU queue number for non-IP unicast frames matching an ingress router leg, e.g. ARP PDUs. CPU queue for IP frames with L2 broadcast DMAC, received by router leg.
    ///
    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn cpu_rleg_qu(&self) -> u32 {
        (self.0 & 0x70000000) >> 28
    }
    pub fn set_cpu_rleg_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 28;
        self.0 &= !0x70000000;
        self.0 |= value;
    }
    /// CPU queue for frames failing SIP RPF check. Related parameters: ANA_L3:COMMON:ROUTING_CFG.RLEG_IP4_SIP_RPF_REDIR_ENA ANA_L3:COMMON:ROUTING_CFG.RLEG_IP6_SIP_RPF_REDIR_ENA ANA_L3:VMID:RLEG_CTRL.RLEG_IP4_SIP_RPF_MODE ANA_L3:VMID:RLEG_CTRL.RLEG_IP6_SIP_RPF_MODE
    ///
    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn cpu_sip_rpf_qu(&self) -> u32 {
        (self.0 & 0x70000) >> 16
    }
    pub fn set_cpu_sip_rpf_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 16;
        self.0 &= !0x70000;
        self.0 |= value;
    }
    /// CPU queue for failed IPv4/IPv6 unicast LPM lookup, invalid ARP entry (ARP_ENA=0) or failed ICMP redirect check. Related enable bits: ANA_L3:VMID:RLEG_CTRL.RLEG_IP4_ICMP_REDIR_ENA ANA_L3:VMID:RLEG_CTRL.RLEG_IP6_ICMP_REDIR_ENA
    ///
    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn cpu_uc_fail_qu(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_cpu_uc_fail_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 4;
        self.0 &= !0x70;
        self.0 |= value;
    }
}
/// Enable (DIP,SMAC) / (DIP,VMID) check
///
/// Bit per port that enables (DIP, DMAC) and/or (DIP, VMID) check.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct DIP_SECURE_ENA(u32);
impl DIP_SECURE_ENA {
    /// Lookup DIP in LPM and check that a) DMAC corresponds to MAC address in ARP table entry and/or b) Frame has been received on the router leg specified in ARP table entry. Any mismatch is signalled to ANA_ACL for use in security rules. DIP check is only performed for L2 forwarded IP unicast frames. Related parameters: ANA_L3:ARP:ARP_CFG_0.MAC_MSB ANA_L3:ARP:ARP_CFG_1.MAC_LSB ANA_L3:ARP:ARP_CFG_0.ARP_VMID ANA_L3:ARP:ARP_CFG_0.SECUR_MATCH_VMID_ENA ANA_L3:ARP:ARP_CFG_0.SECUR_MATCH_MAC_ENA ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.SECUR_DIP_FAIL_STICKY ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.SECUR_IP4_DIP_MATCH_STICK Y ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.SECUR_IP6_DIP_MATCH_STICK Y
    ///
    /// 0: Disable 1: Enable
    pub fn dip_cmp_ena(&self) -> u32 {
        self.0
    }
    pub fn set_dip_cmp_ena(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Enable (DIP,SMAC) / (DIP,VMID) check
///
/// Bit per port that enables (DIP, DMAC) and/or (DIP, VMID) check.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct DIP_SECURE_ENA1(u32);
impl DIP_SECURE_ENA1 {
    /// Refer to DIP_SECURE_ENA.DIP_CMP_ENA description.
    pub fn dip_cmp_ena1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_dip_cmp_ena1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Multicast routing control configuration
///
/// IP multicast traffic enable per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct L3_MC_ENA(u32);
impl L3_MC_ENA {
    /// Enable multicast routing per port. If disabled, IP multicast frames received on the port will not be routed. If disabled and L3_ENA_MODE=1, then routed IP multicast frames will not be transmitted on the port. Related parameters: ANA_L3:COMMON:ROUTING_CFG.L3_ENA_MODE.
    pub fn l3_mc_ena(&self) -> u32 {
        self.0
    }
    pub fn set_l3_mc_ena(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Multicast routing control Configuration
///
/// IP multicast traffic enable per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct L3_MC_ENA1(u32);
impl L3_MC_ENA1 {
    /// Refer to L3_MC_ENA.L3_MC_ENA description.
    pub fn l3_mc_ena1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_l3_mc_ena1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Unicast routing control configuration
///
/// Enable of L3 unicast traffic per port.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct L3_UC_ENA(u32);
impl L3_UC_ENA {
    /// Enable unicast routing per port. If disabled, IP unicast frames received on the port will not be routed. If disabled and L3_ENA_MODE=1, then routed IP unicast frames will not be transmitted on the port. Related parameters: ANA_L3:COMMON:ROUTING_CFG.L3_ENA_MODE.
    pub fn l3_uc_ena(&self) -> u32 {
        self.0
    }
    pub fn set_l3_uc_ena(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Unicast routing control configuration
///
/// Enable of L3 unicast traffic per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct L3_UC_ENA1(u32);
impl L3_UC_ENA1 {
    /// Refer to L3_UC_ENA.L3_UC_ENA description.
    pub fn l3_uc_ena1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_l3_uc_ena1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Miscellanous control parameters
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MISC_CTRL(u32);
impl MISC_CTRL {
    /// Enable update of AC for routed frames.
    pub fn ac_update_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ac_update_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Forwarding Control
///
/// Configuration of forwarding state per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_FWD_CTRL(u32);
impl PORT_FWD_CTRL {
    /// Enable forwarding per physical port. If disabled, frames received on port are discarded and frames are not forwarded to the port. Related parameters: ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.PORT_FWD_DENY_STICKY
    pub fn port_fwd_ena(&self) -> u32 {
        self.0
    }
    pub fn set_port_fwd_ena(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Forwarding Control
///
/// Configuration of forwarding state per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_FWD_CTRL1(u32);
impl PORT_FWD_CTRL1 {
    /// Refer to PORT_FWD_CTRL.PORT_FWD_ENA description.
    pub fn port_fwd_ena1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_port_fwd_ena1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Learning Control
///
/// Configuration of learning state per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_LRN_CTRL(u32);
impl PORT_LRN_CTRL {
    /// Enable/disable learning per physical port. Related parameters: ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.PORT_LRN_DENY_STICKY
    pub fn port_lrn_ena(&self) -> u32 {
        self.0
    }
    pub fn set_port_lrn_ena(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Learning Control
///
/// Configuration of learning state per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_LRN_CTRL1(u32);
impl PORT_LRN_CTRL1 {
    /// Refer to PORT_LRN_CTRL.PORT_LRN_ENA description.
    pub fn port_lrn_ena1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_port_lrn_ena1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Router leg base MAC address
///
/// Configuration of router leg base MAC address.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct RLEG_CFG_0(u32);
impl RLEG_CFG_0 {
    /// Router leg base MAC address, least significant bits. In order to have different MAC addresses per router leg, the base address may be incremented using VID or VMID, ref. RLEG_MAC_TYPE_SEL. This must be configured consistently in REW::RLEG_CFG_0.RLEG_MAC_LSB.
    ///
    /// Bit 0: MAC address, bit 0 ... Bit 23: MAC address, bit 23
    pub fn rleg_mac_lsb(&self) -> u32 {
        (self.0 & 0xffffff00) >> 8
    }
    pub fn set_rleg_mac_lsb(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        let value = value << 8;
        self.0 &= !0xffffff00;
        self.0 |= value;
    }
}
/// Router Leg base MAC address
///
/// Configuration of router leg base MAC address.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct RLEG_CFG_1(u32);
impl RLEG_CFG_1 {
    /// Router leg base MAC address, least significant bits. In order to have different MAC addresses per router leg, the base address may be incremented using VID or VMID, ref. RLEG_MAC_TYPE_SEL. This must be configured consistently in REW::RLEG_CFG_1.RLEG_MAC_MSB.
    ///
    /// Bit 0: MAC address, bit 24 ... Bit 23: MAC address, bit 47
    pub fn rleg_mac_msb(&self) -> u32 {
        self.0 & 0xffffff
    }
    pub fn set_rleg_mac_msb(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
    /// Configuration of router leg specific MAC address. This must be configured consistently in REW::RLEG_CFG_1.RLEG_MAC_TYPE_SEL.
    ///
    /// 0: VMID used to increment base MAC address: RLEG_MAC = RLEG_MAC_MSB[23:0] . ((RLEG_MAC_LSB[23:0] + VMID[7:0]) mod 2**24) 1: VID used to increment base MAC address: RLEG_MAC = RLEG_MAC_MSB[23:0] . ((RLEG_MAC_LSB[23:0] + VID[11:0]) mod 2**24) 2: Base MAC address used for all router legs RLEG_MAC = RLEG_MAC_MSB[23:0] . RLEG_MAC_LSB[23:0] 3: Reserved 3: Reserved.
    pub fn rleg_mac_type_sel(&self) -> u32 {
        (self.0 & 0x3000000) >> 24
    }
    pub fn set_rleg_mac_type_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 24;
        self.0 &= !0x3000000;
        self.0 |= value;
    }
}
/// Routing Configuration
///
/// Configuration of routing checks. Note that these checks only applies to frames matching a router leg.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ROUTING_CFG(u32);
impl ROUTING_CFG {
    /// Enable redirection to CPU of IPv4 frames with IP4 options. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_RLEG_IP_OPT_QU. Packets with IP options are not subject to routing. Related parameters: ANA_L3:COMMON:CPU_QU_CFG.CPU_RLEG_IP_OPT_QU ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.RLEG_MC_IP_OPT_REDIR_STIC KY
    pub fn cpu_ip4_options_redir_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_cpu_ip4_options_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable redirection to CPU of IPv6 frames with Hop-by-Hop options. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_RLEG_IP_OPT_QU. Packets with Hob-by-Hop options are not subject to routing.
    pub fn cpu_ip6_hopbyhop_redir_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_cpu_ip6_hopbyhop_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Control CPU redirection of IP error frames matching a router leg. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_RLEG_IP_HDR_FAIL_QU. The following errors are covered: IPv4 header length error IPv4 header checksum error SIP violations, if enabled in IP4_SIP_ADDR_VIOLATION_REDIR_ENA / IP6_SIP_ADDR_VIOLATION_REDIR_ENA. DIP violations, if enabled in IP4_DIP_ADDR_VIOLATION_REDIR_ENA / IP4_DIP_ADDR_VIOLATION_REDIR_ENA. IP MC frames with unicast DMAC. Related parameters: ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.RLEG_UC_HDR_ERR_REDIR_STI CKY ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.RLEG_MC_HDR_ERR_REDIR_STI CKY
    ///
    /// 0: Disable redirection (errored frames are discarded) 1: Enable redirection to CPU queue
    pub fn cpu_rleg_ip_hdr_fail_redir_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_cpu_rleg_ip_hdr_fail_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Enable DIP checks for IPv4 packets matching a router leg. If enabled and the frame's DIP falls within the given range then the frame is not routed. Bit 0: 0.0.0.0 - 0.255.255.255 Bit 1: 127.0.0.0 - 127.255.255.255 (Loopback network) Bit 2: 240.0.0.0 - 255.255.255.254 (Experimental) Frames which are not routed due to DIP check can be redirected to CPU by setting CPU_RLEG_IP_HDR_FAIL_REDIR_ENA=1.
    ///
    /// '0': Frame is allowed to be routed. '1': Frame is not routed. Frame is redirected to CPU if CPU_RLEG_IP_HDR_FAIL_REDIR_ENA=1.
    pub fn ip4_dip_addr_violation_redir_ena(&self) -> u32 {
        (self.0 & 0x3800) >> 11
    }
    pub fn set_ip4_dip_addr_violation_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 11;
        self.0 &= !0x3800;
        self.0 |= value;
    }
    /// Copy IPv4 frames with broadcast DMAC to CPU. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_RLEG_QU.
    pub fn ip4_l2_bc_copy_ena(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_ip4_l2_bc_copy_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Control handling of IPv4 frames which are otherwise to be L3 forwarded but have Total Length > ANA_L3:VMID:MAX_LEN.IP4_MAX_LEN for egress router leg. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_IP_LEN_QU
    ///
    /// 0: Drop frame 1: Redirect frame to CPU
    pub fn ip4_len_redir(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_ip4_len_redir(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// Route IPv4 multicast frames based on DIP only. I.e. do LPM lookup with only DIP as key. When IP4_MC_DIP_FWD_ENA is not set LPM lookup is performed with DIP+SIP as key.
    pub fn ip4_mc_dip_fwd_ena(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    pub fn set_ip4_mc_dip_fwd_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 27;
        self.0 &= !0x8000000;
        self.0 |= value;
    }
    /// Enable SIP checks for IPv4 packets matching a router leg. If enabled and the frame's SIP falls within the given range then the frame is not routed. Bit 0: 0.0.0.0 - 0.255.255.255 Bit 1: 127.0.0.0 - 127.255.255.255 (Loopback network) Bit 2: 224.0.0.0 - 255.255.255.255 (Multicast/experimental/broadcast) Frames which are not routed due to SIP check can be redirected to CPU by setting CPU_RLEG_IP_HDR_FAIL_REDIR_ENA=1.
    ///
    /// '0': Frame is allowed to be routed. '1': Frame is not routed. Frame is redirected to CPU if CPU_RLEG_IP_HDR_FAIL_REDIR_ENA=1.
    pub fn ip4_sip_addr_violation_redir_ena(&self) -> u32 {
        (self.0 & 0xe0) >> 5
    }
    pub fn set_ip4_sip_addr_violation_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 5;
        self.0 &= !0xe0;
        self.0 |= value;
    }
    /// Enable redirection to CPU of IPv4 UC packets, which match a router leg and have TTL less than 2. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_IP_TTL_FAIL_QU.
    ///
    /// 0: Disable redirection (errored frames are discarded) 1: Enable redirection to CPU queue
    pub fn ip4_ttl_redir_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ip4_ttl_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable DIP checks for IPv6 packets to be routed. If enabled and the frame's DIP falls within the given range then the frame is not routed. Bit 0: ::/128 (Unspecified address) Bit 1: ::1/128 (Loopback address) Frames which are not routed due to DIP check can be redirected to CPU by setting CPU_RLEG_IP_HDR_FAIL_REDIR_ENA=1.
    ///
    /// '0': Frame is allowed to be routed. '1': Frame is not routed. Frame is redirected to CPU if CPU_RLEG_IP_HDR_FAIL_REDIR_ENA=1.
    pub fn ip6_dip_addr_violation_redir_ena(&self) -> u32 {
        (self.0 & 0x18000) >> 15
    }
    pub fn set_ip6_dip_addr_violation_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 15;
        self.0 &= !0x18000;
        self.0 |= value;
    }
    /// Enable redirection to CPU of IPv6 UC packets, which match a router leg and have Hop Limit less than 2. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_IP_TTL_FAIL_QU.
    ///
    /// 0: Disable redirection (errored frames are discarded) 1: Enable redirection to CPU queue
    pub fn ip6_hc_redir_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ip6_hc_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Copy IPv6 frames with broadcast DMAC to CPU. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_RLEG_QU.
    pub fn ip6_l2_bc_copy_ena(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_ip6_l2_bc_copy_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Control handling of IPv6 frames which are otherwise to be L3 forwarded but have Payload Length > ANA_L3:VMID:MAX_LEN.IP6_MAX_LEN for egress router leg. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_IP_LEN_QU
    ///
    /// 0: Drop frame 1: Redirect frame to CPU
    pub fn ip6_len_redir(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_ip6_len_redir(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Route IPv6 multicast frames based on DIP only. I.e. do LPM lookup with only DIP as key. When IP6_MC_DIP_FWD_ENA is not set LPM lookup is performed with DIP+SIP as key.
    pub fn ip6_mc_dip_fwd_ena(&self) -> u32 {
        (self.0 & 0x10000000) >> 28
    }
    pub fn set_ip6_mc_dip_fwd_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 28;
        self.0 &= !0x10000000;
        self.0 |= value;
    }
    /// Enable SIP checks for IPv6 packets to be routed. If enabled and the frame's SIP falls within the given range then the frame is not routed. Bit 0: ::/128 (Unspecified address) Bit 1: ::1/128 (Loopback address) Bit 2: ff00::/8 (IPv6 multicast addresses) Frames which are not routed due to SIP check can be redirected to CPU by setting CPU_RLEG_IP_HDR_FAIL_REDIR_ENA=1.
    ///
    /// '0': Frame is allowed to be routed. '1': Frame is not routed. Frame is redirected to CPU if CPU_RLEG_IP_HDR_FAIL_REDIR_ENA=1.
    pub fn ip6_sip_addr_violation_redir_ena(&self) -> u32 {
        (self.0 & 0x700) >> 8
    }
    pub fn set_ip6_sip_addr_violation_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 8;
        self.0 &= !0x700;
        self.0 |= value;
    }
    /// Controls whether L3_UC_ENA and L3_MC_ENA affects ingress or ingress+egress behaviour. Note that L3_ENA_MODE=1 does not work together with stacking, assuming routing is performed by ingress unit. Related parameters: ANA_L3:COMMON:L3_UC_ENA ANA_L3:COMMON:L3_MC_ENA
    ///
    /// 0: L3_UC_ENA/L3_MC_ENA is only applied on ingress. 1: L3_UC_ENA/L3_MC_ENA is applied on ingress and egress.
    pub fn l3_ena_mode(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    pub fn set_l3_ena_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 29;
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    /// Redirect IPv4 frames failing SIP RPF check to CPU. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_SIP_RPF_QU. Related parameters: ANA_L3:VMID:RLEG_CTRL.RLEG_IP4_SIP_RPF_MODE ANA_L3:COMMON:CPU_QU_CFG.CPU_SIP_RPF_QU
    pub fn rleg_ip4_sip_rpf_redir_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rleg_ip4_sip_rpf_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Redirect IPv6 frames failing SIP RPF check to CPU. CPU queue is configured in ANA_L3:COMMON:CPU_QU_CFG.CPU_SIP_RPF_QU. Related parameters: ANA_L3:VMID:RLEG_CTRL.RLEG_IP6_SIP_RPF_MODE ANA_L3:COMMON:CPU_QU_CFG.CPU_SIP_RPF_QU
    pub fn rleg_ip6_sip_rpf_redir_ena(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_rleg_ip6_sip_rpf_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Control which non-IP unicast frames, matching router leg's MAC address, that shall be redirected to CPU. CPU queue is configured in CPU_QU_CFG.CPU_RLEG_QU.
    ///
    /// 0: Redirect all frames. 1: Redirect ARP frames (Ethertype 0x0806) 2: Redirect RARP frames (Ethertype 0x8035) 3: Redirect ARP and RARP frames.
    pub fn rleg_nonip_uc_redir_mode(&self) -> u32 {
        (self.0 & 0x3000000) >> 24
    }
    pub fn set_rleg_nonip_uc_redir_mode(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 24;
        self.0 &= !0x3000000;
        self.0 |= value;
    }
    /// Change SMAC of routed frame to the SMAC of egress router leg. Normally SMAC is changed in REW, but if routing and PTP is to be supported concurrently , then RT_SMAC_UPDATE_ENA must be set. When RT_SMAC_UPDATE_ENA is set and ingress mirroring is used, then ingress mirrored frames get SMAC and DMAC values corresponding to the routed frame. Related parameters: ANA_ACL::VCAP_S2_MISC_CTRL.ACL_RT_SEL
    pub fn rt_smac_update_ena(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    pub fn set_rt_smac_update_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 26;
        self.0 &= !0x4000000;
        self.0 |= value;
    }
}
/// Service Control
///
/// Miscellaneous service configuration.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SERVICE_CFG(u32);
impl SERVICE_CFG {
    /// Force Egress FID=ANA_L3:VLAN:VLAN_CFG.VLAN_FID when ANA_CL:IPT:VSI_CFG.VSI_ENA==1 and frame has multicast DMAC. See also SERVICE_CFG.VSI_FORCE_MC_EFID_ENA.
    ///
    /// 0: Normal EFID behaviour for multicast  DMAC. 1: Force EFID=ANA_L3:VLAN:VLAN_CFG.VLAN_FID when VSI_ENA==1 and frame has multicast DMAC.
    pub fn isdx_force_mc_efid_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_isdx_force_mc_efid_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Force Egress FID=ANA_L3:VLAN:VLAN_CFG.VLAN_FID when ISDX > 0 and frame has multicast DMAC. See also SERVICE_CFG.VSI_FORCE_MC_EFID_ENA.
    ///
    /// 0: Normal EFID behaviour for multicast  DMAC. 1: Force EFID=ANA_L3:VLAN:VLAN_CFG.VLAN_FIDfor multicast DMAC when ISDX > 0.
    pub fn vsi_force_mc_efid_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_vsi_force_mc_efid_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Enable SIP RPF check
///
/// Bit per port that enables SIP RPF check.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SIP_RPF_ENA(u32);
impl SIP_RPF_ENA {
    /// Enable SIP RPF check per ingress port. For more information, refer to ANA_L3:VMID:RLEG_CTRL.RLEG_IP4_SIP_RPF_MODE ANA_L3:VMID:RLEG_CTRL.RLEG_IP6_SIP_RPF_MODE
    pub fn sip_rpf_ena(&self) -> u32 {
        self.0
    }
    pub fn set_sip_rpf_ena(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Enable SIP RPF check
///
/// Bit per port that enables SIP RPF check.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SIP_RPF_ENA1(u32);
impl SIP_RPF_ENA1 {
    /// Enable SIP RPF check per ingress port. For more information, refer to ANA_L3:VMID:RLEG_CTRL.RLEG_IP4_SIP_RPF_MODE ANA_L3:VMID:RLEG_CTRL.RLEG_IP6_SIP_RPF_MODE
    pub fn sip_rpf_ena1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_sip_rpf_ena1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Enable (SIP,SMAC) / (SIP,VMID) check
///
/// Bit per port that enables (SIP, SMAC) and/or (SIP, VMID) check.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SIP_SECURE_ENA(u32);
impl SIP_SECURE_ENA {
    /// Lookup SIP in LPM and check that a) SMAC corresponds to MAC address in ARP table entry and/or b) Frame has been received on the router leg specified in ARP table entry. Any mismatch is signalled to ANA_ACL for use in security rules. Related parameters: ANA_L3:ARP:ARP_CFG_0.MAC_MSB ANA_L3:ARP:ARP_CFG_1.MAC_LSB ANA_L3:ARP:ARP_CFG_0.ARP_VMID ANA_L3:ARP:ARP_CFG_0.SECUR_MATCH_VMID_ENA ANA_L3:ARP:ARP_CFG_0.SECUR_MATCH_MAC_ENA ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.SECUR_DIP_FAIL_STICKY ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.SECUR_IP4_SIP_MATCH_STICK Y ANA_L3:VLAN_ARP_L3MC_STICKY:L3_ARP_IPMC_STICKY.SECUR_IP6_SIP_MATCH_STICK Y
    ///
    /// 0: Disable 1: Enable
    pub fn sip_cmp_ena(&self) -> u32 {
        self.0
    }
    pub fn set_sip_cmp_ena(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Enable (SIP,SMAC) / (SIP,VMID) check
///
/// Bit per port that enables (SIP, SMAC) and/or (SIP, VMID) check.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SIP_SECURE_ENA1(u32);
impl SIP_SECURE_ENA1 {
    /// Refer to SIP_SECURE_ENA.SIP_CMP_ENA description.
    pub fn sip_cmp_ena1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_sip_cmp_ena1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// VLAN Community port mask
///
/// Configuration of Community port mask. See description of ANA_L3:VLAN:VLAN_CFG.VLAN_PRIVATE_ENA
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_COMMUNITY_CFG(u32);
impl VLAN_COMMUNITY_CFG {
    /// Ports marked in this mask are treated as community ports, if the VLAN is a private VLAN. Ref. ANA_L3:VLAN:VLAN_CFG.VLAN_PRIVATE_ENA
    ///
    /// 0: Not a Community port (i.e. Promiscuous port or Isolated port) 1: Community port.
    pub fn vlan_community_mask(&self) -> u32 {
        self.0
    }
    pub fn set_vlan_community_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// VLAN Community port mask
///
/// Configuration of Community port mask. See description of ANA_L3:VLAN:VLAN_CFG.VLAN_PRIVATE_ENA
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_COMMUNITY_CFG1(u32);
impl VLAN_COMMUNITY_CFG1 {
    /// Refer to VLAN_COMMUNITY_CFG.VLAN_COMMUNITY_MASK description.
    pub fn vlan_community_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_vlan_community_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// VLAN Control
///
/// Basic VLAN related configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_CTRL(u32);
impl VLAN_CTRL {
    /// Enable/disable VLAN lookup. This field must be enabled to allow VLAN and MSTP filtering. For VLAN unaware operation, this field can be disabled.
    pub fn vlan_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vlan_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// VLAN Filter Control
///
/// Configuration of VLAN ingress filtering per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_FILTER_CTRL(u32);
impl VLAN_FILTER_CTRL {
    /// Enable VLAN ingress filtering per port. If a port is enabled in this mask, frames received on the port are discarded if the port is not a member of the classified VLAN. VLAN ingress filtering can also be enabled per VLAN. VLAN ingress filtering is performed if either enabled for ingress port or for VLAN. Related parameters: ANA_L3:VLAN:VLAN_CFG.VLAN_IGR_FILTER_ENA ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.VLAN_IGR_FILTER_STICKY
    pub fn vlan_igr_filter_ena(&self) -> u32 {
        self.0
    }
    pub fn set_vlan_igr_filter_ena(&mut self, value: u32) {
        self.0 = value;
    }
}
/// VLAN Filter Control
///
/// Configuration of VLAN ingress filtering per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_FILTER_CTRL1(u32);
impl VLAN_FILTER_CTRL1 {
    /// Refer to VLAN_FILTER_CTRL.VLAN_IGR_FILTER_ENA description.
    pub fn vlan_igr_filter_ena1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_vlan_igr_filter_ena1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// VLAN isolated port mask
///
/// Configuration of isolated port mask. See description of ANA_L3:VLAN:VLAN_CFG.VLAN_PRIVATE_ENA
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_ISOLATED_CFG(u32);
impl VLAN_ISOLATED_CFG {
    /// Ports marked in this mask are treated as isolated ports, if the VLAN is a private VLAN. Ref. ANA_L3:VLAN:VLAN_CFG.VLAN_PRIVATE_ENA
    ///
    /// 0: Not an Isolated port (i.e. Promiscuous port or Community port) 1: Isolated port.
    pub fn vlan_isolated_mask(&self) -> u32 {
        self.0
    }
    pub fn set_vlan_isolated_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// VLAN isolated port mask
///
/// Configuration of isolated port mask. See description of ANA_L3:VLAN:VLAN_CFG.VLAN_PRIVATE_ENA
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_ISOLATED_CFG1(u32);
impl VLAN_ISOLATED_CFG1 {
    /// Refer to VLAN_ISOLATED_CFG.VLAN_ISOLATED_MASK description.
    pub fn vlan_isolated_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_vlan_isolated_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// IPv4 Virtual Router Leg Configuration
///
/// Configuration of VRRP MAC address for IPv4. Use of VRRP for IPv4 is enabled per router leg in ANA_L3:VMID:RLEG_CTRL.RLEG_IP4_VRID_ENA
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VRRP_IP4_CFG_0(u32);
impl VRRP_IP4_CFG_0 {
    /// Mid part of IPv4 Virtual Router Redundancy Protocol MAC address.
    ///
    /// IPv4 VRRP MAC address Bit 31 downto 8.
    pub fn vrrp_ip4_base_mac_mid(&self) -> u32 {
        (self.0 & 0xffffff00) >> 8
    }
    pub fn set_vrrp_ip4_base_mac_mid(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        let value = value << 8;
        self.0 &= !0xffffff00;
        self.0 |= value;
    }
}
/// IPv4 Virtual Router Leg Configuration
///
/// Configuration of VRRP MAC address for IPv4. Use of VRRP for IPv4 is enabled per router leg in ANA_L3:VMID:RLEG_CTRL.RLEG_IP4_VRID_ENA
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VRRP_IP4_CFG_1(u32);
impl VRRP_IP4_CFG_1 {
    /// Upper part of IPv4 Virtual Router Redundancy Protocol MAC address.
    ///
    /// IPv4 VRRP MAC address Bit 47 downto 32.
    pub fn vrrp_ip4_base_mac_high(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_vrrp_ip4_base_mac_high(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// IPv6 Virtual Router Leg Configuration
///
/// Configuration of VRRP MAC address for IPv6. Use of VRRP for IPv6 is enabled per router leg in ANA_L3:VMID:RLEG_CTRL.RLEG_IP6_VRID_ENA
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VRRP_IP6_CFG_0(u32);
impl VRRP_IP6_CFG_0 {
    /// Mid part of IPv6 Virtual Router Redundancy Protocol MAC address.
    ///
    /// IPv6 VRRP MAC address Bit 31 downto 8.
    pub fn vrrp_ip6_base_mac_mid(&self) -> u32 {
        self.0 & 0xffffff
    }
    pub fn set_vrrp_ip6_base_mac_mid(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// IPv6 Virtual Router Leg Configuration
///
/// Configuration of VRRP MAC address for IPv6. Use of VRRP for IPv6 is enabled per router leg in ANA_L3:VMID:RLEG_CTRL.RLEG_IP6_VRID_ENA
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VRRP_IP6_CFG_1(u32);
impl VRRP_IP6_CFG_1 {
    /// Upper part of IPv6 Virtual Router Redundancy Protocol MAC address.
    ///
    /// IPv6 VRRP MAC address Bit 47 downto 32.
    pub fn vrrp_ip6_base_mac_high(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_vrrp_ip6_base_mac_high(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
