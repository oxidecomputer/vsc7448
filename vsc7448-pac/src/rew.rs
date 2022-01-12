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

use crate::types::RegisterAddress;

// Register groups are stored in the tree as submodules

pub mod common;
pub mod coremem;
pub mod encap;
pub mod isdx_tbl;
pub mod map_res_a;
pub mod map_res_b;
pub mod mip_tbl;
pub mod oam_pdu_mod_cont;
pub mod pdu_mod_cfg;
pub mod port;
pub mod ptp_ctrl;
pub mod ptp_seq_no;
pub mod ram_ctrl;
pub mod vmid;
pub mod voe_port_lm_cnt;
pub mod voe_srv_lm_cnt;

/// Common configuration and status for all rewriter ports
pub struct COMMON(pub(super) u32);
impl COMMON {
    #[inline(always)]
    pub fn CNT_CTRL(&self) -> RegisterAddress<common::CNT_CTRL> {
        RegisterAddress::new(self.0 + 0x21c)
    }
    #[inline(always)]
    pub fn COMMON_CTRL(&self) -> RegisterAddress<common::COMMON_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn DP_MAP(&self) -> RegisterAddress<common::DP_MAP> {
        RegisterAddress::new(self.0 + 0x110)
    }
    #[inline(always)]
    pub fn DSCP_REMAP(&self, index: u32) -> RegisterAddress<common::DSCP_REMAP> {
        assert!(index < 64);
        RegisterAddress::new(self.0 + 0x114 + index * 0x4)
    }
    #[inline(always)]
    pub fn ES0_CTRL(&self) -> RegisterAddress<common::ES0_CTRL> {
        RegisterAddress::new(self.0 + 0xe4)
    }
    #[inline(always)]
    pub fn GCPU_CFG(&self, index: u32) -> RegisterAddress<common::GCPU_CFG> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x22c + index * 0x4)
    }
    #[inline(always)]
    pub fn GCPU_TAG_CFG(&self, index: u32) -> RegisterAddress<common::GCPU_TAG_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x254 + index * 0x4)
    }
    #[inline(always)]
    pub fn HIH_DEF_CFG(&self) -> RegisterAddress<common::HIH_DEF_CFG> {
        RegisterAddress::new(self.0 + 0x26c)
    }
    #[inline(always)]
    pub fn HIH_DEV10G_CFG(&self, index: u32) -> RegisterAddress<common::HIH_DEV10G_CFG> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x25c + index * 0x4)
    }
    #[inline(always)]
    pub fn MIP_CTRL(&self) -> RegisterAddress<common::MIP_CTRL> {
        RegisterAddress::new(self.0 + 0xe8)
    }
    #[inline(always)]
    pub fn MIP_STICKY_EVENT(&self) -> RegisterAddress<common::MIP_STICKY_EVENT> {
        RegisterAddress::new(self.0 + 0x270)
    }
    #[inline(always)]
    pub fn MIRROR_PROBE_CFG(&self, index: u32) -> RegisterAddress<common::MIRROR_PROBE_CFG> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0xec + index * 0x4)
    }
    #[inline(always)]
    pub fn MIRROR_TAG_A_CFG(&self, index: u32) -> RegisterAddress<common::MIRROR_TAG_A_CFG> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0xf8 + index * 0x4)
    }
    #[inline(always)]
    pub fn MIRROR_TAG_B_CFG(&self, index: u32) -> RegisterAddress<common::MIRROR_TAG_B_CFG> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x104 + index * 0x4)
    }
    #[inline(always)]
    pub fn PORT_CTRL(&self, index: u32) -> RegisterAddress<common::PORT_CTRL> {
        assert!(index < 53);
        RegisterAddress::new(self.0 + 0x4 + index * 0x4)
    }
    #[inline(always)]
    pub fn RLEG_CFG_0(&self) -> RegisterAddress<common::RLEG_CFG_0> {
        RegisterAddress::new(self.0 + 0x214)
    }
    #[inline(always)]
    pub fn RLEG_CFG_1(&self) -> RegisterAddress<common::RLEG_CFG_1> {
        RegisterAddress::new(self.0 + 0x218)
    }
    #[inline(always)]
    pub fn STICKY_EVENT(&self) -> RegisterAddress<common::STICKY_EVENT> {
        RegisterAddress::new(self.0 + 0x228)
    }
    #[inline(always)]
    pub fn STICKY_EVENT_CNT_MASK_CFG(&self) -> RegisterAddress<common::STICKY_EVENT_CNT_MASK_CFG> {
        RegisterAddress::new(self.0 + 0x224)
    }
    #[inline(always)]
    pub fn STICKY_EVENT_COUNT(&self) -> RegisterAddress<common::STICKY_EVENT_COUNT> {
        RegisterAddress::new(self.0 + 0x220)
    }
    #[inline(always)]
    pub fn TPID_CFG(&self, index: u32) -> RegisterAddress<common::TPID_CFG> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0xd8 + index * 0x4)
    }
    #[inline(always)]
    pub fn VSTAX_PORT_GRP_CFG(&self, index: u32) -> RegisterAddress<common::VSTAX_PORT_GRP_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x24c + index * 0x4)
    }
}

