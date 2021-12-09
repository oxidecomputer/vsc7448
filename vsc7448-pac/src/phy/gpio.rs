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
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_0(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_3(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_4(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_5(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_6(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_7(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_8(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_9(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_10(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_11(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_12(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SIGDET_GPIO_CONTROL(u16);
impl SIGDET_GPIO_CONTROL {
    pub fn gpio0(&self) -> u16 {
        self.0 & 0x3
    }
    pub fn set_gpio0(&mut self, value: u16) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    pub fn gpio1(&self) -> u16 {
        (self.0 & 0x0) >> 2
    }
    pub fn set_gpio1(&mut self, value: u16) {
        let value = value << 2;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
    pub fn gpio2(&self) -> u16 {
        (self.0 & 0xfc) >> 8
    }
    pub fn set_gpio2(&mut self, value: u16) {
        let value = value << 8;
        assert!(value <= 0xfc);
        self.0 &= !0xfc;
        self.0 |= value;
    }
    pub fn gpio3(&self) -> u16 {
        (self.0 & 0x3fc) >> 10
    }
    pub fn set_gpio3(&mut self, value: u16) {
        let value = value << 10;
        assert!(value <= 0x3fc);
        self.0 &= !0x3fc;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_CONTROL_2(u16);
impl GPIO_CONTROL_2 {
    pub fn coma_mode_output_data(&self) -> u16 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_coma_mode_output_data(&mut self, value: u16) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    pub fn coma_mode_output_enable(&self) -> u16 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_coma_mode_output_enable(&mut self, value: u16) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    pub fn gpio12_gpio13(&self) -> u16 {
        (self.0 & 0x3ffc) >> 14
    }
    pub fn set_gpio12_gpio13(&mut self, value: u16) {
        let value = value << 14;
        assert!(value <= 0x3ffc);
        self.0 &= !0x3ffc;
        self.0 |= value;
    }
    pub fn gpio4(&self) -> u16 {
        self.0 & 0x3
    }
    pub fn set_gpio4(&mut self, value: u16) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    pub fn gpio5(&self) -> u16 {
        (self.0 & 0x0) >> 2
    }
    pub fn set_gpio5(&mut self, value: u16) {
        let value = value << 2;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_15(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_INPUT(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_OUTPUT(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_IN_OUT_CONF(u16);
impl GPIO_IN_OUT_CONF {
    pub fn gpio0(&self) -> u16 {
        self.0 & 0x1
    }
    pub fn set_gpio0(&mut self, value: u16) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    pub fn gpio1(&self) -> u16 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_gpio1(&mut self, value: u16) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    pub fn gpio2(&self) -> u16 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_gpio2(&mut self, value: u16) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    pub fn gpio3(&self) -> u16 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_gpio3(&mut self, value: u16) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    pub fn gpio4(&self) -> u16 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_gpio4(&mut self, value: u16) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    pub fn gpio5(&self) -> u16 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_gpio5(&mut self, value: u16) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MICRO_PAGE(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GPIO_CONTROL_3(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MAC_MODE_AND_FAST_LINK(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct I2C_MUX_CONTROL_1(u16);
impl I2C_MUX_CONTROL_1 {
    pub fn dev_addr(&self) -> u16 {
        (self.0 & 0x180) >> 9
    }
    pub fn set_dev_addr(&mut self, value: u16) {
        let value = value << 9;
        assert!(value <= 0x180);
        self.0 &= !0x180;
        self.0 |= value;
    }
    pub fn port_0_enable(&self) -> u16 {
        self.0 & 0x1
    }
    pub fn set_port_0_enable(&mut self, value: u16) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    pub fn port_1_enable(&self) -> u16 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_port_1_enable(&mut self, value: u16) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    pub fn port_2_enable(&self) -> u16 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_port_2_enable(&mut self, value: u16) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    pub fn port_3_enable(&self) -> u16 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_port_3_enable(&mut self, value: u16) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    pub fn scl_clk_freq(&self) -> u16 {
        (self.0 & 0xc) >> 4
    }
    pub fn set_scl_clk_freq(&mut self, value: u16) {
        let value = value << 4;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct I2C_MUX_CONTROL_2(u16);
impl I2C_MUX_CONTROL_2 {
    pub fn addr(&self) -> u16 {
        (self.0 & 0x7f) >> 7
    }
    pub fn set_addr(&mut self, value: u16) {
        let value = value << 7;
        assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }
    pub fn ena_i2c_mux_access(&self) -> u16 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_ena_i2c_mux_access(&mut self, value: u16) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    pub fn mux_ready(&self) -> u16 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_mux_ready(&mut self, value: u16) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    pub fn phy_port_addr(&self) -> u16 {
        (self.0 & 0x3fe) >> 10
    }
    pub fn set_phy_port_addr(&mut self, value: u16) {
        let value = value << 10;
        assert!(value <= 0x3fe);
        self.0 &= !0x3fe;
        self.0 |= value;
    }
    pub fn rd(&self) -> u16 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_rd(&mut self, value: u16) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct I2C_MUX_DATA_READ_WRITE(u16);
impl I2C_MUX_DATA_READ_WRITE {
    pub fn read_data(&self) -> u16 {
        (self.0 & 0x0) >> 8
    }
    pub fn set_read_data(&mut self, value: u16) {
        let value = value << 8;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
    pub fn write_data(&self) -> u16 {
        self.0 & 0xff
    }
    pub fn set_write_data(&mut self, value: u16) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct RECOVERED_CLOCK_0_CONTROL(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct RECOVERED_CLOCK_1_CONTROL(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ENHANCED_LED_CONTROL(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TEMP_CONF(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TEMP_VAL(pub u16);
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct EXTENDED_REVISION(u16);
impl EXTENDED_REVISION {
    pub fn tesla_e(&self) -> u16 {
        self.0 & 0x1
    }
    pub fn set_tesla_e(&mut self, value: u16) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
