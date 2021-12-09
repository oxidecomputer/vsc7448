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
/// Fan controller configuration
#[derive(From, Into)]
pub struct FAN_CFG(u32);
impl FAN_CFG {
    /// The system clock period given in the clock period in PS divided by 100.

    ///

    /// Values below 40 are reserved.
    pub fn clk_period(&self) -> u32 {
        (self.0 & 0x7f80) >> 7
    }
    pub fn set_clk_period(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x7f80);
        self.0 &= !0x7f80;
        self.0 |= value;
    }
    /// Define the duty cycle

    ///

    /// 0x00: Always "off" 0xFF: Always "on"
    pub fn duty_cycle(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_duty_cycle(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xff0000);
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    /// Configure behavior of TACH input tick counter, see DEVCPU_GCB::FAN_CNT for more infromation.
    pub fn fan_stat_cfg(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_fan_stat_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable gating of the TACH input by the PWM output so that only TACH pulses received when PWM is "on" are counted.

    ///

    /// 0: Disabled 1: Enabled
    pub fn gate_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_gate_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Define the polarity of the PWM output.

    ///

    /// 0: PWM is logic 1 when "on" 1: PWM is logic 0 when "on"
    pub fn inv_pol(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_inv_pol(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set the frequency of the PWM output

    ///

    /// 0: 25 kHz 1: 120 Hz 2: 100 Hz 3: 80 Hz 4: 60 Hz 5: 40 Hz 6: 20 Hz 7: 10 Hz
    pub fn pwm_freq(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_pwm_freq(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }
    /// Configure the PWM output to be open collector
    pub fn pwm_open_col_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_pwm_open_col_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Currently active interrupts
#[derive(From, Into)]
pub struct SIO_INTR_IDENT(u32);
impl SIO_INTR_IDENT {
    /// Shows the currently active interrupts. This register is the result of the SIO_INTR interrupts with the disabled interrupts (from SIO_INTR_ENA and SIO_GPIO_INTR_ENA) removed.

    ///

    /// 0: No active interrupt for given gpio 1: Active interrupt for given gpio
    pub fn sio_intr_ident(&self) -> u32 {
        self.0
    }
    pub fn set_sio_intr_ident(&mut self, value: u32) {
        self.0 = value;
    }
}
