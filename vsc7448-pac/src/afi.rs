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

pub mod dti_misc;
pub mod dti_tbl;
pub mod frm_tbl;
pub mod misc;
pub mod port_tbl;
pub mod tti_misc;
pub mod tti_tbl;
pub mod tti_ticks;
pub mod tupe;

/// Miscellaneous DTI configration and status information
pub struct DTI_MISC(pub(super) u32);
impl DTI_MISC {
    pub fn DTI_CNT_DOWN(&self) -> RegisterAddress<dti_misc::DTI_CNT_DOWN> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn DTI_CTRL(&self) -> RegisterAddress<dti_misc::DTI_CTRL> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn DTI_FC_CNT_DOWN(&self) -> RegisterAddress<dti_misc::DTI_FC_CNT_DOWN> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Delay Triggered Injection Table
pub struct DTI_TBL(pub(super) u32);
impl DTI_TBL {
    pub fn DTI_CNT(&self) -> RegisterAddress<dti_tbl::DTI_CNT> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn DTI_DURATION(&self) -> RegisterAddress<dti_tbl::DTI_DURATION> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn DTI_FRM(&self) -> RegisterAddress<dti_tbl::DTI_FRM> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn DTI_MODE(&self) -> RegisterAddress<dti_tbl::DTI_MODE> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn DTI_PORT_QU(&self) -> RegisterAddress<dti_tbl::DTI_PORT_QU> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// Frame Table
pub struct FRM_TBL(pub(super) u32);
impl FRM_TBL {
    pub fn FRM_ENTRY_PART0(&self) -> RegisterAddress<frm_tbl::FRM_ENTRY_PART0> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn FRM_NEXT_AND_TYPE(&self) -> RegisterAddress<frm_tbl::FRM_NEXT_AND_TYPE> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Miscellaneous AFI configuration
pub struct MISC(pub(super) u32);
impl MISC {
    pub fn DTI_DURATION_TICK_LEN(&self) -> RegisterAddress<misc::DTI_DURATION_TICK_LEN> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn ERR(&self) -> RegisterAddress<misc::ERR> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn MISC_CTRL(&self) -> RegisterAddress<misc::MISC_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn NEW_FRM_CTRL(&self) -> RegisterAddress<misc::NEW_FRM_CTRL> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn NEW_FRM_INFO(&self) -> RegisterAddress<misc::NEW_FRM_INFO> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn STICKY_INFO(&self) -> RegisterAddress<misc::STICKY_INFO> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn STICKY_INFO_ENA(&self) -> RegisterAddress<misc::STICKY_INFO_ENA> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn WARN(&self) -> RegisterAddress<misc::WARN> {
        RegisterAddress::new(self.0 + 0x10)
    }
}

/// Port parameter configuration
pub struct PORT_TBL(pub(super) u32);
impl PORT_TBL {
    pub fn PORT_CFG(&self) -> RegisterAddress<port_tbl::PORT_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PORT_FRM_OUT(&self) -> RegisterAddress<port_tbl::PORT_FRM_OUT> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Miscellaneous TTI configration
pub struct TTI_MISC(pub(super) u32);
impl TTI_MISC {
    pub fn TTI_CAL_SLOT_CNT(&self, index: u32) -> RegisterAddress<tti_misc::TTI_CAL_SLOT_CNT> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x10 + index * 0x4)
    }
    pub fn TTI_CAL_SLOT_PTRS(&self, index: u32) -> RegisterAddress<tti_misc::TTI_CAL_SLOT_PTRS> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
    pub fn TTI_CAL_STATE(&self, index: u32) -> RegisterAddress<tti_misc::TTI_CAL_STATE> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x20 + index * 0x4)
    }
    pub fn TTI_CTRL(&self) -> RegisterAddress<tti_misc::TTI_CTRL> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn TTI_CTRL2(&self) -> RegisterAddress<tti_misc::TTI_CTRL2> {
        RegisterAddress::new(self.0 + 0x34)
    }
    pub fn TTI_INJ_CNT(&self) -> RegisterAddress<tti_misc::TTI_INJ_CNT> {
        RegisterAddress::new(self.0 + 0x38)
    }
    pub fn TTI_PORT_FRM_OUT(&self) -> RegisterAddress<tti_misc::TTI_PORT_FRM_OUT> {
        RegisterAddress::new(self.0 + 0x40)
    }
    pub fn TTI_RAND_STATE(&self) -> RegisterAddress<tti_misc::TTI_RAND_STATE> {
        RegisterAddress::new(self.0 + 0x3c)
    }
}

/// Timer Triggered Injection Table
pub struct TTI_TBL(pub(super) u32);
impl TTI_TBL {
    pub fn TTI_FRM(&self) -> RegisterAddress<tti_tbl::TTI_FRM> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn TTI_MISC_CFG(&self) -> RegisterAddress<tti_tbl::TTI_MISC_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn TTI_PORT_QU(&self) -> RegisterAddress<tti_tbl::TTI_PORT_QU> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn TTI_TICKS(&self) -> RegisterAddress<tti_tbl::TTI_TICKS> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn TTI_TIMER(&self) -> RegisterAddress<tti_tbl::TTI_TIMER> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn TTI_TUPE_CTRL(&self) -> RegisterAddress<tti_tbl::TTI_TUPE_CTRL> {
        RegisterAddress::new(self.0 + 0x14)
    }
}

/// TTI Tick configuration
pub struct TTI_TICKS(pub(super) u32);
impl TTI_TICKS {
    pub fn TTI_TICK_BASE(&self) -> RegisterAddress<tti_ticks::TTI_TICK_BASE> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn TTI_TICK_LEN_0_3(&self) -> RegisterAddress<tti_ticks::TTI_TICK_LEN_0_3> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn TTI_TICK_LEN_4_7(&self) -> RegisterAddress<tti_ticks::TTI_TICK_LEN_4_7> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn TTI_TICK_STATE(&self, index: u32) -> RegisterAddress<tti_ticks::TTI_TICK_STATE> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0xc + index * 0x4)
    }
}

/// AFI Table UPdata Engine (AFI TUPE)
pub struct TUPE(pub(super) u32);
impl TUPE {
    pub fn TUPE_ADDR(&self) -> RegisterAddress<tupe::TUPE_ADDR> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn TUPE_CMD1(&self) -> RegisterAddress<tupe::TUPE_CMD1> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn TUPE_CRIT1(&self) -> RegisterAddress<tupe::TUPE_CRIT1> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn TUPE_CRIT2(&self) -> RegisterAddress<tupe::TUPE_CRIT2> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn TUPE_CRIT3(&self, index: u32) -> RegisterAddress<tupe::TUPE_CRIT3> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x10 + index * 0x4)
    }
    pub fn TUPE_MISC(&self) -> RegisterAddress<tupe::TUPE_MISC> {
        RegisterAddress::new(self.0 + 0x0)
    }
}
