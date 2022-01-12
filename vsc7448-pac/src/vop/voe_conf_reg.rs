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
/// Miscellanous per VOE configuration
///
/// Miscellaneous per VOE configuration.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VOE_MISC_CONFIG(u32);
impl VOE_MISC_CONFIG {
    /// If this field is asserted, the VOE will count bytes instead of frames. This is not 100% supported and tested. Feature is only available for Service / Path VOEs. Byte count is disabled for Port VOEs.
    #[inline(always)]
    pub fn lm_cnt_byte(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_lm_cnt_byte(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// The VOE will process either Y.1731 or MPLS OAM PDUs depending on the configuration of this register. To enable a specific VOE for MPLS OAM processing, the corresponding bit in this register must be asserted.
    ///
    /// '0': VOE is configured to process Y.1731 OAM PDUs '1': VOE is configured to process MPLS-TP OAM PDUs
    #[inline(always)]
    pub fn mpls_oam_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_mpls_oam_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable the VOE for Synthetic Loss Measurements. If enabled, the normal LM counters are used differently than when running standard frame loss measurements. The Rx counters are used to count SLR/SL1 frames received from different Peer MEPs. The Tx counters are used to count SLR/SL1 frames transmitted to different Peer MEPs. Note that there is no counting of data frames or other NON SL OAM PDUs. Asserting this register will avoid any other VOEs from updating the LM counters of this VOE as part of a hierarchical LM counter update.
    #[inline(always)]
    pub fn sl_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_sl_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
