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

/// Register `RCOMP_STATUS`
///
/// RCOMP Status
///
/// Status register bits for the RCOMP
#[derive(From, Into)]
pub struct RCOMP_STATUS(u32);
impl RCOMP_STATUS {    ///
    /// Resistor comparison activity
    ///
    /// 0: resistor measurement finished or inactive 1: resistor measurement in progress
    pub fn busy(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_busy(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }    ///
    /// Alarm signal if rcomp isn't best choice anymore
    ///
    /// 0: inactive 1: active
    pub fn delta_alert(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_delta_alert(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }    ///
    /// Measured resistor value
    ///
    /// 0: maximum resistance value 15: minimum resistance value
    pub fn rcomp(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_rcomp(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}

/// Register `SYNC_ETH_CFG`
///
/// SYNC ETH Configuration
///
/// This register is replicated once per recovered clock output.
#[derive(From, Into)]
pub struct SYNC_ETH_CFG(u32);
impl SYNC_ETH_CFG {    ///
    /// Set to enable recovered clock pad
    ///
    /// 0: Disable (high-impedance) 1: Enable (output recovered clock)
    pub fn reco_clk_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_reco_clk_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Select recovered clock divider.
    ///
    /// 0: No clock dividing 1: Divide clock by 5 2: Divide clock by 4 3: Reserved
    pub fn sel_reco_clk_div(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_sel_reco_clk_div(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x6);
        self.0 &= !0x6;
        self.0 |= value;
    }    ///
    /// Select recovered clock source.
    ///
    /// 0 through 8: Select SD1G 0 through 8. 9 through 24: Select SD6G 0 through 15. 25 through 28: Select SD10G 0 through 3. 29 through 36: Select SD6G 16 through 23. 37 selects LCPLL2 CPU clock output. Other values are reserved.
    pub fn sel_reco_clk_src(&self) -> u32 {
        (self.0 & 0x1f8) >> 3
    }
    pub fn set_sel_reco_clk_src(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x1f8);
        self.0 &= !0x1f8;
        self.0 |= value;
    }
}

/// Register `SYNC_ETH_PLL2_CFG`
///
/// Configuration of recovered clock from 2'nd PLL
#[derive(From, Into)]
pub struct SYNC_ETH_PLL2_CFG(u32);
impl SYNC_ETH_PLL2_CFG {    ///
    /// This field is used instead of HSIO::PLL5G_CFG0.CPU_CLK_DIV for the PLL number 2.
    pub fn cpu_clk_div(&self) -> u32 {
        (self.0 & 0xfc) >> 2
    }
    pub fn set_cpu_clk_div(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xfc);
        self.0 &= !0xfc;
        self.0 |= value;
    }    ///
    /// Enable auto-squelching for sync. ethernet clock output: when set the clock output will stop toggling (keep its last value constantly) when PLL is out of lock.
    pub fn pll2_auto_squelch_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_pll2_auto_squelch_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Divider setting for the PLL number 2's recovered clock output. These settings are applied prior to sending recovered clock to the optional PAD-divder (see HSIO::SYNC_ETH_CFG.SEL_RECO_CLK_DIV.)
    ///
    /// 0: No clock dividing 1: Divide clock by 2
    pub fn pll2_reco_clk_div(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_pll2_reco_clk_div(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
