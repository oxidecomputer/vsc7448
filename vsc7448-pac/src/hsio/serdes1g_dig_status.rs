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

/// Register `SERDES1G_DFT_STATUS`
///
/// SERDES1G DFT Status
///
/// Status register of SERDES1G DFT functions
#[derive(From, Into)]
pub struct SERDES1G_DFT_STATUS(u32);
impl SERDES1G_DFT_STATUS {    ///
    /// BIST activity
    ///
    /// 0: BIST inactive 1: BIST active
    pub fn bist_active(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_bist_active(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// BIST completion state (low-active)
    ///
    /// 0: BIST completed 1: not completed
    pub fn bist_complete_n(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_bist_complete_n(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// BIST result
    ///
    /// 0: No error found 1: Errors during BIST found
    pub fn bist_error(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_bist_error(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// BIST sync result
    ///
    /// 0: Synchronization successful 1: Synchronization on BIST data failed
    pub fn bist_nosync(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_bist_nosync(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// RC-PLL BIST result
    ///
    /// 0: No error found 1: Errors during BIST found
    pub fn pll_bist_failed(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_pll_bist_failed(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }    ///
    /// RC-PLL BIST not done flag
    ///
    /// 0: BIST done 1: BIST not started or active
    pub fn pll_bist_not_done(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_pll_bist_not_done(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }    ///
    /// RC-PLL BIST timeout error flag
    ///
    /// 0: No timeout occured 1: Timeout occured
    pub fn pll_bist_timeout_err(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_pll_bist_timeout_err(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
}

/// Register `SERDES1G_MISC_CFG`
///
/// SERDES1G Misc Configuration
///
/// Configuration register for miscellaneous functions
#[derive(From, Into)]
pub struct SERDES1G_MISC_CFG(u32);
impl SERDES1G_MISC_CFG {    ///
    /// Enable deserializer cp/md handling for 100fx mode
    ///
    /// 0: Disable 1: Enable
    pub fn des_100fx_cpmd_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_des_100fx_cpmd_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }    ///
    /// Select simple 100fx mode
    ///
    /// 0: Normal mode 1: Simple mode
    pub fn des_100fx_cpmd_mode(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_des_100fx_cpmd_mode(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }    ///
    /// Swap cp/md signals in 100fx mode
    ///
    /// 0: No swapping of cp and md 1: Swap cp and md
    pub fn des_100fx_cpmd_swap(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_des_100fx_cpmd_swap(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }    ///
    /// Select mode of kick-out-of-180-degree functionality
    pub fn des_100fx_kick_mode(&self) -> u32 {
        (self.0 & 0x1800) >> 11
    }
    pub fn set_des_100fx_kick_mode(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x1800);
        self.0 &= !0x1800;
        self.0 |= value;
    }    ///
    /// Lane Reset
    ///
    /// 0: No reset 1: Reset (not self-clearing)
    pub fn lane_rst(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_lane_rst(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Enable data inversion received from Deserializer
    ///
    /// 0: Disable 1: Enable
    pub fn rx_data_inv_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_rx_data_inv_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Enable RX-Low-Power feature (Power control by LPI-FSM in connected PCS)
    ///
    /// 0: Disable 1: Enable
    pub fn rx_lpi_mode_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_rx_lpi_mode_ena(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }    ///
    /// Enable data inversion sent to Serializer
    ///
    /// 0: Disable 1: Enable
    pub fn tx_data_inv_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_tx_data_inv_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Enable TX-Low-Power feature (Power control by LPI-FSM in connected PCS)
    ///
    /// 0: Disable 1: Enable
    pub fn tx_lpi_mode_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_tx_lpi_mode_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}