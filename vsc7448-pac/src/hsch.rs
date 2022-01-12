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

pub mod hsch_cfg;
pub mod hsch_dwrr;
pub mod hsch_inp_state;
pub mod hsch_l0_cfg;
pub mod hsch_l1_cfg;
pub mod hsch_l1w;
pub mod hsch_leak_lists;
pub mod hsch_misc;
pub mod hsch_status;
pub mod qshp_alloc_cfg;
pub mod qshp_cfg;
pub mod qshp_status;

/// Configuration of scheduling system and shapers
pub struct HSCH_CFG(pub(super) u32);
impl HSCH_CFG {
    #[inline(always)]
    pub fn CIR_CFG(&self) -> RegisterAddress<hsch_cfg::CIR_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn EIR_CFG(&self) -> RegisterAddress<hsch_cfg::EIR_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn SE_CFG(&self) -> RegisterAddress<hsch_cfg::SE_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn SE_CONNECT(&self) -> RegisterAddress<hsch_cfg::SE_CONNECT> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn SE_DLB_SENSE(&self) -> RegisterAddress<hsch_cfg::SE_DLB_SENSE> {
        RegisterAddress::new(self.0 + 0x10)
    }
}

/// Configure DWRR weights
pub struct HSCH_DWRR(pub(super) u32);
impl HSCH_DWRR {
    #[inline(always)]
    pub fn DWRR_ENTRY(&self) -> RegisterAddress<hsch_dwrr::DWRR_ENTRY> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Status of scheduling element inputs
pub struct HSCH_INP_STATE(pub(super) u32);
impl HSCH_INP_STATE {
    #[inline(always)]
    pub fn INP_STATE(&self) -> RegisterAddress<hsch_inp_state::INP_STATE> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Hierarchy configuration
pub struct HSCH_L0_CFG(pub(super) u32);
impl HSCH_L0_CFG {
    #[inline(always)]
    pub fn HSCH_L0_CFG(&self) -> RegisterAddress<hsch_l0_cfg::HSCH_L0_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Win data for layer 1
pub struct HSCH_L1W(pub(super) u32);
impl HSCH_L1W {
    #[inline(always)]
    pub fn HSCH_L1W(&self, index: u32) -> RegisterAddress<hsch_l1w::HSCH_L1W> {
        assert!(index < 64);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
}

/// Hierarchy configuration
pub struct HSCH_L1_CFG(pub(super) u32);
impl HSCH_L1_CFG {
    #[inline(always)]
    pub fn HSCH_L1_CFG(&self) -> RegisterAddress<hsch_l1_cfg::HSCH_L1_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Configuration of leak lists
pub struct HSCH_LEAK_LISTS(pub(super) u32);
impl HSCH_LEAK_LISTS {
    #[inline(always)]
    pub fn HSCH_LEAK_CFG(&self, index: u32) -> RegisterAddress<hsch_leak_lists::HSCH_LEAK_CFG> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x10 + index * 0x4)
    }
    #[inline(always)]
    pub fn HSCH_TIMER_CFG(&self, index: u32) -> RegisterAddress<hsch_leak_lists::HSCH_TIMER_CFG> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
}

/// Miscellaneous scheduler configuration
pub struct HSCH_MISC(pub(super) u32);
impl HSCH_MISC {
    #[inline(always)]
    pub fn DEBUG_CTRL(&self) -> RegisterAddress<hsch_misc::DEBUG_CTRL> {
        RegisterAddress::new(self.0 + 0x218)
    }
    #[inline(always)]
    pub fn EQ_STAT(&self) -> RegisterAddress<hsch_misc::EQ_STAT> {
        RegisterAddress::new(self.0 + 0x214)
    }
    #[inline(always)]
    pub fn EVENTS_CORE(&self) -> RegisterAddress<hsch_misc::EVENTS_CORE> {
        RegisterAddress::new(self.0 + 0x210)
    }
    #[inline(always)]
    pub fn EVENT_ENQ_ERR(&self) -> RegisterAddress<hsch_misc::EVENT_ENQ_ERR> {
        RegisterAddress::new(self.0 + 0x220)
    }
    #[inline(always)]
    pub fn FLUSH_CTRL(&self) -> RegisterAddress<hsch_misc::FLUSH_CTRL> {
        RegisterAddress::new(self.0 + 0x20c)
    }
    #[inline(always)]
    pub fn HSCH_CFG_CFG(&self) -> RegisterAddress<hsch_misc::HSCH_CFG_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn HSCH_FORCE_CTRL(&self) -> RegisterAddress<hsch_misc::HSCH_FORCE_CTRL> {
        RegisterAddress::new(self.0 + 0x228)
    }
    #[inline(always)]
    pub fn HSCH_LARGE_ENA(&self, index: u32) -> RegisterAddress<hsch_misc::HSCH_LARGE_ENA> {
        assert!(index < 14);
        RegisterAddress::new(self.0 + 0x1c0 + index * 0x4)
    }
    #[inline(always)]
    pub fn HSCH_MISC_CFG(&self) -> RegisterAddress<hsch_misc::HSCH_MISC_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn HSCH_UPDATE_STAT(&self) -> RegisterAddress<hsch_misc::HSCH_UPDATE_STAT> {
        RegisterAddress::new(self.0 + 0x21c)
    }
    #[inline(always)]
    pub fn OUTB_CPU_SHARE_ENA(&self) -> RegisterAddress<hsch_misc::OUTB_CPU_SHARE_ENA> {
        RegisterAddress::new(self.0 + 0x1f8)
    }
    #[inline(always)]
    pub fn OUTB_SHARE_ENA(&self, index: u32) -> RegisterAddress<hsch_misc::OUTB_SHARE_ENA> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x1fc + index * 0x4)
    }
    #[inline(always)]
    pub fn PFC_CFG(&self, index: u32) -> RegisterAddress<hsch_misc::PFC_CFG> {
        assert!(index < 53);
        RegisterAddress::new(self.0 + 0xec + index * 0x4)
    }
    #[inline(always)]
    pub fn PORT_MODE(&self, index: u32) -> RegisterAddress<hsch_misc::PORT_MODE> {
        assert!(index < 57);
        RegisterAddress::new(self.0 + 0x8 + index * 0x4)
    }
    #[inline(always)]
    pub fn SYS_CLK_PER(&self) -> RegisterAddress<hsch_misc::SYS_CLK_PER> {
        RegisterAddress::new(self.0 + 0x224)
    }
}

/// Status of scheduling system and shapers
pub struct HSCH_STATUS(pub(super) u32);
impl HSCH_STATUS {
    #[inline(always)]
    pub fn CIR_STATE(&self) -> RegisterAddress<hsch_status::CIR_STATE> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn EIR_STATE(&self) -> RegisterAddress<hsch_status::EIR_STATE> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn SE_STATE(&self) -> RegisterAddress<hsch_status::SE_STATE> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// Assign queue shapers to queues
pub struct QSHP_ALLOC_CFG(pub(super) u32);
impl QSHP_ALLOC_CFG {
    #[inline(always)]
    pub fn QSHP_ALLOC_CFG(&self) -> RegisterAddress<qshp_alloc_cfg::QSHP_ALLOC_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn QSHP_CONNECT(&self) -> RegisterAddress<qshp_alloc_cfg::QSHP_CONNECT> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// Configuration of queue shapers
pub struct QSHP_CFG(pub(super) u32);
impl QSHP_CFG {
    #[inline(always)]
    pub fn QSHP_CFG(&self) -> RegisterAddress<qshp_cfg::QSHP_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn QSHP_CIR_CFG(&self) -> RegisterAddress<qshp_cfg::QSHP_CIR_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Status of queue shapers
pub struct QSHP_STATUS(pub(super) u32);
impl QSHP_STATUS {
    #[inline(always)]
    pub fn QSHP_CIR_STATE(&self) -> RegisterAddress<qshp_status::QSHP_CIR_STATE> {
        RegisterAddress::new(self.0 + 0x0)
    }
}
