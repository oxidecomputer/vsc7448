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

pub mod qlimit_mon;
pub mod qlimit_port;
pub mod qlimit_queue;
pub mod qlimit_se;
pub mod qlimit_shr;
pub mod qmap_qos_tbl;
pub mod qmap_se_tbl;
pub mod qmap_vport_tbl;
pub mod stat;
pub mod system;

/// Shared memory pool monitoring
pub struct QLIMIT_MON(pub(super) u32);
impl QLIMIT_MON {
    pub fn QLIMIT_CONG_CNT_MAX_STAT(&self) -> RegisterAddress<qlimit_mon::QLIMIT_CONG_CNT_MAX_STAT> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn QLIMIT_MON_CFG(&self) -> RegisterAddress<qlimit_mon::QLIMIT_MON_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn QLIMIT_SHR_WM_STAT(&self, index: u32) -> RegisterAddress<qlimit_mon::QLIMIT_SHR_WM_STAT> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x3c + index * 0x4)
    }
}

/// Queue Limitation Configuration
pub struct QLIMIT_PORT(pub(super) u32);
impl QLIMIT_PORT {
    pub fn QLIMIT_CONG_CNT(&self) -> RegisterAddress<qlimit_port::QLIMIT_CONG_CNT> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn QLIMIT_PORT_CFG(&self) -> RegisterAddress<qlimit_port::QLIMIT_PORT_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Queue Size Table
pub struct QLIMIT_QUEUE(pub(super) u32);
impl QLIMIT_QUEUE {
    pub fn DROP_STAT_CTRL(&self) -> RegisterAddress<qlimit_queue::DROP_STAT_CTRL> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// SE Size Table
pub struct QLIMIT_SE(pub(super) u32);
impl QLIMIT_SE {
    pub fn QLIMIT_SE_USE(&self) -> RegisterAddress<qlimit_se::QLIMIT_SE_USE> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn QUEUE_SIZE(&self) -> RegisterAddress<qlimit_se::QUEUE_SIZE> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Shared memory pool configuration
pub struct QLIMIT_SHR(pub(super) u32);
impl QLIMIT_SHR {
    pub fn QLIMIT_CONG_CNT_STAT(&self) -> RegisterAddress<qlimit_shr::QLIMIT_CONG_CNT_STAT> {
        RegisterAddress::new(self.0 + 0x34)
    }
    pub fn QLIMIT_DIS_CFG(&self) -> RegisterAddress<qlimit_shr::QLIMIT_DIS_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn QLIMIT_QUE_ACT_CFG(&self) -> RegisterAddress<qlimit_shr::QLIMIT_QUE_ACT_CFG> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    pub fn QLIMIT_QUE_CONG_CFG(&self) -> RegisterAddress<qlimit_shr::QLIMIT_QUE_CONG_CFG> {
        RegisterAddress::new(self.0 + 0x28)
    }
    pub fn QLIMIT_SE_CONG_CFG(&self) -> RegisterAddress<qlimit_shr::QLIMIT_SE_CONG_CFG> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn QLIMIT_SHR_ATOP_CFG(&self, index: u32) -> RegisterAddress<qlimit_shr::QLIMIT_SHR_ATOP_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x8 + index * 0x4)
    }
    pub fn QLIMIT_SHR_CTOP_CFG(&self, index: u32) -> RegisterAddress<qlimit_shr::QLIMIT_SHR_CTOP_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x10 + index * 0x4)
    }
    pub fn QLIMIT_SHR_FILL_STAT(&self) -> RegisterAddress<qlimit_shr::QLIMIT_SHR_FILL_STAT> {
        RegisterAddress::new(self.0 + 0x38)
    }
    pub fn QLIMIT_SHR_QDIV_CFG(&self, index: u32) -> RegisterAddress<qlimit_shr::QLIMIT_SHR_QDIV_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x20 + index * 0x4)
    }
    pub fn QLIMIT_SHR_QLIM_CFG(&self, index: u32) -> RegisterAddress<qlimit_shr::QLIMIT_SHR_QLIM_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x18 + index * 0x4)
    }
    pub fn QLIMIT_SHR_TOP_CFG(&self, index: u32) -> RegisterAddress<qlimit_shr::QLIMIT_SHR_TOP_CFG> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
}

/// Select scheduling modes
pub struct QMAP_QOS_TBL(pub(super) u32);
impl QMAP_QOS_TBL {
    pub fn QMAP_QOS_TBL(&self) -> RegisterAddress<qmap_qos_tbl::QMAP_QOS_TBL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn QMAP_SE_TBL(&self) -> RegisterAddress<qmap_qos_tbl::QMAP_SE_TBL> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Select scheduling modes
pub struct QMAP_SE_TBL(pub(super) u32);
impl QMAP_SE_TBL {
    pub fn QMAP_VPORT_TBL(&self, index: u32) -> RegisterAddress<qmap_se_tbl::QMAP_VPORT_TBL> {
        assert!(index < 53);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
}

/// Mapping into virtual ports
pub struct QMAP_VPORT_TBL(pub(super) u32);
impl QMAP_VPORT_TBL {
    pub fn STAT_CFG(&self) -> RegisterAddress<qmap_vport_tbl::STAT_CFG> {
        RegisterAddress::new(self.0 + 0x1d4)
    }
}

/// Frame statistics
pub struct STAT(pub(super) u32);
impl STAT {
    pub fn CNT(&self) -> RegisterAddress<stat::CNT> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn QLIMIT_SHR_FILL_MAX_STAT(&self) -> RegisterAddress<stat::QLIMIT_SHR_FILL_MAX_STAT> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// Switch configuration
pub struct SYSTEM(pub(super) u32);
impl SYSTEM {
    pub fn FWD_CPU_DROP_CNT(&self) -> RegisterAddress<system::FWD_CPU_DROP_CNT> {
        RegisterAddress::new(self.0 + 0xfc)
    }
    pub fn FWD_CTRL(&self) -> RegisterAddress<system::FWD_CTRL> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn FWD_DROP_EVENTS(&self, index: u32) -> RegisterAddress<system::FWD_DROP_EVENTS> {
        assert!(index < 57);
        RegisterAddress::new(self.0 + 0xc + index * 0x4)
    }
    pub fn FWD_STAT_CNT(&self, index: u32) -> RegisterAddress<system::FWD_STAT_CNT> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0xf0 + index * 0x4)
    }
    pub fn MAP_CFG_CFG(&self) -> RegisterAddress<system::MAP_CFG_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn QMAP_PORT_MODE(&self, index: u32) -> RegisterAddress<system::QMAP_PORT_MODE> {
        assert!(index < 53);
        RegisterAddress::new(self.0 + 0x100 + index * 0x4)
    }
    pub fn STAT_CNT_CFG(&self) -> RegisterAddress<system::STAT_CNT_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}