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

/// Register `QSHP_CIR_CFG`
///
/// Queue Shaping configuration
#[derive(From, Into)]
pub struct QSHP_CIR_CFG(u32);
impl QSHP_CIR_CFG {    ///
    /// Burst capacity of this shaper. Unit is 4096 kilobytes. The shaper is disabled when CIR_BURST=0.
    pub fn cir_burst(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_cir_burst(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }    ///
    /// Leak rate for this shaper. Unit is defined by the leak list period the shaper is attached to (see HSCH_LEAK_CFG.LEAK_TIME).
    pub fn cir_rate(&self) -> u32 {
        (self.0 & 0x7fffc0) >> 6
    }
    pub fn set_cir_rate(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x7fffc0);
        self.0 &= !0x7fffc0;
        self.0 |= value;
    }
}

/// Register `SE_STATE`
///
/// State of the inputs to this SE
#[derive(From, Into)]
pub struct SE_STATE(u32);
impl SE_STATE {    ///
    /// The queue selector must be updated about the state of this element
    pub fn force_upd(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_force_upd(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
