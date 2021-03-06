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

pub mod sd10g65_apc;
pub mod sd10g65_dft;
pub mod sd10g65_rcpll_bist;
pub mod sd10g65_sync_ctrl;
pub mod sd10g65_vscope2;

/// SD10G65 APC Configuration and Status Register set
pub struct SD10G65_APC(pub(super) u32);
impl SD10G65_APC {
    #[inline(always)]
    pub fn APC_COMMON_CFG0(&self) -> RegisterAddress<sd10g65_apc::APC_COMMON_CFG0> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn APC_DFE1_CTRL(&self) -> RegisterAddress<sd10g65_apc::APC_DFE1_CTRL> {
        RegisterAddress::new(self.0 + 0x78)
    }
    #[inline(always)]
    pub fn APC_DFE1_PAR_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_DFE1_PAR_CFG> {
        RegisterAddress::new(self.0 + 0x80)
    }
    #[inline(always)]
    pub fn APC_DFE1_TIMER_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_DFE1_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0x7c)
    }
    #[inline(always)]
    pub fn APC_DFE2_CTRL(&self) -> RegisterAddress<sd10g65_apc::APC_DFE2_CTRL> {
        RegisterAddress::new(self.0 + 0x84)
    }
    #[inline(always)]
    pub fn APC_DFE2_PAR_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_DFE2_PAR_CFG> {
        RegisterAddress::new(self.0 + 0x8c)
    }
    #[inline(always)]
    pub fn APC_DFE2_TIMER_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_DFE2_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0x88)
    }
    #[inline(always)]
    pub fn APC_DFE3_CTRL(&self) -> RegisterAddress<sd10g65_apc::APC_DFE3_CTRL> {
        RegisterAddress::new(self.0 + 0x90)
    }
    #[inline(always)]
    pub fn APC_DFE3_PAR_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_DFE3_PAR_CFG> {
        RegisterAddress::new(self.0 + 0x98)
    }
    #[inline(always)]
    pub fn APC_DFE3_TIMER_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_DFE3_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0x94)
    }
    #[inline(always)]
    pub fn APC_DFE4_CTRL(&self) -> RegisterAddress<sd10g65_apc::APC_DFE4_CTRL> {
        RegisterAddress::new(self.0 + 0x9c)
    }
    #[inline(always)]
    pub fn APC_DFE4_PAR_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_DFE4_PAR_CFG> {
        RegisterAddress::new(self.0 + 0xa4)
    }
    #[inline(always)]
    pub fn APC_DFE4_TIMER_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_DFE4_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0xa0)
    }
    #[inline(always)]
    pub fn APC_EQZ_AGC_CTRL(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_AGC_CTRL> {
        RegisterAddress::new(self.0 + 0x6c)
    }
    #[inline(always)]
    pub fn APC_EQZ_AGC_PAR_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_AGC_PAR_CFG> {
        RegisterAddress::new(self.0 + 0x74)
    }
    #[inline(always)]
    pub fn APC_EQZ_AGC_TIMER_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_AGC_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0x70)
    }
    #[inline(always)]
    pub fn APC_EQZ_COMMON_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_COMMON_CFG> {
        RegisterAddress::new(self.0 + 0x24)
    }
    #[inline(always)]
    pub fn APC_EQZ_CTRL_STATUS(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_CTRL_STATUS> {
        RegisterAddress::new(self.0 + 0x30)
    }
    #[inline(always)]
    pub fn APC_EQZ_C_CTRL(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_C_CTRL> {
        RegisterAddress::new(self.0 + 0x54)
    }
    #[inline(always)]
    pub fn APC_EQZ_C_PAR_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_C_PAR_CFG> {
        RegisterAddress::new(self.0 + 0x5c)
    }
    #[inline(always)]
    pub fn APC_EQZ_C_TIMER_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_C_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0x58)
    }
    #[inline(always)]
    pub fn APC_EQZ_GAIN_ADJ_CTRL_CFG(
        &self,
    ) -> RegisterAddress<sd10g65_apc::APC_EQZ_GAIN_ADJ_CTRL_CFG> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    #[inline(always)]
    pub fn APC_EQZ_GAIN_CTRL_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_GAIN_CTRL_CFG> {
        RegisterAddress::new(self.0 + 0x28)
    }
    #[inline(always)]
    pub fn APC_EQZ_LD_CTRL(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_LD_CTRL> {
        RegisterAddress::new(self.0 + 0x34)
    }
    #[inline(always)]
    pub fn APC_EQZ_LD_CTRL_CFG0(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_LD_CTRL_CFG0> {
        RegisterAddress::new(self.0 + 0x38)
    }
    #[inline(always)]
    pub fn APC_EQZ_LD_CTRL_CFG1(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_LD_CTRL_CFG1> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    #[inline(always)]
    pub fn APC_EQZ_L_CTRL(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_L_CTRL> {
        RegisterAddress::new(self.0 + 0x60)
    }
    #[inline(always)]
    pub fn APC_EQZ_L_PAR_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_L_PAR_CFG> {
        RegisterAddress::new(self.0 + 0x68)
    }
    #[inline(always)]
    pub fn APC_EQZ_L_TIMER_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_L_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0x64)
    }
    #[inline(always)]
    pub fn APC_EQZ_OFFS_CTRL(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_OFFS_CTRL> {
        RegisterAddress::new(self.0 + 0x48)
    }
    #[inline(always)]
    pub fn APC_EQZ_OFFS_PAR_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_OFFS_PAR_CFG> {
        RegisterAddress::new(self.0 + 0x50)
    }
    #[inline(always)]
    pub fn APC_EQZ_OFFS_TIMER_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_OFFS_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0x4c)
    }
    #[inline(always)]
    pub fn APC_EQZ_PAT_MATCH_CFG0(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_PAT_MATCH_CFG0> {
        RegisterAddress::new(self.0 + 0x40)
    }
    #[inline(always)]
    pub fn APC_EQZ_PAT_MATCH_CFG1(&self) -> RegisterAddress<sd10g65_apc::APC_EQZ_PAT_MATCH_CFG1> {
        RegisterAddress::new(self.0 + 0x44)
    }
    #[inline(always)]
    pub fn APC_FLEXCTRL_CNT_STATUS(&self) -> RegisterAddress<sd10g65_apc::APC_FLEXCTRL_CNT_STATUS> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn APC_IS_CAL_CFG0(&self) -> RegisterAddress<sd10g65_apc::APC_IS_CAL_CFG0> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    #[inline(always)]
    pub fn APC_IS_CAL_CFG1(&self) -> RegisterAddress<sd10g65_apc::APC_IS_CAL_CFG1> {
        RegisterAddress::new(self.0 + 0x20)
    }
    #[inline(always)]
    pub fn APC_LC_SOFTCTRL_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_LC_SOFTCTRL_CFG> {
        RegisterAddress::new(self.0 + 0xa8)
    }
    #[inline(always)]
    pub fn APC_LC_SOFTCTRL_CFG1(&self) -> RegisterAddress<sd10g65_apc::APC_LC_SOFTCTRL_CFG1> {
        RegisterAddress::new(self.0 + 0xac)
    }
    #[inline(always)]
    pub fn APC_LD_CAL_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_LD_CAL_CFG> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn APC_PARCTRL_FSM1_TIMER_CFG(
        &self,
    ) -> RegisterAddress<sd10g65_apc::APC_PARCTRL_FSM1_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn APC_PARCTRL_FSM2_TIMER_CFG(
        &self,
    ) -> RegisterAddress<sd10g65_apc::APC_PARCTRL_FSM2_TIMER_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn APC_PARCTRL_SYNC_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_PARCTRL_SYNC_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn APC_TOP_CTRL_CFG(&self) -> RegisterAddress<sd10g65_apc::APC_TOP_CTRL_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// SD10G65 DFT Configuration and Status Register set
pub struct SD10G65_DFT(pub(super) u32);
impl SD10G65_DFT {
    #[inline(always)]
    pub fn DFT_BIST_CFG0(&self) -> RegisterAddress<sd10g65_dft::DFT_BIST_CFG0> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn DFT_BIST_CFG1(&self) -> RegisterAddress<sd10g65_dft::DFT_BIST_CFG1> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn DFT_BIST_CFG2(&self) -> RegisterAddress<sd10g65_dft::DFT_BIST_CFG2> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn DFT_BIST_CFG3(&self) -> RegisterAddress<sd10g65_dft::DFT_BIST_CFG3> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn DFT_CLK_CMP_CFG(&self) -> RegisterAddress<sd10g65_dft::DFT_CLK_CMP_CFG> {
        RegisterAddress::new(self.0 + 0x34)
    }
    #[inline(always)]
    pub fn DFT_CLK_CMP_MAXVAL(&self) -> RegisterAddress<sd10g65_dft::DFT_CLK_CMP_MAXVAL> {
        RegisterAddress::new(self.0 + 0x40)
    }
    #[inline(always)]
    pub fn DFT_CLK_CMP_TIMER(&self) -> RegisterAddress<sd10g65_dft::DFT_CLK_CMP_TIMER> {
        RegisterAddress::new(self.0 + 0x38)
    }
    #[inline(always)]
    pub fn DFT_CLK_CMP_VALUE(&self) -> RegisterAddress<sd10g65_dft::DFT_CLK_CMP_VALUE> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    #[inline(always)]
    pub fn DFT_CLK_GEN_CFG(&self) -> RegisterAddress<sd10g65_dft::DFT_CLK_GEN_CFG> {
        RegisterAddress::new(self.0 + 0x48)
    }
    #[inline(always)]
    pub fn DFT_ERR_STAT(&self) -> RegisterAddress<sd10g65_dft::DFT_ERR_STAT> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    #[inline(always)]
    pub fn DFT_MAIN_STAT(&self) -> RegisterAddress<sd10g65_dft::DFT_MAIN_STAT> {
        RegisterAddress::new(self.0 + 0x24)
    }
    #[inline(always)]
    pub fn DFT_PRBS_STAT(&self) -> RegisterAddress<sd10g65_dft::DFT_PRBS_STAT> {
        RegisterAddress::new(self.0 + 0x20)
    }
    #[inline(always)]
    pub fn DFT_RX_CFG(&self) -> RegisterAddress<sd10g65_dft::DFT_RX_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn DFT_RX_MASK_CFG(&self) -> RegisterAddress<sd10g65_dft::DFT_RX_MASK_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn DFT_RX_PAT_CFG(&self) -> RegisterAddress<sd10g65_dft::DFT_RX_PAT_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn DFT_TX_CFG(&self) -> RegisterAddress<sd10g65_dft::DFT_TX_CFG> {
        RegisterAddress::new(self.0 + 0x28)
    }
    #[inline(always)]
    pub fn DFT_TX_CMP_DAT_STAT(&self) -> RegisterAddress<sd10g65_dft::DFT_TX_CMP_DAT_STAT> {
        RegisterAddress::new(self.0 + 0x30)
    }
    #[inline(always)]
    pub fn DFT_TX_ERR_INSERT_CFG(&self) -> RegisterAddress<sd10g65_dft::DFT_TX_ERR_INSERT_CFG> {
        RegisterAddress::new(self.0 + 0x44)
    }
    #[inline(always)]
    pub fn DFT_TX_PAT_CFG(&self) -> RegisterAddress<sd10g65_dft::DFT_TX_PAT_CFG> {
        RegisterAddress::new(self.0 + 0x2c)
    }
}

/// SD10G65 RX and TX RCPLL BIST Configuration and Status Register set
pub struct SD10G65_RCPLL_BIST(pub(super) u32);
impl SD10G65_RCPLL_BIST {
    #[inline(always)]
    pub fn SD10G65_RCPLL_BIST_CFG0(
        &self,
    ) -> RegisterAddress<sd10g65_rcpll_bist::SD10G65_RCPLL_BIST_CFG0> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn SD10G65_RCPLL_BIST_CFG1(
        &self,
    ) -> RegisterAddress<sd10g65_rcpll_bist::SD10G65_RCPLL_BIST_CFG1> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn SD10G65_RCPLL_BIST_CFG2(
        &self,
    ) -> RegisterAddress<sd10g65_rcpll_bist::SD10G65_RCPLL_BIST_CFG2> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn SD10G65_RCPLL_BIST_CFG3(
        &self,
    ) -> RegisterAddress<sd10g65_rcpll_bist::SD10G65_RCPLL_BIST_CFG3> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn SD10G65_RCPLL_BIST_STAT0(
        &self,
    ) -> RegisterAddress<sd10g65_rcpll_bist::SD10G65_RCPLL_BIST_STAT0> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn SD10G65_RCPLL_BIST_STAT1(
        &self,
    ) -> RegisterAddress<sd10g65_rcpll_bist::SD10G65_RCPLL_BIST_STAT1> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn SD10G65_RX_RCPLL_BIST_CFG4(
        &self,
    ) -> RegisterAddress<sd10g65_rcpll_bist::SD10G65_RX_RCPLL_BIST_CFG4> {
        RegisterAddress::new(self.0 + 0x10)
    }
}

/// SYNC_CTRL Configuration and Status Register set
pub struct SD10G65_SYNC_CTRL(pub(super) u32);
impl SD10G65_SYNC_CTRL {
    #[inline(always)]
    pub fn RX_SYNC_CTRL_CFG(&self) -> RegisterAddress<sd10g65_sync_ctrl::RX_SYNC_CTRL_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn RX_SYNC_CTRL_STAT(&self) -> RegisterAddress<sd10g65_sync_ctrl::RX_SYNC_CTRL_STAT> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn SYNC_CTRL_CFG(&self) -> RegisterAddress<sd10g65_sync_ctrl::SYNC_CTRL_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn SYNC_CTRL_STAT(&self) -> RegisterAddress<sd10g65_sync_ctrl::SYNC_CTRL_STAT> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// SD10G65 VSCOPE Configuration and Status Register set
pub struct SD10G65_VSCOPE2(pub(super) u32);
impl SD10G65_VSCOPE2 {
    #[inline(always)]
    pub fn VSCOPE_CNT(&self) -> RegisterAddress<sd10g65_vscope2::VSCOPE_CNT> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn VSCOPE_DBG_LSB(&self) -> RegisterAddress<sd10g65_vscope2::VSCOPE_DBG_LSB> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn VSCOPE_HW_SCAN_CFG1(&self) -> RegisterAddress<sd10g65_vscope2::VSCOPE_HW_SCAN_CFG1> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn VSCOPE_HW_SCAN_CFG2(&self) -> RegisterAddress<sd10g65_vscope2::VSCOPE_HW_SCAN_CFG2> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn VSCOPE_MAIN_CFG(&self) -> RegisterAddress<sd10g65_vscope2::VSCOPE_MAIN_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn VSCOPE_PAT_LOCK_CFG(&self) -> RegisterAddress<sd10g65_vscope2::VSCOPE_PAT_LOCK_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn VSCOPE_STAT(&self) -> RegisterAddress<sd10g65_vscope2::VSCOPE_STAT> {
        RegisterAddress::new(self.0 + 0x10)
    }
}
