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
/// The counter's least significant 32 bits.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct STAT_LSB_CNT(u32);
impl STAT_LSB_CNT {
    /// This register contains the least significant 32 bits of a counter.
    #[inline(always)]
    pub fn lsb_cnt(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_lsb_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// The counter's most significant 8 bits.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct STAT_MSB_CNT(u32);
impl STAT_MSB_CNT {
    /// The counter's most significant 8 bits. The field stores the value in the counters of a flow from bit 32 to the most significant bit. Reading: The MSB part of the counter is latched to a shadow register, when the LSB part is read. As a result, the LSB part must always be read first, and the MSB part must be read immediately after the LSB part is read. Writing: The procedure for writing differs depending on counter group: ANA_AC:STAT_CNT_CFG_PORT: LSB part must be written first, followed by MSB part. All other counter groups: MSB part must be written first, followed by LSB part.
    #[inline(always)]
    pub fn msb_cnt(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_msb_cnt(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
