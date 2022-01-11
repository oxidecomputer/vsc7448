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
/// Address register for VCore accesses
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VA_ADDR(u32);
impl VA_ADDR {
    /// The address to access in the VCore domain, all addresses must be 32-bit alligned (i.e. the two least significant bit must always be 0). When accesses are initiated using the VA_DATA_INCR register, then this field is autoincremented by 4 at the end of the transfer. The memory region of the VCore that maps to switch-core registers may not be accessed by using these registers.
    #[inline(always)]
    pub fn va_addr(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_va_addr(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Control register for VCore accesses
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VA_CTRL(u32);
impl VA_CTRL {
    /// This field is set by hardware when an access into VCore domain is started, and cleared when the access is done.
    #[inline(always)]
    pub fn va_busy(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_va_busy(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// This field is set to the value of VA_CTRL.VA_BUSY whenever one of the data registers VA_DATA, VA_DATA_INCR, or VA_DATA_RO is read. By examining this field it is possible to determine if VA_BUSY was set at the time a read from one of these registers was performed.
    #[inline(always)]
    pub fn va_busy_rd(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_va_busy_rd(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// If the VCore access logic detects an error this field is set based on the nature of the error. This is a read-only field which is cleared by the VCore access logic when a new access is (sucessfully) accepted.
    ///
    /// 0: No errors detected. 1: SBA not ready when accessed. 2: SBA reported error. 3: DATA or ADDR written during active access.
    #[inline(always)]
    pub fn va_err(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    #[inline(always)]
    pub fn set_va_err(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 2;
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Controls the size of the access inside VCore domain. It is possible to do 32-bit, 16-bit and 8-bit accesses. For 8bit and 16bit write-accesses data must be aligned appropriately inside the 32bit write-data word (i.e. for a byte-write to address 0x20001003 data has to be placed in [31:24]). Likewise for 8bit and 16bit read operations, here data is alligned accordingly to address.
    ///
    /// 0: 32bit 1: Reserved, do not use 2: 8bit 3: 16bit
    #[inline(always)]
    pub fn va_size(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    #[inline(always)]
    pub fn set_va_size(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 4;
        self.0 &= !0x30;
        self.0 |= value;
    }
}
/// Data register for VCore accesses
///
/// The VA_DATA, VA_DATA_INCR, and VA_DATA_INERT registers are used for indirect access into the VCore domain. The functionality of the VA_DATA_INCR and VA_DATA_INERT registers are similar to this register - but with minor exceptions. These exceptions are fleshed out in the description of the respective registers.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VA_DATA(u32);
impl VA_DATA {
    /// Reading or writing from/to this field initiates accesses into the VCore domain. While an access is ongoing (VA_CTRL.VA_BUSY is set) this field may not be written. It is possible to read this field while an access is ongoing, but the data returned will be 0x80000000. When writing to this field; a write into the VCore domain is initiated to the address specified in the VA_ADDR register, with the data that was written to this field. Only 32-bit writes are supported. This field may not be written to untill the VA_CTRL.VA_BUSY indicates that no accesses is ongoing. When reading from this field; a read from the VCore domain is initiated from the address specified in the VA_ADDR register. Important: The data that is returned from reading this field (and stating an access) is not the result of the newly initiated read, instead the data from the last access is returned. The result of the newly initiated read access will be ready once the VA_CTRL.VA_BUSY field shows that the access is done. Note: When the result of a read-access is read from this field (the second read), a new access will automatically be intiated. This is desirable when reading a series of addresses from VCore domain. If a new access is not desirable, then the result should be read from the VA_DATA_INERT register instead of this field!
    #[inline(always)]
    pub fn va_data(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_va_data(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Data register for VCore accesses (w. auto increment of address)
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VA_DATA_INCR(u32);
impl VA_DATA_INCR {
    /// This field behaves in the same way as VA_DATA.VA_DATA. Except when an access is initiated by using this field (either read or write); the address register (VA_ADDR) is automatically incremented by 4 at the end of the access, i.e. when VA_CTRL.VA_BUSY is deasserted.
    #[inline(always)]
    pub fn va_data_incr(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_va_data_incr(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Data register for VCore accesses (will not initiate access)
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VA_DATA_INERT(u32);
impl VA_DATA_INERT {
    /// This field behaves in the same way as VA_DATA.VA_DATA. Except accesses (read or write) does not initiate VCore accesses. Writing to this register just overwrites the value currently held by all of the data registers (VA_DATA, VA_DATA_INCR, and VA_DATA_INERT).
    #[inline(always)]
    pub fn va_data_inert(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_va_data_inert(&mut self, value: u32) {
        self.0 = value;
    }
}
