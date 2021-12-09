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
/// Errors from AFI block
///
/// These bits shall never get set.
#[derive(From, Into)]
pub struct ERR(u32);
impl ERR {
    /// FRM_OUT_CNT for a port was zero while an ack from FRD was received. If enabled in STICKY_INFO_ENA.FRM_OUT_NEG_INFO_ENA, the corresponding port number is stored in STICKY_INFO.PORT_NUM.
    pub fn err_frm_out_neg_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_err_frm_out_neg_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Miscellanous AFI control parameters
#[derive(From, Into)]
pub struct MISC_CTRL(u32);
impl MISC_CTRL {
    /// Enable AFI. Must be set to 1 before any use of AFI.
    pub fn afi_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_afi_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Control information about new frame received by AFI for injection.
#[derive(From, Into)]
pub struct NEW_FRM_CTRL(u32);
impl NEW_FRM_CTRL {
    /// Valid bit for NEW_FRM_INFO.FRM_INFO.
    pub fn vld(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vld(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Frame information about new frame received by AFI for injection.
#[derive(From, Into)]
pub struct NEW_FRM_INFO(u32);
impl NEW_FRM_INFO {
    /// Frame information for new frame received by AFI. The inforrmation must be copied to FRM_INFO bits in AFI:FRM_TBL:FRM_ENTRY_PART0.PART0. Once FRM_INFO has been copied to FRM_TBL, then NEW_FRM_CTRL.VLD must be cleared.
    pub fn frm_info(&self) -> u32 {
        self.0 & 0x7ffff
    }
    pub fn set_frm_info(&mut self, value: u32) {
        assert!(value <= 0x7ffff);
        self.0 &= !0x7ffff;
        self.0 |= value;
    }
}
/// Additional information about sticky bit events.
///
/// The information is updated if a) Enabled in STICKY_INFO_ENA and b) STICKY_INFO_WR_CNT is > 0
#[derive(From, Into)]
pub struct STICKY_INFO(u32);
impl STICKY_INFO {
    /// Port number corresponding to sticky bit event.
    pub fn port_num(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_port_num(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// Number of times STICKY_INFO.PORT_NUM and  STICKY_INFO.TTI_PTR is allowed to be updated.
    ///
    /// 0: Illegal 1: Max one update of STICKY_INFO allowed (decremented for each update). 2: Max two updates of STICKY_INFO allowed (decremented for each update). ... 15: Any number of updates of STICKY_INFO allowed (not decremented).
    pub fn sticky_info_wr_cnt(&self) -> u32 {
        (self.0 & 0xf0000000) >> 28
    }
    pub fn set_sticky_info_wr_cnt(&mut self, value: u32) {
        let value = value << 28;
        assert!(value <= 0xf0000000);
        self.0 &= !0xf0000000;
        self.0 |= value;
    }
    /// TTI pointer corresponding to sticky bit event.
    pub fn tti_ptr(&self) -> u32 {
        (self.0 & 0xfff00) >> 8
    }
    pub fn set_tti_ptr(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xfff00);
        self.0 &= !0xfff00;
        self.0 |= value;
    }
}
/// Enabling of additional information about sticky bit events.
#[derive(From, Into)]
pub struct STICKY_INFO_ENA(u32);
impl STICKY_INFO_ENA {
    /// Enable updating of STICKY_INFO.PORT_NUM for WARN_ENQ_STOP_STICKY.
    pub fn enq_stop_info_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_enq_stop_info_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable updating of STICKY_INFO.PORT_NUM for WARN_FRM_OUT_MAX_STICKY.
    pub fn frm_out_max_info_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_frm_out_max_info_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enable updating of STICKY_INFO.PORT_NUM for ERR_FRM_OUT_NEG_STICKY.
    pub fn frm_out_neg_info_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_frm_out_neg_info_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable updating of STICKY_INFO.TTI_PTR for WARN_TTI_BUSY_STICKY.
    pub fn tti_busy_info_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_tti_busy_info_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// Warnings from AFI block
///
/// These bits shall normally not get set.
#[derive(From, Into)]
pub struct WARN(u32);
impl WARN {
    /// DTI_CNT_DOWN has reached its maximum negative value.
    pub fn warn_dti_cnt_down_max_neg_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_warn_dti_cnt_down_max_neg_sticky(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// QSYS has asserted enq_stop. i.e. requested AFI to stop injecting frames. This should normally not occur. If enabled in STICKY_INFO_ENA.ENQ_STOP_INFO_ENA, the affected port number is stored in STICKY_INFO.PORT_NUM.
    pub fn warn_enq_stop_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_warn_enq_stop_sticky(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// FRM_OUT_MAX has been reached for port. This is unusual, but may occur when a large number of frames are successively injected to queues of the same port or if the port flow controlled or is shaped to a low bandwidth. The number of injections that can be outstanding (i.e. waiting to be transmitted out of QSYS) for a port	at a time is configured in AFI:PORT_TBL:PORT_CFG.FRM_OUT_MAX. If enabled in STICKY_INFO_ENA.FRM_OUT_MAX_INFO_ENA, the corresponding port number is stored in STICKY_INFO.PORT_NUM.
    pub fn warn_frm_out_max_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_warn_frm_out_max_sticky(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// A new frame for injection was received by AFI, but NEW_FRM_CTRL.VLD was still 1. Check that NEW_FRM_CTRL.VLD is cleared upon copying NEW_FRM_INFO.FRM_INFO to FRM_TBL.
    pub fn warn_new_frm_vld_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_warn_new_frm_vld_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// A TTI in TTI_TBL was elected to be processed, but was already being processed. This may indicate that the length of a calendar slot is too short (ref. AFI:TTI_MISC:TTI_CAL_SLOT_PTRS). If enabled in STICKY_INFO_ENA.TTI_BUSY_INFO_ENA, the corresponding TTI pointer is stored in STICKY_INFO.TTI_PTR.
    pub fn warn_tti_busy_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_warn_tti_busy_sticky(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
}
