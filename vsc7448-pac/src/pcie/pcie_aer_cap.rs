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
/// Word offset 6
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ADV_ERR_CAP_CTRL(u32);
impl ADV_ERR_CAP_CTRL {
    pub fn ecrc_check_cap(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_ecrc_check_cap(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    pub fn ecrc_check_en(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_ecrc_check_en(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    pub fn ecrc_gen_cap(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_ecrc_gen_cap(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    pub fn ecrc_gen_en(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_ecrc_gen_en(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    pub fn first_err_pointer(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_first_err_pointer(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}
/// Word offset 0
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct AER_EXT_CAP_HDR(u32);
impl AER_EXT_CAP_HDR {
    pub fn cap_id(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_cap_id(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    pub fn cap_version(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    pub fn set_cap_version(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xf0000);
        self.0 &= !0xf0000;
        self.0 |= value;
    }
    pub fn next_offset(&self) -> u32 {
        (self.0 & 0xfff00000) >> 20
    }
    pub fn set_next_offset(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0xfff00000);
        self.0 &= !0xfff00000;
        self.0 |= value;
    }
}
/// Word offset 5
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CORR_ERR_MASK(u32);
impl CORR_ERR_MASK {
    pub fn advisory_non_fatal_err_mask(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_advisory_non_fatal_err_mask(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    pub fn bad_dllp_mask(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_bad_dllp_mask(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    pub fn bad_tlp_mask(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_bad_tlp_mask(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    pub fn corrected_int_err_mask(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_corrected_int_err_mask(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    pub fn replay_no_roleover_mask(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_replay_no_roleover_mask(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    pub fn rpl_timer_timeout_mask(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_rpl_timer_timeout_mask(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    pub fn rx_err_mask(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_rx_err_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Word offset 4
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CORR_ERR_STATUS(u32);
impl CORR_ERR_STATUS {
    pub fn advisory_non_fatal_err_status(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_advisory_non_fatal_err_status(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    pub fn bad_dllp_status(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_bad_dllp_status(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    pub fn bad_tlp_status(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_bad_tlp_status(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    pub fn corrected_int_err_status(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_corrected_int_err_status(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    pub fn replay_no_roleover_status(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_replay_no_roleover_status(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    pub fn rpl_timer_timeout_status(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_rpl_timer_timeout_status(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    pub fn rx_err_status(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_rx_err_status(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Word offset 7
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct HDR_LOG_REG_0(u32);
impl HDR_LOG_REG_0 {
    pub fn first_dword(&self) -> u32 {
        self.0
    }
    pub fn set_first_dword(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Word offset 8
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct HDR_LOG_REG_1(u32);
impl HDR_LOG_REG_1 {
    pub fn second_dword(&self) -> u32 {
        self.0
    }
    pub fn set_second_dword(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Word offset 9
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct HDR_LOG_REG_2(u32);
impl HDR_LOG_REG_2 {
    pub fn third_dword(&self) -> u32 {
        self.0
    }
    pub fn set_third_dword(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Word offset 10
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct HDR_LOG_REG_3(u32);
impl HDR_LOG_REG_3 {
    pub fn fourth_dword(&self) -> u32 {
        self.0
    }
    pub fn set_fourth_dword(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Word offset 2
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct UNCORR_ERR_MASK(u32);
impl UNCORR_ERR_MASK {
    pub fn atomic_egress_blocked_err_mask(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_atomic_egress_blocked_err_mask(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    pub fn cmplt_abort_err_mask(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_cmplt_abort_err_mask(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    pub fn cmplt_timeout_err_mask(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_cmplt_timeout_err_mask(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    pub fn dl_protocol_err_mask(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_dl_protocol_err_mask(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    pub fn ecrc_err_mask(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_ecrc_err_mask(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    pub fn fc_protocol_err_mask(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_fc_protocol_err_mask(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    pub fn internal_err_mask(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_internal_err_mask(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
    pub fn malf_tlp_err_mask(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_malf_tlp_err_mask(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    pub fn pois_tlp_err_mask(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_pois_tlp_err_mask(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    pub fn rec_overflow_err_mask(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rec_overflow_err_mask(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    pub fn sur_dwn_err_mask(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_sur_dwn_err_mask(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    pub fn unexp_cmplt_err_mask(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_unexp_cmplt_err_mask(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    pub fn unsupported_req_err_mask(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_unsupported_req_err_mask(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
}
/// Word offset 3
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct UNCORR_ERR_SEV(u32);
impl UNCORR_ERR_SEV {
    pub fn atomic_egress_blocked_err_severity(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_atomic_egress_blocked_err_severity(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    pub fn cmplt_abort_err_severity(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_cmplt_abort_err_severity(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    pub fn cmplt_timeout_err_severity(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_cmplt_timeout_err_severity(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    pub fn dl_protocol_err_severity(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_dl_protocol_err_severity(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    pub fn ecrc_err_severity(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_ecrc_err_severity(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    pub fn fc_protocol_err_severity(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_fc_protocol_err_severity(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    pub fn internal_err_severity(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_internal_err_severity(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
    pub fn malf_tlp_err_severity(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_malf_tlp_err_severity(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    pub fn pois_tlp_err_severity(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_pois_tlp_err_severity(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    pub fn rec_overflow_err_severity(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rec_overflow_err_severity(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    pub fn sur_dwn_err_severity(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_sur_dwn_err_severity(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    pub fn unexp_cmplt_err_severity(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_unexp_cmplt_err_severity(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    pub fn unsupported_req_err_severity(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_unsupported_req_err_severity(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
}
/// Word offset 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct UNCORR_ERR_STATUS(u32);
impl UNCORR_ERR_STATUS {
    pub fn atomic_egress_blocked_err_status(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_atomic_egress_blocked_err_status(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    pub fn cmplt_abort_err_status(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_cmplt_abort_err_status(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    pub fn cmplt_timeout_err_status(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_cmplt_timeout_err_status(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    pub fn dl_protocol_err_status(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_dl_protocol_err_status(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    pub fn ecrc_err_status(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_ecrc_err_status(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    pub fn fc_protocol_err_status(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_fc_protocol_err_status(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    pub fn internal_err_status(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_internal_err_status(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
    pub fn malf_tlp_err_status(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_malf_tlp_err_status(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    pub fn pois_tlp_err_status(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_pois_tlp_err_status(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    pub fn rec_overflow_err_status(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rec_overflow_err_status(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    pub fn sur_dwn_err_status(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_sur_dwn_err_status(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    pub fn unexp_cmplt_err_status(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_unexp_cmplt_err_status(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    pub fn unsupported_req_err_status(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_unsupported_req_err_status(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
}