/// Access core memory
pub struct COREMEM(pub(super) u32);
impl COREMEM {
    #[inline(always)]
    pub fn CM_ADDR(&self) -> RegisterAddress<coremem::CM_ADDR> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn CM_DATA(&self) -> RegisterAddress<coremem::CM_DATA> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// Encapsulation RAM configuration
pub struct ENCAP(pub(super) u32);
impl ENCAP {
    #[inline(always)]
    pub fn CW_VAL(&self) -> RegisterAddress<encap::CW_VAL> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    #[inline(always)]
    pub fn LABEL_VAL(&self, index: u32) -> RegisterAddress<encap::LABEL_VAL> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x20 + index * 0x4)
    }
    #[inline(always)]
    pub fn LL_DMAC_LSB(&self) -> RegisterAddress<encap::LL_DMAC_LSB> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn LL_DMAC_MSB(&self) -> RegisterAddress<encap::LL_DMAC_MSB> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn LL_ETYPE(&self) -> RegisterAddress<encap::LL_ETYPE> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn LL_SMAC_LSB(&self) -> RegisterAddress<encap::LL_SMAC_LSB> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn LL_SMAC_MSB(&self) -> RegisterAddress<encap::LL_SMAC_MSB> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn LL_TAG_CFG(&self) -> RegisterAddress<encap::LL_TAG_CFG> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    #[inline(always)]
    pub fn LL_TAG_REMARK_CFG(&self, index: u32) -> RegisterAddress<encap::LL_TAG_REMARK_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x48 + index * 0x4)
    }
    #[inline(always)]
    pub fn LL_TAG_VAL(&self, index: u32) -> RegisterAddress<encap::LL_TAG_VAL> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x40 + index * 0x4)
    }
    #[inline(always)]
    pub fn MPLS_LABEL_CFG(&self) -> RegisterAddress<encap::MPLS_LABEL_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn MPLS_REMARK_CFG(&self, index: u32) -> RegisterAddress<encap::MPLS_REMARK_CFG> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x30 + index * 0x4)
    }
    #[inline(always)]
    pub fn RSV_LABEL_CFG(&self) -> RegisterAddress<encap::RSV_LABEL_CFG> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn RSV_LABEL_VAL(&self) -> RegisterAddress<encap::RSV_LABEL_VAL> {
        RegisterAddress::new(self.0 + 0x2c)
    }
}

/// ISDX configuration table
pub struct ISDX_TBL(pub(super) u32);
impl ISDX_TBL {
    #[inline(always)]
    pub fn COS_CTRL(&self, index: u32) -> RegisterAddress<isdx_tbl::COS_CTRL> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
}

