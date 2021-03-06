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
/// TWI hold time configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TWI_CONFIG(u32);
impl TWI_CONFIG {
    /// Configure the hold time delay to apply to SDA after SCK when transmitting from the device. This delay is expressed in a number of VCore System clock cycles. The delay value should be as close to 300ns as possible without going below 300ns.
    ///
    /// Set to (300ns/4.8ns + 2) = 65
    #[inline(always)]
    pub fn twi_cnt_reload(&self) -> u32 {
        (self.0 & 0x1fe) >> 1
    }
    #[inline(always)]
    pub fn set_twi_cnt_reload(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 1;
        self.0 &= !0x1fe;
        self.0 |= value;
    }
    /// Set this field to enable hold time on the TWI SDA output. When enabled the TWI_CONFIG.TWI_CNT_RELOAD field determines the amount of hold time to apply to SDA.
    #[inline(always)]
    pub fn twi_delay_enable(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_twi_delay_enable(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
