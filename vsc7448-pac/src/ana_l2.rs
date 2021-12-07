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
pub mod isdx;
pub mod lrn_limit;
pub mod port_limit;
pub mod sticky;
pub mod sticky_mask;

/// Common configurations for all ports
pub struct COMMON(pub(super) u32);
impl COMMON {
    pub fn AUTO_LRN_CFG(&self) -> RegisterAddress<common::AUTO_LRN_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn AUTO_LRN_CFG1(&self) -> RegisterAddress<common::AUTO_LRN_CFG1> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn FILTER_LOCAL_CTRL(&self) -> RegisterAddress<common::FILTER_LOCAL_CTRL> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn FILTER_LOCAL_CTRL1(&self) -> RegisterAddress<common::FILTER_LOCAL_CTRL1> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn FILTER_OTHER_CTRL(&self) -> RegisterAddress<common::FILTER_OTHER_CTRL> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn FWD_CFG(&self) -> RegisterAddress<common::FWD_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn INTR(&self) -> RegisterAddress<common::INTR> {
        RegisterAddress::new(self.0 + 0x158)
    }
    pub fn INTR_ENA(&self) -> RegisterAddress<common::INTR_ENA> {
        RegisterAddress::new(self.0 + 0x15c)
    }
    pub fn LRN_CFG(&self) -> RegisterAddress<common::LRN_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn LRN_COPY_CFG(&self) -> RegisterAddress<common::LRN_COPY_CFG> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    pub fn LRN_COPY_CFG1(&self) -> RegisterAddress<common::LRN_COPY_CFG1> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn LRN_SECUR_CFG(&self) -> RegisterAddress<common::LRN_SECUR_CFG> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn LRN_SECUR_CFG1(&self) -> RegisterAddress<common::LRN_SECUR_CFG1> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn LRN_SECUR_LOCKED_CFG(&self) -> RegisterAddress<common::LRN_SECUR_LOCKED_CFG> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn LRN_SECUR_LOCKED_CFG1(&self) -> RegisterAddress<common::LRN_SECUR_LOCKED_CFG1> {
        RegisterAddress::new(self.0 + 0x28)
    }
    pub fn MOVELOG_STICKY(&self) -> RegisterAddress<common::MOVELOG_STICKY> {
        RegisterAddress::new(self.0 + 0x14c)
    }
    pub fn MOVELOG_STICKY1(&self) -> RegisterAddress<common::MOVELOG_STICKY1> {
        RegisterAddress::new(self.0 + 0x150)
    }
    pub fn PORT_DLB_CFG(&self, index: u32) -> RegisterAddress<common::PORT_DLB_CFG> {
        assert!(index < 53);
        RegisterAddress::new(self.0 + 0x34 + index * 0x4)
    }
    pub fn SCAN_FID_CFG(&self, index: u32) -> RegisterAddress<common::SCAN_FID_CFG> {
        assert!(index < 16);
        RegisterAddress::new(self.0 + 0x10c + index * 0x4)
    }
    pub fn SCAN_FID_CTRL(&self) -> RegisterAddress<common::SCAN_FID_CTRL> {
        RegisterAddress::new(self.0 + 0x108)
    }
    pub fn VSTAX_CTRL(&self) -> RegisterAddress<common::VSTAX_CTRL> {
        RegisterAddress::new(self.0 + 0x154)
    }
}

/// Ingress service table configuration
pub struct ISDX(pub(super) u32);
impl ISDX {
    pub fn DLB_CFG(&self) -> RegisterAddress<isdx::DLB_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn DLB_COS_CFG(&self, index: u32) -> RegisterAddress<isdx::DLB_COS_CFG> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x18 + index * 0x4)
    }
    pub fn ISDX_BASE_CFG(&self) -> RegisterAddress<isdx::ISDX_BASE_CFG> {
        RegisterAddress::new(self.0 + 0x38)
    }
    pub fn MISC_CFG(&self) -> RegisterAddress<isdx::MISC_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn PORT_LIMIT_CTRL(&self) -> RegisterAddress<isdx::PORT_LIMIT_CTRL> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PORT_MASK_CFG(&self) -> RegisterAddress<isdx::PORT_MASK_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn PORT_MASK_CFG1(&self) -> RegisterAddress<isdx::PORT_MASK_CFG1> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn QGRP_CFG(&self) -> RegisterAddress<isdx::QGRP_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn SERVICE_CTRL(&self) -> RegisterAddress<isdx::SERVICE_CTRL> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// Learn limits per FID
pub struct LRN_LIMIT(pub(super) u32);
impl LRN_LIMIT {
    pub fn FID_LIMIT_STATUS(&self) -> RegisterAddress<lrn_limit::FID_LIMIT_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn INTR_IDENT(&self) -> RegisterAddress<lrn_limit::INTR_IDENT> {
        RegisterAddress::new(self.0 + 0x160)
    }
}

/// Learn limits per PORT and GLAG
pub struct PORT_LIMIT(pub(super) u32);
impl PORT_LIMIT {
    pub fn FID_LIMIT_CTRL(&self) -> RegisterAddress<port_limit::FID_LIMIT_CTRL> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PORT_LIMIT_STATUS(&self) -> RegisterAddress<port_limit::PORT_LIMIT_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Sticky diagnostic status
pub struct STICKY(pub(super) u32);
impl STICKY {
    pub fn ISDX_COS_CFG(&self, index: u32) -> RegisterAddress<sticky::ISDX_COS_CFG> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x3c + index * 0x4)
    }
}

/// Sticky diagnostic global port counter event configuration
pub struct STICKY_MASK(pub(super) u32);
impl STICKY_MASK {
    pub fn STICKY(&self) -> RegisterAddress<sticky_mask::STICKY> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn STICKY_MASK(&self) -> RegisterAddress<sticky_mask::STICKY_MASK> {
        RegisterAddress::new(self.0 + 0x0)
    }
}