/// Mapping resource A
pub struct MAP_RES_A(pub(super) u32);
impl MAP_RES_A {
    #[inline(always)]
    pub fn MAP_LBL_A(&self) -> RegisterAddress<map_res_a::MAP_LBL_A> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn MAP_VAL_A(&self) -> RegisterAddress<map_res_a::MAP_VAL_A> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Mapping resource B
pub struct MAP_RES_B(pub(super) u32);
impl MAP_RES_B {
    #[inline(always)]
    pub fn MAP_LBL_B(&self) -> RegisterAddress<map_res_b::MAP_LBL_B> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn MAP_VAL_B(&self) -> RegisterAddress<map_res_b::MAP_VAL_B> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// MIP table
pub struct MIP_TBL(pub(super) u32);
impl MIP_TBL {
    #[inline(always)]
    pub fn CCM_HMO_CTRL(&self) -> RegisterAddress<mip_tbl::CCM_HMO_CTRL> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn LBM_MAC_HIGH(&self) -> RegisterAddress<mip_tbl::LBM_MAC_HIGH> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn LBM_MAC_LOW(&self) -> RegisterAddress<mip_tbl::LBM_MAC_LOW> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn MIP_CFG(&self) -> RegisterAddress<mip_tbl::MIP_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn MIP_VID_CTRL(&self) -> RegisterAddress<mip_tbl::MIP_VID_CTRL> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// VOE additional information
pub struct OAM_PDU_MOD_CONT(pub(super) u32);
impl OAM_PDU_MOD_CONT {
    #[inline(always)]
    pub fn CCM_LM_INFO_REG(&self) -> RegisterAddress<oam_pdu_mod_cont::CCM_LM_INFO_REG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn CCM_LM_RX_B_REG(&self) -> RegisterAddress<oam_pdu_mod_cont::CCM_LM_RX_B_REG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn CCM_LM_TX_B_REG(&self) -> RegisterAddress<oam_pdu_mod_cont::CCM_LM_TX_B_REG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn LM_CNT_FRAME(&self) -> RegisterAddress<oam_pdu_mod_cont::LM_CNT_FRAME> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn TEMP_CNT_REG(&self) -> RegisterAddress<oam_pdu_mod_cont::TEMP_CNT_REG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// OAM_PDU_MOD misc. configuration
pub struct PDU_MOD_CFG(pub(super) u32);
impl PDU_MOD_CFG {
    #[inline(always)]
    pub fn DM_PTP_DOMAIN_CFG(&self, index: u32) -> RegisterAddress<pdu_mod_cfg::DM_PTP_DOMAIN_CFG> {
        assert!(index < 53);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
    #[inline(always)]
    pub fn RD_LAST_PORT_BYTE_CNT_LSB(
        &self,
    ) -> RegisterAddress<pdu_mod_cfg::RD_LAST_PORT_BYTE_CNT_LSB> {
        RegisterAddress::new(self.0 + 0xe0)
    }
    #[inline(always)]
    pub fn RD_LAST_PORT_BYTE_CNT_MSB(
        &self,
    ) -> RegisterAddress<pdu_mod_cfg::RD_LAST_PORT_BYTE_CNT_MSB> {
        RegisterAddress::new(self.0 + 0xdc)
    }
    #[inline(always)]
    pub fn RD_LAST_PORT_FRM_CNT_LSB(
        &self,
    ) -> RegisterAddress<pdu_mod_cfg::RD_LAST_PORT_FRM_CNT_LSB> {
        RegisterAddress::new(self.0 + 0xd8)
    }
    #[inline(always)]
    pub fn RD_LAST_PORT_LM_CNT_LSB(&self) -> RegisterAddress<pdu_mod_cfg::RD_LAST_PORT_LM_CNT_LSB> {
        RegisterAddress::new(self.0 + 0xd4)
    }
}

/// Port based configuration and status for rewriter
pub struct PORT(pub(super) u32);
impl PORT {
    #[inline(always)]
    pub fn DEI_MAP_DE0(&self, index: u32) -> RegisterAddress<port::DEI_MAP_DE0> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x44 + index * 0x4)
    }
    #[inline(always)]
    pub fn DEI_MAP_DE1(&self, index: u32) -> RegisterAddress<port::DEI_MAP_DE1> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x64 + index * 0x4)
    }
    #[inline(always)]
    pub fn DSCP_MAP(&self) -> RegisterAddress<port::DSCP_MAP> {
        RegisterAddress::new(self.0 + 0x88)
    }
    #[inline(always)]
    pub fn HIH_CTRL(&self) -> RegisterAddress<port::HIH_CTRL> {
        RegisterAddress::new(self.0 + 0xac)
    }
    #[inline(always)]
    pub fn PCP_MAP_DE0(&self, index: u32) -> RegisterAddress<port::PCP_MAP_DE0> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x4 + index * 0x4)
    }
    #[inline(always)]
    pub fn PCP_MAP_DE1(&self, index: u32) -> RegisterAddress<port::PCP_MAP_DE1> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x24 + index * 0x4)
    }
    #[inline(always)]
    pub fn PORT_VLAN_CFG(&self) -> RegisterAddress<port::PORT_VLAN_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn PTP_EDLY_CFG(&self) -> RegisterAddress<port::PTP_EDLY_CFG> {
        RegisterAddress::new(self.0 + 0x98)
    }
    #[inline(always)]
    pub fn PTP_IDLY1_CFG(&self) -> RegisterAddress<port::PTP_IDLY1_CFG> {
        RegisterAddress::new(self.0 + 0x9c)
    }
    #[inline(always)]
    pub fn PTP_IDLY2_CFG(&self) -> RegisterAddress<port::PTP_IDLY2_CFG> {
        RegisterAddress::new(self.0 + 0xa0)
    }
    #[inline(always)]
    pub fn PTP_MISC_CFG(&self) -> RegisterAddress<port::PTP_MISC_CFG> {
        RegisterAddress::new(self.0 + 0x94)
    }
    #[inline(always)]
    pub fn PTP_MODE_CFG(&self, index: u32) -> RegisterAddress<port::PTP_MODE_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x8c + index * 0x4)
    }
    #[inline(always)]
    pub fn PTP_SMAC_HIGH(&self) -> RegisterAddress<port::PTP_SMAC_HIGH> {
        RegisterAddress::new(self.0 + 0xa8)
    }
    #[inline(always)]
    pub fn PTP_SMAC_LOW(&self) -> RegisterAddress<port::PTP_SMAC_LOW> {
        RegisterAddress::new(self.0 + 0xa4)
    }
    #[inline(always)]
    pub fn TAG_CTRL(&self) -> RegisterAddress<port::TAG_CTRL> {
        RegisterAddress::new(self.0 + 0x84)
    }
}

