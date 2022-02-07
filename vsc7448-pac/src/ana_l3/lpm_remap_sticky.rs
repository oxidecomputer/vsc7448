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
/// Router Leg Diagnostic
///
/// Diagnostic bits related to longest prefix matching.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct L3_LPM_REMAP_STICKY(u32);
impl L3_LPM_REMAP_STICKY {
    /// Set if an IPv4 multicast longest prefix match has been found and the returned L3MC_IDX is less than the number of L3MC table entries.
    #[inline(always)]
    pub fn lpm_ip4mc_found_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_lpm_ip4mc_found_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set if routing lookup found an IPv4 unicast longest prefix match.
    #[inline(always)]
    pub fn lpm_ip4uc_found_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_lpm_ip4uc_found_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set if an IPv6 multicast longest prefix match has been found and the returned L3MC_IDX is less than the number of L3MC table entries.
    #[inline(always)]
    pub fn lpm_ip6mc_found_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_lpm_ip6mc_found_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set if routing lookup found an IPv6 unicast longest prefix match.
    #[inline(always)]
    pub fn lpm_ip6uc_found_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_lpm_ip6uc_found_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set if an IPv4 SIP security lookup has found a match in LPM lookup. Note that this bit is not set for DIP security LPM lookups
    #[inline(always)]
    pub fn secur_ip4_lpm_found_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_secur_ip4_lpm_found_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set if an IPv6 SIP security lookup has found a match in LPM lookup. Note that this bit is not set for DIP security LPM lookups
    #[inline(always)]
    pub fn secur_ip6_lpm_found_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_secur_ip6_lpm_found_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
}
