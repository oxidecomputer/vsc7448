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

pub mod chip_regs;
pub mod fan_ctrl;
pub mod gpio;
pub mod memitgr;
pub mod miim;
pub mod miim_read_scan;
pub mod miim_slave;
pub mod sio_ctrl;
pub mod sw_regs;
pub mod temp_sensor;
pub mod vcore_access;
pub mod vrap;

/// Not documented
pub struct CHIP_REGS(pub(super) u32);
impl CHIP_REGS {
    pub fn CHIP_ID(&self) -> RegisterAddress<chip_regs::CHIP_ID> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn FEA_STAT(&self) -> RegisterAddress<chip_regs::FEA_STAT> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn GPR(&self) -> RegisterAddress<chip_regs::GPR> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn HW_CFG(&self) -> RegisterAddress<chip_regs::HW_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn HW_SGPIO_SD_CFG(&self) -> RegisterAddress<chip_regs::HW_SGPIO_SD_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn HW_STAT(&self) -> RegisterAddress<chip_regs::HW_STAT> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn SOFT_RST(&self) -> RegisterAddress<chip_regs::SOFT_RST> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// Fan controller configuration and status
pub struct FAN_CTRL(pub(super) u32);
impl FAN_CTRL {
    pub fn FAN_CFG(&self) -> RegisterAddress<fan_ctrl::FAN_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SIO_INTR_IDENT(&self, index: u32) -> RegisterAddress<fan_ctrl::SIO_INTR_IDENT> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0xfc + index * 0x4)
    }
}

/// Not documented
pub struct GPIO(pub(super) u32);
impl GPIO {
    pub fn GPIO_ALT(&self, index: u32) -> RegisterAddress<gpio::GPIO_ALT> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x40 + index * 0x4)
    }
    pub fn GPIO_ALT1(&self, index: u32) -> RegisterAddress<gpio::GPIO_ALT1> {
        assert!(index < 2);
        RegisterAddress::new(self.0 + 0x48 + index * 0x4)
    }
    pub fn GPIO_IN(&self) -> RegisterAddress<gpio::GPIO_IN> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn GPIO_IN1(&self) -> RegisterAddress<gpio::GPIO_IN1> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn GPIO_INTR(&self) -> RegisterAddress<gpio::GPIO_INTR> {
        RegisterAddress::new(self.0 + 0x28)
    }
    pub fn GPIO_INTR1(&self) -> RegisterAddress<gpio::GPIO_INTR1> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    pub fn GPIO_INTR_ENA(&self) -> RegisterAddress<gpio::GPIO_INTR_ENA> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn GPIO_INTR_ENA1(&self) -> RegisterAddress<gpio::GPIO_INTR_ENA1> {
        RegisterAddress::new(self.0 + 0x34)
    }
    pub fn GPIO_INTR_IDENT(&self) -> RegisterAddress<gpio::GPIO_INTR_IDENT> {
        RegisterAddress::new(self.0 + 0x38)
    }
    pub fn GPIO_INTR_IDENT1(&self) -> RegisterAddress<gpio::GPIO_INTR_IDENT1> {
        RegisterAddress::new(self.0 + 0x3c)
    }
    pub fn GPIO_OE(&self) -> RegisterAddress<gpio::GPIO_OE> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn GPIO_OE1(&self) -> RegisterAddress<gpio::GPIO_OE1> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn GPIO_OUT(&self) -> RegisterAddress<gpio::GPIO_OUT> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn GPIO_OUT1(&self) -> RegisterAddress<gpio::GPIO_OUT1> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn GPIO_OUT_CLR(&self) -> RegisterAddress<gpio::GPIO_OUT_CLR> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn GPIO_OUT_CLR1(&self) -> RegisterAddress<gpio::GPIO_OUT_CLR1> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn GPIO_OUT_SET(&self) -> RegisterAddress<gpio::GPIO_OUT_SET> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn GPIO_OUT_SET1(&self) -> RegisterAddress<gpio::GPIO_OUT_SET1> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn VA_DATA_INERT(&self) -> RegisterAddress<gpio::VA_DATA_INERT> {
        RegisterAddress::new(self.0 + 0x10)
    }
}