/// PTP Control
pub struct PTP_CTRL(pub(super) u32);
impl PTP_CTRL {
    #[inline(always)]
    pub fn PTP_CPUVD_MODE_CFG(&self, index: u32) -> RegisterAddress<ptp_ctrl::PTP_CPUVD_MODE_CFG> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x8 + index * 0x4)
    }
    #[inline(always)]
    pub fn PTP_RSRV_NOT_ZERO(&self) -> RegisterAddress<ptp_ctrl::PTP_RSRV_NOT_ZERO> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn PTP_RSRV_NOT_ZERO_1(&self) -> RegisterAddress<ptp_ctrl::PTP_RSRV_NOT_ZERO_1> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    #[inline(always)]
    pub fn PTP_TWOSTEP_CTRL(&self) -> RegisterAddress<ptp_ctrl::PTP_TWOSTEP_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn PTP_TWOSTEP_STAMP(&self) -> RegisterAddress<ptp_ctrl::PTP_TWOSTEP_STAMP> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// Sequence numbers for PTP frames
pub struct PTP_SEQ_NO(pub(super) u32);
impl PTP_SEQ_NO {
    #[inline(always)]
    pub fn PTP_SEQ_NO(&self, index: u32) -> RegisterAddress<ptp_seq_no::PTP_SEQ_NO> {
        assert!(index < 256);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
}

/// Access core memory
pub struct RAM_CTRL(pub(super) u32);
impl RAM_CTRL {
    #[inline(always)]
    pub fn RAM_INIT(&self) -> RegisterAddress<ram_ctrl::RAM_INIT> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Egress Mapped VLAN (EVMID) configuration
pub struct VMID(pub(super) u32);
impl VMID {
    #[inline(always)]
    pub fn RLEG_CTRL(&self) -> RegisterAddress<vmid::RLEG_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// OAM LM port counters pr. priority
pub struct VOE_PORT_LM_CNT(pub(super) u32);
impl VOE_PORT_LM_CNT {
    #[inline(always)]
    pub fn PORT_BYTE_CNT_LSB(&self) -> RegisterAddress<voe_port_lm_cnt::PORT_BYTE_CNT_LSB> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn PORT_BYTE_CNT_MSB(&self) -> RegisterAddress<voe_port_lm_cnt::PORT_BYTE_CNT_MSB> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn PORT_FRM_CNT_LSB(&self) -> RegisterAddress<voe_port_lm_cnt::PORT_FRM_CNT_LSB> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn PORT_LM_CNT_LSB(&self) -> RegisterAddress<voe_port_lm_cnt::PORT_LM_CNT_LSB> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// OAM Service LM counters pr. priority
pub struct VOE_SRV_LM_CNT(pub(super) u32);
impl VOE_SRV_LM_CNT {
    #[inline(always)]
    pub fn SRV_LM_CNT_LSB(&self) -> RegisterAddress<voe_srv_lm_cnt::SRV_LM_CNT_LSB> {
        RegisterAddress::new(self.0 + 0x0)
    }
}
