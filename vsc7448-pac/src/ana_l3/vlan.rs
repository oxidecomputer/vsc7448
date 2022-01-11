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
/// Bundle dual leaky bucket policer index
///
/// Specifies BUM policer index.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct BUM_CFG(u32);
impl BUM_CFG {
    /// Broadcast, Unknown and Multicast traffic leaky bucket index This index can be overruled by index from ISDX table if ANA_L2:ISDX:MISC_CFG.BUM_SLB_ENA is set. Related parameters: ANA_AC_POL:BUM_SLB
    pub fn bum_slb_idx(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_bum_slb_idx(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// TUPE Control
///
/// Control value for Table UPdate Engine (TUPE). See ANA_L3:TUPE.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TUPE_CTRL(u32);
impl TUPE_CTRL {
    /// Control value for Table UPdate Engine (TUPE). Note that unused bits in VLAN_PORT_MASK may also be used to control which VLAN table entries TUPE shall update. See ANA_L3:TUPE.
    pub fn tupe_ctrl(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_tupe_ctrl(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// VLAN Configuration
///
/// Various configuration of VLAN handles
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_CFG(u32);
impl VLAN_CFG {
    /// FID to be used for learning and forwarding.
    pub fn vlan_fid(&self) -> u32 {
        (self.0 & 0x1fff00) >> 8
    }
    pub fn set_vlan_fid(&mut self, value: u32) {
        assert!(value <= 0x1fff);
        let value = value << 8;
        self.0 &= !0x1fff00;
        self.0 |= value;
    }
    /// Disable flooding of frames with unknown DMAC on a per VLAN basis. Note that when VLAN_FLOOD_DIS=1, then frames with broadcast or multicast DMAC are only forwarded if installed in MAC table.
    pub fn vlan_flood_dis(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_vlan_flood_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Enable VLAN ingress filtering per VLAN. If a enabled, frames received on a port, which is not a member of the classified VLAN, are discarded. VLAN ingress filtering can also be enabled per ingress port. VLAN ingress filtering is performed if either enabled for ingress port or for VLAN. Related parameters: ANA_L3:COMMON:VLAN_FILTER_CTRL ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.VLAN_IGR_FILTER_STICKY
    pub fn vlan_igr_filter_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_vlan_igr_filter_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Disable learning of SMAC of frames received on this VLAN. Related parameters: ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.VLAN_LRN_DENY_STICKY
    pub fn vlan_lrn_dis(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_vlan_lrn_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// VLAN mirror enable flag. If this field is set, frames classified to this ingress VLAN are mirrored.
    pub fn vlan_mirror_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vlan_mirror_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Pointer to STP instance associated with VLAN. The value must not exceed the size of the MSTP table, ref. MSTP.
    pub fn vlan_mstp_ptr(&self) -> u32 {
        (self.0 & 0x7f000000) >> 24
    }
    pub fn set_vlan_mstp_ptr(&mut self, value: u32) {
        assert!(value <= 0x7f);
        let value = value << 24;
        self.0 &= !0x7f000000;
        self.0 |= value;
    }
    /// Enable / disable this VLAN as a Private VLAN (PVLAN). Ports within a PVLAN are categorized into three different port types: Promiscuous ports: A promiscuous port can communicate with all ports in the PVLAN, including the isolated and community ports. Isolated ports: An isolated port has complete Layer 2 separation from the other ports within the same PVLAN, but not from the promiscuous ports. PVLANs block all traffic to isolated ports except traffic from promiscuous ports. Traffic from isolated port is forwarded only to promiscuous ports. Community ports: Community ports communicate among themselves and with the PVLAN's promiscuous ports. Community ports cannot communicate with isolated ports. Related parameters: ANA_L3:COMMON:VLAN_ISOLATED_CFG ANA_L3:COMMON:VLAN_COMMUNITY_CFG
    pub fn vlan_private_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_vlan_private_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enable router leg in VLAN. If enabled, the ID of the router leg is configured in VMID_CFG.VMID.
    pub fn vlan_rleg_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_vlan_rleg_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable secure forwarding on a per VLAN basis. When secure forwarding is enabled, only frames with known SMAC are forwarded.
    ///
    /// 0: Forwarding is allowed regardless of SMAC being known or unknown. 1: Forwarding is only allowed for frames with known SMAC.
    pub fn vlan_sec_fwd_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_vlan_sec_fwd_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
}
/// VLAN Port Mask Configuration
///
/// Configuration of VLAN port mask.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_MASK_CFG(u32);
impl VLAN_MASK_CFG {
    /// Specify mask of ports belonging to VLAN. Note: Initialization value for addresses 0,1 and 4095 is '1...1' Initialization value for all other addresses is 0 Related parameters: ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.VLAN_LOOKUP_INVLD_STICKY
    ///
    /// 0: Port does not belong to the VLAN 1: Port belongs to the VLAN
    pub fn vlan_port_mask(&self) -> u32 {
        self.0
    }
    pub fn set_vlan_port_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// VLAN Port Mask Configuration
///
/// Configuration of VLAN port mask.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_MASK_CFG1(u32);
impl VLAN_MASK_CFG1 {
    /// Refer to VLAN_MASK_CFG.VLAN_PORT_MASK description.
    pub fn vlan_port_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_vlan_port_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Router Leg Identification / Mapped VLAN ID
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VMID_CFG(u32);
impl VMID_CFG {
    /// Routing: VMID, identifying VLAN's router leg. Security check: "Mapped VLAN ID".
    pub fn vmid(&self) -> u32 {
        self.0 & 0x7f
    }
    pub fn set_vmid(&mut self, value: u32) {
        assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }
}
