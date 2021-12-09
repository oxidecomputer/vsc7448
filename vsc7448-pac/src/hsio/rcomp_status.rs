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
/// RCOMP Configuration 0
///
/// Configuration register 0 for RCOMP
#[derive(From, Into)]
pub struct RCOMP_CFG0(u32);
impl RCOMP_CFG0 {
    /// Overwrite measured resistor value with value programmed in rcomp_val
    ///
    /// 0: normal mode 1: overwrite mode
    pub fn force_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_force_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// RCOMP operation mode
    ///
    /// 0: inactive 1: perform calibration permanently 2: perform calibration once 3: perform calibration once and generate alarm if necessary
    pub fn mode_sel(&self) -> u32 {
        (self.0 & 0x300) >> 8
    }
    pub fn set_mode_sel(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x300);
        self.0 &= !0x300;
        self.0 |= value;
    }
    /// Enable power-down after calibration was done
    ///
    /// 0: disable power-down 1: enable power-down
    pub fn pwd_ena(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_pwd_ena(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Resistor comparator value
    ///
    /// 0: maximum resistance value 15: minimum resistance value
    pub fn rcomp_val(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_rcomp_val(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Start calibration
    ///
    /// 0: idle/inactive 1: start (activate)
    pub fn run_cal(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_run_cal(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Speed selection. Setting time for analog circuit after changing resistor settings.
    ///
    /// 0: max period 1: max period/2 2: max period/4 3: max period/8
    pub fn speed_sel(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    pub fn set_speed_sel(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xc00);
        self.0 &= !0xc00;
        self.0 |= value;
    }
}
