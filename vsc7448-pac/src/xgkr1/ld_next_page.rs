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
/// NxtPg transmit 15-0
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct KR_7X0016(u32);
impl KR_7X0016 {
    /// Next page to transmit D[15:0]
    #[inline(always)]
    pub fn np_tx0(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_np_tx0(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// NxtPg transmit 31-16
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct KR_7X0017(u32);
impl KR_7X0017 {
    /// Next page to transmit D[31:16]
    #[inline(always)]
    pub fn np_tx1(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_np_tx1(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// NxtPg transmit 47-32
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct KR_7X0018(u32);
impl KR_7X0018 {
    /// Next page to transmit D[47:32]
    #[inline(always)]
    pub fn np_tx2(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_np_tx2(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
