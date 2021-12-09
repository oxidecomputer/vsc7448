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
/// Allows sending a single CCM frame to CPU
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CCM_HMO_CTRL(u32);
impl CCM_HMO_CTRL {
    /// Send the next received CCM frame to CPU. Cleared by HW when a CPU copy has been send to CPU
    pub fn ccm_copy_once_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ccm_copy_once_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Interval used for setting CCM_COPY_ONCE_ENA based on ANA_CL::MIP_CTRL.MIP_CCM_HMO_SET_SHOT. CCM_COPY_ONCE_ENA are only set by hardware if MIP_CCM_INTERVAL_MASK[CCM_HMO_CTRL.CCM_INTERVAL] is set
    pub fn ccm_interval(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_ccm_interval(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x6);
        self.0 &= !0x6;
        self.0 |= value;
    }
}
/// MAC address - bits 47:32
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct LBM_MAC_HIGH(u32);
impl LBM_MAC_HIGH {
    /// Destination MAC address bits 47:32 used for LBM. If LBM_MAC_HIGH = 0 and LBM_MAC_LOW = 0, the MAC address check for LBM frames is disabled.
    pub fn lbm_mac_high(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_lbm_mac_high(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// MIP configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MIP_CFG(u32);
impl MIP_CFG {
    /// If set, OAM Y.1731 CCM frames with the correct encapsulation and the correct MEL are copied to the CPU.
    pub fn ccm_copy_ena(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_ccm_copy_ena(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// CPU extraction queue when frame is forwarded to CPU.
    pub fn cpu_mip_qu(&self) -> u32 {
        (self.0 & 0xe) >> 1
    }
    pub fn set_cpu_mip_qu(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0xe);
        self.0 &= !0xe;
        self.0 |= value;
    }
    /// Handling of OAM Y.1731 frames with OpCode=GENERIC_OPCODE_VAL, correct encapsulation, and correct MEL.
    ///
    /// 0: No handling 1: Copy to CPU 2: Redirect to CPU 3: Discard
    pub fn generic_opcode_cfg(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_generic_opcode_cfg(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// Generic Opcode. See GENERIC_OPCODE_CFG.
    pub fn generic_opcode_val(&self) -> u32 {
        (self.0 & 0x3fc0) >> 6
    }
    pub fn set_generic_opcode_val(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x3fc0);
        self.0 &= !0x3fc0;
        self.0 |= value;
    }
    /// If set, OAM Y.1731 LBM frames with the correct encapsulation, the correct MEL, and the correct destination MAC address are redirected to the CPU.
    pub fn lbm_redir_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_lbm_redir_ena(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// If set, OAM Y.1731 LTM frames with the correct encapsulation and the correct MEL are redirected to the CPU.
    pub fn ltm_redir_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_ltm_redir_ena(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// MEL value for the MIP.
    pub fn mel_val(&self) -> u32 {
        (self.0 & 0x380000) >> 19
    }
    pub fn set_mel_val(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x380000);
        self.0 &= !0x380000;
        self.0 |= value;
    }
    /// MIP location.
    ///
    /// 0: ANA_IN_MIP 1: ANA_OU_MIP
    pub fn pipeline_pt(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pipeline_pt(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Handling of OAM Y.1731 frames with OpCode=RAPS, correct encapsulation, and correct MEL.
    ///
    /// 0: No handling 1: Copy to CPU 2: Redirect to CPU 3: Discard
    pub fn raps_cfg(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    pub fn set_raps_cfg(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0xc000);
        self.0 &= !0xc000;
        self.0 |= value;
    }
}
/// Controls classified VID check
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MIP_CL_VID_CTRL(u32);
impl MIP_CL_VID_CTRL {
    /// Check of VID to match before frame is accepted as MIP.
    ///
    /// 0: VID check is disabled. Frame is always accepted. 1: Accept untagged frames. Tagged frames are not accepted. 2: Accept tagged frames with outer VID = VID_VAL. Untagged frames are not accepted. 3: Accept untagged frames or tagged frames with outer VID = VID_VAL
    pub fn vid_sel(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_vid_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Required outer VID to identify frame as MIP.
    pub fn vid_val(&self) -> u32 {
        (self.0 & 0x3ffc) >> 2
    }
    pub fn set_vid_val(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x3ffc);
        self.0 &= !0x3ffc;
        self.0 |= value;
    }
}
/// Generalized MPLS exception handling
///
/// This register allows exception handling of reserved MPLS labels and IP control protocols.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PROFILE_CFG(u32);
impl PROFILE_CFG {
    /// If the frame is forwarded to the CPU (via PROFILE_CFG.FWD_SEL), it will be forwarded to the CPU queue configured in this field.
    pub fn cpu_qu(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_cpu_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Controls S2 custom rule selection.
    ///
    /// Bit 0: Selects custom key to use (0: CUSTOM_1, 1: CUSTOM_2) Bit 1: Enables custom key for first lookup Bit 2: Enables custom key for second lookup
    pub fn custom_ace_ena(&self) -> u32 {
        (self.0 & 0x38000) >> 15
    }
    pub fn set_custom_ace_ena(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x38000);
        self.0 &= !0x38000;
        self.0 |= value;
    }
    /// CPU forward configuration per profile.
    ///
    /// 0: Normal forward 1: Enable redirection to CPU queue: PROFILE_CFG.CPU_QU 2: Enable copy to CPU queue: PROFILE_CFG.CPU_QU 3: Discard the frame
    pub fn fwd_sel(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    pub fn set_fwd_sel(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x18);
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// Controls if profile traffic should be part of OAM LM count.
    pub fn lm_cnt_dis(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_lm_cnt_dis(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Controls if profile traffic should be normalized according to normal action handling: nxt_normalize.
    pub fn normalize_dis(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_normalize_dis(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Determines the next CLM lookup. Could also specify no further CLM lookups
    ///
    /// 0: no overrule 2: SGL_MLBS 3: DBL_MLBS 4: TRI_MLBS 12: CUSTOM1 13: CUSTOM2 14: CUSTOM4 15: No Lookup other: reserved
    pub fn nxt_key_type(&self) -> u32 {
        (self.0 & 0x1e0) >> 5
    }
    pub fn set_nxt_key_type(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x1e0);
        self.0 &= !0x1e0;
        self.0 |= value;
    }
    /// Determines frame pointer movement.
    pub fn nxt_norm_w16_offset(&self) -> u32 {
        (self.0 & 0x1e00) >> 9
    }
    pub fn set_nxt_norm_w16_offset(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x1e00);
        self.0 &= !0x1e00;
        self.0 |= value;
    }
    /// Controls Protocol layer (frame_type) at frame pointer position after update PROFILE_CFG.NXT_NORM_W16_OFFSET.
    ///
    /// 0: ETH  - Frame pointer points to start of DMAC. 1: CW (IP / MPLS PW CW / MPLS ACH) - Frame pointer points to MPLS CW/ACH or IP version. 2: MPLS - Frame pointer points to MPLS label. 3: DATA -"Raw" data, i.e. unknown protocol type.
    pub fn nxt_type_after_offset(&self) -> u32 {
        (self.0 & 0x6000) >> 13
    }
    pub fn set_nxt_type_after_offset(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x6000);
        self.0 &= !0x6000;
        self.0 |= value;
    }
}
