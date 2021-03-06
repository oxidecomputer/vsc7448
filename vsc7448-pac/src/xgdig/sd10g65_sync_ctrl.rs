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
/// RX_SYNC_CTRL config register
///
/// RX Sync control configuration register, synchronize I2 of one RX to the I2 of another RX
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RX_SYNC_CTRL_CFG(u32);
impl RX_SYNC_CTRL_CFG {
    /// Clear RX I2 value
    #[inline(always)]
    pub fn rx_i2_clr(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_rx_i2_clr(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Keep current RX I2 value constant
    #[inline(always)]
    pub fn rx_i2_hold(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_rx_i2_hold(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Source selection for RX lane synchronization
    ///
    /// 0: Do not use external sync_ctrl info 1: Select sync_ctrl info from external DES
    #[inline(always)]
    pub fn rx_lane_sync_src(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_rx_lane_sync_src(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// RX_SYNC_CTRL status register
///
/// RX Sync control status register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RX_SYNC_CTRL_STAT(u32);
impl RX_SYNC_CTRL_STAT {
    /// RX Lane synchronization fifo overflow
    #[inline(always)]
    pub fn rx_lane_sync_fifo_of_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_rx_lane_sync_fifo_of_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// SYNC_CTRL config register
///
/// Sync control configuration register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SYNC_CTRL_CFG(u32);
impl SYNC_CTRL_CFG {
    /// Source selection for lane synchronization
    ///
    /// 0: Select external DES 1: Select F2DF 2: Select local DES 3: Disable sync_ctrl
    #[inline(always)]
    pub fn lane_sync_src(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_lane_sync_src(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
/// SYNC_CTRL status register
///
/// Sync control status register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SYNC_CTRL_STAT(u32);
impl SYNC_CTRL_STAT {
    /// Lane synchronization fifo overflow
    #[inline(always)]
    pub fn lane_sync_fifo_of_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_lane_sync_fifo_of_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
