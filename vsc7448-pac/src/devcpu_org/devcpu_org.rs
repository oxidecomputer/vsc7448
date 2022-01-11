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
/// Error counters
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ERR_CNTS(u32);
impl ERR_CNTS {
    /// The "No Action" indication is triggered when a target is accessed with a non-existing address. In other words, the target did not contain a register at the requested address.
    pub fn err_no_action(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    pub fn set_err_no_action(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 4;
        self.0 &= !0xf0;
        self.0 |= value;
    }
    /// The "Target Busy" indication is triggered when an interface tries to access a target which is currently reset or if another interface is using the target.
    pub fn err_tgt_busy(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    pub fn set_err_tgt_busy(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 16;
        self.0 &= !0xf0000;
        self.0 |= value;
    }
    /// The "Unknown Target Module" indication is triggered when a non-existing target is requested. In other words there was no target with the requested target-id.
    pub fn err_utm(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_err_utm(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// The "Watch Dog Drop" indication is triggered when a target is too long about processing a request. Usually requests are processed immediately but some accesses requires interaction with the core-logic, when this logic is in reset or during very heavy traffic load there is a chance of timing out in the target.
    pub fn err_wd_drop(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    pub fn set_err_wd_drop(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 8;
        self.0 &= !0xf00;
        self.0 |= value;
    }
    /// The "Watch Dog Drop Origin" indication is triggered when the origin does not receive reply from the CSR ring within a given amount of time. This cannot happen during normal operation.
    pub fn err_wd_drop_org(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    pub fn set_err_wd_drop_org(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 12;
        self.0 &= !0xf000;
        self.0 |= value;
    }
}
/// General purpose register
///
/// This register is shared between all interfaces on the Origin.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPR(u32);
impl GPR {
    /// General purpose 32-bit registers for debug and software development. The contents of this register can always (safely) be read. However write operations from different masters (to this register), which occur at (exactly) the same time, will fail.
    pub fn gpr(&self) -> u32 {
        self.0
    }
    pub fn set_gpr(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Physical interface configuration and status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct IF_CFGSTAT(u32);
impl IF_CFGSTAT {
    /// SI interface: This is the number of padding bytes to insert before read-data is shifted out of the device.This is needed when using high serial interface frequencies.
    pub fn if_cfg(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_if_cfg(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Interface number; software can use this field to determine which interface that is currently used for accessing the device.
    ///
    /// 0: VCore System 1: VRAP 2: SI 3: MIIM
    pub fn if_num(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_if_num(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 24;
        self.0 &= !0xff000000;
        self.0 |= value;
    }
    /// SI interface: This field is set if the SI interface has read data from device before it was ready (this can happen if the SI frequency is too high or when too few padding bytes has been specified).
    pub fn if_stat(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_if_stat(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
}
/// Physical interface control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct IF_CTRL(u32);
impl IF_CTRL {
    /// This register configures critical interface parameters, it is constructed so that it can always be written correctly no matter the current state of the interface. When initializing a physical interface, then this is the first register that must be written, the state of the interface at that point may be unknown and therefore the following scheme is required to bring the interface to a known state: When writing a 4-bit value to this field construct a 32-bit data-word as follows: a) copy the 4-bit value into bits 3:0, 11:8, 19:16, and 27:24. b) reverse the 4-bit value and copy into bits 7:4, 15:12, 23:20, and 31:28. Example: To write the value 2 to this field; the 32-bit data-word to write is "0x42424242". Bit 0 configures endianness (when applicable), 0:Little-Endian, 1:Big-Endian. Bit 1 configures bit-order (when applicable), 0:MSB-first, 1:LSB-first. Bit 2,3 are reserved and should be kept 0. For the SI interface the default value of this field is 0x1. For all other interfaces the default value  is 0x0.
    pub fn if_ctrl(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_if_ctrl(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// Mailbox
///
/// This register is shared between all interfaces on the Origin.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MAILBOX(u32);
impl MAILBOX {
    /// Mailbox register which is shared between all interfaces on the Origin. Atomic (safe) modifications to the contents of this register can be performed by using the DEVCPU_ORG::MAILBOX_CLR and DEVCPU_ORG::MAILBOX_SET registers.
    pub fn mailbox(&self) -> u32 {
        self.0
    }
    pub fn set_mailbox(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Atomic clear of mailbox
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MAILBOX_CLR(u32);
impl MAILBOX_CLR {
    /// Set bits in this register to atomically clear corresponding bits in the DEVCPU_ORG::MAILBOX register. This register return 0 on read.
    pub fn mailbox_clr(&self) -> u32 {
        self.0
    }
    pub fn set_mailbox_clr(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Atomic set of mailbox
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MAILBOX_SET(u32);
impl MAILBOX_SET {
    /// Set bits in this register to atomically set corresponding bits in the DEVCPU_ORG::MAILBOX register. This register return 0 on read.
    pub fn mailbox_set(&self) -> u32 {
        self.0
    }
    pub fn set_mailbox_set(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Origin configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ORG_CFG(u32);
impl ORG_CFG {
    /// Set this field to hold back CSR scheduling when this interface want to access a target that is "in use".
    pub fn blocking_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_blocking_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set this field to schedule requests from this interface without looking at the state of other interfaces. The default operation is that an interface waits for a target to be free-up before scheduling (new) requests for a particular target.
    pub fn drop_mode_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_drop_mode_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Clear this field to make write accesses return status. By default write operations return status OK because they are finished before status of the access is known. All non-OK responses will be logged in DEVCPU_ORG::ERR_CNTS no matter the value of this field.
    pub fn fast_wr(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_fast_wr(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Semaphore 0
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SEMA0(u32);
impl SEMA0 {
    /// General Semaphore.The first interface to read this field will be granted the semaphore (reading this field returns 0x1). Once the semaphore has been granted, all reads return '0' from this field (until the semaphore has been released). Any interface can release the semaphore by writing (any value) to this field.
    ///
    /// 0: Semaphore ownership denied. 1: Semaphore has been granted.
    pub fn sema0(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_sema0(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Semaphore 0 owner
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SEMA0_OWNER(u32);
impl SEMA0_OWNER {
    /// Current owner of the semaphore. This field is a one-hot encoded vector, each bit in this vector correspond to an interface on the origin. If this field return 0, then the semaphore was free at the time of reading the register.
    ///
    /// 0: Semaphore is free. 1: VCore System owns semaphore 2: VRAP owns semaphore 4: SI owns smaphore 8: MIIM owns semaphore
    pub fn sema0_owner(&self) -> u32 {
        self.0
    }
    pub fn set_sema0_owner(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Semaphore 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SEMA1(u32);
impl SEMA1 {
    /// General Semaphore.The first interface to read this field will be granted the semaphore (reading this field returns 0x1). Once the semaphore has been granted, all reads return '0' from this field (until the semaphore has been released). Any interface can release the semaphore by writing (any value) to this field.
    ///
    /// 0: Semaphore ownership denied. 1: Semaphore has been granted.
    pub fn sema1(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_sema1(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Semaphore 1 owner
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SEMA1_OWNER(u32);
impl SEMA1_OWNER {
    /// Current owner of the semaphore. This field is a one-hot encoded vector, each bit in this vector correspond to an interface on the origin. If this field return 0, then the semaphore was free at the time of reading the register.
    ///
    /// 0: Semaphore is free. 1: VCore System owns semaphore 2: VRAP owns semaphore 4: SI owns smaphore 8: MIIM owns semaphore
    pub fn sema1_owner(&self) -> u32 {
        self.0
    }
    pub fn set_sema1_owner(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Semaphore configuration
///
/// This register is shared between all interfaces on the Origin.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SEMA_CFG(u32);
impl SEMA_CFG {
    /// By default semaphore-interrupt is generated when a semaphore is free. By setting this field interrupt is generated when semaphore is taken, bit 0 corresponds to semaphore 0, bit 1 to semaphore 1.
    ///
    /// 0: Interrupt on taken semaphore 1: Interrupt on free semaphore
    pub fn sema_intr_pol(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_sema_intr_pol(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
/// Timeout configuration
///
/// This register is shared between all interfaces on the Origin.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TIMEOUT_CFG(u32);
impl TIMEOUT_CFG {
    /// The contents of this field controls the timeout delay for the CSR system. Setting this field to 0 disables timeout. Timeout is handled as follows: A counter that decrements continually, when reaching 0 it will wrap to the value specified by this field. When a target has been processing a request for three "wraps" the target time-out and generate a WD_DROP indication. In the origin an Interface that has been processing a request for four "wraps" will time out and generate a WD_DROP_ORG indication.
    pub fn timeout_cfg(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_timeout_cfg(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
