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

/// Register `DTI_CNT_DOWN`
///
/// DTI count down counters
#[derive(From, Into)]
pub struct DTI_CNT_DOWN(u32);
impl DTI_CNT_DOWN {
    /// Remaining number of clock cycles before next injection. May become negative while waiting for table/injection access. Two's complement encoded. Should be set to 0 when (re)starting DTI (unless an initial delay is desirable).
    pub fn cnt_down(&self) -> u32 {
        self.0 & 0x7fffffff
    }
    pub fn set_cnt_down(&mut self, value: u32) {
        assert!(value <= 0x7fffffff);
        self.0 &= !0x7fffffff;
        self.0 |= value;
    }
}

/// Register `DTI_DURATION`
///
/// Duration of last DTI run
#[derive(From, Into)]
pub struct DTI_DURATION(u32);
impl DTI_DURATION {
    /// Duration of last DTI run in DTI Duration Ticks. Before starting a DTI, DURATION must be set to 0. When AFI:DTI_MISC:DTI_CTRL.ENA becomes 0, DURATION is updated with the duration of the DTI run. While a DTI is running DURATION holds an internal time stamp of when the DTI was started. This value is not intended for SW usage. Related parameters: AFI:MISC:DTI_DURATION_TICK_LEN.DTI_DURATION_TICK_LEN
    pub fn duration(&self) -> u32 {
        self.0 & 0x7fffffff
    }
    pub fn set_duration(&mut self, value: u32) {
        assert!(value <= 0x7fffffff);
        self.0 &= !0x7fffffff;
        self.0 |= value;
    }
}

/// Register `DTI_FC_CNT_DOWN`
///
/// DTI flow control count down counters
///
/// When FC is experienced between AFI and FRD, then DTI an injecting DTI must "backoff" for FC_POSTPONE_LEN clock cycles. This is handled by this counter.
#[derive(From, Into)]
pub struct DTI_FC_CNT_DOWN(u32);
impl DTI_FC_CNT_DOWN {
    /// Remaining number of clock cycles before DTI is allowed to attempt injection again after experiencing FC from FRD. This field is set to FC_POSTPONE_LEN when FC from FRD is experienced. Note: Unlike CNT_DOWN, FC_CNT_DOWN is always >=0 (so no two's complement encoding).
    pub fn fc_cnt_down(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_fc_cnt_down(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
