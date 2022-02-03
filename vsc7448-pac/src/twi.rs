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

pub mod twi;

/// Two-Wire Interface controller
pub struct TWI(pub(super) u32);
impl TWI {
    #[inline(always)]
    pub fn ACK_GEN_CALL(&self) -> RegisterAddress<twi::ACK_GEN_CALL> {
        RegisterAddress::new(self.0 + 0x98)
    }
    #[inline(always)]
    pub fn CFG(&self) -> RegisterAddress<twi::CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn CLR_ACTIVITY(&self) -> RegisterAddress<twi::CLR_ACTIVITY> {
        RegisterAddress::new(self.0 + 0x5c)
    }
    #[inline(always)]
    pub fn CLR_GEN_CALL(&self) -> RegisterAddress<twi::CLR_GEN_CALL> {
        RegisterAddress::new(self.0 + 0x68)
    }
    #[inline(always)]
    pub fn CLR_INTR(&self) -> RegisterAddress<twi::CLR_INTR> {
        RegisterAddress::new(self.0 + 0x40)
    }
    #[inline(always)]
    pub fn CLR_RD_REQ(&self) -> RegisterAddress<twi::CLR_RD_REQ> {
        RegisterAddress::new(self.0 + 0x50)
    }
    #[inline(always)]
    pub fn CLR_RX_DONE(&self) -> RegisterAddress<twi::CLR_RX_DONE> {
        RegisterAddress::new(self.0 + 0x58)
    }
    #[inline(always)]
    pub fn CLR_RX_OVER(&self) -> RegisterAddress<twi::CLR_RX_OVER> {
        RegisterAddress::new(self.0 + 0x48)
    }
    #[inline(always)]
    pub fn CLR_RX_UNDER(&self) -> RegisterAddress<twi::CLR_RX_UNDER> {
        RegisterAddress::new(self.0 + 0x44)
    }
    #[inline(always)]
    pub fn CLR_START_DET(&self) -> RegisterAddress<twi::CLR_START_DET> {
        RegisterAddress::new(self.0 + 0x64)
    }
    #[inline(always)]
    pub fn CLR_STOP_DET(&self) -> RegisterAddress<twi::CLR_STOP_DET> {
        RegisterAddress::new(self.0 + 0x60)
    }
    #[inline(always)]
    pub fn CLR_TX_ABRT(&self) -> RegisterAddress<twi::CLR_TX_ABRT> {
        RegisterAddress::new(self.0 + 0x54)
    }
    #[inline(always)]
    pub fn CLR_TX_OVER(&self) -> RegisterAddress<twi::CLR_TX_OVER> {
        RegisterAddress::new(self.0 + 0x4c)
    }
    #[inline(always)]
    pub fn COMP_PARAM_1(&self) -> RegisterAddress<twi::COMP_PARAM_1> {
        RegisterAddress::new(self.0 + 0xf4)
    }
    #[inline(always)]
    pub fn COMP_TYPE(&self) -> RegisterAddress<twi::COMP_TYPE> {
        RegisterAddress::new(self.0 + 0xfc)
    }
    #[inline(always)]
    pub fn COMP_VERSION(&self) -> RegisterAddress<twi::COMP_VERSION> {
        RegisterAddress::new(self.0 + 0xf8)
    }
    #[inline(always)]
    pub fn CTRL(&self) -> RegisterAddress<twi::CTRL> {
        RegisterAddress::new(self.0 + 0x6c)
    }
    #[inline(always)]
    pub fn DATA_CMD(&self) -> RegisterAddress<twi::DATA_CMD> {
        RegisterAddress::new(self.0 + 0x10)
    }
    #[inline(always)]
    pub fn ENABLE_STATUS(&self) -> RegisterAddress<twi::ENABLE_STATUS> {
        RegisterAddress::new(self.0 + 0x9c)
    }
    #[inline(always)]
    pub fn FS_SCL_HCNT(&self) -> RegisterAddress<twi::FS_SCL_HCNT> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    #[inline(always)]
    pub fn FS_SCL_LCNT(&self) -> RegisterAddress<twi::FS_SCL_LCNT> {
        RegisterAddress::new(self.0 + 0x20)
    }
    #[inline(always)]
    pub fn INTR_MASK(&self) -> RegisterAddress<twi::INTR_MASK> {
        RegisterAddress::new(self.0 + 0x30)
    }
    #[inline(always)]
    pub fn INTR_STAT(&self) -> RegisterAddress<twi::INTR_STAT> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    #[inline(always)]
    pub fn RAW_INTR_STAT(&self) -> RegisterAddress<twi::RAW_INTR_STAT> {
        RegisterAddress::new(self.0 + 0x34)
    }
    #[inline(always)]
    pub fn RESERVED1(&self) -> RegisterAddress<twi::RESERVED1> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn RESERVED2(&self, index: u8) -> RegisterAddress<twi::RESERVED2> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x24 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn RESERVED3(&self) -> RegisterAddress<twi::RESERVED3> {
        RegisterAddress::new(self.0 + 0x7c)
    }
    #[inline(always)]
    pub fn RESERVED4(&self) -> RegisterAddress<twi::RESERVED4> {
        RegisterAddress::new(self.0 + 0x88)
    }
    #[inline(always)]
    pub fn RESERVED5(&self) -> RegisterAddress<twi::RESERVED5> {
        RegisterAddress::new(self.0 + 0x8c)
    }
    #[inline(always)]
    pub fn RESERVED6(&self) -> RegisterAddress<twi::RESERVED6> {
        RegisterAddress::new(self.0 + 0x90)
    }
    #[inline(always)]
    pub fn RESERVED7(&self, index: u8) -> RegisterAddress<twi::RESERVED7> {
        assert!(index < 21);
        RegisterAddress::new(self.0 + 0xa0 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn RESERVED8(&self) -> RegisterAddress<twi::RESERVED8> {
        RegisterAddress::new(self.0 + 0x84)
    }
    #[inline(always)]
    pub fn RXFLR(&self) -> RegisterAddress<twi::RXFLR> {
        RegisterAddress::new(self.0 + 0x78)
    }
    #[inline(always)]
    pub fn RX_TL(&self) -> RegisterAddress<twi::RX_TL> {
        RegisterAddress::new(self.0 + 0x38)
    }
    #[inline(always)]
    pub fn SAR(&self) -> RegisterAddress<twi::SAR> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn SDA_SETUP(&self) -> RegisterAddress<twi::SDA_SETUP> {
        RegisterAddress::new(self.0 + 0x94)
    }
    #[inline(always)]
    pub fn SS_SCL_HCNT(&self) -> RegisterAddress<twi::SS_SCL_HCNT> {
        RegisterAddress::new(self.0 + 0x14)
    }
    #[inline(always)]
    pub fn SS_SCL_LCNT(&self) -> RegisterAddress<twi::SS_SCL_LCNT> {
        RegisterAddress::new(self.0 + 0x18)
    }
    #[inline(always)]
    pub fn STAT(&self) -> RegisterAddress<twi::STAT> {
        RegisterAddress::new(self.0 + 0x70)
    }
    #[inline(always)]
    pub fn TAR(&self) -> RegisterAddress<twi::TAR> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn TXFLR(&self) -> RegisterAddress<twi::TXFLR> {
        RegisterAddress::new(self.0 + 0x74)
    }
    #[inline(always)]
    pub fn TX_ABRT_SOURCE(&self) -> RegisterAddress<twi::TX_ABRT_SOURCE> {
        RegisterAddress::new(self.0 + 0x80)
    }
    #[inline(always)]
    pub fn TX_TL(&self) -> RegisterAddress<twi::TX_TL> {
        RegisterAddress::new(self.0 + 0x3c)
    }
}
