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

/// Register `STAT_CFG`
///
/// Statistics configuration
#[derive(From, Into)]
pub struct STAT_CFG(u32);
impl STAT_CFG {    ///
    /// Set STAT_CLEAR_SHOT to clear counters for the port or service index selected by STAT_VIEW. Auto-cleared when complete (1us). Multiple counters can be cleared at the same time by setting multiple bits in STAT_CLEAR_SHOT.
    ///
    /// Bit 0: Clear Rx port counters (Packet, LS byte and MS byte) Bit 1: Clear Tx port counters (Packet, LS byte and MS byte) Bit 2: Clear ingress service counters (Packet, LS byte and MS byte) Bit 3: Clear egress service counters (Packet, LS byte and MS byte) When bits 0-1 are used a port number must be configured in STAT_VIEW. When bits 2 is used an ingress service index must be configured in STAT_VIEW. When bits 3 is used an egress service index must be configured in STAT_VIEW.
    pub fn stat_clear_shot(&self) -> u32 {
        (self.0 & 0x3c0000) >> 18
    }
    pub fn set_stat_clear_shot(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x3c0000);
        self.0 &= !0x3c0000;
        self.0 |= value;
    }    ///
    /// Set to enable use of all of the service counter memory for packet counting.
    pub fn stat_srv_pkt_only(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_stat_srv_pkt_only(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Selects the port or service for which counters can be accessed using QSYS:STAT. Also used to select the port or service index for which to clear statistics counters, ref. STAT_CFG.STAT_CLEAR_SHOT.
    pub fn stat_view(&self) -> u32 {
        (self.0 & 0x3ffe0) >> 5
    }
    pub fn set_stat_view(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x3ffe0);
        self.0 &= !0x3ffe0;
        self.0 |= value;
    }    ///
    /// Counters are by default wrapping when exceeding their maximum value, and software must thus do a subtraction with the previous readen value to see how much the total count has changed. If wrapping is disabled, the counters will clear on read, and saturate at their maximum value. Software can thus detect that a counter overflow has happened, and do not need storing the previous read values. The configuration exists replicated per statistics group as the STAT_CLEAR_SHOT describes.
    pub fn stat_wrap_dis(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_stat_wrap_dis(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}