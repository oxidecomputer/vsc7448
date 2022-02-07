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
/// Packet memory status
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MMGT(u32);
impl MMGT {
    /// Number of 6 free memory words.
    #[inline(always)]
    pub fn freecnt(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_freecnt(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// Number of 6 frames awaiting releasing.
    #[inline(always)]
    pub fn relcnt(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    #[inline(always)]
    pub fn set_relcnt(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 16;
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
/// Packet Memory Status
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MMGT_FAST(u32);
impl MMGT_FAST {
    /// Number of words in the fast pool
    #[inline(always)]
    pub fn freevld(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    #[inline(always)]
    pub fn set_freevld(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 4;
        self.0 &= !0xf0;
        self.0 |= value;
    }
    /// Number of frames awaiting release in the fast pool
    #[inline(always)]
    pub fn relvld(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_relvld(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
