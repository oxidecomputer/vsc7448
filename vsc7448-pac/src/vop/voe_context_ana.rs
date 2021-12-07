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

/// Register `CT_CCM_TLV_INFO_ANA`
///
/// Contains info om G.8113.1 LBM/LBR TLVs
#[derive(From, Into)]
pub struct CT_CCM_TLV_INFO_ANA(u32);
impl CT_CCM_TLV_INFO_ANA {    pub fn ct_if_status_value_ana(&self) -> u32 {
        (self.0 & 0x7) >> 0
    }
    pub fn set_ct_if_status_value_ana(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }    pub fn ct_if_status_vld_ana(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_ct_if_status_vld_ana(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    pub fn ct_port_statis_value_ana(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_ct_port_statis_value_ana(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }    pub fn ct_port_status_vld_ana(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_ct_port_status_vld_ana(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
}

/// Register `CT_OAM_DATA1_REW`
///
/// Context data
#[derive(From, Into)]
pub struct CT_OAM_DATA1_REW(u32);
impl CT_OAM_DATA1_REW {    pub fn ct_oam_misc_rew(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_ct_oam_misc_rew(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `CT_OAM_DATA_ANA`
///
/// Analyzer context data
#[derive(From, Into)]
pub struct CT_OAM_DATA_ANA(u32);
impl CT_OAM_DATA_ANA {    ///
    /// Context register containing Sequence Number or Transaction ID
    pub fn ct_oam_seq_ana(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_ct_oam_seq_ana(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `CT_OAM_INFO_ANA`
///
/// [MCC_DEBUG] Context for ports on the ANA interface
///
/// [MCC_DEBUG] Tx LM frame counters  by VOE.
#[derive(From, Into)]
pub struct CT_OAM_INFO_ANA(u32);
impl CT_OAM_INFO_ANA {    pub fn ct_block_data_ana(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_ct_block_data_ana(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }    pub fn ct_ccm_lm_as_sel_ana(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_ct_ccm_lm_as_sel_ana(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }    pub fn ct_chk_seq_ana(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_ct_chk_seq_ana(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG]
    pub fn ct_entry_valid_ana(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_ct_entry_valid_ana(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }    pub fn ct_frame_prio_ana(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_ct_frame_prio_ana(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG]
    ///
    /// 0: OAM Frame is TX 1: OAM Frame is RX
    pub fn ct_lookup_type_ana(&self) -> u32 {
        (self.0 & 0x3c000000) >> 26
    }
    pub fn set_ct_lookup_type_ana(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0x3c000000);
        self.0 &= !0x3c000000;
        self.0 |= value;
    }    pub fn ct_non_oam_err_cnt_ana(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_ct_non_oam_err_cnt_ana(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    pub fn ct_non_oam_fwd_err_ana(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_ct_non_oam_fwd_err_ana(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    pub fn ct_oam_gen_idx_ana(&self) -> u32 {
        (self.0 & 0x1c0000) >> 18
    }
    pub fn set_ct_oam_gen_idx_ana(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x1c0000);
        self.0 &= !0x1c0000;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG] OAM PDU currently being processed
    pub fn ct_oam_pdu_ana(&self) -> u32 {
        (self.0 & 0x3e00000) >> 21
    }
    pub fn set_ct_oam_pdu_ana(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x3e00000);
        self.0 &= !0x3e00000;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG] OAM type currently being processed
    pub fn ct_oam_type_ana(&self) -> u32 {
        (self.0 & 0x3) >> 0
    }
    pub fn set_ct_oam_type_ana(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }    ///
    /// Determines if the PDU is to be counted as Selected OAM or NON Selected OAM.
    ///
    /// 0: Count as NON Selected OAM 1: Count as Selected OAM
    pub fn ct_sel_oam_ana(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_ct_sel_oam_ana(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG] Source port.
    pub fn ct_src_port_ana(&self) -> u32 {
        (self.0 & 0x3f000) >> 12
    }
    pub fn set_ct_src_port_ana(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x3f000);
        self.0 &= !0x3f000;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG]
    pub fn ct_upd_seq_ana(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_ct_upd_seq_ana(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
}

/// Register `CT_OAM_STICKY_ANA`
///
/// [MCC_DEBUG]
///
/// [MCC_DEBUG]
#[derive(From, Into)]
pub struct CT_OAM_STICKY_ANA(u32);
impl CT_OAM_STICKY_ANA {    pub fn ct_ccm_nonzero_endtlv_ana(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_ct_ccm_nonzero_endtlv_ana(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG] PDU was correctly validaded by the VOE and is ready to be processed.
    pub fn ct_ccm_period_err_ana(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_ct_ccm_period_err_ana(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG] PDU was correctly validaded by the VOE and is ready to be processed.
    pub fn ct_ccm_prio_err_ana(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_ct_ccm_prio_err_ana(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG]
    pub fn ct_extract_cause_ana(&self) -> u32 {
        (self.0 & 0x3e000) >> 13
    }
    pub fn set_ct_extract_cause_ana(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x3e000);
        self.0 &= !0x3e000;
        self.0 |= value;
    }    pub fn ct_extract_qu_ana(&self) -> u32 {
        (self.0 & 0x1c00) >> 10
    }
    pub fn set_ct_extract_qu_ana(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x1c00);
        self.0 &= !0x1c00;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG]
    pub fn ct_mel_high_ana(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_ct_mel_high_ana(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG] PDU was correctly validaded by the VOE and is ready to be processed.
    pub fn ct_pdu_hw_ena_ana(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_ct_pdu_hw_ena_ana(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }    pub fn ct_sam_seq_ccm_ana(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_ct_sam_seq_ccm_ana(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }    pub fn ct_sam_seq_idx_ana(&self) -> u32 {
        (self.0 & 0xf8) >> 3
    }
    pub fn set_ct_sam_seq_idx_ana(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0xf8);
        self.0 &= !0xf8;
        self.0 |= value;
    }    pub fn ct_sam_seq_lbm_ana(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_ct_sam_seq_lbm_ana(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }    pub fn ct_synlm_peer_idx_ana(&self) -> u32 {
        (self.0 & 0x7) >> 0
    }
    pub fn set_ct_synlm_peer_idx_ana(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }    ///
    /// [MCC_DEBUG] PDU was correctly validaded by the VOE and is ready to be processed.
    pub fn ct_valid_pdu_ana(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_ct_valid_pdu_ana(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
}
