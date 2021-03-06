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
/// CIR status
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CIR_STATE(u32);
impl CIR_STATE {
    /// Current fill level. Unit is 1 bit.
    #[inline(always)]
    pub fn cir_lvl(&self) -> u32 {
        self.0 & 0xffffff
    }
    #[inline(always)]
    pub fn set_cir_lvl(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// EIR status
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct EIR_STATE(u32);
impl EIR_STATE {
    /// Current fill level. Unit is 1 bit.
    #[inline(always)]
    pub fn eir_lvl(&self) -> u32 {
        self.0 & 0xffffff
    }
    #[inline(always)]
    pub fn set_eir_lvl(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// State of the inputs to this SE
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SE_STATE(u32);
impl SE_STATE {
    /// The queue selector must be updated about the state of this element
    #[inline(always)]
    pub fn force_upd(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_force_upd(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
