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

pub mod cfg;
pub mod coremem;
pub mod ram_ctrl;
pub mod rate_limit_cfg;
pub mod rate_limit_status;
pub mod status;

/// Configuration registers
pub struct CFG(pub(super) u32);
impl CFG {
    #[inline(always)]
    pub fn BUF_CFG(&self, index: u8) -> RegisterAddress<cfg::BUF_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn CLR_BUF(&self, index: u8) -> RegisterAddress<cfg::CLR_BUF> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x294 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn DBG_CTRL(&self) -> RegisterAddress<cfg::DBG_CTRL> {
        RegisterAddress::new(self.0 + 0xa48)
    }
    #[inline(always)]
    pub fn DEV_TX_STOP_WM_CFG(&self, index: u8) -> RegisterAddress<cfg::DEV_TX_STOP_WM_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x454 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn ETH_FC_CFG(&self, index: u8) -> RegisterAddress<cfg::ETH_FC_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x60c + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn ETH_PFC_CFG(&self, index: u8) -> RegisterAddress<cfg::ETH_PFC_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x6e8 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn IPG_SHRINK_CFG(&self, index: u8) -> RegisterAddress<cfg::IPG_SHRINK_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x1b8 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn MAC_ADDR_BASE_HIGH_CFG(
        &self,
        index: u8,
    ) -> RegisterAddress<cfg::MAC_ADDR_BASE_HIGH_CFG> {
        debug_assert!(index < 53);
        RegisterAddress::new(self.0 + 0x8a0 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn MAC_ADDR_BASE_LOW_CFG(&self, index: u8) -> RegisterAddress<cfg::MAC_ADDR_BASE_LOW_CFG> {
        debug_assert!(index < 53);
        RegisterAddress::new(self.0 + 0x974 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn MAC_CFG(&self, index: u8) -> RegisterAddress<cfg::MAC_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x7c4 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn RATE_CTRL(&self, index: u8) -> RegisterAddress<cfg::RATE_CTRL> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0xdc + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn RX_PAUSE_CFG(&self, index: u8) -> RegisterAddress<cfg::RX_PAUSE_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x530 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn SCH_STOP_WM_CFG(&self, index: u8) -> RegisterAddress<cfg::SCH_STOP_WM_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x29c + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn TX_START_WM_CFG(&self, index: u8) -> RegisterAddress<cfg::TX_START_WM_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x378 + u32::from(index) * 0x4)
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

/// Access core memory
pub struct RAM_CTRL(pub(super) u32);
impl RAM_CTRL {
    #[inline(always)]
    pub fn RAM_INIT(&self) -> RegisterAddress<ram_ctrl::RAM_INIT> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Configuration registers for rate limit modes
pub struct RATE_LIMIT_CFG(pub(super) u32);
impl RATE_LIMIT_CFG {
    #[inline(always)]
    pub fn TX_FRAME_RATE_START_CFG(
        &self,
        index: u8,
    ) -> RegisterAddress<rate_limit_cfg::TX_FRAME_RATE_START_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x1b8 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn TX_IPG_STRETCH_RATIO_CFG(
        &self,
        index: u8,
    ) -> RegisterAddress<rate_limit_cfg::TX_IPG_STRETCH_RATIO_CFG> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0xdc + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn TX_RATE_LIMIT_HDR_CFG(&self) -> RegisterAddress<rate_limit_cfg::TX_RATE_LIMIT_HDR_CFG> {
        RegisterAddress::new(self.0 + 0x294)
    }
    #[inline(always)]
    pub fn TX_RATE_LIMIT_MODE(
        &self,
        index: u8,
    ) -> RegisterAddress<rate_limit_cfg::TX_RATE_LIMIT_MODE> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
}

/// Status registers for rate limit modes
pub struct RATE_LIMIT_STATUS(pub(super) u32);
impl RATE_LIMIT_STATUS {
    #[inline(always)]
    pub fn TX_RATE_LIMIT_STICKY(
        &self,
        index: u8,
    ) -> RegisterAddress<rate_limit_status::TX_RATE_LIMIT_STICKY> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
}

/// Status registers
pub struct STATUS(pub(super) u32);
impl STATUS {
    #[inline(always)]
    pub fn AGED_FRMS(&self, index: u8) -> RegisterAddress<status::AGED_FRMS> {
        debug_assert!(index < 55);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn BUF_OFLW_STICKY(&self, index: u8) -> RegisterAddress<status::BUF_OFLW_STICKY> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0xe0 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn BUF_UFLW_STICKY(&self, index: u8) -> RegisterAddress<status::BUF_UFLW_STICKY> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0xe8 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn CELL_BUS_STICKY(&self) -> RegisterAddress<status::CELL_BUS_STICKY> {
        RegisterAddress::new(self.0 + 0xdc)
    }
    #[inline(always)]
    pub fn PRE_CNT_OFLW_STICKY(&self) -> RegisterAddress<status::PRE_CNT_OFLW_STICKY> {
        RegisterAddress::new(self.0 + 0xf0)
    }
}
