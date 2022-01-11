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
/// Rewriter router leg configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct RLEG_CTRL(u32);
impl RLEG_CTRL {
    /// VID for egress router leg. This must be configured consistently in ANA_L3:VMID:RLEG_CTRL.RLEG_EVID.
    ///
    /// n: VID
    pub fn rleg_evid(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_rleg_evid(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    /// Control the value of VSTAX.TAG.WAS_TAGGED field in the stack header for frames that are L3 forwarded to a stack port.
    ///
    /// 0: VSTAX.TAG.WAS_TAGGED = 0 1: VSTAX.TAG.WAS_TAGGED = 1
    pub fn rleg_vstax2_was_tagged(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_rleg_vstax2_was_tagged(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
}
