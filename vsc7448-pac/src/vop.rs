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

pub mod ana_cosid_map_conf;
pub mod common;
pub mod coremem;
pub mod port_cosid_map_conf;
pub mod ram_ctrl;
pub mod rew_cosid_map_conf;
pub mod sam_cosid_seq_cnt;
pub mod voe_ccm_lm;
pub mod voe_conf;
pub mod voe_conf_reg;
pub mod voe_context_ana;
pub mod voe_context_rew;
pub mod voe_crc_err;
pub mod voe_stat;

/// COSID / Color config in ANA - Service VOEs
pub struct ANA_COSID_MAP_CONF(pub(super) u32);
impl ANA_COSID_MAP_CONF {
    pub fn COSID_MAP_TABLE_ANA(&self) -> RegisterAddress<ana_cosid_map_conf::COSID_MAP_TABLE_ANA> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn LBR_CRC_ERR_CNT(&self) -> RegisterAddress<ana_cosid_map_conf::LBR_CRC_ERR_CNT> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Configuration Registers for VOP.
pub struct COMMON(pub(super) u32);
impl COMMON {
    pub fn COMMON_MEP_MC_MAC_LSB(&self) -> RegisterAddress<common::COMMON_MEP_MC_MAC_LSB> {
        RegisterAddress::new(self.0 + 0x128)
    }
    pub fn CPU_EXTR_CFG(&self) -> RegisterAddress<common::CPU_EXTR_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn CPU_EXTR_CFG_1(&self) -> RegisterAddress<common::CPU_EXTR_CFG_1> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn CPU_EXTR_MPLS(&self) -> RegisterAddress<common::CPU_EXTR_MPLS> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn HMO_FORCE_SLOT_CFG(&self, index: u32) -> RegisterAddress<common::HMO_FORCE_SLOT_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x84 + index * 0x4)
    }
    pub fn HMO_PERIOD_CFG(&self, index: u32) -> RegisterAddress<common::HMO_PERIOD_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x7c + index * 0x4)
    }
    pub fn HMO_TIMER_CFG(&self) -> RegisterAddress<common::HMO_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0x8c)
    }
    pub fn INTR(&self, index: u32) -> RegisterAddress<common::INTR> {
        assert!(index < 34);
        RegisterAddress::new(self.0 + 0xa0 + index * 0x4)
    }
    pub fn LOC_CTRL(&self) -> RegisterAddress<common::LOC_CTRL> {
        RegisterAddress::new(self.0 + 0x5c)
    }
    pub fn LOC_PERIOD_CFG(&self, index: u32) -> RegisterAddress<common::LOC_PERIOD_CFG> {
        assert!(index < 7);
        RegisterAddress::new(self.0 + 0x60 + index * 0x4)
    }
    pub fn LOC_SCAN_STICKY(&self) -> RegisterAddress<common::LOC_SCAN_STICKY> {
        RegisterAddress::new(self.0 + 0x90)
    }
    pub fn MASTER_INTR_CTRL(&self) -> RegisterAddress<common::MASTER_INTR_CTRL> {
        RegisterAddress::new(self.0 + 0x94)
    }
    pub fn MPLS_GENERIC_CODEPOINT(&self, index: u32) -> RegisterAddress<common::MPLS_GENERIC_CODEPOINT> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x3c + index * 0x4)
    }
    pub fn OAM_GENERIC_CFG(&self, index: u32) -> RegisterAddress<common::OAM_GENERIC_CFG> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x1c + index * 0x4)
    }
    pub fn VERSION_CTRL(&self) -> RegisterAddress<common::VERSION_CTRL> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn VERSION_CTRL_2(&self) -> RegisterAddress<common::VERSION_CTRL_2> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn VERSION_CTRL_MPLS(&self) -> RegisterAddress<common::VERSION_CTRL_MPLS> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn VOE32_INTR(&self, index: u32) -> RegisterAddress<common::VOE32_INTR> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x98 + index * 0x4)
    }
    pub fn VOP_CTRL(&self) -> RegisterAddress<common::VOP_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Access core memory
