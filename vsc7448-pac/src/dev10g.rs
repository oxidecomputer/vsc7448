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

pub mod dev_cfg_status;
pub mod dev_statistics_32bit;
pub mod dev_statistics_40bit;
pub mod mac_cfg_status;
pub mod pcs2x6g_configuration;
pub mod pcs2x6g_ext_configuration;
pub mod pcs2x6g_status;
pub mod pcs_xaui_configuration;
pub mod pcs_xaui_status;
pub mod pcs_xaui_tstpat_configuration;
pub mod pcs_xaui_tstpat_status;

/// Device 10G Configuration and Status Registers
pub struct DEV_CFG_STATUS(pub(super) u32);
impl DEV_CFG_STATUS {
    pub fn DEV_LB_CFG(&self) -> RegisterAddress<dev_cfg_status::DEV_LB_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn DEV_MISC_CFG(&self) -> RegisterAddress<dev_cfg_status::DEV_MISC_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn DEV_PORT_PROTECT(&self) -> RegisterAddress<dev_cfg_status::DEV_PORT_PROTECT> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn DEV_RST_CTRL(&self) -> RegisterAddress<dev_cfg_status::DEV_RST_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn DEV_RX_STATUS(&self) -> RegisterAddress<dev_cfg_status::DEV_RX_STATUS> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn DEV_STICKY(&self) -> RegisterAddress<dev_cfg_status::DEV_STICKY> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn EEE_CFG(&self) -> RegisterAddress<dev_cfg_status::EEE_CFG> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn INTR(&self) -> RegisterAddress<dev_cfg_status::INTR> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn INTR_ENA(&self) -> RegisterAddress<dev_cfg_status::INTR_ENA> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn INTR_IDENT(&self) -> RegisterAddress<dev_cfg_status::INTR_IDENT> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn PFC_PAUSE_MODE_CTRL(&self) -> RegisterAddress<dev_cfg_status::PFC_PAUSE_MODE_CTRL> {
        RegisterAddress::new(self.0 + 0x28)
    }
    pub fn PTP_CFG(&self) -> RegisterAddress<dev_cfg_status::PTP_CFG> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    pub fn PTP_CFG_BTDLY(&self) -> RegisterAddress<dev_cfg_status::PTP_CFG_BTDLY> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn PTP_EVENTS(&self) -> RegisterAddress<dev_cfg_status::PTP_EVENTS> {
        RegisterAddress::new(self.0 + 0x34)
    }
}

