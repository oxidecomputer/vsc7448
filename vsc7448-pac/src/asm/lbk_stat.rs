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
pub struct LBK_AGING_STICKY(u32);
impl LBK_AGING_STICKY {
    /// Frames are dropped due to aging in loopback FIFO.
    ///
    /// bitmask per port.
    pub fn lbk_aging_sticky(&self) -> u32 {
        self.0
    }
    pub fn set_lbk_aging_sticky(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Stickybits
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct LBK_OVFLW_STICKY(u32);
impl LBK_OVFLW_STICKY {
    /// Frames are dropped due to FIFO overflow.
    ///
    /// bitmask per port.
    pub fn lbk_ovflw_sticky(&self) -> u32 {
        self.0
    }
    pub fn set_lbk_ovflw_sticky(&mut self, value: u32) {
        self.0 = value;
    }
}
