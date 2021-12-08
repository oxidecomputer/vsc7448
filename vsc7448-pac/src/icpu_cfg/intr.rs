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
use derive_more::{From, Into};

/// Register `DEV_INTR_BYPASS`
///
/// Device interrupt bypass enable
#[derive(From, Into)]
pub struct DEV_INTR_BYPASS(u32);
impl DEV_INTR_BYPASS {
    /// This register allows bypass of ICPU_CFG::DEV_INTR_STICKY for individual device interrupt sources. When a device interrupt source is in bypass mode then ICPU_CFG::DEV_INTR_RAW is used instead of ICPU_CFG::DEV_INTR_STICKY. See note on bypass in ICPU_CFG::INTR_BYPASS.
    pub fn dev_intr_bypass(&self) -> u32 {
        (self.0 & 0xffffffff) >> 0
    }
    pub fn set_dev_intr_bypass(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffffffff);
        self.0 &= !0xffffffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_BYPASS1`
///
/// Device interrupt bypass enable
#[derive(From, Into)]
pub struct DEV_INTR_BYPASS1(u32);
impl DEV_INTR_BYPASS1 {
    /// See ICPU_CFG::DEV_INTR_BYPASS for description, this register holds bits above 32.
    pub fn dev_intr_bypass1(&self) -> u32 {
        (self.0 & 0x1fffff) >> 0
    }
    pub fn set_dev_intr_bypass1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_ENA`
///
/// Device interrupt enable
#[derive(From, Into)]
pub struct DEV_INTR_ENA(u32);
impl DEV_INTR_ENA {
    /// Set to enable propagation of individual device interrupt sources to the main interrupt controller.
    pub fn dev_intr_ena(&self) -> u32 {
        (self.0 & 0xffffffff) >> 0
    }
    pub fn set_dev_intr_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffffffff);
        self.0 &= !0xffffffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_ENA1`
///
/// Device interrupt enable
#[derive(From, Into)]
pub struct DEV_INTR_ENA1(u32);
impl DEV_INTR_ENA1 {
    /// See ICPU_CFG::DEV_INTR_ENA for description, this register holds bits above 32.
    pub fn dev_intr_ena1(&self) -> u32 {
        (self.0 & 0x1fffff) >> 0
    }
    pub fn set_dev_intr_ena1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_IDENT`
///
/// Currently active device interrupts
#[derive(From, Into)]
pub struct DEV_INTR_IDENT(u32);
impl DEV_INTR_IDENT {
    /// Shows the currently active interrupt sources. For interrupt sources that are not bypassed this register is a result of AND'ing ICPU_CFG::DEV_INTR_STICKY with ICPU_CFG::DEV_INTR_ENA.
    pub fn dev_intr_ident(&self) -> u32 {
        (self.0 & 0xffffffff) >> 0
    }
    pub fn set_dev_intr_ident(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffffffff);
        self.0 &= !0xffffffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_POL`
///
/// Device interrupt polarity
#[derive(From, Into)]
pub struct DEV_INTR_POL(u32);
impl DEV_INTR_POL {
    /// Set individual bits in this register to configure polarity of the corresponding device interrupt.

    ///

    /// 0: Device interrupt is active low 1: Device interrupt is active high
    pub fn dev_intr_pol(&self) -> u32 {
        (self.0 & 0xffffffff) >> 0
    }
    pub fn set_dev_intr_pol(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffffffff);
        self.0 &= !0xffffffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_POL1`
///
/// Device interrupt polarity
#[derive(From, Into)]
pub struct DEV_INTR_POL1(u32);
impl DEV_INTR_POL1 {
    /// See ICPU_CFG::DEV_INTR_POL for description, this register holds bits above 32.
    pub fn dev_intr_pol1(&self) -> u32 {
        (self.0 & 0x1fffff) >> 0
    }
    pub fn set_dev_intr_pol1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_RAW`
///
/// Device interrupt raw status
#[derive(From, Into)]
pub struct DEV_INTR_RAW(u32);
impl DEV_INTR_RAW {
    /// Shows the current value of individual device interrupt sources. All sources are active high (sources have been corrected for polarity as configured in ICPU_CFG::DEV_INTR_POL).
    pub fn dev_intr_raw(&self) -> u32 {
        (self.0 & 0xffffffff) >> 0
    }
    pub fn set_dev_intr_raw(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffffffff);
        self.0 &= !0xffffffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_RAW1`
///
/// Device interrupt raw status
#[derive(From, Into)]
pub struct DEV_INTR_RAW1(u32);
impl DEV_INTR_RAW1 {
    /// See ICPU_CFG::DEV_INTR_RAW for description, this register holds bits above 32.
    pub fn dev_intr_raw1(&self) -> u32 {
        (self.0 & 0x1fffff) >> 0
    }
    pub fn set_dev_intr_raw1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_STICKY`
///
/// Device interrupt sticky status
#[derive(From, Into)]
pub struct DEV_INTR_STICKY(u32);
impl DEV_INTR_STICKY {
    /// This register is set based on device interrupt sourec events. See ICPU_CFG::DEV_INTR_TRIGGER for more information. Bits in this register remains set until cleared by software.
    pub fn dev_intr_sticky(&self) -> u32 {
        (self.0 & 0xffffffff) >> 0
    }
    pub fn set_dev_intr_sticky(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffffffff);
        self.0 &= !0xffffffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_STICKY1`
///
/// Device interrupt sticky status
#[derive(From, Into)]
pub struct DEV_INTR_STICKY1(u32);
impl DEV_INTR_STICKY1 {
    /// See ICPU_CFG::DEV_INTR_STICKY for description, this register holds bits above 32.
    pub fn dev_intr_sticky1(&self) -> u32 {
        (self.0 & 0x1fffff) >> 0
    }
    pub fn set_dev_intr_sticky1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_TRIGGER`
///
/// Device interrupt trigger mode
#[derive(From, Into)]
pub struct DEV_INTR_TRIGGER(u32);
impl DEV_INTR_TRIGGER {
    /// Configure trigger mode of individual device interrupt sources. The trigger mode determines how the value of the ICPU_CFG::DEV_INTR_RAW register is transfered to the ICPU_CFG::DEV_INTR_STICKY register. This register is replicated, the first replication controls bit 0 in the encoding, the second replication controls bit 1 in the encoding. I.e. to configure edge-triggered interrupt for device 3; set ICPU_CFG::DEV_INTR_TRIGGER[0][3]='1' and ICPU_CFG::DEV_INTR_TRIGGER[1][3]='0'. For level-triggered interrupts ICPU_CFG::DEV_INTR_STICKY is set for as long as the corresponding bit in ICPU_CFG::DEV_INTR_RAW is high - i.e. is not possible to clear a bit in ICPU_CFG::DEV_INTR_STICKY until the corresponding ICPU_CFG::DEV_INTR_RAW is zero. For edge-triggeded interrupts ICPU_CFG::DEV_INTR_STICKY is set when the corresponding bit in ICPU_CFG::DEV_INTR_RAW changes value. For falling-edge-triggeded interrupts ICPU_CFG::DEV_INTR_STICKY is set when the corresponding bit in ICPU_CFG::DEV_INTR_RAW changes from '1' to '0'. For rising-edge-triggeded interrupts ICPU_CFG::DEV_INTR_STICKY is set when the corresponding bit in ICPU_CFG::DEV_INTR_RAW changes from '0' to '1'.

    ///

    /// 0: Interrupt is level-activated 1: Interrupt is edge-triggered 2: Interrupt is falling-edge-triggered 3: Interrupt is rising-edge-triggered
    pub fn dev_intr_trigger(&self) -> u32 {
        (self.0 & 0xffffffff) >> 0
    }
    pub fn set_dev_intr_trigger(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffffffff);
        self.0 &= !0xffffffff;
        self.0 |= value;
    }
}

/// Register `DEV_INTR_TRIGGER1`
///
/// Device interrupt trigger mode
#[derive(From, Into)]
pub struct DEV_INTR_TRIGGER1(u32);
impl DEV_INTR_TRIGGER1 {
    /// See ICPU_CFG::DEV_INTR_TRIGGER for description, this register holds bits above 32.
    pub fn dev_intr_trigger1(&self) -> u32 {
        (self.0 & 0x1fffff) >> 0
    }
    pub fn set_dev_intr_trigger1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}

/// Register `DST_INTR_IDENT`
///
/// Currently active interrupt sources per destination
///
/// Replicated per destination interrupt.
#[derive(From, Into)]
pub struct DST_INTR_IDENT(u32);
impl DST_INTR_IDENT {
    /// Shows the currently active interrupt sources per destination interrupt. The contents of this register is equal to ICPU_CFG::INTR_IDENT AND'ed with the corresponding ICPU_CFG::DST_INTR_MAP. If any bit is set in this register the corresponding destination interrupt is asserted.
    pub fn dst_intr_ident(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_dst_intr_ident(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `DST_INTR_MAP`
///
/// Mapping of source to destination interrupts
///
/// Replicated per destination interrupt.
#[derive(From, Into)]
pub struct DST_INTR_MAP(u32);
impl DST_INTR_MAP {
    /// Set to enable mapping of individual interrupt sources to interrupt destinations. This register is replicated once for each destination interrupt.
    pub fn dst_intr_map(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_dst_intr_map(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `EXT_DST_INTR_DRV`
///
/// External interrupt destination output drive mode
#[derive(From, Into)]
pub struct EXT_DST_INTR_DRV(u32);
impl EXT_DST_INTR_DRV {
    /// This register configures drive mode of the corresponding external destination interrupt.

    ///

    /// 0: Only drive external interrupt output when asserted (tristate when inactive) 1: External interrupt output is always driven
    pub fn ext_dst_intr_drv(&self) -> u32 {
        (self.0 & 0x3) >> 0
    }
    pub fn set_ext_dst_intr_drv(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}

/// Register `EXT_DST_INTR_POL`
///
/// External destination interrupt polarity
#[derive(From, Into)]
pub struct EXT_DST_INTR_POL(u32);
impl EXT_DST_INTR_POL {
    /// Set individual bits in this register to configure polarity of the corresponding external destination interrupt.

    ///

    /// 0: External interrupt output is active low 1: External interrupt output is active high
    pub fn ext_dst_intr_pol(&self) -> u32 {
        (self.0 & 0x3) >> 0
    }
    pub fn set_ext_dst_intr_pol(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}

/// Register `EXT_SRC_INTR_POL`
///
/// External source interrupt polarity
#[derive(From, Into)]
pub struct EXT_SRC_INTR_POL(u32);
impl EXT_SRC_INTR_POL {
    /// Set individual bits in this register to configure polarity of the corresponding external source interrupt.

    ///

    /// 0: External interrupt input is active low 1: External interrupt input is active high
    pub fn ext_src_intr_pol(&self) -> u32 {
        (self.0 & 0x3) >> 0
    }
    pub fn set_ext_src_intr_pol(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}

/// Register `INTR_BYPASS`
///
/// Interrupt bypass enable
#[derive(From, Into)]
pub struct INTR_BYPASS(u32);
impl INTR_BYPASS {
    /// This register allows bypass of ICPU_CFG::INTR_STICKY for individual interrupt sources. When an interrupt source is in bypass mode then ICPU_CFG::INTR_RAW is used instead of ICPU_CFG::INTR_STICKY. Note: Enabling bypass does not make sense for all interrupt sources. It should only be used when the corresponding interrupt is sticky at the soruce. For example manual extraction data available interrupts can be configured to bypass, because the interrupt will remain asserted until the available data has been extracted. Note: The device interrupt is bypassed per default, "stickyness" is already implemented by ICPU_CFG::DEV_INTR_STICKY.
    pub fn intr_bypass(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_intr_bypass(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `INTR_ENA`
///
/// Interrupt enable
#[derive(From, Into)]
pub struct INTR_ENA(u32);
impl INTR_ENA {
    /// Set to enable propagation of individual interrupt sources to destinations. Atomic access to this register (needed in a multithreaded system) can be implemented by the ICPU_CFG::INTR_ENA_CLR and ICPU_CFG::INTR_ENA_SET registers.
    pub fn intr_ena(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_intr_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `INTR_ENA_CLR`
///
/// Atomic clear of interrupt enable
#[derive(From, Into)]
pub struct INTR_ENA_CLR(u32);
impl INTR_ENA_CLR {
    /// Set bit(s) in this register to clear the corresponding bits in ICPU_CFG::INTR_ENA. This register can be used for atomic access to ICPU_CFG::INTR_ENA register.
    pub fn intr_ena_clr(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_intr_ena_clr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `INTR_ENA_SET`
///
/// Atomic set of interrupt
#[derive(From, Into)]
pub struct INTR_ENA_SET(u32);
impl INTR_ENA_SET {
    /// Set bit(s) in this register to set the corresponding bits in ICPU_CFG::INTR_ENA. This register can be used for atomic access to ICPU_CFG::INTR_ENA register.
    pub fn intr_ena_set(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_intr_ena_set(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `INTR_FORCE`
///
/// Interrupt force sticky event
#[derive(From, Into)]
pub struct INTR_FORCE(u32);
impl INTR_FORCE {
    /// Set to force corresponding ICPU_CFG::INTR_STICKY bits. This field may be useful during development of software interrupt handler functions.
    pub fn intr_force(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_intr_force(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `INTR_IDENT`
///
/// Currently active interrupt sources
#[derive(From, Into)]
pub struct INTR_IDENT(u32);
impl INTR_IDENT {
    /// Shows the currently active interrupt sources. For interrupt sources that are not bypassed this register is a result of AND'ing ICPU_CFG::INTR_STICKY with ICPU_CFG::INTR_ENA.
    pub fn intr_ident(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_intr_ident(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `INTR_RAW`
///
/// Interrupt raw status
#[derive(From, Into)]
pub struct INTR_RAW(u32);
impl INTR_RAW {
    /// Shows the current value of individual interrupt sources. All sources are active high (the external interrupts has been corrected for polarity as configured in ICPU_CFG::EXT_SRC_INTR_POL).
    pub fn intr_raw(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_intr_raw(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `INTR_STICKY`
///
/// Interrupt sticky status
#[derive(From, Into)]
pub struct INTR_STICKY(u32);
impl INTR_STICKY {
    /// This register is set based on source interrupt events or by debug-force. See ICPU_CFG::INTR_TRIGGER and ICPU_CFG::INTR_FORCE for more information. Bits in this register remains set until cleared by software.
    pub fn intr_sticky(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_intr_sticky(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `INTR_TRIGGER`
///
/// Interrupt trigger mode
#[derive(From, Into)]
pub struct INTR_TRIGGER(u32);
impl INTR_TRIGGER {
    /// Configure trigger mode of individual interrupt sources. The trigger mode determines how the value of the ICPU_CFG::INTR_RAW register is transfered to the ICPU_CFG::INTR_STICKY register. This register is replicated, the first replication controls bit 0 in the encoding, the second replication controls bit 1 in the encoding. I.e. to configure falling-edge-triggered interrupt for interrupt source 5; set ICPU_CFG::INTR_TRIGGER[0][5]='0' and ICPU_CFG::INTR_TRIGGER[1][5]='1'. For level-triggered interrupts ICPU_CFG::INTR_STICKY is set for as long as the corresponding bit in ICPU_CFG::INTR_RAW is high - i.e. is not possible to clear a bit in ICPU_CFG::INTR_STICKY until the corresponding ICPU_CFG::INTR_RAW is zero. For edge-triggeded interrupts ICPU_CFG::INTR_STICKY is set when the corresponding bit in ICPU_CFG::INTR_RAW changes value. For falling-edge-triggeded interrupts ICPU_CFG::INTR_STICKY is set when the corresponding bit in ICPU_CFG::INTR_RAW changes from '1' to '0'. For rising-edge-triggeded interrupts ICPU_CFG::INTR_STICKY is set when the corresponding bit in ICPU_CFG::INTR_RAW changes from '0' to '1'.

    ///

    /// 0: Interrupt is level-activated 1: Interrupt is edge-triggered 2: Interrupt is falling-edge-triggered 3: Interrupt is rising-edge-triggered
    pub fn intr_trigger(&self) -> u32 {
        (self.0 & 0x1fffffff) >> 0
    }
    pub fn set_intr_trigger(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffffff);
        self.0 &= !0x1fffffff;
        self.0 |= value;
    }
}

/// Register `MPU8051_IROM`
///
/// 8051 ROM configuration
#[derive(From, Into)]
pub struct MPU8051_IROM(u32);
impl MPU8051_IROM {
    /// This Field specifies the offset into AHB space from which the 8051 must fetch its IROM code during firmware startup.
    pub fn rom_offset(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_rom_offset(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
