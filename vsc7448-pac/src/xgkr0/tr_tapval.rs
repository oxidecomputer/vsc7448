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
/// VS tap C0 value
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TR_C0VAL(u32);
impl TR_C0VAL {
    /// C0 value
    #[inline(always)]
    pub fn c0_val(&self) -> u32 {
        self.0 & 0x7f
    }
    #[inline(always)]
    pub fn set_c0_val(&mut self, value: u32) {
        debug_assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }
}
/// VS tap CM value
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TR_CMVAL(u32);
impl TR_CMVAL {
    /// CM value
    #[inline(always)]
    pub fn cm_val(&self) -> u32 {
        self.0 & 0x7f
    }
    #[inline(always)]
    pub fn set_cm_val(&mut self, value: u32) {
        debug_assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }
}
/// VS tap CP value
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TR_CPVAL(u32);
impl TR_CPVAL {
    /// CP value
    #[inline(always)]
    pub fn cp_val(&self) -> u32 {
        self.0 & 0x7f
    }
    #[inline(always)]
    pub fn set_cp_val(&mut self, value: u32) {
        debug_assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }
}
