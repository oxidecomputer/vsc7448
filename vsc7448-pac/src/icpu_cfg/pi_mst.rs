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
/// PI Master Configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PI_MST_CFG(u32);
impl PI_MST_CFG {
    /// Disable Automatic Tester Equipment mode for the parallel interface. This mode is used during production testing for controlled loading of CPU.
    #[inline]
    pub fn ate_mode_dis(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline]
    pub fn set_ate_mode_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Controls the clock for the PI Controller. Setting this field to 0 or 1 value is illegal.
    ///
    /// The PI interface frequency is: 250MHz/CLK_DIV.
    #[inline]
    pub fn clk_div(&self) -> u32 {
        self.0 & 0x1f
    }
    #[inline]
    pub fn set_clk_div(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
    /// Controls the clock-polarity of the PI Master.
    ///
    /// 0: Rising edge of the clock sets and samples signals. 1: Falling edge of the clock sets and samples signals.
    #[inline]
    pub fn clk_pol(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline]
    pub fn set_clk_pol(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Controls drive of address and control signals between transfers. See also PI_MST_CTRL.CS_TRISTATE_CTRL.
    ///
    /// 0: Address and control signals are high-Z between transfers. 1: Address and control signals are driven between transfers.
    #[inline]
    pub fn tristate_ctrl(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline]
    pub fn set_tristate_ctrl(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
}
/// PI Master Control Register
///
/// This is a replicated register, where each replication holds the configurations for one chip select. Changes to a value in one of the replicated instances apply only to that chip select.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PI_MST_CTRL(u32);
impl PI_MST_CTRL {
    /// Number of PI_Clk cycles from address driven to PI_nCS[x] low.
    #[inline]
    pub fn cscc(&self) -> u32 {
        (self.0 & 0x60) >> 5
    }
    #[inline]
    pub fn set_cscc(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 5;
        self.0 &= !0x60;
        self.0 |= value;
    }
    /// Controls drive of PI_nCS[x] between transfers.
    ///
    /// 0: PI_nCS[x] is high-Z between transfers. 1: PI_nCS[x] is driven inactive between transfers.
    #[inline]
    pub fn cs_tristate_ctrl(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline]
    pub fn set_cs_tristate_ctrl(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Determines the number of PI_Clk cycles from the start of a transfer until a timeout occurs. This field is only valid when timeout for device-paced transfer is enabled.
    ///
    /// 000: 16 PI_Clk cycles 001: 32 PI_Clk cycles 010: 64 PI_Clk cycles 011: 128 PI_Clk cycles 100: 256 PI_Clk cycles 101: 512 PI_Clk cycles 110: 1024 PI_Clk cycles 111: 2048 PI_Clk cycles
    #[inline]
    pub fn device_paced_timeout(&self) -> u32 {
        (self.0 & 0x1c0000) >> 18
    }
    #[inline]
    pub fn set_device_paced_timeout(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 18;
        self.0 &= !0x1c0000;
        self.0 |= value;
    }
    /// Enable timeout on device-paced transfers. If enabled, a device_paced_transfer transfer does not wait indefinitely for assertion of PI_nDone. If a timeout occurs, the TIMEOUT_ERR_STICKY bit is set in the status register and the current transfer is terminated (read-data will be invalid).
    #[inline]
    pub fn device_paced_timeout_ena(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline]
    pub fn set_device_paced_timeout_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// Device-paced transfer enable. When enabled, use PI_nDone to end a transfer.
    ///
    /// 0: Disabled 1: Enabled
    #[inline]
    pub fn device_paced_xfer_ena(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    #[inline]
    pub fn set_device_paced_xfer_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Polarity of PI_nDone for device-paced transfers.
    ///
    /// 0: PI_nDone is active low 1: PI_nDone is active high
    #[inline]
    pub fn done_pol(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline]
    pub fn set_done_pol(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Number of PI_Clk cycles to insert at the end of a transfer.
    #[inline]
    pub fn hldcc(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline]
    pub fn set_hldcc(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Number of PI_Clk cycles from PI_nCS[x] low to PI_nOE low.
    #[inline]
    pub fn oecc(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    #[inline]
    pub fn set_oecc(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 3;
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// Controls when data is sampled in relation to assertion of PI_nDone for device-paced reads.
    ///
    /// 0: Data is sampled one PI_Clk cycle after PI_nDone goes active. 1: Data is sampled on the same PI_Clk cycle where PI_nDone goes active.
    #[inline]
    pub fn smpl_on_done(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    #[inline]
    pub fn set_smpl_on_done(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Number of wait states measured in PI_Clk cycles on both read and write transfers.
    #[inline]
    pub fn waitcc(&self) -> u32 {
        (self.0 & 0x7f80) >> 7
    }
    #[inline]
    pub fn set_waitcc(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 7;
        self.0 &= !0x7f80;
        self.0 |= value;
    }
}
/// PI Master Status Registers
///
/// This is a replicated register, where each replication holds the status for one chip select.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PI_MST_STATUS(u32);
impl PI_MST_STATUS {
    /// If a timeout is enabled and timeout occurs during a device-paced transfer, this bit is set.
    #[inline]
    pub fn timeout_err_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline]
    pub fn set_timeout_err_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PI Slave Configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PI_SLV_CFG(u32);
impl PI_SLV_CFG {
    /// Set to configure PI interface for big-endian mode.
    #[inline]
    pub fn bigendian(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    #[inline]
    pub fn set_bigendian(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Configures number of clock cycles after detecting asserted CS to control/data is sampled.
    ///
    /// n: Wait n clocks after detecting CS before sampling control/data.
    #[inline]
    pub fn cswait(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline]
    pub fn set_cswait(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// Configure a minimum number of cycles after release of NCS until ndone is no longer driven by device. The device will deassert ndone value when NCS is released, this setting allows parking of the ndone at inactive level before releasing drive of output.
    ///
    /// 0: Immediate release of ndone (async path from CS input). n: Wait at least n+1 clocks after CS release until ndone is released.
    #[inline]
    pub fn donepark(&self) -> u32 {
        (self.0 & 0x3f0000) >> 16
    }
    #[inline]
    pub fn set_donepark(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 16;
        self.0 &= !0x3f0000;
        self.0 |= value;
    }
    /// Set this field to force the value in ICPU_CFG::PI_SLV_CFG.DONEPOL_VAL into ICPU_CFG::GENERAL_CTRL.IF_PI_SLV_DONEPOL. This field is immediately cleared.
    #[inline]
    pub fn donepol_set(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    #[inline]
    pub fn set_donepol_set(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 23;
        self.0 &= !0x800000;
        self.0 |= value;
    }
    /// See ICPU_CFG::PI_SLV_CFG.DONEPOL_SET for more information.
    #[inline]
    pub fn donepol_val(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    #[inline]
    pub fn set_donepol_val(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 24;
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    /// Configures number of clock cycles to delay nDone indication for read operations after accerss is done.
    ///
    /// n: Wait n clocks after read data ready before asserting nDone indication.
    #[inline]
    pub fn donewait(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline]
    pub fn set_donewait(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
}
