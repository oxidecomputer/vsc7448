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
/// DTI counter
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DTI_CNT(u32);
impl DTI_CNT {
    /// DTI_MODE.MODE=0, 2: Number of remaining frame sequences to inject. Configured by SW, decremented by AFI. DTI_MODE.MODE=1: Number of frames injected.
    #[inline(always)]
    pub fn cnt(&self) -> u32 {
        self.0 & 0x7fffffff
    }
    #[inline(always)]
    pub fn set_cnt(&mut self, value: u32) {
        debug_assert!(value <= 0x7fffffff);
        self.0 &= !0x7fffffff;
        self.0 |= value;
    }
}
/// Duration of last DTI run
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DTI_DURATION(u32);
impl DTI_DURATION {
    /// Duration of last DTI run in DTI Duration Ticks. Before starting a DTI, DURATION must be set to 0. When AFI:DTI_MISC:DTI_CTRL.ENA becomes 0, DURATION is updated with the duration of the DTI run. While a DTI is running DURATION holds an internal time stamp of when the DTI was started. This value is not intended for SW usage. Related parameters: AFI:MISC:DTI_DURATION_TICK_LEN.DTI_DURATION_TICK_LEN
    #[inline(always)]
    pub fn duration(&self) -> u32 {
        self.0 & 0x7fffffff
    }
    #[inline(always)]
    pub fn set_duration(&mut self, value: u32) {
        debug_assert!(value <= 0x7fffffff);
        self.0 &= !0x7fffffff;
        self.0 |= value;
    }
}
/// DTI Frame Table pointers
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DTI_FRM(u32);
impl DTI_FRM {
    /// Pointer to first frame in frame sequence.
    #[inline(always)]
    pub fn first_frm_ptr(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_first_frm_ptr(&mut self, value: u32) {
        debug_assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    /// Pointer to next frame in frame sequence. Must be set to same value as FIRST_FRM_PTR when (re)starting DTI.
    #[inline(always)]
    pub fn next_frm_ptr(&self) -> u32 {
        (self.0 & 0xfff0000) >> 16
    }
    #[inline(always)]
    pub fn set_next_frm_ptr(&mut self, value: u32) {
        debug_assert!(value <= 0xfff);
        let value = value << 16;
        self.0 &= !0xfff0000;
        self.0 |= value;
    }
}
/// Configuration of DTI Mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DTI_MODE(u32);
impl DTI_MODE {
    /// See AFI:DTI_TBL:DTI_MODE.MODE, encoding 2.
    #[inline(always)]
    pub fn dti_next(&self) -> u32 {
        (self.0 & 0x1f000000) >> 24
    }
    #[inline(always)]
    pub fn set_dti_next(&mut self, value: u32) {
        debug_assert!(value <= 0x1f);
        let value = value << 24;
        self.0 &= !0x1f000000;
        self.0 |= value;
    }
    /// Number of clock cycles the DTI shall be postponed after experiencing flow control from FRD. Such DTIs may have to be postponed to avoid blocking frame table access for TTIs. Value should be big enough to cover situation where all active DTIs are flow controlled. The configured value is counted down in DTI_FC_CNT_DOWN and when zero is reached, another injection attempt is made. In the meantime DTI_CNT_DOWN goes negative, such that the additional delay is subtracted from the next delay in the DTI sequence.
    #[inline(always)]
    pub fn fc_postpone_len(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    #[inline(always)]
    pub fn set_fc_postpone_len(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        let value = value << 16;
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    /// Controls whether FC_POSTPONE_LEN is added to DTI_FC_CNT_DOWN or DTI_CNT_DOWN when flow control from FRD is encountered.
    ///
    /// 0: Add FC_POSTPONE_LEN to DTI_FC_CNT_DOWN when FC is encountered. 1: Add FC_POSTPONE_LEN to DTI_CNT_DOWN when FC is encountered.
    #[inline(always)]
    pub fn fc_postpone_mode(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    #[inline(always)]
    pub fn set_fc_postpone_mode(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 31;
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Number of remaining injections of current frame in frame sequence (NEXT_FRM_PTR). Must be set to 0 when (re)starting DTI.
    #[inline(always)]
    pub fn frm_inj_cnt(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline(always)]
    pub fn set_frm_inj_cnt(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// Configuration of DTI mode.
    ///
    /// 0: Frame sequence shall be injected DTI_CNT.CNT times. 1: Frame sequence shall be injected until stopped (DTI_CTRL.ENA=0). Number of frames injected is counted in DTI_CNT.CNT. 2: Frame sequence shall be injected DTI_CNT.CNT times. Once this is done, the DTI pointed to by AFI:DTI_TBL:DTI_MODE.DTI_NEXT will be enabled. This can be used to concatenate DTIs. 3: Reserved.
    #[inline(always)]
    pub fn mode(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_mode(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// If the last frame table entry of a DTI sequence is a delay, then this is termed a "trailing delay". By setting TRAILING_DELAY_SEQ_CNT=N a trailing delay is only applied for every Nth sequence injection. This can be used to "fine tune" the bandwidth of a DTI sequence. TRAILING_DELAY_SEQ_CNT=0 disables this feature. If a DTI sequence (in the frame table) has no trailing delay, then TRAILING_DELAY_SEQ_CNT has no effect.
    ///
    /// 0: Disable feature. 1: Apply trailing delay for every sequence injected. 2: Apply trailing delay for every 2nd sequence injected. 3: Apply trailing delay for every 3rd sequence injected. ...
    #[inline(always)]
    pub fn trailing_delay_seq_cnt(&self) -> u32 {
        (self.0 & 0xfc) >> 2
    }
    #[inline(always)]
    pub fn set_trailing_delay_seq_cnt(&mut self, value: u32) {
        debug_assert!(value <= 0x3f);
        let value = value << 2;
        self.0 &= !0xfc;
        self.0 |= value;
    }
}
/// Port and queue for injected frames.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DTI_PORT_QU(u32);
impl DTI_PORT_QU {
    /// Port number which injection queue transmits on. Injection queue is selected by QU_NUM.
    #[inline(always)]
    pub fn port_num(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_port_num(&mut self, value: u32) {
        debug_assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// QU_NUM selects the queue, which the frame is injected into. For details, refer to the functional description of the queue system in the datasheet.
    #[inline(always)]
    pub fn qu_num(&self) -> u32 {
        (self.0 & 0xffff00) >> 8
    }
    #[inline(always)]
    pub fn set_qu_num(&mut self, value: u32) {
        debug_assert!(value <= 0xffff);
        let value = value << 8;
        self.0 &= !0xffff00;
        self.0 |= value;
    }
}