/// Device Statistics Registers
pub struct DEV_STATISTICS_32BIT(pub(super) u32);
impl DEV_STATISTICS_32BIT {
    pub fn RX_ALIGNMENT_LOST_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_ALIGNMENT_LOST_CNT> {
        RegisterAddress::new(self.0 + 0x80)
    }
    pub fn RX_BC_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_BC_CNT> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn RX_CRC_ERR_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_CRC_ERR_CNT> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn RX_FRAGMENTS_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_FRAGMENTS_CNT> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn RX_HIH_CKSM_ERR_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_HIH_CKSM_ERR_CNT> {
        RegisterAddress::new(self.0 + 0x94)
    }
    pub fn RX_IN_RANGE_LEN_ERR_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_IN_RANGE_LEN_ERR_CNT> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn RX_IPG_SHRINK_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_IPG_SHRINK_CNT> {
        RegisterAddress::new(self.0 + 0x50)
    }
    pub fn RX_JABBERS_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_JABBERS_CNT> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn RX_MC_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_MC_CNT> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn RX_OUT_OF_RANGE_LEN_ERR_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_OUT_OF_RANGE_LEN_ERR_CNT> {
        RegisterAddress::new(self.0 + 0x28)
    }
    pub fn RX_OVERSIZE_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_OVERSIZE_CNT> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    pub fn RX_PAUSE_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_PAUSE_CNT> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn RX_SIZE1024TO1518_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_SIZE1024TO1518_CNT> {
        RegisterAddress::new(self.0 + 0x48)
    }
    pub fn RX_SIZE128TO255_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_SIZE128TO255_CNT> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    pub fn RX_SIZE1519TOMAX_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_SIZE1519TOMAX_CNT> {
        RegisterAddress::new(self.0 + 0x4c)
    }
    pub fn RX_SIZE256TO511_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_SIZE256TO511_CNT> {
        RegisterAddress::new(self.0 + 0x40)
    }
    pub fn RX_SIZE512TO1023_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_SIZE512TO1023_CNT> {
        RegisterAddress::new(self.0 + 0x44)
    }
    pub fn RX_SIZE64_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_SIZE64_CNT> {
        RegisterAddress::new(self.0 + 0x34)
    }
    pub fn RX_SIZE65TO127_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_SIZE65TO127_CNT> {
        RegisterAddress::new(self.0 + 0x38)
    }
    pub fn RX_SYMBOL_ERR_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_SYMBOL_ERR_CNT> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn RX_TAGGED_FRMS_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_TAGGED_FRMS_CNT> {
        RegisterAddress::new(self.0 + 0x84)
    }
    pub fn RX_UC_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_UC_CNT> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn RX_UNDERSIZE_CNT(&self) -> RegisterAddress<dev_statistics_32bit::RX_UNDERSIZE_CNT> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn RX_UNSUP_OPCODE_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_UNSUP_OPCODE_CNT> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn RX_UNTAGGED_FRMS_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_UNTAGGED_FRMS_CNT> {
        RegisterAddress::new(self.0 + 0x88)
    }
    pub fn RX_XGMII_PROT_ERR_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::RX_XGMII_PROT_ERR_CNT> {
        RegisterAddress::new(self.0 + 0x98)
    }
    pub fn TX_BC_CNT(&self) -> RegisterAddress<dev_statistics_32bit::TX_BC_CNT> {
        RegisterAddress::new(self.0 + 0x60)
    }
    pub fn TX_MC_CNT(&self) -> RegisterAddress<dev_statistics_32bit::TX_MC_CNT> {
        RegisterAddress::new(self.0 + 0x5c)
    }
    pub fn TX_PAUSE_CNT(&self) -> RegisterAddress<dev_statistics_32bit::TX_PAUSE_CNT> {
        RegisterAddress::new(self.0 + 0x54)
    }
    pub fn TX_SIZE1024TO1518_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::TX_SIZE1024TO1518_CNT> {
        RegisterAddress::new(self.0 + 0x78)
    }
    pub fn TX_SIZE128TO255_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::TX_SIZE128TO255_CNT> {
        RegisterAddress::new(self.0 + 0x6c)
    }
    pub fn TX_SIZE1519TOMAX_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::TX_SIZE1519TOMAX_CNT> {
        RegisterAddress::new(self.0 + 0x7c)
    }
    pub fn TX_SIZE256TO511_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::TX_SIZE256TO511_CNT> {
        RegisterAddress::new(self.0 + 0x70)
    }
    pub fn TX_SIZE512TO1023_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::TX_SIZE512TO1023_CNT> {
        RegisterAddress::new(self.0 + 0x74)
    }
    pub fn TX_SIZE64_CNT(&self) -> RegisterAddress<dev_statistics_32bit::TX_SIZE64_CNT> {
        RegisterAddress::new(self.0 + 0x64)
    }
    pub fn TX_SIZE65TO127_CNT(&self) -> RegisterAddress<dev_statistics_32bit::TX_SIZE65TO127_CNT> {
        RegisterAddress::new(self.0 + 0x68)
    }
    pub fn TX_TAGGED_FRMS_CNT(&self) -> RegisterAddress<dev_statistics_32bit::TX_TAGGED_FRMS_CNT> {
        RegisterAddress::new(self.0 + 0x8c)
    }
    pub fn TX_UC_CNT(&self) -> RegisterAddress<dev_statistics_32bit::TX_UC_CNT> {
        RegisterAddress::new(self.0 + 0x58)
    }
    pub fn TX_UNTAGGED_FRMS_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_32bit::TX_UNTAGGED_FRMS_CNT> {
        RegisterAddress::new(self.0 + 0x90)
    }
}

