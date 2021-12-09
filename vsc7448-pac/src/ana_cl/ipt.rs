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
/// ISDX configuration
#[derive(From, Into)]
pub struct ISDX_CFG(u32);
impl ISDX_CFG {
    /// The index selects which profile in the Layer-2 control protocol table to use (ANA_CL:L2CP_TBL).

    ///

    /// 0: Disable use of L2CP_IDX. Default port-based index used instead. >0: L2CP_IDX selects the L2CP profile to use.
    pub fn l2cp_idx(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_l2cp_idx(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// MIP table index. The index is used when enabled by VCAP_CLM action MIP_SEL and selects which MIP table entry to use (ANA_CL:MIP_TBL).
    pub fn mip_idx(&self) -> u32 {
        (self.0 & 0xffc0) >> 6
    }
    pub fn set_mip_idx(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0xffc0);
        self.0 &= !0xffc0;
        self.0 |= value;
    }
}
/// Entry in mapping table
#[derive(From, Into)]
pub struct MAP_ENTRY(u32);
impl MAP_ENTRY {
    /// COS ID. The classified COS ID is set to COSID_VAL if SET_CTRL.COSID_ENA is set.
    pub fn cosid_val(&self) -> u32 {
        (self.0 & 0xe000) >> 13
    }
    pub fn set_cosid_val(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0xe000);
        self.0 &= !0xe000;
        self.0 |= value;
    }
    /// DEI value. The classified DEI is set to DEI_VAL if SET_CTRL.DEI_ENA is set.
    pub fn dei_val(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_dei_val(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Drop precedence level. The classified DP level is set to DP_VAL if SET_CTRL.DP_ENA is set.
    pub fn dp_val(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    pub fn set_dp_val(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x30000);
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// DSCP value. The classified DSCP is set to DSCP_VAL if SET_CTRL.DSCP_ENA is set.
    pub fn dscp_val(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_dscp_val(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// Disable forwarding for frames hitting this entry.
    pub fn fwd_dis(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    pub fn set_fwd_dis(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x2000000);
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    /// Path color value used by OAM MEP.
    pub fn path_color_val(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_path_color_val(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    /// Path COS ID used by OAM MEP.
    pub fn path_cosid_val(&self) -> u32 {
        (self.0 & 0xe00000) >> 21
    }
    pub fn set_path_cosid_val(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0xe00000);
        self.0 &= !0xe00000;
        self.0 |= value;
    }
    /// PCP value. The classified PCP is set to PCP_VAL if SET_CTRL.PCP_ENA is set.
    pub fn pcp_val(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    pub fn set_pcp_val(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x1c0);
        self.0 &= !0x1c0;
        self.0 |= value;
    }
    /// QoS class. The classified QoS class is set to QOS_VAL if SET_CTRL.QOS_ENA is set.
    pub fn qos_val(&self) -> u32 {
        (self.0 & 0x1c00) >> 10
    }
    pub fn set_qos_val(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x1c00);
        self.0 &= !0x1c00;
        self.0 |= value;
    }
    /// TC bits. The classified TC bits are set to TC_VAL if SET_CTRL.TC_ENA is set.
    pub fn tc_val(&self) -> u32 {
        (self.0 & 0x1c0000) >> 18
    }
    pub fn set_tc_val(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x1c0000);
        self.0 &= !0x1c0000;
        self.0 |= value;
    }
}
/// OAM MEP configuration
#[derive(From, Into)]
pub struct OAM_MEP_CFG(u32);
impl OAM_MEP_CFG {
    /// Force selected VOE to handle all frames as data.
    pub fn independent_mel_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_independent_mel_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// VOE index.
    pub fn mep_idx(&self) -> u32 {
        (self.0 & 0xffc) >> 2
    }
    pub fn set_mep_idx(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xffc);
        self.0 &= !0xffc;
        self.0 |= value;
    }
    /// Enable use of MEP_IDX for selecting which VOE to use for EVC OAM processing.
    pub fn mep_idx_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_mep_idx_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Virtual Switching Instance configuration
#[derive(From, Into)]
pub struct VSI_CFG(u32);
impl VSI_CFG {
    /// Configures if learning and forwarding is based on VLAN or Virtual Switching Instance. If VSI_ENA=1, then the following entry is used for lookup in ANA_L3:VLAN: ANA_L3:VLAN[vsi_val + 4096] i.e. the upper 1K entries in ANA_L3:VLAN are "VSI entries".

    ///

    /// 0: Use classified VID for lookup in VLAN table 1: Use VSI for lookup in VLAN table
    pub fn vsi_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vsi_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Virtual Switching Instance used if VSI_ENA is set.
    pub fn vsi_val(&self) -> u32 {
        (self.0 & 0x7fe) >> 1
    }
    pub fn set_vsi_val(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x7fe);
        self.0 &= !0x7fe;
        self.0 |= value;
    }
}
