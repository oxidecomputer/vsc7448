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

/// Register `CLK_ADJ_CFG`
///
/// Generated clock adjustment configuration
///
/// One replication exsists per time domain
#[derive(From, Into)]
pub struct CLK_ADJ_CFG(u32);
impl CLK_ADJ_CFG {
    /// Clock frequency adjustment direction.

    ///

    /// 0: Positive adjustment. Every adjustment adds 1ns to the counter. => clock period is decreased, clock frequency is increased 1: Negative adjustment. Every adjustment subtracts 1ns from the counter. => clock period is increased, clock frequency is decreased
    pub fn clk_adj_dir(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_clk_adj_dir(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Clock frequency adjust enable.

    ///

    /// 0: Adjustment Disabled 1: Adjustment Enabled
    pub fn clk_adj_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_clk_adj_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `CLK_ADJ_FRQ`
///
/// Generated clock frequency adjustment
///
/// Adjust master timer acording to detected PPB error. Ex: 1PBB: Adjust every 1ns. 50.006PBB: Adjust every 19.997.600ps
#[derive(From, Into)]
pub struct CLK_ADJ_FRQ(u32);
impl CLK_ADJ_FRQ {
    /// Clock frequency adjust./

    ///

    /// N: Number of unadjusted CLK_ADJ_UNIT after which the counter for the clock must be adjusted.
    pub fn clk_adj(&self) -> u32 {
        self.0 & 0x3fffffff
    }
    pub fn set_clk_adj(&mut self, value: u32) {
        assert!(value <= 0x3fffffff);
        self.0 &= !0x3fffffff;
        self.0 |= value;
    }
    /// Clock frequency adjust unit./

    ///

    /// 0: Adjustment made every CLK_ADJ picoseconds. 1: Adjustment made every CLK_ADJ nanoseconds.
    pub fn clk_adj_unit(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_clk_adj_unit(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }
}

/// Register `PTP_INTR_IDENT`
///
/// Current interrupt status
#[derive(From, Into)]
pub struct PTP_INTR_IDENT(u32);
impl PTP_INTR_IDENT {
    /// Bit n will be high if an interrupt is currently pending for pin <n>.
    pub fn intr_ptp_ident(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_intr_ptp_ident(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}

/// Register `PTP_MISC_CFG`
///
/// Misc PTP configurations
#[derive(From, Into)]
pub struct PTP_MISC_CFG(u32);
impl PTP_MISC_CFG {
    /// Enable master counter.

    ///

    /// 0: Master counter disabled and reset 1: Master counter enabled
    pub fn ptp_ena(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    pub fn set_ptp_ena(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x1c0);
        self.0 &= !0x1c0;
        self.0 |= value;
    }
    /// Hold master counter.

    ///

    /// 0: Master counter counts if enabled 1: Master counter will stay at the reached value
    pub fn ptp_hold(&self) -> u32 {
        (self.0 & 0x38) >> 3
    }
    pub fn set_ptp_hold(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x38);
        self.0 &= !0x38;
        self.0 |= value;
    }
    /// The PTP_CUR timers will be frozen when this field is set for a domain, in order to return concurrent values.
    pub fn ptp_tod_freeze(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_ptp_tod_freeze(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}

/// Register `PTP_PIN_INTR`
///
/// Pending interrupt mask
#[derive(From, Into)]
pub struct PTP_PIN_INTR(u32);
impl PTP_PIN_INTR {
    /// One bit per pin set when an active edge is seen.
    pub fn intr_ptp(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_intr_ptp(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}

/// Register `PTP_PIN_INTR_ENA`
///
/// Enable interrupts per pin
#[derive(From, Into)]
pub struct PTP_PIN_INTR_ENA(u32);
impl PTP_PIN_INTR_ENA {
    /// Enable interrupt per ptp pin.
    pub fn intr_ptp_ena(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_intr_ptp_ena(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}