/// Device Statistics Registers
pub struct DEV_STATISTICS_40BIT(pub(super) u32);
impl DEV_STATISTICS_40BIT {
    pub fn RX_BAD_BYTES_CNT(&self) -> RegisterAddress<dev_statistics_40bit::RX_BAD_BYTES_CNT> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn RX_BAD_BYTES_MSB_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_40bit::RX_BAD_BYTES_MSB_CNT> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn RX_IN_BYTES_CNT(&self) -> RegisterAddress<dev_statistics_40bit::RX_IN_BYTES_CNT> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn RX_IN_BYTES_MSB_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_40bit::RX_IN_BYTES_MSB_CNT> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn RX_OK_BYTES_CNT(&self) -> RegisterAddress<dev_statistics_40bit::RX_OK_BYTES_CNT> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn RX_OK_BYTES_MSB_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_40bit::RX_OK_BYTES_MSB_CNT> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn TX_OK_BYTES_CNT(&self) -> RegisterAddress<dev_statistics_40bit::TX_OK_BYTES_CNT> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn TX_OK_BYTES_MSB_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_40bit::TX_OK_BYTES_MSB_CNT> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn TX_OUT_BYTES_CNT(&self) -> RegisterAddress<dev_statistics_40bit::TX_OUT_BYTES_CNT> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn TX_OUT_BYTES_MSB_CNT(
        &self,
    ) -> RegisterAddress<dev_statistics_40bit::TX_OUT_BYTES_MSB_CNT> {
        RegisterAddress::new(self.0 + 0x1c)
    }
}

/// MAC10G Configuration and Status Registers
pub struct MAC_CFG_STATUS(pub(super) u32);
impl MAC_CFG_STATUS {
    pub fn MAC_ADV_CHK_CFG(&self) -> RegisterAddress<mac_cfg_status::MAC_ADV_CHK_CFG> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn MAC_ENA_CFG(&self) -> RegisterAddress<mac_cfg_status::MAC_ENA_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn MAC_LB_CFG(&self) -> RegisterAddress<mac_cfg_status::MAC_LB_CFG> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn MAC_LFS_CFG(&self) -> RegisterAddress<mac_cfg_status::MAC_LFS_CFG> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn MAC_MAXLEN_CFG(&self) -> RegisterAddress<mac_cfg_status::MAC_MAXLEN_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn MAC_MODE_CFG(&self) -> RegisterAddress<mac_cfg_status::MAC_MODE_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn MAC_NUM_TAGS_CFG(&self) -> RegisterAddress<mac_cfg_status::MAC_NUM_TAGS_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn MAC_RX_LANE_STICKY_0(&self) -> RegisterAddress<mac_cfg_status::MAC_RX_LANE_STICKY_0> {
        RegisterAddress::new(self.0 + 0x28)
    }
    pub fn MAC_RX_LANE_STICKY_1(&self) -> RegisterAddress<mac_cfg_status::MAC_RX_LANE_STICKY_1> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    pub fn MAC_STICKY(&self) -> RegisterAddress<mac_cfg_status::MAC_STICKY> {
        RegisterAddress::new(self.0 + 0x34)
    }
    pub fn MAC_TAGS_CFG(&self, index: u32) -> RegisterAddress<mac_cfg_status::MAC_TAGS_CFG> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x10 + index * 0x4)
    }
    pub fn MAC_TX_MONITOR_STICKY(&self) -> RegisterAddress<mac_cfg_status::MAC_TX_MONITOR_STICKY> {
        RegisterAddress::new(self.0 + 0x30)
    }
}

