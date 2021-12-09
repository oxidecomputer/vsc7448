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
/// Event mask for counters.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct STAT_GLOBAL_EVENT_MASK(u32);
impl STAT_GLOBAL_EVENT_MASK {
    /// This value stores the event mask which indicates the counter of all flows to count certain events. If set to '1' the respective event is not filtered and can trigger the counter. If set to '0' the respective event is filtered and the counter will treat the frame as if no event has occurred. Which type of frame is counted is defined in: STAT_CFG, CFG_CNT_FRM_TYPE. The following events apply to irleg stat: Bit0: Count acl_discarded traffic Bit1: Count received IP UC traffic Bit2: Count received IP MC traffic Bit3: Count ip_uc_routed traffic Bit4: Count ip_mc_routed traffic Bit5: Count ip_mc_rpf_discarded traffic Bit6: Count ip_ttl_discarded traffic
    ///
    /// 0: This event will not trigger counting. 1: Enable counting for frames with this event.
    pub fn global_event_mask(&self) -> u32 {
        self.0 & 0x7f
    }
    pub fn set_global_event_mask(&mut self, value: u32) {
        assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }
}
/// The counter's least significant 32 bits.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct STAT_LSB_CNT(u32);
impl STAT_LSB_CNT {
    /// This register contains the least significant 32 bits of a counter.
    pub fn lsb_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_lsb_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
