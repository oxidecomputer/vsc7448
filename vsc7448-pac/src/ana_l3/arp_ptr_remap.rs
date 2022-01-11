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
/// ARP Pointer Remap Configuration
///
/// This information is used for LPM VCAP actions of type ARP_PTR and with ARP_PTR_REMAP_ENA=1.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ARP_PTR_REMAP_CFG(u32);
impl ARP_PTR_REMAP_CFG {
    /// Address of ARP entry in ARP Table (ANA_L3:ARP).
    #[inline(always)]
    pub fn arp_ptr(&self) -> u32 {
        self.0 & 0x7ff
    }
    #[inline(always)]
    pub fn set_arp_ptr(&mut self, value: u32) {
        assert!(value <= 0x7ff);
        self.0 &= !0x7ff;
        self.0 |= value;
    }
    /// Number of Equal Cost Multiple Paths. Overrules any value in LPM VCAP action.
    ///
    /// 0: 1 path 1: 2 paths 2: 3 paths ...
    #[inline(always)]
    pub fn ecmp_cnt(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    #[inline(always)]
    pub fn set_ecmp_cnt(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 16;
        self.0 &= !0xf0000;
        self.0 |= value;
    }
}
