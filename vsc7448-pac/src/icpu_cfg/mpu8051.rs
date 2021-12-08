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

/// Register `MEMACC`
///
/// 8051 memory offset configuration
///
/// When loading (or examining) onchip 8051 memory, then it is only possible to move 32-bit words. This is why bits [17:16] and [1:0] of this register is not implemented. Setting START and STOP addresses determines how many words that are loaded (or examined). For example, when loading programs of less than 64KBytes, decreasing the stop address will speed up the load time. When manually loading or examining the onchip 8051 memory via an external CPU the data has to be put somewhere in SBA memory space on its way into or out-of the onchip 8051 memory, for this the 8 x 32-bit general purpose registers starting at 0x70000000 is a good choice. By using all (or some) of these registers it is possible to move up to 8 32-bit words to/from the onchip memory per access.
#[derive(From, Into)]
pub struct MEMACC(u32);
impl MEMACC {    ///
    /// Starting 32-bit word address when loading or examining the onchip 8051 memory.
    pub fn memacc_start(&self) -> u32 {
        (self.0 & 0xfffc) >> 2
    }
    pub fn set_memacc_start(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xfffc);
        self.0 &= !0xfffc;
        self.0 |= value;
    }    ///
    /// Ending 32-bit word address when loading or examining the onchip 8051 memory, the value of this field must be equal to or higher than the MEMACC.MEMACC_START field.
    pub fn memacc_stop(&self) -> u32 {
        (self.0 & 0x3ffff) >> 18
    }
    pub fn set_memacc_stop(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x3ffff);
        self.0 &= !0x3ffff;
        self.0 |= value;
    }
}

