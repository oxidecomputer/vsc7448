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
    #[inline]
    pub fn ccm_copy_once_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline]
    pub fn set_ccm_copy_once_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Interval used for automatically setting CCM_COPY_ONCE_ENA based on REW::MIP_CTRL.MIP_CCM_HMO_SET_SHOT CCM_COPY_ONCE_ENA are only set by hardware if MIP_CCM_INTERVAL_MASK[CCM_HMO_CTRL.CCM_INTERVAL] is set
    #[inline]
    pub fn ccm_interval(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline]
    pub fn set_ccm_interval(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
}
/// MAC address - bits 47:32
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct LBM_MAC_HIGH(u32);
impl LBM_MAC_HIGH {
    /// Destination MAC address bits 47:32 used for LBM. If LBM_MAC_HIGH = 0 and LBM_MAC_LOW = 0, the MAC address check for LBM frames is disabled.
    #[inline]
    pub fn lbm_mac_high(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline]
    pub fn set_lbm_mac_high(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// MAC address configuration - bits 31:0
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct LBM_MAC_LOW(u32);
impl LBM_MAC_LOW {
    /// Destination MAC address bit 31:0 used for LBM. See LBM_MAC_HIGH.
    #[inline]
    pub fn lbm_mac_low(&self) -> u32 {
        self.0
    }
    #[inline]
    pub fn set_lbm_mac_low(&mut self, value: u32) {
        self.0 = value;
    }
}
/// MIP configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MIP_CFG(u32);
impl MIP_CFG {
    /// If set, OAM Y.1731 CCM frames with the correct encapsulation and the correct MEL are copied to the CPU.
    #[inline]
    pub fn ccm_copy_ena(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline]
    pub fn set_ccm_copy_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// CPU extraction queue when frame is forwarded to CPU.
    #[inline]
    pub fn cpu_mip_qu(&self) -> u32 {
        (self.0 & 0xe) >> 1
    }
    #[inline]
    pub fn set_cpu_mip_qu(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 1;
        self.0 &= !0xe;
        self.0 |= value;
    }
    /// Handling of OAM Y.1731 frames with Opcode=GENERIC_OPCODE_VAL, correct encapsulation, and correct MEL.
    ///
    /// 0: No handling 1: Copy to CPU 2: Redirect to CPU 3: Discard
    #[inline]
    pub fn generic_opcode_cfg(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    #[inline]
    pub fn set_generic_opcode_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 4;
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// Generic Opcode. See GENERIC_OPCODE_CFG.
    #[inline]
    pub fn generic_opcode_val(&self) -> u32 {
        (self.0 & 0x3fc0) >> 6
    }
    #[inline]
    pub fn set_generic_opcode_val(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 6;
        self.0 &= !0x3fc0;
        self.0 |= value;
    }
    /// If set, OAM Y.1731 LBM frames with the correct encapsulation, the correct MEL, and the correct destination MAC address are redirected to the CPU.
    #[inline]
    pub fn lbm_redir_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline]
    pub fn set_lbm_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// If set, OAM Y.1731 LTM frames with the correct encapsulation and the correct MEL are redirected to the CPU.
    #[inline]
    pub fn ltm_redir_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline]
    pub fn set_ltm_redir_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// MEL value for the MIP.
    #[inline]
    pub fn mel_val(&self) -> u32 {
        (self.0 & 0x380000) >> 19
    }
    #[inline]
    pub fn set_mel_val(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 19;
        self.0 &= !0x380000;
        self.0 |= value;
    }
    /// MIP location. This influences in particular the update of loss measurement statistics for MEPs.
    ///
    /// 0: REW_IN_MIP 1: REW_OU_MIP
    #[inline]
    pub fn pipeline_pt(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline]
    pub fn set_pipeline_pt(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Handling of OAM Y.1731 frames with Opcode=RAPS, correct encapsulation, and correct MEL.
    ///
    /// 0: No handling 1: Copy to CPU 2: Redirect to CPU 3: Discard
    #[inline]
    pub fn raps_cfg(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    #[inline]
    pub fn set_raps_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 14;
        self.0 &= !0xc000;
        self.0 |= value;
    }
}
/// Controls classified VID check
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MIP_VID_CTRL(u32);
impl MIP_VID_CTRL {
    /// Outer VID check. Configure how the outer frame VID is matched before a frame is accepted as MIP.
    ///
    /// 0: VID check is disabled. Frame is always accepted. 1: Accept untagged frames. Tagged frames are not accepted. 2: Accept single tagged frames with VID = VID_VAL. Untagged frames or frames with multiple VLAN tags are not accepted. 3: Accept untagged frames and single tagged frames with VID = VID_VAL. Frames with multiple VLAN tags are not accepted.
    #[inline]
    pub fn vid_sel(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline]
    pub fn set_vid_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Required outer VID to identify frame as MIP.
    ///
    /// n: Outer VID value
    #[inline]
    pub fn vid_val(&self) -> u32 {
        (self.0 & 0x3ffc) >> 2
    }
    #[inline]
    pub fn set_vid_val(&mut self, value: u32) {
        assert!(value <= 0xfff);
        let value = value << 2;
        self.0 &= !0x3ffc;
        self.0 |= value;
    }
}
