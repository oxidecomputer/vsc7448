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
/// Ingress Protection Configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct IPT(u32);
impl IPT {
    /// ISDX protection configuration.
    ///
    /// 0: No protection Value applicable for forwarding from UNI to a protected NNI: 1: Set PROT_ACTIVE=1 to REW if the ISDX's protection group uses the protection entity. Values applicable for forwarding from a protected NNI to UNI: 2: Working entity. Discard received frame if the ISDX's protection group uses the protection entity, i.e. ANA_CL:PPT:PP_CFG.STATE=1 3: Protection entity: Discard received frame if the ISDX's protection group uses the working entity, i.e. ANA_CL:PPT:PP_CFG.STATE=0
    #[inline(always)]
    pub fn ipt_cfg(&self) -> u32 {
        (self.0 & 0x1800) >> 11
    }
    #[inline(always)]
    pub fn set_ipt_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 11;
        self.0 &= !0x1800;
        self.0 |= value;
    }
    /// Pointer to protection state in ANA_CL:PPT for the protection group, which this ISDX is a member of.
    #[inline(always)]
    pub fn ppt_idx(&self) -> u32 {
        self.0 & 0x1ff
    }
    #[inline(always)]
    pub fn set_ppt_idx(&mut self, value: u32) {
        assert!(value <= 0x1ff);
        self.0 &= !0x1ff;
        self.0 |= value;
    }
    /// Configures the protection pipeline point where ISDX protection operates at.
    ///
    /// 0: ANA_IPT_PROT 1: ANA_OU_PROT 2: ANA_MID_PROT 3: ANA_IN_PROT
    #[inline(always)]
    pub fn prot_pipeline_pt(&self) -> u32 {
        (self.0 & 0x600) >> 9
    }
    #[inline(always)]
    pub fn set_prot_pipeline_pt(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 9;
        self.0 &= !0x600;
        self.0 |= value;
    }
}
/// ISDX configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct ISDX_CFG(u32);
impl ISDX_CFG {
    /// The index selects which profile in the Layer-2 control protocol table to use (ANA_CL:L2CP_TBL).
    ///
    /// 0: Disable use of L2CP_IDX. Default port-based index used instead. >0: L2CP_IDX selects the L2CP profile to use.
    #[inline(always)]
    pub fn l2cp_idx(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_l2cp_idx(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// MIP table index. The index is used when enabled by VCAP_CLM action MIP_SEL and selects which MIP table entry to use (ANA_CL:MIP_TBL).
    #[inline(always)]
    pub fn mip_idx(&self) -> u32 {
        (self.0 & 0xffc0) >> 6
    }
    #[inline(always)]
    pub fn set_mip_idx(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 6;
        self.0 &= !0xffc0;
        self.0 |= value;
    }
}
/// OAM MEP configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct OAM_MEP_CFG(u32);
impl OAM_MEP_CFG {
    /// Force selected VOE to handle all frames as data.
    #[inline(always)]
    pub fn independent_mel_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_independent_mel_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// VOE index.
    #[inline(always)]
    pub fn mep_idx(&self) -> u32 {
        (self.0 & 0xffc) >> 2
    }
    #[inline(always)]
    pub fn set_mep_idx(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 2;
        self.0 &= !0xffc;
        self.0 |= value;
    }
    /// Enable use of MEP_IDX for selecting which VOE to use for EVC OAM processing.
    #[inline(always)]
    pub fn mep_idx_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_mep_idx_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Virtual Switching Instance configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VSI_CFG(u32);
impl VSI_CFG {
    /// Configures if learning and forwarding is based on VLAN or Virtual Switching Instance. If VSI_ENA=1, then the following entry is used for lookup in ANA_L3:VLAN: ANA_L3:VLAN[vsi_val + 4096] i.e. the upper 1K entries in ANA_L3:VLAN are "VSI entries".
    ///
    /// 0: Use classified VID for lookup in VLAN table 1: Use VSI for lookup in VLAN table
    #[inline(always)]
    pub fn vsi_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_vsi_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Virtual Switching Instance used if VSI_ENA is set.
    #[inline(always)]
    pub fn vsi_val(&self) -> u32 {
        (self.0 & 0x7fe) >> 1
    }
    #[inline(always)]
    pub fn set_vsi_val(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 1;
        self.0 &= !0x7fe;
        self.0 |= value;
    }
}
