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
/// Tail dropping level
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct ATOP(u32);
impl ATOP {
    /// When a source port consumes more than this level in the packet memory, frames are tail dropped, unconditionally of destination.
    ///
    /// See RES_CFG
    #[inline(always)]
    pub fn atop(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_atop(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// Total raw memory use before tail dropping is activated
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct ATOP_TOT_CFG(u32);
impl ATOP_TOT_CFG {
    /// Tail dropping is activate on a port when the port use has exceeded the ATOP watermark for the port, and the total memory use has exceeded this watermark.
    ///
    /// See RES_CFG
    #[inline(always)]
    pub fn atop_tot(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_atop_tot(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// Forward pressure level
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct FWD_PRESSURE(u32);
impl FWD_PRESSURE {
    /// When ingress queue request count exceeds this level, maximum number of generated frame copies is limited to the FWD_PRESSURE_COPYCNT value.
    ///
    /// See RES_CFG
    #[inline(always)]
    pub fn fwd_pressure(&self) -> u32 {
        (self.0 & 0x7ff8) >> 3
    }
    #[inline(always)]
    pub fn set_fwd_pressure(&mut self, value: u32) {
        assert!(value <= 0xfff);
        let value = value << 3;
        self.0 &= !0x7ff8;
        self.0 |= value;
    }
    /// Maximum frame copy count when fwd pressure is activated.
    #[inline(always)]
    pub fn fwd_pressure_copycnt(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_fwd_pressure_copycnt(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Watermarks for flow control condition per switch port.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PAUSE_CFG(u32);
impl PAUSE_CFG {
    /// Enable pause feedback to the MAC, allowing transmission of pause frames or HDX collisions to limit ingress data rate.
    #[inline(always)]
    pub fn pause_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_pause_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Start pausing ingress stream when the amount of memory consumed by the port exceeds this watermark. The TOTPAUSE condition must also be met.
    ///
    /// See RES_CFG
    #[inline(always)]
    pub fn pause_start(&self) -> u32 {
        (self.0 & 0x1ffe000) >> 13
    }
    #[inline(always)]
    pub fn set_pause_start(&mut self, value: u32) {
        assert!(value <= 0xfff);
        let value = value << 13;
        self.0 &= !0x1ffe000;
        self.0 |= value;
    }
    /// Stop pausing ingress stream when the amount of memory consumed by the port is below  this watermark.
    ///
    /// See RES_CFG.
    #[inline(always)]
    pub fn pause_stop(&self) -> u32 {
        (self.0 & 0x1ffe) >> 1
    }
    #[inline(always)]
    pub fn set_pause_stop(&mut self, value: u32) {
        assert!(value <= 0xfff);
        let value = value << 1;
        self.0 &= !0x1ffe;
        self.0 |= value;
    }
}
/// Configure total memory pause condition
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PAUSE_TOT_CFG(u32);
impl PAUSE_TOT_CFG {
    /// Assert TOTPAUSE condition when total memory allocation is above this watermark.
    ///
    /// See RES_CFG
    #[inline(always)]
    pub fn pause_tot_start(&self) -> u32 {
        (self.0 & 0xfff000) >> 12
    }
    #[inline(always)]
    pub fn set_pause_tot_start(&mut self, value: u32) {
        assert!(value <= 0xfff);
        let value = value << 12;
        self.0 &= !0xfff000;
        self.0 |= value;
    }
    /// Deassert TOTPAUSE condition when total memory allocation is below this watermark.
    ///
    /// See RES_CFG
    #[inline(always)]
    pub fn pause_tot_stop(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_pause_tot_stop(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
