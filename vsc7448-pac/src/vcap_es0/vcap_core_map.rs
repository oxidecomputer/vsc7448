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
/// Core index
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VCAP_CORE_IDX(u32);
impl VCAP_CORE_IDX {
    /// Set to index of specific core to access the mapping of that core via VCAP_SUPER::VCAP_CORE_MAP.
    #[inline(always)]
    pub fn core_idx(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_core_idx(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// Mapping of core
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VCAP_CORE_MAP(u32);
impl VCAP_CORE_MAP {
    /// Configure ownership of core n (defined by VCAP_SUPER::VCAP_CORE_IDX). When a core is mapped to a specific VCAP; lookups for that VCAP will be applied to the core. VCAP priority is still observed, a match in two cores will only cause the most significant rule to be "hit" (highest address.) After reset all cores are in power-save mode.
    ///
    /// Applies only to the Super VCAP.
    #[inline(always)]
    pub fn core_map(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_core_map(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
