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

/// Register `PTP_CUR_NSEC`
///
/// Current time of day
///
/// Current time of day, nanoseconds part.
#[derive(From, Into)]
pub struct PTP_CUR_NSEC(u32);
impl PTP_CUR_NSEC {    ///
    /// Time of day naoseconds, latched when NSF was read.
    pub fn ptp_cur_nsec(&self) -> u32 {
        (self.0 & 0x3fffffff) >> 0
    }
    pub fn set_ptp_cur_nsec(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3fffffff);
        self.0 &= !0x3fffffff;
        self.0 |= value;
    }
}

/// Register `PTP_CUR_NSF`
///
/// Current timestamping value
#[derive(From, Into)]
pub struct PTP_CUR_NSF(u32);
impl PTP_CUR_NSF {    ///
    /// Returns the current value of the timestamping clock. The time of day registers will be latched when this register is read.
    pub fn ptp_cur_nsf(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_ptp_cur_nsf(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `PTP_CUR_SEC_LSB`
///
/// Current time of day
#[derive(From, Into)]
pub struct PTP_CUR_SEC_LSB(u32);
impl PTP_CUR_SEC_LSB {    ///
    /// Value of current tod secs, latched when NSF was read.
    pub fn ptp_cur_sec_lsb(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_ptp_cur_sec_lsb(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `PTP_SYS_CLK_CFG`
///
/// System clock configuration
#[derive(From, Into)]
pub struct PTP_SYS_CLK_CFG(u32);
impl PTP_SYS_CLK_CFG {    ///
    /// Must be configured to running system clock period, rounded down to closest interger nanoseconds value.
    pub fn ptp_sys_clk_per_ns(&self) -> u32 {
        (self.0 & 0x1f0) >> 4
    }
    pub fn set_ptp_sys_clk_per_ns(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x1f0);
        self.0 &= !0x1f0;
        self.0 |= value;
    }    ///
    /// Must be configured to number of 100ps to add on top of the PTP_SYS_CLK_PER_NS value to get to the correct clock period.
    pub fn ptp_sys_clk_per_ps100(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_ptp_sys_clk_per_ps100(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}