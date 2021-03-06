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
/// Event handling configuration.
///
/// This register group defines how to handle the incoming events.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct STAT_CFG(u32);
impl STAT_CFG {
    /// Configure whether to count frames or bytes.
    ///
    /// '0': Count frames. '1': Count bytes.
    #[inline(always)]
    pub fn cfg_cnt_byte(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_cfg_cnt_byte(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Selects which frames to count.
    ///
    /// "000": The frames without any event signal or frame error signal asserted are counted. "001": The frames with unmasked (enabled) events asserted but with no error indications are counted. "010": The frames with both event signal and the error signal asserted are counted. "011": The frames with event signal asserted are counted in spite of the error indications. "100": The frames with the error signal asserted, but with no event signal are counted. "101": The frames with error signal asserted are counted in spite of the accompied event indications. | Error | Event -----+-------+------ 000 |   N	|   N -----+-------+------ 001 |   N	|   Y -----+-------+------ 010 |   Y	|   Y -----+-------+------ 011 |   -	|   Y -----+-------+------ 100 |   Y	|   N -----+-------+------ 101 |   Y	|   - -----+-------+------
    #[inline(always)]
    pub fn cfg_cnt_frm_type(&self) -> u32 {
        (self.0 & 0xe) >> 1
    }
    #[inline(always)]
    pub fn set_cfg_cnt_frm_type(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 1;
        self.0 &= !0xe;
        self.0 |= value;
    }
    /// This field is to configure the counters of a flow to count frames with certain priorities. The field contains one bit per priority. Note that with the default value of this field, counting is disabled for all priorities.
    ///
    /// 0: Do not count frames with this priority. 1: Count frames with this priority.
    #[inline(always)]
    pub fn cfg_prio_mask(&self) -> u32 {
        (self.0 & 0xff0) >> 4
    }
    #[inline(always)]
    pub fn set_cfg_prio_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 4;
        self.0 &= !0xff0;
        self.0 |= value;
    }
}
/// Sticky bits for events.
///
/// It is the sticky bits of events. If one event is triggered, the corresponding bit is set to '1' before it is cleared. To write '1' into this bit will clear the sticky bit.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct STAT_EVENTS_STICKY(u32);
impl STAT_EVENTS_STICKY {
    /// These are the sticky bits of events. There is a sticky bit for an event for each flow.
    ///
    /// '1': The corresponding event is triggered since it is cleared last time. '0': No such event is triggered since it is cleared last time.
    #[inline(always)]
    pub fn sticky_bits(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_sticky_bits(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// The counter's least significant 32 bits.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct STAT_LSB_CNT(u32);
impl STAT_LSB_CNT {
    /// This register contains the least significant 32 bits of a counter.
    #[inline(always)]
    pub fn lsb_cnt(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_lsb_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// The counter's most significant 8 bits.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct STAT_MSB_CNT(u32);
impl STAT_MSB_CNT {
    /// The counter's most significant 8 bits. The field stores the value in the counters of a flow from bit 32 to the most significant bit. Reading: The MSB part of the counter is latched to a shadow register, when the LSB part is read. As a result, the LSB part must always be read first, and the MSB part must be read immediately after the LSB part is read. Writing: The procedure for writing differs depending on counter group: ANA_AC:STAT_CNT_CFG_PORT: LSB part must be written first, followed by MSB part. All other counter groups: MSB part must be written first, followed by LSB part.
    #[inline(always)]
    pub fn msb_cnt(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_msb_cnt(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