/// PCS2X6G Configuration Registers
pub struct PCS2X6G_CONFIGURATION(pub(super) u32);
impl PCS2X6G_CONFIGURATION {
    pub fn PCS2X6G_CFG(&self) -> RegisterAddress<pcs2x6g_configuration::PCS2X6G_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// PCS2x6G Extended Configuration Registers
pub struct PCS2X6G_EXT_CONFIGURATION(pub(super) u32);
impl PCS2X6G_EXT_CONFIGURATION {
    pub fn PCS2X6G_EXT_CFG(&self) -> RegisterAddress<pcs2x6g_ext_configuration::PCS2X6G_EXT_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// PCS2X6G Status Registers
pub struct PCS2X6G_STATUS(pub(super) u32);
impl PCS2X6G_STATUS {
    pub fn PCS2X6G_ERR_CNT_STAT(&self) -> RegisterAddress<pcs2x6g_status::PCS2X6G_ERR_CNT_STAT> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn PCS2X6G_ERR_STATUS(&self) -> RegisterAddress<pcs2x6g_status::PCS2X6G_ERR_STATUS> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PCS2X6G_STATUS(&self) -> RegisterAddress<pcs2x6g_status::PCS2X6G_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// PCS XAUI Configuration Registers
pub struct PCS_XAUI_CONFIGURATION(pub(super) u32);
impl PCS_XAUI_CONFIGURATION {
    pub fn PCS_XAUI_CFG(&self) -> RegisterAddress<pcs_xaui_configuration::PCS_XAUI_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn PCS_XAUI_EXT_CFG(&self) -> RegisterAddress<pcs_xaui_configuration::PCS_XAUI_EXT_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PCS_XAUI_INTERLEAVE_MODE_CFG(
        &self,
    ) -> RegisterAddress<pcs_xaui_configuration::PCS_XAUI_INTERLEAVE_MODE_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn PCS_XAUI_LPI_CFG(&self) -> RegisterAddress<pcs_xaui_configuration::PCS_XAUI_LPI_CFG> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn PCS_XAUI_RX_ERR_CNT_CFG(
        &self,
    ) -> RegisterAddress<pcs_xaui_configuration::PCS_XAUI_RX_ERR_CNT_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn PCS_XAUI_SD_CFG(&self) -> RegisterAddress<pcs_xaui_configuration::PCS_XAUI_SD_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn PCS_XAUI_TX_SEQ_CFG(
        &self,
    ) -> RegisterAddress<pcs_xaui_configuration::PCS_XAUI_TX_SEQ_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
}

/// PCS XAUI Status Registers
pub struct PCS_XAUI_STATUS(pub(super) u32);
impl PCS_XAUI_STATUS {
    pub fn PCS_XAUI_CGALIGN_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_CGALIGN_STATUS> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn PCS_XAUI_DESKEW_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_DESKEW_STATUS> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PCS_XAUI_LPI_STATUS(&self) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_LPI_STATUS> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn PCS_XAUI_RX_ERROR_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_RX_ERROR_STATUS> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn PCS_XAUI_RX_FIFO_CG_ERR_L3_CNT_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_RX_FIFO_CG_ERR_L3_CNT_STATUS> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn PCS_XAUI_RX_FIFO_D_ERR_L2_CNT_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_RX_FIFO_D_ERR_L2_CNT_STATUS> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn PCS_XAUI_RX_FIFO_OF_ERR_L0_CNT_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_RX_FIFO_OF_ERR_L0_CNT_STATUS> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn PCS_XAUI_RX_FIFO_UF_ERR_L1_CNT_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_RX_FIFO_UF_ERR_L1_CNT_STATUS> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn PCS_XAUI_RX_SEQ_REC_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_RX_SEQ_REC_STATUS> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn PCS_XAUI_RX_STATUS(&self) -> RegisterAddress<pcs_xaui_status::PCS_XAUI_RX_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// PCS XAUI Testpattern Configuration Registers
pub struct PCS_XAUI_TSTPAT_CONFIGURATION(pub(super) u32);
impl PCS_XAUI_TSTPAT_CONFIGURATION {
    pub fn PCS_XAUI_TSTPAT_CFG(
        &self,
    ) -> RegisterAddress<pcs_xaui_tstpat_configuration::PCS_XAUI_TSTPAT_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// PCS XAUI Testpattern Status Registers
pub struct PCS_XAUI_TSTPAT_STATUS(pub(super) u32);
impl PCS_XAUI_TSTPAT_STATUS {
    pub fn PCS_XAUI_TSTPAT_RX_SEQ_CNT_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_tstpat_status::PCS_XAUI_TSTPAT_RX_SEQ_CNT_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn PCS_XAUI_TSTPAT_TX_SEQ_CNT_STATUS(
        &self,
    ) -> RegisterAddress<pcs_xaui_tstpat_status::PCS_XAUI_TSTPAT_TX_SEQ_CNT_STATUS> {
        RegisterAddress::new(self.0 + 0x4)
    }
}