/// Register `MEMACC_CTRL`
///
/// 8051 memory load/examine configuration/status
#[derive(From, Into)]
pub struct MEMACC_CTRL(u32);
impl MEMACC_CTRL {    ///
    /// Set this field to start an access with the parameters specified by MEMACC_CTRL.MEMACC_EXAMINE, MEMACC.MEMACC_START, MEMACC.MEMACC_STOP, and MEMACC_SBA.MEMACC_SBA_START. This field is cleared when the requested number of 32-bit words has been transfered.
    pub fn memacc_do(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_memacc_do(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// This field controls if the onchip 8051 memory is either loaded (written) or examined (read).
    ///
    /// 0: Load data from SBA to onchip memory. 1: Examine data from onchip memory to SBA.
    pub fn memacc_examine(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_memacc_examine(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}

/// Register `MEMACC_SBA`
///
/// SBA memory offset configuration
///
/// There is no stop address in the SBA address space. The number of 32-bit words which is moved per access is determined by the MEMACC.MEMACC_START and MEMACC.MEMACC_STOP.
#[derive(From, Into)]
pub struct MEMACC_SBA(u32);
impl MEMACC_SBA {    ///
    /// This field determines where in the SBA memory space (32-bit alligned) the automatic load/examine mechanims reads/writes data to/from the onchip 8051 memory.
    pub fn memacc_sba_start(&self) -> u32 {
        (self.0 & 0x3) >> 2
    }
    pub fn set_memacc_sba_start(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}

/// Register `MPU8051_CFG`
///
/// 8051 configuration
#[derive(From, Into)]
pub struct MPU8051_CFG(u32);
impl MPU8051_CFG {    ///
    /// This field controls if the VCore UART or the 8051's internal UART is conencted to the chip IOs. The default, when the UART is always used. By clearing this field the 8051's internal UART will be connected to the chip IOs, this field only applies to an 8051 enabled system - clearing this field has no effect in a MIPS based VCore System.
    pub fn uart_sys_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_uart_sys_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `MPU8051_MMAP`
///
/// 8051 memory mapping mechanism
///
/// The MAP_* and MSADDR_* fields in this register is similar to the corresponding 8051 SFR register for control mapping the on-chip memory into the 8051 memory space. These fields must be used to configure 8051 memory mapping if the 8051 on-chip memory is loaded manually via an external processor. If the 8051 program itself does loading of on-chip memory then it must instead use the SFR equivalents.
#[derive(From, Into)]
pub struct MPU8051_MMAP(u32);
impl MPU8051_MMAP {    ///
    /// Set to map 8051 code-accesses in the high 32KByte memory range to on-chip memory instead of FLASH.
    pub fn map_code_high(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_map_code_high(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Set to map 8051 code-accesses in the low 32KByte memory range to on-chip memory instead of FLASH.
    pub fn map_code_low(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_map_code_low(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Set to map 8051 data-accesses in the high 32KByte memory range to on-chip memory instead of FLASH.
    pub fn map_data_high(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_map_data_high(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Set to map 8051 data-accesses in the low 32KByte memory range to on-chip memory instead of FLASH.
    pub fn map_data_low(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_map_data_low(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Configure which half of the on-chip memory an 8051 data-accesses in the low 32KByte memory range (when mapped to on-chip memory) actually use. When set to 0, the low half of the on-chip 64KByte is accessed, when set to 1 the high half is accessed.
    pub fn msaddr_code_high(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_msaddr_code_high(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }    ///
    /// Configure which half of the on-chip memory an 8051 code-accesses in the low 32KByte memory range (when mapped to on-chip memory) actually use. When set to 0, the low half of the on-chip 64KByte is accessed, when set to 1 the high half is accessed.
    pub fn msaddr_code_low(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_msaddr_code_low(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }    ///
    /// Configure which half of the on-chip memory an 8051 data-accesses in the high 32KByte memory range (when mapped to on-chip memory) actually use. When set to 0, the low half of the on-chip 64KByte is accessed, when set to 1 the high half is accessed.
    pub fn msaddr_data_high(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_msaddr_data_high(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }    ///
    /// Configure which half of the on-chip memory an 8051 data-accesses in the low 32KByte memory range (when mapped to on-chip memory) actually use. When set to 0, the low half of the on-chip 64KByte is accessed, when set to 1 the high half is accessed.
    pub fn msaddr_data_low(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_msaddr_data_low(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}

/// Register `MPU8051_STAT`
///
/// 8051 status
///
/// These read only fields can be used for debugging 8051 programs.
#[derive(From, Into)]
pub struct MPU8051_STAT(u32);
impl MPU8051_STAT {    ///
    /// A read-only copy of the 8051 GPR register at SFR address 0xF0.
    pub fn mpu8051_gpr(&self) -> u32 {
        (self.0 & 0xff) >> 0
    }
    pub fn set_mpu8051_gpr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }    ///
    /// This field is set if the 8051 get a shared bus error while fetching code or data from offchip memory, this field can only be cleared by a reset of the 8051.
    pub fn mpu8051_sba_err(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_mpu8051_sba_err(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }    ///
    /// Set when the 8051 has stopped itself by setting bit 2 in the PCON SFR register.
    pub fn mpu8051_stop(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_mpu8051_stop(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
}

/// Register `SW_MODE`
///
/// Manual control of the SPI interface
///
/// Note: There are 4 chip selects in total, but only chip select 0 is mapped to IO-pin (SI_nEn). The rest of the SPI chip selects are available as alternate functions on GPIOs, these must be enabled in the GPIO controller before they can be controlled via this register.
#[derive(From, Into)]
pub struct SW_MODE(u32);
impl SW_MODE {    ///
    /// Set to enable software pin control mode (Bit banging), when set software has direct control of the SPI interface. This mode is used for writing into flash.
    pub fn sw_pin_ctrl_mode(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_sw_pin_ctrl_mode(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }    ///
    /// Value to drive on SI_nEn outputs, each bit in this field maps to a corresponding chip-select (0 though 3). This field is only used if SW_MODE.SW_PIN_CTRL_MODE is set. Note: Chip selects 1 though 3 are available as alternate GPIO functions.
    pub fn sw_spi_cs(&self) -> u32 {
        (self.0 & 0x1e0) >> 5
    }
    pub fn set_sw_spi_cs(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x1e0);
        self.0 &= !0x1e0;
        self.0 |= value;
    }    ///
    /// This field has not effect, chip selects are always driven.
    pub fn sw_spi_cs_oe(&self) -> u32 {
        (self.0 & 0x1e) >> 1
    }
    pub fn set_sw_spi_cs_oe(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x1e);
        self.0 &= !0x1e;
        self.0 |= value;
    }    ///
    /// Value to drive on SI_Clk output. This field is only used if SW_MODE.SW_PIN_CTRL_MODE is set.
    pub fn sw_spi_sck(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_sw_spi_sck(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }    ///
    /// Set to enable drive of SI_Clk output. This field is only used if SW_MODE.SW_PIN_CTRL_MODE is set.
    pub fn sw_spi_sck_oe(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_sw_spi_sck_oe(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }    ///
    /// Current value of the SI_DI input.
    pub fn sw_spi_sdi(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_sw_spi_sdi(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Value to drive on SI_DO output. This field is only used if SW_MODE.SW_PIN_CTRL_MODE is set.
    pub fn sw_spi_sdo(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_sw_spi_sdo(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }    ///
    /// Set to enable drive of SI_DO output. This field is only used if SW_MODE.SW_PIN_CTRL_MODE is set.
    pub fn sw_spi_sdo_oe(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_sw_spi_sdo_oe(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
}