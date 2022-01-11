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
/// Temperature sensor sample period
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TEMP_SENSOR_CFG(u32);
impl TEMP_SENSOR_CFG {
    /// The time to keep RUN asserted during a sample period, time is in 1024 SwC clock cycles.
    ///
    /// 0: Reserved n: n * 1024 * clock-period between samples
    #[inline]
    pub fn run_wid(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline]
    pub fn set_run_wid(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// The delay between temperature sensor samples in 1024 SwC clock cycles. The default value corresponds to 500us. The delay have to be big enough to allow the temperature sensor to complete its sample.
    ///
    /// 0: Reserved n: n * 1024 * clock-period between samples
    #[inline]
    pub fn sample_per(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline]
    pub fn set_sample_per(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Temperature Sensor Control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TEMP_SENSOR_CTRL(u32);
impl TEMP_SENSOR_CTRL {
    /// Set to force clock signal towards temperature sensor. This field only works when SAMPLE_ENA is cleared.
    #[inline]
    pub fn force_clk(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline]
    pub fn set_force_clk(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set to force RESETN signal towards temperature sensor (release of reset). This field only works when SAMPLE_ENA is cleared.
    #[inline]
    pub fn force_no_rst(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline]
    pub fn set_force_no_rst(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set to force PDB signal towards temperature sensor. This field only works when SAMPLE_ENA is cleared.
    #[inline]
    pub fn force_power_up(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline]
    pub fn set_force_power_up(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set to force RUN signal towards temperature sensor. This field only works when SAMPLE_ENA is cleared.
    #[inline]
    pub fn force_run(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline]
    pub fn set_force_run(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set to force reading of temperature irregardless of when Temperature sensor says done.
    #[inline]
    pub fn force_temp_rd(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline]
    pub fn set_force_temp_rd(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Set this field to enable sampling of temperature. Approximately 500us after setting this field DEVCPU_GCB::TEMP_SENSOR_STAT.TEMP_VALID will be set together with a valid temperature value. After this the temperature will be updated every 500us for as long as this field remains set. Clear ths field to disable temperature sensor.
    #[inline]
    pub fn sample_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline]
    pub fn set_sample_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Temperature Sensor Status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TEMP_SENSOR_STAT(u32);
impl TEMP_SENSOR_STAT {
    /// Temperature data readout, this field is valid when DEVCPU_GCB::TEMP_SENSOR_STAT.TEMP_VALID is set. This field is continually updated (approximately every 500us) while the termperature sensor is enabled, see DEVCPU_GCB::TEMP_SENSOR_CTRL.SAMPLE_ENA for more information.
    ///
    /// Temperature(C)=177.4 - 0.8777*DATA
    #[inline]
    pub fn temp(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline]
    pub fn set_temp(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// This fied is set when valid temperature data is available in DEVCPU_GCB::TEMP_SENSOR_STAT.TEMP.
    #[inline]
    pub fn temp_valid(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline]
    pub fn set_temp_valid(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
}
