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
/// Random Early Discard (WRED) configuration
///
/// Configuration of Random Early Discard (RED) profile per RedGroup/QoS class/DP level. The profile index is calulated as Group*24+(DP-1)*8+QOS. There are no profiled for DP=0..
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct WRED_PROFILE(u32);
impl WRED_PROFILE {
    /// See WM_RED_LOW. Unit is 2816 bytes.
    #[inline(always)]
    pub fn wm_red_high(&self) -> u32 {
        self.0 & 0x7ff
    }
    #[inline(always)]
    pub fn set_wm_red_high(&mut self, value: u32) {
        assert!(value <= 0x7ff);
        self.0 &= !0x7ff;
        self.0 |= value;
    }
    /// When enqueuing a frame, RED is active if the egress memory consumption for the applicaple profile is above WM_RED_LEVEL. The probability of random early discarding is calculated as: (Memory consumption - WM_RED_LOW)/(WM_RED_HIGH - WM_RED_LOW). Unit is 2816 bytes.
    #[inline(always)]
    pub fn wm_red_low(&self) -> u32 {
        (self.0 & 0x3ff800) >> 11
    }
    #[inline(always)]
    pub fn set_wm_red_low(&mut self, value: u32) {
        assert!(value <= 0x7ff);
        let value = value << 11;
        self.0 &= !0x3ff800;
        self.0 |= value;
    }
}
