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
/// Ingress service dual leaky bucket policer, base address
///
/// Specifies service DLB policer index. If ANA_L2::FWD_CFG.QUEUE_DEFAULT_SDLB_ENA is set some of the indeces are used for queue default DLB (see ANA_L2::PORT_DLB_CFG.QUEUE_DLB_IDX). The ingress service DLB policer is selected as DLB_IDX + DLB_COS_OFFSET[frame's classified  COS ID].
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DLB_CFG(u32);
impl DLB_CFG {
    /// Dual leaky bucket base address.
    #[inline(always)]
    pub fn dlb_idx(&self) -> u32 {
        self.0 & 0x1fff
    }
    #[inline(always)]
    pub fn set_dlb_idx(&mut self, value: u32) {
        assert!(value <= 0x1fff);
        self.0 &= !0x1fff;
        self.0 |= value;
    }
}
/// Ingress service dual leaky bucket policer, offset per COS ID
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DLB_COS_CFG(u32);
impl DLB_COS_CFG {
    /// Dual leaky bucket offset per COS ID.
    #[inline(always)]
    pub fn dlb_cos_offset(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_dlb_cos_offset(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Ingress service counter set, base address
///
/// The ingress service counter set is selected as ISDX_BASE_ADDR + ISDX_COS_OFFSET[frame's classified  COS ID].
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct ISDX_BASE_CFG(u32);
impl ISDX_BASE_CFG {
    /// Ingress service counter set base address.
    #[inline(always)]
    pub fn isdx_base_addr(&self) -> u32 {
        self.0 & 0x1fff
    }
    #[inline(always)]
    pub fn set_isdx_base_addr(&mut self, value: u32) {
        assert!(value <= 0x1fff);
        self.0 &= !0x1fff;
        self.0 |= value;
    }
}
/// Ingress service counter set, offset per COS ID
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct ISDX_COS_CFG(u32);
impl ISDX_COS_CFG {
    /// Ingress service counter set offset per COS ID.
    #[inline(always)]
    pub fn isdx_cos_offset(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_isdx_cos_offset(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Controls various indexes.
///
/// Controls BDLB and BUM indexes and SDLB policer pipeline point.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MISC_CFG(u32);
impl MISC_CFG {
    /// Bundle Policer Dual leaky bucket index. If ANA_L2::FWD_CFG.PORT_DEFAULT_BDLB_ENA is set some of the indeces are used for port default DLB (see ANA_L2::PORT_DLB_CFG.PORT_DLB_IDX). Related parameters: ANA_AC_POL:BDLB
    #[inline(always)]
    pub fn bdlb_idx(&self) -> u32 {
        self.0 & 0x3ff
    }
    #[inline(always)]
    pub fn set_bdlb_idx(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
    /// Controls if BUM index from VLAN table is overruled.
    #[inline(always)]
    pub fn bum_slb_ena(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_bum_slb_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// BUM policer index overrules index from VLAN table if BUM_SLB_ENA is set. Related parameters: ANA_L3:VLAN:BUM_CFG ANA_AC_POL:BUM_SLB
    #[inline(always)]
    pub fn bum_slb_idx(&self) -> u32 {
        (self.0 & 0xffc00) >> 10
    }
    #[inline(always)]
    pub fn set_bum_slb_idx(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 10;
        self.0 &= !0xffc00;
        self.0 |= value;
    }
    /// Configures the pipeline point for stat and SDLB policing. When injecting or looping at a pipeline point after PIPELINE_PT will not cause SDLB policing and ISDX counter updates. When extracting at a pipeline point before PIPELINE_PT will not cause SDLB policing and ISDX counter updates.
    ///
    /// 0: NONE 1: ANA_VRAP 2: ANA_PORT_VOE 3: ANA_CL 4: ANA_CLM 5: ANA_IPT_PROT 6: ANA_OU_MIP 7: ANA_OU_SW 8: ANA_OU_PROT 9: ANA_OU_VOE 10: ANA_MID_PROT 11: ANA_IN_VOE 12: ANA_IN_PROT 13: ANA_IN_SW 14: ANA_IN_MIP 15: ANA_VLAN
    #[inline(always)]
    pub fn pipeline_pt(&self) -> u32 {
        (self.0 & 0x1e00000) >> 21
    }
    #[inline(always)]
    pub fn set_pipeline_pt(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 21;
        self.0 &= !0x1e00000;
        self.0 |= value;
    }
}
/// Ingress service port mask configuration
///
/// Configures port mask to be used in relation to service
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PORT_MASK_CFG(u32);
impl PORT_MASK_CFG {
    /// Configures ingress service portmask. Can be used as replacement for VLAN PORTMASK or as an further filtering of VLAN PORTMASK depending on ANA_L2:ISDX:SERVICE_CTRL.PORT_MASK_REPLACE_ENA.
    ///
    /// 'XX...XXX': Where X is '0' or '1', representing a port mask.
    #[inline(always)]
    pub fn port_mask(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_port_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Ingress service port mask configuration
///
/// Configures port mask to be used in relation to service
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PORT_MASK_CFG1(u32);
impl PORT_MASK_CFG1 {
    /// Refer to PORT_MASK_CFG.PORT_MASK description.
    ///
    /// 'XX...XXX': Where X is '0' or '1', representing a port mask.
    #[inline(always)]
    pub fn port_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    #[inline(always)]
    pub fn set_port_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Controls QGRP handling
///
/// Controls QGRP index and QSYS OAM drop counting.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct QGRP_CFG(u32);
impl QGRP_CFG {
    /// Configures QSYS group
    #[inline(always)]
    pub fn qgrp_idx(&self) -> u32 {
        (self.0 & 0xffc) >> 2
    }
    #[inline(always)]
    pub fn set_qgrp_idx(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 2;
        self.0 &= !0xffc;
        self.0 |= value;
    }
    /// Configures OAM type for traffic detected as OAM by Classification to be used by QSYS to determining if dropped frames should be counted or not. Ref: QFWD:QMAP_QOS_TBL:DROP_STAT_CTRL.DROP_STAT_OAM_CNT_SEL
    ///
    /// 0: Not OAM - Frames dropped by QSYS	are always counted in QSYS drop stat. 1: EVC OAM - Frames dropped by QSYS which are classified as OAM will selectively be counted  in QSYS drop stat as EVC OAM. 2: OVC / PW OAM - Frames dropped by QSYS which are classified as OAM will selectively be counted	in QSYS drop stat as OVC / PW OAM. 3: DOWN MEP OAM - Frames dropped by QSYS which are classified as OAM will never be counted  in QSYS drop stat.
    #[inline(always)]
    pub fn qgrp_oam_type(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_qgrp_oam_type(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
/// Ingress service forwarding configuration
///
/// Configures service based forwarding.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SERVICE_CTRL(u32);
impl SERVICE_CTRL {
    /// Overrule aggregation code from ANA_CL.
    ///
    /// 0: Use aggregation code calculated in classifier. See AGGR_VAL for mode to disable aggregation. 1: Use SERVICE_CTRL.AGGR_VAL as aggregation code.
    #[inline(always)]
    pub fn aggr_replace_ena(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    #[inline(always)]
    pub fn set_aggr_replace_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Aggregation code value.
    ///
    /// If AGGR_REPLACE_ENA is set then the aggregation code is replaced with this value. If AGGR_REPLACE_ENA is cleared and AGGR_VAL != 0 then no aggregation is applied.
    #[inline(always)]
    pub fn aggr_val(&self) -> u32 {
        (self.0 & 0x1e00000) >> 21
    }
    #[inline(always)]
    pub fn set_aggr_val(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 21;
        self.0 &= !0x1e00000;
        self.0 |= value;
    }
    /// Enable forwarding based on CDA
    #[inline(always)]
    pub fn cda_fwd_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline(always)]
    pub fn set_cda_fwd_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Select ES0 key type.
    ///
    /// 0: Use VID key in ES0. 1: Use ISDX key in ES0.
    #[inline(always)]
    pub fn es0_isdx_key_ena(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_es0_isdx_key_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Configured address when ISDX_BASED_FWD_ENA or ISDX_BASED_SRC_ENA is set. The encoding of this field is specified by FWD_TYPE
    ///
    /// FWD_TYPE= UPSID_PN: FWD_ADDR(9:5) = UPSID FWD_ADDR(4:0) = UPSPN Specifies static unicast forwarding to lport FWD_TYPE = MC_IDX: Specifies static multicast forwarding to the ports indexed by MC_IDX into ANA_AC:PGID
    #[inline(always)]
    pub fn fwd_addr(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_fwd_addr(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    /// Address type when ISDX_BASED_FWD_ENA or ISDX_BASED_SRC_ENA is set.
    ///
    /// 0: UPSID_PN 3: MC_IDX 7: NO_ADDR other: Reserved
    #[inline(always)]
    pub fn fwd_type(&self) -> u32 {
        (self.0 & 0x7000) >> 12
    }
    #[inline(always)]
    pub fn set_fwd_type(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 12;
        self.0 &= !0x7000;
        self.0 |= value;
    }
    /// Controls service based forwarding. Note: Setting this bit disables use of ISDX_BASED_SRC_ENA.
    ///
    /// 0: DMAC based forwarding. 1: SERVICE_CTRL.CDA_FWD_ENA=1: DMAC based forwarding SERVICE_CTRL.CDA_FWD_ENA=0: (forwarding not influenced by DMAC lookup): SERVICE_CTRL.FWD_TYPE = 0 (UPSID_PN): Forward to port in SERVICE_CTRL.FWD_ADDR SERVICE_CTRL.FWD_TYPE = 3 (MC_IDX): Forward by means of MC_IDX specified in SERVICE_CTRL.FWD_ADDR SERVICE_CTRL.FWD_TYPE = 7 (NO_ADDR): Flood forward.
    #[inline(always)]
    pub fn isdx_based_fwd_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_isdx_based_fwd_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Enable service based learning. When set traffic associated with a ISDX are seen as received on the interface configured for the ISDX instead of the port interface configured in ANA_CL:PORT:PORT_ID_CFG. Note: This cannot be use together with ISDX_BASED_FWD_ENA.
    ///
    /// 0: Traffic is learned as received on logical port (configured in ANA_CL:PORT:PORT_ID_CFG). 1: Traffic is learned as received on the interface configured in ANA_L2:ISDX:SERVICE_CTRL.FWD_TYPE and ANA_L2:ISDX:SERVICE_CTRL.FWD_ADDR).
    #[inline(always)]
    pub fn isdx_based_src_ena(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    #[inline(always)]
    pub fn set_isdx_based_src_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Configures replacement of VLAN PORTMASK with ANA_L2:ISDX:PORT_MASK_CFG
    #[inline(always)]
    pub fn port_mask_replace_ena(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    #[inline(always)]
    pub fn set_port_mask_replace_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 25;
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    /// Avoids applying source mask
    ///
    /// 0: Source port mask is applied. 1: Source mask is ignored.
    #[inline(always)]
    pub fn src_mask_dis(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_src_mask_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
}