pub struct COREMEM(pub(super) u32);
impl COREMEM {
    pub fn CM_ADDR(&self) -> RegisterAddress<coremem::CM_ADDR> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn CM_DATA(&self) -> RegisterAddress<coremem::CM_DATA> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn RAM_INIT(&self) -> RegisterAddress<coremem::RAM_INIT> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// COSID / Color config - Port VOEs
pub struct PORT_COSID_MAP_CONF(pub(super) u32);
impl PORT_COSID_MAP_CONF {
    pub fn COSID_MAP_CFG_REW(&self) -> RegisterAddress<port_cosid_map_conf::COSID_MAP_CFG_REW> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PORT_RX_COSID_MAP(&self) -> RegisterAddress<port_cosid_map_conf::PORT_RX_COSID_MAP> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn PORT_RX_COSID_MAP1(&self) -> RegisterAddress<port_cosid_map_conf::PORT_RX_COSID_MAP1> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PORT_TX_COSID_MAP(&self) -> RegisterAddress<port_cosid_map_conf::PORT_TX_COSID_MAP> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// Access core memory
pub struct RAM_CTRL(pub(super) u32);
impl RAM_CTRL {
    pub fn SAM_LBR_RX_TRANSID_ERR_CNT(&self, index: u32) -> RegisterAddress<ram_ctrl::SAM_LBR_RX_TRANSID_ERR_CNT> {
        assert!(index < 7);
        RegisterAddress::new(self.0 + 0x70 + index * 0x4)
    }
}

/// COSID / Color config in REW - Service VOEs
pub struct REW_COSID_MAP_CONF(pub(super) u32);
impl REW_COSID_MAP_CONF {
    pub fn COSID_MAP_CFG_ANA(&self) -> RegisterAddress<rew_cosid_map_conf::COSID_MAP_CFG_ANA> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn COSID_MAP_TABLE_REW(&self) -> RegisterAddress<rew_cosid_map_conf::COSID_MAP_TABLE_REW> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// SAM per COSID sequence counters
pub struct SAM_COSID_SEQ_CNT(pub(super) u32);
impl SAM_COSID_SEQ_CNT {
    pub fn PORT_TX_COSID_MAP1(&self) -> RegisterAddress<sam_cosid_seq_cnt::PORT_TX_COSID_MAP1> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn SAM_LBM_TX_TRANSID(&self, index: u32) -> RegisterAddress<sam_cosid_seq_cnt::SAM_LBM_TX_TRANSID> {
        assert!(index < 7);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
    pub fn SAM_LBR_RX_FRM_CNT(&self, index: u32) -> RegisterAddress<sam_cosid_seq_cnt::SAM_LBR_RX_FRM_CNT> {
        assert!(index < 7);
        RegisterAddress::new(self.0 + 0x38 + index * 0x4)
    }
    pub fn SAM_LBR_RX_TRANSID(&self, index: u32) -> RegisterAddress<sam_cosid_seq_cnt::SAM_LBR_RX_TRANSID> {
        assert!(index < 7);
        RegisterAddress::new(self.0 + 0x54 + index * 0x4)
    }
    pub fn SAM_LBR_TX_FRM_CNT(&self, index: u32) -> RegisterAddress<sam_cosid_seq_cnt::SAM_LBR_TX_FRM_CNT> {
        assert!(index < 7);
        RegisterAddress::new(self.0 + 0x1c + index * 0x4)
    }
}

/// VOE CCM-LM samples
pub struct VOE_CCM_LM(pub(super) u32);
impl VOE_CCM_LM {
    pub fn CCM_TX_FCB_CFG(&self) -> RegisterAddress<voe_ccm_lm::CCM_TX_FCB_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SLM_TX_FRM_CNT(&self) -> RegisterAddress<voe_ccm_lm::SLM_TX_FRM_CNT> {
        RegisterAddress::new(self.0 + 0x74)
    }
}

/// Configuration per Vitesse OAM MEP Endpoints (VOE) for Y.1731 OAM
pub struct VOE_CONF(pub(super) u32);
impl VOE_CONF {
    pub fn CCM_CFG(&self) -> RegisterAddress<voe_conf::CCM_CFG> {
        RegisterAddress::new(self.0 + 0x40)
    }
    pub fn CCM_MEGID_CFG(&self, index: u32) -> RegisterAddress<voe_conf::CCM_MEGID_CFG> {
        assert!(index < 12);
        RegisterAddress::new(self.0 + 0x44 + index * 0x4)
    }
    pub fn G_8113_1_CFG(&self) -> RegisterAddress<voe_conf::G_8113_1_CFG> {
        RegisterAddress::new(self.0 + 0x9c)
    }
    pub fn G_8113_1_REMOTE_MIPID(&self) -> RegisterAddress<voe_conf::G_8113_1_REMOTE_MIPID> {
        RegisterAddress::new(self.0 + 0xa0)
    }
    pub fn G_8113_1_REMOTE_MIPID1(&self) -> RegisterAddress<voe_conf::G_8113_1_REMOTE_MIPID1> {
        RegisterAddress::new(self.0 + 0xa4)
    }
    pub fn G_8113_1_REMOTE_MIPID2(&self) -> RegisterAddress<voe_conf::G_8113_1_REMOTE_MIPID2> {
        RegisterAddress::new(self.0 + 0xa8)
    }
    pub fn LOOPBACK_CFG(&self) -> RegisterAddress<voe_conf::LOOPBACK_CFG> {
        RegisterAddress::new(self.0 + 0x38)
    }
    pub fn LOOPBACK_ENA(&self) -> RegisterAddress<voe_conf::LOOPBACK_ENA> {
        RegisterAddress::new(self.0 + 0x34)
    }
    pub fn MEP_UC_MAC_LSB(&self) -> RegisterAddress<voe_conf::MEP_UC_MAC_LSB> {
        RegisterAddress::new(self.0 + 0x28)
    }
    pub fn MEP_UC_MAC_MSB(&self) -> RegisterAddress<voe_conf::MEP_UC_MAC_MSB> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    pub fn OAM_CNT_DATA_CTRL(&self) -> RegisterAddress<voe_conf::OAM_CNT_DATA_CTRL> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn OAM_CNT_OAM_CTRL(&self) -> RegisterAddress<voe_conf::OAM_CNT_OAM_CTRL> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn OAM_CPU_COPY_CTRL(&self) -> RegisterAddress<voe_conf::OAM_CPU_COPY_CTRL> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn OAM_HW_CTRL(&self) -> RegisterAddress<voe_conf::OAM_HW_CTRL> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn PATH_VOE_CFG(&self) -> RegisterAddress<voe_conf::PATH_VOE_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn PDU_VOE_PASS(&self) -> RegisterAddress<voe_conf::PDU_VOE_PASS> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn PEER_MEPID_CFG(&self) -> RegisterAddress<voe_conf::PEER_MEPID_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn SAM_COSID_SEQ_CFG(&self) -> RegisterAddress<voe_conf::SAM_COSID_SEQ_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn SAM_NON_OAM_SEQ_CFG(&self) -> RegisterAddress<voe_conf::SAM_NON_OAM_SEQ_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn SLM_CONFIG(&self) -> RegisterAddress<voe_conf::SLM_CONFIG> {
        RegisterAddress::new(self.0 + 0x74)
    }
    pub fn SLM_PEER_LIST(&self, index: u32) -> RegisterAddress<voe_conf::SLM_PEER_LIST> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x7c + index * 0x4)
    }
    pub fn SLM_TEST_ID(&self) -> RegisterAddress<voe_conf::SLM_TEST_ID> {
        RegisterAddress::new(self.0 + 0x78)
    }
    pub fn TX_TRANSID_UPDATE(&self) -> RegisterAddress<voe_conf::TX_TRANSID_UPDATE> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    pub fn VOE_CTRL(&self) -> RegisterAddress<voe_conf::VOE_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn VOE_MEPID_CFG(&self) -> RegisterAddress<voe_conf::VOE_MEPID_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn VOE_MISC_CONFIG(&self) -> RegisterAddress<voe_conf::VOE_MISC_CONFIG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// VOE configuration registers
pub struct VOE_CONF_REG(pub(super) u32);
impl VOE_CONF_REG {
    pub fn COMMON_MEP_MC_MAC_MSB(&self) -> RegisterAddress<voe_conf_reg::COMMON_MEP_MC_MAC_MSB> {
        RegisterAddress::new(self.0 + 0x12c)
    }
}

/// [MCC_DEBUG] Contains the context for the VOE if in REW.
pub struct VOE_CONTEXT_ANA(pub(super) u32);
impl VOE_CONTEXT_ANA {
    pub fn CT_CCM_TLV_INFO_ANA(&self) -> RegisterAddress<voe_context_ana::CT_CCM_TLV_INFO_ANA> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn CT_OAM_DATA1_REW(&self) -> RegisterAddress<voe_context_ana::CT_OAM_DATA1_REW> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn CT_OAM_DATA_ANA(&self) -> RegisterAddress<voe_context_ana::CT_OAM_DATA_ANA> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn CT_OAM_INFO_ANA(&self) -> RegisterAddress<voe_context_ana::CT_OAM_INFO_ANA> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn CT_OAM_STICKY_ANA(&self) -> RegisterAddress<voe_context_ana::CT_OAM_STICKY_ANA> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// [MCC_DEBUG] Contains the context for the VOE if in REW.
pub struct VOE_CONTEXT_REW(pub(super) u32);
impl VOE_CONTEXT_REW {
    pub fn CCM_RX_FCB_CFG(&self) -> RegisterAddress<voe_context_rew::CCM_RX_FCB_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn CT_CCM_TLV_INFO_REW(&self) -> RegisterAddress<voe_context_rew::CT_CCM_TLV_INFO_REW> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn CT_OAM_DATA_REW(&self) -> RegisterAddress<voe_context_rew::CT_OAM_DATA_REW> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn CT_OAM_INFO_REW(&self) -> RegisterAddress<voe_context_rew::CT_OAM_INFO_REW> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn CT_OAM_STICKY_REW(&self) -> RegisterAddress<voe_context_rew::CT_OAM_STICKY_REW> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// Count the number of CRC errors in Rx LBR / TST TLVs
pub struct VOE_CRC_ERR(pub(super) u32);
impl VOE_CRC_ERR {
    pub fn CT_OAM_DATA1_ANA(&self) -> RegisterAddress<voe_crc_err::CT_OAM_DATA1_ANA> {
        RegisterAddress::new(self.0 + 0x10)
    }
}

/// Per VOE statistics and counters (Y.1731 OAM)
pub struct VOE_STAT(pub(super) u32);
impl VOE_STAT {
    pub fn AUTO_HIT_ME_ONCE(&self) -> RegisterAddress<voe_stat::AUTO_HIT_ME_ONCE> {
        RegisterAddress::new(self.0 + 0x50)
    }
    pub fn CCM_ERR(&self) -> RegisterAddress<voe_stat::CCM_ERR> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn CCM_RX_ERR_1(&self) -> RegisterAddress<voe_stat::CCM_RX_ERR_1> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn CCM_RX_FRM_CNT(&self) -> RegisterAddress<voe_stat::CCM_RX_FRM_CNT> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn CCM_RX_LAST(&self) -> RegisterAddress<voe_stat::CCM_RX_LAST> {
        RegisterAddress::new(self.0 + 0x68)
    }
    pub fn CCM_RX_SEQ_CFG(&self) -> RegisterAddress<voe_stat::CCM_RX_SEQ_CFG> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn CCM_RX_WARNING(&self) -> RegisterAddress<voe_stat::CCM_RX_WARNING> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn CCM_STAT(&self) -> RegisterAddress<voe_stat::CCM_STAT> {
        RegisterAddress::new(self.0 + 0x64)
    }
    pub fn CCM_TX_SEQ_CFG(&self) -> RegisterAddress<voe_stat::CCM_TX_SEQ_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn DM_PDU_CNT(&self) -> RegisterAddress<voe_stat::DM_PDU_CNT> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    pub fn G_8113_1_REMOTE_MIPID3(&self) -> RegisterAddress<voe_stat::G_8113_1_REMOTE_MIPID3> {
        RegisterAddress::new(self.0 + 0xac)
    }
    pub fn INTR_ENA(&self) -> RegisterAddress<voe_stat::INTR_ENA> {
        RegisterAddress::new(self.0 + 0x70)
    }
    pub fn INTR_STICKY(&self) -> RegisterAddress<voe_stat::INTR_STICKY> {
        RegisterAddress::new(self.0 + 0x6c)
    }
    pub fn LBM_TX_TRANSID_CFG(&self) -> RegisterAddress<voe_stat::LBM_TX_TRANSID_CFG> {
        RegisterAddress::new(self.0 + 0x28)
    }
    pub fn LBR_RX_FRM_CNT(&self) -> RegisterAddress<voe_stat::LBR_RX_FRM_CNT> {
        RegisterAddress::new(self.0 + 0x34)
    }
    pub fn LBR_RX_TRANSID_CFG(&self) -> RegisterAddress<voe_stat::LBR_RX_TRANSID_CFG> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn LBR_RX_TRANSID_ERR_CNT(&self) -> RegisterAddress<voe_stat::LBR_RX_TRANSID_ERR_CNT> {
        RegisterAddress::new(self.0 + 0x38)
    }
    pub fn LBR_TX_FRM_CNT(&self) -> RegisterAddress<voe_stat::LBR_TX_FRM_CNT> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    pub fn LM_PDU_CNT(&self) -> RegisterAddress<voe_stat::LM_PDU_CNT> {
        RegisterAddress::new(self.0 + 0x40)
    }
    pub fn OAM_RX_STICKY(&self) -> RegisterAddress<voe_stat::OAM_RX_STICKY> {
        RegisterAddress::new(self.0 + 0x5c)
    }
    pub fn OAM_RX_STICKY2(&self) -> RegisterAddress<voe_stat::OAM_RX_STICKY2> {
        RegisterAddress::new(self.0 + 0x60)
    }
    pub fn OAM_TX_STICKY(&self) -> RegisterAddress<voe_stat::OAM_TX_STICKY> {
        RegisterAddress::new(self.0 + 0x58)
    }
    pub fn PDU_EXTRACT(&self) -> RegisterAddress<voe_stat::PDU_EXTRACT> {
        RegisterAddress::new(self.0 + 0x4c)
    }
    pub fn RX_OAM_DISCARD(&self) -> RegisterAddress<voe_stat::RX_OAM_DISCARD> {
        RegisterAddress::new(self.0 + 0x48)
    }
    pub fn RX_OAM_FRM_CNT(&self) -> RegisterAddress<voe_stat::RX_OAM_FRM_CNT> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn RX_SEL_OAM_CNT(&self) -> RegisterAddress<voe_stat::RX_SEL_OAM_CNT> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SYNLM_EXTRACT(&self) -> RegisterAddress<voe_stat::SYNLM_EXTRACT> {
        RegisterAddress::new(self.0 + 0x54)
    }
    pub fn TX_OAM_DISCARD(&self) -> RegisterAddress<voe_stat::TX_OAM_DISCARD> {
        RegisterAddress::new(self.0 + 0x44)
    }
    pub fn TX_OAM_FRM_CNT(&self) -> RegisterAddress<voe_stat::TX_OAM_FRM_CNT> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn TX_SEL_OAM_CNT(&self) -> RegisterAddress<voe_stat::TX_SEL_OAM_CNT> {
        RegisterAddress::new(self.0 + 0x8)
    }
}