/// Memory integrity monitor
pub struct MEMITGR(pub(super) u32);
impl MEMITGR {
    pub fn FAN_CNT(&self) -> RegisterAddress<memitgr::FAN_CNT> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn MEMITGR_CTRL(&self) -> RegisterAddress<memitgr::MEMITGR_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn MEMITGR_DIV(&self) -> RegisterAddress<memitgr::MEMITGR_DIV> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn MEMITGR_IDX(&self) -> RegisterAddress<memitgr::MEMITGR_IDX> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn MEMITGR_INFO(&self) -> RegisterAddress<memitgr::MEMITGR_INFO> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn MEMITGR_STAT(&self) -> RegisterAddress<memitgr::MEMITGR_STAT> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// Not documented
pub struct MIIM(pub(super) u32);
impl MIIM {
    pub fn GPIO_SD_MAP(&self, index: u32) -> RegisterAddress<miim::GPIO_SD_MAP> {
        assert!(index < 16);
        RegisterAddress::new(self.0 + 0x50 + index * 0x4)
    }
    pub fn MII_CFG(&self) -> RegisterAddress<miim::MII_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn MII_CFG_7226(&self) -> RegisterAddress<miim::MII_CFG_7226> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn MII_CMD(&self) -> RegisterAddress<miim::MII_CMD> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn MII_DATA(&self) -> RegisterAddress<miim::MII_DATA> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn MII_SCAN_0(&self) -> RegisterAddress<miim::MII_SCAN_0> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn MII_SCAN_1(&self) -> RegisterAddress<miim::MII_SCAN_1> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn MII_SCAN_LAST_RSLTS(&self) -> RegisterAddress<miim::MII_SCAN_LAST_RSLTS> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn MII_STATUS(&self) -> RegisterAddress<miim::MII_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Not documented
pub struct MIIM_READ_SCAN(pub(super) u32);
impl MIIM_READ_SCAN {
    pub fn MII_SCAN_LAST_RSLTS_VLD(&self) -> RegisterAddress<miim_read_scan::MII_SCAN_LAST_RSLTS_VLD> {
        RegisterAddress::new(self.0 + 0x20)
    }
}

/// Not documented
pub struct MIIM_SLAVE(pub(super) u32);
impl MIIM_SLAVE {
    pub fn TEMP_SENSOR_STAT(&self) -> RegisterAddress<miim_slave::TEMP_SENSOR_STAT> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// Serial IO control configuration
pub struct SIO_CTRL(pub(super) u32);
impl SIO_CTRL {
    pub fn MIIM_SLAVE_CFG(&self) -> RegisterAddress<sio_ctrl::MIIM_SLAVE_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SIO_CFG(&self) -> RegisterAddress<sio_ctrl::SIO_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn SIO_CLOCK(&self) -> RegisterAddress<sio_ctrl::SIO_CLOCK> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn SIO_INPUT_DATA(&self, index: u32) -> RegisterAddress<sio_ctrl::SIO_INPUT_DATA> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
    pub fn SIO_INTR(&self, index: u32) -> RegisterAddress<sio_ctrl::SIO_INTR> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0xe8 + index * 0x4)
    }
    pub fn SIO_INTR_ENA(&self) -> RegisterAddress<sio_ctrl::SIO_INTR_ENA> {
        RegisterAddress::new(self.0 + 0xf8)
    }
    pub fn SIO_INTR_POL(&self, index: u32) -> RegisterAddress<sio_ctrl::SIO_INTR_POL> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0xa8 + index * 0x4)
    }
    pub fn SIO_INTR_RAW(&self, index: u32) -> RegisterAddress<sio_ctrl::SIO_INTR_RAW> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0xb8 + index * 0x4)
    }
    pub fn SIO_INTR_TRIGGER0(&self, index: u32) -> RegisterAddress<sio_ctrl::SIO_INTR_TRIGGER0> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0xc8 + index * 0x4)
    }
    pub fn SIO_INTR_TRIGGER1(&self, index: u32) -> RegisterAddress<sio_ctrl::SIO_INTR_TRIGGER1> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0xd8 + index * 0x4)
    }
    pub fn SIO_PORT_CFG(&self, index: u32) -> RegisterAddress<sio_ctrl::SIO_PORT_CFG> {
        assert!(index < 32);
        RegisterAddress::new(self.0 + 0x18 + index * 0x4)
    }
    pub fn SIO_PORT_ENA(&self) -> RegisterAddress<sio_ctrl::SIO_PORT_ENA> {
        RegisterAddress::new(self.0 + 0x98)
    }
    pub fn SIO_PWM_CFG(&self, index: u32) -> RegisterAddress<sio_ctrl::SIO_PWM_CFG> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x9c + index * 0x4)
    }
}

/// Registers for software/software interaction
pub struct SW_REGS(pub(super) u32);
impl SW_REGS {
    pub fn FEA_DIS(&self) -> RegisterAddress<sw_regs::FEA_DIS> {
        RegisterAddress::new(self.0 + 0x1c)
    }
}

/// Temperature sensor control
pub struct TEMP_SENSOR(pub(super) u32);
impl TEMP_SENSOR {
    pub fn MII_SCAN_RSLTS_STICKY(&self, index: u32) -> RegisterAddress<temp_sensor::MII_SCAN_RSLTS_STICKY> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
    pub fn TEMP_SENSOR_CFG(&self) -> RegisterAddress<temp_sensor::TEMP_SENSOR_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn TEMP_SENSOR_CTRL(&self) -> RegisterAddress<temp_sensor::TEMP_SENSOR_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Registers for accessing the VCore
pub struct VCORE_ACCESS(pub(super) u32);
impl VCORE_ACCESS {
    pub fn SW_INTR(&self) -> RegisterAddress<vcore_access::SW_INTR> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn VA_ADDR(&self) -> RegisterAddress<vcore_access::VA_ADDR> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn VA_CTRL(&self) -> RegisterAddress<vcore_access::VA_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn VA_DATA(&self) -> RegisterAddress<vcore_access::VA_DATA> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn VA_DATA_INCR(&self) -> RegisterAddress<vcore_access::VA_DATA_INCR> {
        RegisterAddress::new(self.0 + 0xc)
    }
}

/// VRAP controller
pub struct VRAP(pub(super) u32);
impl VRAP {
    pub fn MEMITGR_DBG(&self) -> RegisterAddress<vrap::MEMITGR_DBG> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn VRAP_ACCESS_STAT(&self) -> RegisterAddress<vrap::VRAP_ACCESS_STAT> {
        RegisterAddress::new(self.0 + 0x0)
    }
}