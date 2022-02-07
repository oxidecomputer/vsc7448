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
/// 8051 memory offset configuration
///
/// When loading (or examining) onchip 8051 memory, then it is only possible to move 32-bit words. This is why bits [17:16] and [1:0] of this register is not implemented. Setting START and STOP addresses determines how many words that are loaded (or examined). For example, when loading programs of less than 64KBytes, decreasing the stop address will speed up the load time. When manually loading or examining the onchip 8051 memory via an external CPU the data has to be put somewhere in SBA memory space on its way into or out-of the onchip 8051 memory, for this the 8 x 32-bit general purpose registers starting at 0x70000000 is a good choice. By using all (or some) of these registers it is possible to move up to 8 32-bit words to/from the onchip memory per access.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MEMACC(u32);
impl MEMACC {
    /// Starting 32-bit word address when loading or examining the onchip 8051 memory.
    #[inline(always)]
    pub fn memacc_start(&self) -> u32 {
        (self.0 & 0xfffc) >> 2
    }
    #[inline(always)]
    pub fn set_memacc_start(&mut self, value: u32) {
        assert!(value <= 0x3fff);
        let value = value << 2;
        self.0 &= !0xfffc;
        self.0 |= value;
    }
    /// Ending 32-bit word address when loading or examining the onchip 8051 memory, the value of this field must be equal to or higher than the MEMACC.MEMACC_START field.
    #[inline(always)]
    pub fn memacc_stop(&self) -> u32 {
        (self.0 & 0xfffc0000) >> 18
    }
    #[inline(always)]
    pub fn set_memacc_stop(&mut self, value: u32) {
        assert!(value <= 0x3fff);
        let value = value << 18;
        self.0 &= !0xfffc0000;
        self.0 |= value;
    }
}
/// 8051 memory load/examine configuration/status
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MEMACC_CTRL(u32);
impl MEMACC_CTRL {
    /// Set this field to start an access with the parameters specified by MEMACC_CTRL.MEMACC_EXAMINE, MEMACC.MEMACC_START, MEMACC.MEMACC_STOP, and MEMACC_SBA.MEMACC_SBA_START. This field is cleared when the requested number of 32-bit words has been transfered.
    #[inline(always)]
    pub fn memacc_do(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_memacc_do(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// This field controls if the onchip 8051 memory is either loaded (written) or examined (read).
    ///
    /// 0: Load data from SBA to onchip memory. 1: Examine data from onchip memory to SBA.
    #[inline(always)]
    pub fn memacc_examine(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_memacc_examine(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// SBA memory offset configuration
///
/// There is no stop address in the SBA address space. The number of 32-bit words which is moved per access is determined by the MEMACC.MEMACC_START and MEMACC.MEMACC_STOP.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MEMACC_SBA(u32);
impl MEMACC_SBA {
    /// This field determines where in the SBA memory space (32-bit alligned) the automatic load/examine mechanims reads/writes data to/from the onchip 8051 memory.
    #[inline(always)]
    pub fn memacc_sba_start(&self) -> u32 {
        (self.0 & 0xfffffffc) >> 2
    }
    #[inline(always)]
    pub fn set_memacc_sba_start(&mut self, value: u32) {
        assert!(value <= 0x3fffffff);
        let value = value << 2;
        self.0 &= !0xfffffffc;
        self.0 |= value;
    }
}
/// 8051 configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MPU8051_CFG(u32);
impl MPU8051_CFG {
    /// This field controls if the VCore UART or the 8051's internal UART is conencted to the chip IOs. The default, when the UART is always used. By clearing this field the 8051's internal UART will be connected to the chip IOs, this field only applies to an 8051 enabled system - clearing this field has no effect in a MIPS based VCore System.
    #[inline(always)]
    pub fn uart_sys_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_uart_sys_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// 8051 ROM configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MPU8051_IROM(u32);
impl MPU8051_IROM {
    /// This Field specifies the offset into AHB space from which the 8051 must fetch its IROM code during firmware startup.
    #[inline(always)]
    pub fn rom_offset(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    #[inline(always)]
    pub fn set_rom_offset(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 16;
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
/// 8051 memory mapping mechanism
///
/// The MAP_* and MSADDR_* fields in this register is similar to the corresponding 8051 SFR register for control mapping the on-chip memory into the 8051 memory space. These fields must be used to configure 8051 memory mapping if the 8051 on-chip memory is loaded manually via an external processor. If the 8051 program itself does loading of on-chip memory then it must instead use the SFR equivalents.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MPU8051_MMAP(u32);
impl MPU8051_MMAP {
    /// Set to map 8051 code-accesses in the high 32KByte memory range to on-chip memory instead of FLASH.
    #[inline(always)]
    pub fn map_code_high(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_map_code_high(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set to map 8051 code-accesses in the low 32KByte memory range to on-chip memory instead of FLASH.
    #[inline(always)]
    pub fn map_code_low(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_map_code_low(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set to map 8051 data-accesses in the high 32KByte memory range to on-chip memory instead of FLASH.
    #[inline(always)]
    pub fn map_data_high(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_map_data_high(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set to map 8051 data-accesses in the low 32KByte memory range to on-chip memory instead of FLASH.
    #[inline(always)]
    pub fn map_data_low(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_map_data_low(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Configure which half of the on-chip memory an 8051 data-accesses in the low 32KByte memory range (when mapped to on-chip memory) actually use. When set to 0, the low half of the on-chip 64KByte is accessed, when set to 1 the high half is accessed.
    #[inline(always)]
    pub fn msaddr_code_high(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_msaddr_code_high(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Configure which half of the on-chip memory an 8051 code-accesses in the low 32KByte memory range (when mapped to on-chip memory) actually use. When set to 0, the low half of the on-chip 64KByte is accessed, when set to 1 the high half is accessed.
    #[inline(always)]
    pub fn msaddr_code_low(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_msaddr_code_low(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Configure which half of the on-chip memory an 8051 data-accesses in the high 32KByte memory range (when mapped to on-chip memory) actually use. When set to 0, the low half of the on-chip 64KByte is accessed, when set to 1 the high half is accessed.
    #[inline(always)]
    pub fn msaddr_data_high(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_msaddr_data_high(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Configure which half of the on-chip memory an 8051 data-accesses in the low 32KByte memory range (when mapped to on-chip memory) actually use. When set to 0, the low half of the on-chip 64KByte is accessed, when set to 1 the high half is accessed.
    #[inline(always)]
    pub fn msaddr_data_low(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_msaddr_data_low(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
}
/// 8051 status
///
/// These read only fields can be used for debugging 8051 programs.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MPU8051_STAT(u32);
impl MPU8051_STAT {
    /// A read-only copy of the 8051 GPR register at SFR address 0xF0.
    #[inline(always)]
    pub fn mpu8051_gpr(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_mpu8051_gpr(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// This field is set if the 8051 get a shared bus error while fetching code or data from offchip memory, this field can only be cleared by a reset of the 8051.
    #[inline(always)]
    pub fn mpu8051_sba_err(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_mpu8051_sba_err(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Set when the 8051 has stopped itself by setting bit 2 in the PCON SFR register.
    #[inline(always)]
    pub fn mpu8051_stop(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_mpu8051_stop(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
}
