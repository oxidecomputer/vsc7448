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
/// Configuration of common leaky bucket handling
#[derive(From, Into)]
pub struct DLB_CTRL(u32);
impl DLB_CTRL {
    /// TIMESCALE_VAL and BASE_TICK_CNT controls the the rate interval as well as the rate granularity available for LB rate configuration Refer to TIMESCALE_VAL for details.
    pub fn base_tick_cnt(&self) -> u32 {
        (self.0 & 0x7fff0) >> 4
    }
    pub fn set_base_tick_cnt(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x7fff0);
        self.0 &= !0x7fff0;
        self.0 |= value;
    }
    /// Specifies the clock period in unit of 0.1ns.
    ///
    /// 40: 4ns (250MHz) 64: 6,4ns (156.25MHz) 192: 19,2 ns (52.08MHz)
    pub fn clk_period_01ns(&self) -> u32 {
        (self.0 & 0x7f80000) >> 19
    }
    pub fn set_clk_period_01ns(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x7f80000);
        self.0 &= !0x7f80000;
        self.0 |= value;
    }
    /// Enables adding of frame bytes to the leaky buckets.
    ///
    /// 0: Disable bucket addition 1: Enable bucket addition
    pub fn dlb_add_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_dlb_add_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enables leaking from the leaky buckets.
    ///
    /// 0: Disable bucket leaking 1: Enable bucket leaking
    pub fn leak_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_leak_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Configuration of leaky bucket value
#[derive(From, Into)]
pub struct LB_BUCKET_VAL(u32);
impl LB_BUCKET_VAL {
    /// Number of bytes in leaky bucket.
    pub fn bucket_val(&self) -> u32 {
        (self.0 & 0xffffe00) >> 9
    }
    pub fn set_bucket_val(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0xffffe00);
        self.0 &= !0xffffe00;
        self.0 |= value;
    }
    /// Number of subbytes in leaky bucket.
    pub fn rem_val(&self) -> u32 {
        self.0 & 0x1ff
    }
    pub fn set_rem_val(&mut self, value: u32) {
        assert!(value <= 0x1ff);
        self.0 &= !0x1ff;
        self.0 |= value;
    }
}
