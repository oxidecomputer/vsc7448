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

pub mod cpu_system_ctrl;
pub mod fdma;
pub mod intr;
pub mod manual_xtrinj;
pub mod memctrl;
pub mod mpu8051;
pub mod pcie;
pub mod pi_mst;
pub mod spi_mst;
pub mod timers;
pub mod twi_delay;
pub mod twi_spike_filter;

/// Configurations for the CPU system.
pub struct CPU_SYSTEM_CTRL(pub(super) u32);
impl CPU_SYSTEM_CTRL {
    #[inline(always)]
    pub fn GENERAL_CTRL(&self) -> RegisterAddress<cpu_system_ctrl::GENERAL_CTRL> {
        RegisterAddress::new(self.0 + 0x24)
    }
    #[inline(always)]
    pub fn GENERAL_STAT(&self) -> RegisterAddress<cpu_system_ctrl::GENERAL_STAT> {
        RegisterAddress::new(self.0 + 0x28)
    }
    #[inline(always)]
    pub fn GPR(&self, index: u8) -> RegisterAddress<cpu_system_ctrl::GPR> {
        debug_assert!(index < 8);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn RESET(&self) -> RegisterAddress<cpu_system_ctrl::RESET> {
        RegisterAddress::new(self.0 + 0x20)
    }
}

/// Frame DMA
pub struct FDMA(pub(super) u32);
impl FDMA {
    #[inline(always)]
    pub fn FDMA_CH_ACTIVATE(&self) -> RegisterAddress<fdma::FDMA_CH_ACTIVATE> {
        RegisterAddress::new(self.0 + 0xd0)
    }
    #[inline(always)]
    pub fn FDMA_CH_CFG(&self, index: u8) -> RegisterAddress<fdma::FDMA_CH_CFG> {
        debug_assert!(index < 10);
        RegisterAddress::new(self.0 + 0x18c + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_CH_CNT(&self, index: u8) -> RegisterAddress<fdma::FDMA_CH_CNT> {
        debug_assert!(index < 10);
        RegisterAddress::new(self.0 + 0xdc + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_CH_DISABLE(&self) -> RegisterAddress<fdma::FDMA_CH_DISABLE> {
        RegisterAddress::new(self.0 + 0xd4)
    }
    #[inline(always)]
    pub fn FDMA_CH_FORCEDIS(&self) -> RegisterAddress<fdma::FDMA_CH_FORCEDIS> {
        RegisterAddress::new(self.0 + 0xd8)
    }
    #[inline(always)]
    pub fn FDMA_CH_INJ_TOKEN_CNT(&self, index: u8) -> RegisterAddress<fdma::FDMA_CH_INJ_TOKEN_CNT> {
        debug_assert!(index < 8);
        RegisterAddress::new(self.0 + 0x104 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_CH_INJ_TOKEN_TICK_CNT(
        &self,
        index: u8,
    ) -> RegisterAddress<fdma::FDMA_CH_INJ_TOKEN_TICK_CNT> {
        debug_assert!(index < 8);
        RegisterAddress::new(self.0 + 0x144 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_CH_INJ_TOKEN_TICK_RLD(
        &self,
        index: u8,
    ) -> RegisterAddress<fdma::FDMA_CH_INJ_TOKEN_TICK_RLD> {
        debug_assert!(index < 8);
        RegisterAddress::new(self.0 + 0x124 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_CH_SAFE(&self) -> RegisterAddress<fdma::FDMA_CH_SAFE> {
        RegisterAddress::new(self.0 + 0xcc)
    }
    #[inline(always)]
    pub fn FDMA_CH_STAT(&self) -> RegisterAddress<fdma::FDMA_CH_STAT> {
        RegisterAddress::new(self.0 + 0xc8)
    }
    #[inline(always)]
    pub fn FDMA_CONST(&self) -> RegisterAddress<fdma::FDMA_CONST> {
        RegisterAddress::new(self.0 + 0x1c0)
    }
    #[inline(always)]
    pub fn FDMA_DCB_DATAL(&self, index: u8) -> RegisterAddress<fdma::FDMA_DCB_DATAL> {
        debug_assert!(index < 10);
        RegisterAddress::new(self.0 + 0x50 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_DCB_DATAP(&self, index: u8) -> RegisterAddress<fdma::FDMA_DCB_DATAP> {
        debug_assert!(index < 10);
        RegisterAddress::new(self.0 + 0x28 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_DCB_LLP(&self, index: u8) -> RegisterAddress<fdma::FDMA_DCB_LLP> {
        debug_assert!(index < 10);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_DCB_LLP_PREV(&self, index: u8) -> RegisterAddress<fdma::FDMA_DCB_LLP_PREV> {
        debug_assert!(index < 10);
        RegisterAddress::new(self.0 + 0xa0 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_DCB_STAT(&self, index: u8) -> RegisterAddress<fdma::FDMA_DCB_STAT> {
        debug_assert!(index < 10);
        RegisterAddress::new(self.0 + 0x78 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn FDMA_EVT_ERR(&self) -> RegisterAddress<fdma::FDMA_EVT_ERR> {
        RegisterAddress::new(self.0 + 0x164)
    }
    #[inline(always)]
    pub fn FDMA_EVT_ERR_CODE(&self) -> RegisterAddress<fdma::FDMA_EVT_ERR_CODE> {
        RegisterAddress::new(self.0 + 0x168)
    }
    #[inline(always)]
    pub fn FDMA_GCFG(&self) -> RegisterAddress<fdma::FDMA_GCFG> {
        RegisterAddress::new(self.0 + 0x1b4)
    }
    #[inline(always)]
    pub fn FDMA_GSTAT(&self) -> RegisterAddress<fdma::FDMA_GSTAT> {
        RegisterAddress::new(self.0 + 0x1b8)
    }
    #[inline(always)]
    pub fn FDMA_IDLECNT(&self) -> RegisterAddress<fdma::FDMA_IDLECNT> {
        RegisterAddress::new(self.0 + 0x1bc)
    }
    #[inline(always)]
    pub fn FDMA_INTR_ENA(&self) -> RegisterAddress<fdma::FDMA_INTR_ENA> {
        RegisterAddress::new(self.0 + 0x184)
    }
    #[inline(always)]
    pub fn FDMA_INTR_FRM(&self) -> RegisterAddress<fdma::FDMA_INTR_FRM> {
        RegisterAddress::new(self.0 + 0x174)
    }
    #[inline(always)]
    pub fn FDMA_INTR_FRM_ENA(&self) -> RegisterAddress<fdma::FDMA_INTR_FRM_ENA> {
        RegisterAddress::new(self.0 + 0x178)
    }
    #[inline(always)]
    pub fn FDMA_INTR_IDENT(&self) -> RegisterAddress<fdma::FDMA_INTR_IDENT> {
        RegisterAddress::new(self.0 + 0x188)
    }
    #[inline(always)]
    pub fn FDMA_INTR_LLP(&self) -> RegisterAddress<fdma::FDMA_INTR_LLP> {
        RegisterAddress::new(self.0 + 0x16c)
    }
    #[inline(always)]
    pub fn FDMA_INTR_LLP_ENA(&self) -> RegisterAddress<fdma::FDMA_INTR_LLP_ENA> {
        RegisterAddress::new(self.0 + 0x170)
    }
    #[inline(always)]
    pub fn FDMA_INTR_SIG(&self) -> RegisterAddress<fdma::FDMA_INTR_SIG> {
        RegisterAddress::new(self.0 + 0x17c)
    }
    #[inline(always)]
    pub fn FDMA_INTR_SIG_ENA(&self) -> RegisterAddress<fdma::FDMA_INTR_SIG_ENA> {
        RegisterAddress::new(self.0 + 0x180)
    }
}

/// Interrupt controller
pub struct INTR(pub(super) u32);
impl INTR {
    #[inline(always)]
    pub fn DEV_INTR_BYPASS(&self) -> RegisterAddress<intr::DEV_INTR_BYPASS> {
        RegisterAddress::new(self.0 + 0x7c)
    }
    #[inline(always)]
    pub fn DEV_INTR_BYPASS1(&self) -> RegisterAddress<intr::DEV_INTR_BYPASS1> {
        RegisterAddress::new(self.0 + 0x80)
    }
    #[inline(always)]
    pub fn DEV_INTR_ENA(&self) -> RegisterAddress<intr::DEV_INTR_ENA> {
        RegisterAddress::new(self.0 + 0x84)
    }
    #[inline(always)]
    pub fn DEV_INTR_ENA1(&self) -> RegisterAddress<intr::DEV_INTR_ENA1> {
        RegisterAddress::new(self.0 + 0x88)
    }
    #[inline(always)]
    pub fn DEV_INTR_IDENT(&self) -> RegisterAddress<intr::DEV_INTR_IDENT> {
        RegisterAddress::new(self.0 + 0x8c)
    }
    #[inline(always)]
    pub fn DEV_INTR_IDENT1(&self) -> RegisterAddress<intr::DEV_INTR_IDENT1> {
        RegisterAddress::new(self.0 + 0x90)
    }
    #[inline(always)]
    pub fn DEV_INTR_POL(&self) -> RegisterAddress<intr::DEV_INTR_POL> {
        RegisterAddress::new(self.0 + 0x54)
    }
    #[inline(always)]
    pub fn DEV_INTR_POL1(&self) -> RegisterAddress<intr::DEV_INTR_POL1> {
        RegisterAddress::new(self.0 + 0x58)
    }
    #[inline(always)]
    pub fn DEV_INTR_RAW(&self) -> RegisterAddress<intr::DEV_INTR_RAW> {
        RegisterAddress::new(self.0 + 0x5c)
    }
    #[inline(always)]
    pub fn DEV_INTR_RAW1(&self) -> RegisterAddress<intr::DEV_INTR_RAW1> {
        RegisterAddress::new(self.0 + 0x60)
    }
    #[inline(always)]
    pub fn DEV_INTR_STICKY(&self) -> RegisterAddress<intr::DEV_INTR_STICKY> {
        RegisterAddress::new(self.0 + 0x74)
    }
    #[inline(always)]
    pub fn DEV_INTR_STICKY1(&self) -> RegisterAddress<intr::DEV_INTR_STICKY1> {
        RegisterAddress::new(self.0 + 0x78)
    }
    #[inline(always)]
    pub fn DEV_INTR_TRIGGER(&self, index: u8) -> RegisterAddress<intr::DEV_INTR_TRIGGER> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x64 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn DEV_INTR_TRIGGER1(&self, index: u8) -> RegisterAddress<intr::DEV_INTR_TRIGGER1> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x6c + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn DST_INTR_IDENT(&self, index: u8) -> RegisterAddress<intr::DST_INTR_IDENT> {
        debug_assert!(index < 4);
        RegisterAddress::new(self.0 + 0x38 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn DST_INTR_MAP(&self, index: u8) -> RegisterAddress<intr::DST_INTR_MAP> {
        debug_assert!(index < 4);
        RegisterAddress::new(self.0 + 0x28 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn EXT_DST_INTR_DRV(&self) -> RegisterAddress<intr::EXT_DST_INTR_DRV> {
        RegisterAddress::new(self.0 + 0x50)
    }
    #[inline(always)]
    pub fn EXT_DST_INTR_POL(&self) -> RegisterAddress<intr::EXT_DST_INTR_POL> {
        RegisterAddress::new(self.0 + 0x4c)
    }
    #[inline(always)]
    pub fn EXT_SRC_INTR_POL(&self) -> RegisterAddress<intr::EXT_SRC_INTR_POL> {
        RegisterAddress::new(self.0 + 0x48)
    }
    #[inline(always)]
    pub fn INTR_BYPASS(&self) -> RegisterAddress<intr::INTR_BYPASS> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn INTR_ENA(&self) -> RegisterAddress<intr::INTR_ENA> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn INTR_ENA_CLR(&self) -> RegisterAddress<intr::INTR_ENA_CLR> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    #[inline(always)]
    pub fn INTR_ENA_SET(&self) -> RegisterAddress<intr::INTR_ENA_SET> {
        RegisterAddress::new(self.0 + 0x20)
    }
    #[inline(always)]
    pub fn INTR_FORCE(&self) -> RegisterAddress<intr::INTR_FORCE> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn INTR_IDENT(&self) -> RegisterAddress<intr::INTR_IDENT> {
        RegisterAddress::new(self.0 + 0x24)
    }
    #[inline(always)]
    pub fn INTR_RAW(&self) -> RegisterAddress<intr::INTR_RAW> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn INTR_STICKY(&self) -> RegisterAddress<intr::INTR_STICKY> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn INTR_TRIGGER(&self, index: u8) -> RegisterAddress<intr::INTR_TRIGGER> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x4 + u32::from(index) * 0x4)
    }
}

/// Manual extraction and injection via FDMA
pub struct MANUAL_XTRINJ(pub(super) u32);
impl MANUAL_XTRINJ {
    #[inline(always)]
    pub fn MANUAL_CFG(&self) -> RegisterAddress<manual_xtrinj::MANUAL_CFG> {
        RegisterAddress::new(self.0 + 0x8000)
    }
    #[inline(always)]
    pub fn MANUAL_INJ(&self, index: u16) -> RegisterAddress<manual_xtrinj::MANUAL_INJ> {
        debug_assert!(index < 4096);
        RegisterAddress::new(self.0 + 0x4000 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn MANUAL_INTR(&self) -> RegisterAddress<manual_xtrinj::MANUAL_INTR> {
        RegisterAddress::new(self.0 + 0x8004)
    }
    #[inline(always)]
    pub fn MANUAL_INTR_ENA(&self) -> RegisterAddress<manual_xtrinj::MANUAL_INTR_ENA> {
        RegisterAddress::new(self.0 + 0x8008)
    }
    #[inline(always)]
    pub fn MANUAL_XTR(&self, index: u16) -> RegisterAddress<manual_xtrinj::MANUAL_XTR> {
        debug_assert!(index < 4096);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
}

/// DDR2/3 memory controller
pub struct MEMCTRL(pub(super) u32);
impl MEMCTRL {
    #[inline(always)]
    pub fn MEMCTRL_CFG(&self) -> RegisterAddress<memctrl::MEMCTRL_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn MEMCTRL_CTRL(&self) -> RegisterAddress<memctrl::MEMCTRL_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn MEMCTRL_DFT(&self) -> RegisterAddress<memctrl::MEMCTRL_DFT> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    #[inline(always)]
    pub fn MEMCTRL_DQS_AUTO(&self, index: u8) -> RegisterAddress<memctrl::MEMCTRL_DQS_AUTO> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x48 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn MEMCTRL_DQS_DLY(&self, index: u8) -> RegisterAddress<memctrl::MEMCTRL_DQS_DLY> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x40 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn MEMCTRL_MR0_VAL(&self) -> RegisterAddress<memctrl::MEMCTRL_MR0_VAL> {
        RegisterAddress::new(self.0 + 0x28)
    }
    #[inline(always)]
    pub fn MEMCTRL_MR1_VAL(&self) -> RegisterAddress<memctrl::MEMCTRL_MR1_VAL> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    #[inline(always)]
    pub fn MEMCTRL_MR2_VAL(&self) -> RegisterAddress<memctrl::MEMCTRL_MR2_VAL> {
        RegisterAddress::new(self.0 + 0x30)
    }
    #[inline(always)]
    pub fn MEMCTRL_MR3_VAL(&self) -> RegisterAddress<memctrl::MEMCTRL_MR3_VAL> {
        RegisterAddress::new(self.0 + 0x34)
    }
    #[inline(always)]
    pub fn MEMCTRL_REF_PERIOD(&self) -> RegisterAddress<memctrl::MEMCTRL_REF_PERIOD> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn MEMCTRL_STAT(&self) -> RegisterAddress<memctrl::MEMCTRL_STAT> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn MEMCTRL_TERMRES_CTRL(&self) -> RegisterAddress<memctrl::MEMCTRL_TERMRES_CTRL> {
        RegisterAddress::new(self.0 + 0x38)
    }
    #[inline(always)]
    pub fn MEMCTRL_TIMING0(&self) -> RegisterAddress<memctrl::MEMCTRL_TIMING0> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn MEMCTRL_TIMING1(&self) -> RegisterAddress<memctrl::MEMCTRL_TIMING1> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn MEMCTRL_TIMING2(&self) -> RegisterAddress<memctrl::MEMCTRL_TIMING2> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    #[inline(always)]
    pub fn MEMCTRL_TIMING3(&self) -> RegisterAddress<memctrl::MEMCTRL_TIMING3> {
        RegisterAddress::new(self.0 + 0x20)
    }
    #[inline(always)]
    pub fn MEMCTRL_TIMING4(&self) -> RegisterAddress<memctrl::MEMCTRL_TIMING4> {
        RegisterAddress::new(self.0 + 0x24)
    }
    #[inline(always)]
    pub fn MEMCTRL_ZQCAL(&self) -> RegisterAddress<memctrl::MEMCTRL_ZQCAL> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn MEMPHY_CFG(&self) -> RegisterAddress<memctrl::MEMPHY_CFG> {
        RegisterAddress::new(self.0 + 0x50)
    }
    #[inline(always)]
    pub fn MEMPHY_DFT(&self) -> RegisterAddress<memctrl::MEMPHY_DFT> {
        RegisterAddress::new(self.0 + 0x54)
    }
    #[inline(always)]
    pub fn MEMPHY_DLLCFG0(&self, index: u8) -> RegisterAddress<memctrl::MEMPHY_DLLCFG0> {
        debug_assert!(index < 3);
        RegisterAddress::new(self.0 + 0x58 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn MEMPHY_DLLCFG1(&self, index: u8) -> RegisterAddress<memctrl::MEMPHY_DLLCFG1> {
        debug_assert!(index < 3);
        RegisterAddress::new(self.0 + 0x64 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn MEMPHY_DQ_DLY_TRM(&self, index: u8) -> RegisterAddress<memctrl::MEMPHY_DQ_DLY_TRM> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x70 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn MEMPHY_ZCAL(&self) -> RegisterAddress<memctrl::MEMPHY_ZCAL> {
        RegisterAddress::new(self.0 + 0x78)
    }
    #[inline(always)]
    pub fn MEMPHY_ZCAL_FORCE(&self) -> RegisterAddress<memctrl::MEMPHY_ZCAL_FORCE> {
        RegisterAddress::new(self.0 + 0x84)
    }
    #[inline(always)]
    pub fn MEMPHY_ZCAL_OVR(&self) -> RegisterAddress<memctrl::MEMPHY_ZCAL_OVR> {
        RegisterAddress::new(self.0 + 0x80)
    }
    #[inline(always)]
    pub fn MEMPHY_ZCAL_STAT(&self) -> RegisterAddress<memctrl::MEMPHY_ZCAL_STAT> {
        RegisterAddress::new(self.0 + 0x7c)
    }
}

/// Configuration/status for the 8051
pub struct MPU8051(pub(super) u32);
impl MPU8051 {
    #[inline(always)]
    pub fn MEMACC(&self) -> RegisterAddress<mpu8051::MEMACC> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn MEMACC_CTRL(&self) -> RegisterAddress<mpu8051::MEMACC_CTRL> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn MEMACC_SBA(&self) -> RegisterAddress<mpu8051::MEMACC_SBA> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn MPU8051_CFG(&self) -> RegisterAddress<mpu8051::MPU8051_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn MPU8051_IROM(&self) -> RegisterAddress<mpu8051::MPU8051_IROM> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn MPU8051_MMAP(&self) -> RegisterAddress<mpu8051::MPU8051_MMAP> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn MPU8051_STAT(&self) -> RegisterAddress<mpu8051::MPU8051_STAT> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// PCIe endpoint
pub struct PCIE(pub(super) u32);
impl PCIE {
    #[inline(always)]
    pub fn PCIEMST_BAR1_MASK(&self) -> RegisterAddress<pcie::PCIEMST_BAR1_MASK> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    #[inline(always)]
    pub fn PCIEMST_BAR1_OFFSET(&self) -> RegisterAddress<pcie::PCIEMST_BAR1_OFFSET> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn PCIEMST_BAR2_MASK(&self) -> RegisterAddress<pcie::PCIEMST_BAR2_MASK> {
        RegisterAddress::new(self.0 + 0x24)
    }
    #[inline(always)]
    pub fn PCIEMST_BAR2_OFFSET(&self) -> RegisterAddress<pcie::PCIEMST_BAR2_OFFSET> {
        RegisterAddress::new(self.0 + 0x20)
    }
    #[inline(always)]
    pub fn PCIEMST_REPLY_INFO(&self) -> RegisterAddress<pcie::PCIEMST_REPLY_INFO> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn PCIEPCS_BEACON(&self) -> RegisterAddress<pcie::PCIEPCS_BEACON> {
        RegisterAddress::new(self.0 + 0x34)
    }
    #[inline(always)]
    pub fn PCIEPCS_CFG(&self) -> RegisterAddress<pcie::PCIEPCS_CFG> {
        RegisterAddress::new(self.0 + 0x30)
    }
    #[inline(always)]
    pub fn PCIESLV_FDMA(&self) -> RegisterAddress<pcie::PCIESLV_FDMA> {
        RegisterAddress::new(self.0 + 0x28)
    }
    #[inline(always)]
    pub fn PCIESLV_SBA(&self) -> RegisterAddress<pcie::PCIESLV_SBA> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    #[inline(always)]
    pub fn PCIE_AUX_CFG(&self) -> RegisterAddress<pcie::PCIE_AUX_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn PCIE_CFG(&self) -> RegisterAddress<pcie::PCIE_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn PCIE_CTRL(&self) -> RegisterAddress<pcie::PCIE_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn PCIE_DBG_STAT(&self) -> RegisterAddress<pcie::PCIE_DBG_STAT> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn PCIE_INTR(&self) -> RegisterAddress<pcie::PCIE_INTR> {
        RegisterAddress::new(self.0 + 0x38)
    }
    #[inline(always)]
    pub fn PCIE_INTR_CFG(&self, index: u8) -> RegisterAddress<pcie::PCIE_INTR_CFG> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x48 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn PCIE_INTR_COMMON_CFG(&self) -> RegisterAddress<pcie::PCIE_INTR_COMMON_CFG> {
        RegisterAddress::new(self.0 + 0x44)
    }
    #[inline(always)]
    pub fn PCIE_INTR_ENA(&self) -> RegisterAddress<pcie::PCIE_INTR_ENA> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    #[inline(always)]
    pub fn PCIE_INTR_IDENT(&self) -> RegisterAddress<pcie::PCIE_INTR_IDENT> {
        RegisterAddress::new(self.0 + 0x40)
    }
    #[inline(always)]
    pub fn PCIE_INTR_STAT(&self, index: u8) -> RegisterAddress<pcie::PCIE_INTR_STAT> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x50 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn PCIE_STAT(&self) -> RegisterAddress<pcie::PCIE_STAT> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// Parallel Interface Configuration
pub struct PI_MST(pub(super) u32);
impl PI_MST {
    #[inline(always)]
    pub fn PI_MST_CFG(&self) -> RegisterAddress<pi_mst::PI_MST_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn PI_MST_CTRL(&self) -> RegisterAddress<pi_mst::PI_MST_CTRL> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn PI_MST_STATUS(&self) -> RegisterAddress<pi_mst::PI_MST_STATUS> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn PI_SLV_CFG(&self) -> RegisterAddress<pi_mst::PI_SLV_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
}

/// SPI boot master
pub struct SPI_MST(pub(super) u32);
impl SPI_MST {
    #[inline(always)]
    pub fn SPI_MST_CFG(&self) -> RegisterAddress<spi_mst::SPI_MST_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn SPI_MST_STATUS(&self, index: u8) -> RegisterAddress<spi_mst::SPI_MST_STATUS> {
        debug_assert!(index < 4);
        RegisterAddress::new(self.0 + 0x4 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn SW_MODE(&self) -> RegisterAddress<spi_mst::SW_MODE> {
        RegisterAddress::new(self.0 + 0x14)
    }
}

/// Timers
pub struct TIMERS(pub(super) u32);
impl TIMERS {
    #[inline(always)]
    pub fn TIMER_CTRL(&self, index: u8) -> RegisterAddress<timers::TIMER_CTRL> {
        debug_assert!(index < 3);
        RegisterAddress::new(self.0 + 0x20 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn TIMER_RELOAD_VALUE(&self, index: u8) -> RegisterAddress<timers::TIMER_RELOAD_VALUE> {
        debug_assert!(index < 3);
        RegisterAddress::new(self.0 + 0x14 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn TIMER_TICK_DIV(&self) -> RegisterAddress<timers::TIMER_TICK_DIV> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn TIMER_VALUE(&self, index: u8) -> RegisterAddress<timers::TIMER_VALUE> {
        debug_assert!(index < 3);
        RegisterAddress::new(self.0 + 0x8 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn WDT(&self) -> RegisterAddress<timers::WDT> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// TWI hold time configuration
pub struct TWI_DELAY(pub(super) u32);
impl TWI_DELAY {
    #[inline(always)]
    pub fn TWI_CONFIG(&self) -> RegisterAddress<twi_delay::TWI_CONFIG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// TWI spike filter configuration
pub struct TWI_SPIKE_FILTER(pub(super) u32);
impl TWI_SPIKE_FILTER {
    #[inline(always)]
    pub fn TWI_SPIKE_FILTER_CFG(&self) -> RegisterAddress<twi_spike_filter::TWI_SPIKE_FILTER_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}
