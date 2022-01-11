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
/// Common frame type configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GLOBAL_CNT_FRM_TYPE_CFG(u32);
impl GLOBAL_CNT_FRM_TYPE_CFG {
    /// Configures which frames is counted in relation to FCS error and configured event mask (GLOBAL_EVENT_MASK).
    ///
    /// "000": Frames without any event signal or FCS errored frames are counted. "001": Frames with unmasked (enabled) events without FCS error are counted. "010": Frames with unmasked (enabled) events with FCS error are counted. "011": Frames with unmasked (enabled) events independent of FCS error are counted. "100": Frames with FCS error but with no event signal are counted. "101": Frames with FCS error are unconditionally counted.
    #[inline]
    pub fn global_cfg_cnt_frm_type(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline]
    pub fn set_global_cfg_cnt_frm_type(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Global configuration register
///
/// Global configuration register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct STAT_GLOBAL_CFG(u32);
impl STAT_GLOBAL_CFG {
    /// This field is to configure the counters of all flows to count the byte number or the frame number.
    ///
    /// '0': Count frames. '1': Count bytes.
    #[inline]
    pub fn global_cfg_cnt_byte(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline]
    pub fn set_global_cfg_cnt_byte(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Event mask for counters.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct STAT_GLOBAL_EVENT_MASK(u32);
impl STAT_GLOBAL_EVENT_MASK {
    /// This value stores the event mask which indicates the counter of all flows to count certain events. If set to '1' the respective event is not filtered and can trigger the counter. If set to '0' the respective event is filtered and the counter will treat the frame as if no event has occurred. The following events apply to erleg stat: Bit0: Count acl_discarded traffic Bit1: Count ip_uc_routed traffic Bit2: Count ip_mc_routed traffic Bit3: Count ip_mc_switched traffic Bit4: Count ip_mc_ttl_discarded traffic
    ///
    /// 0: This event will not trigger counting. 1: Enable counting for frames with this event.
    #[inline]
    pub fn global_event_mask(&self) -> u32 {
        self.0 & 0x1f
    }
    #[inline]
    pub fn set_global_event_mask(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}
