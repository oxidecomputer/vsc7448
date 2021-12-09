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
/// Configuration of mapping tables 0 and 1
///
/// Lookup 0 and 1 values
#[derive(From, Into)]
pub struct MAP_VAL_A(u32);
impl MAP_VAL_A {
    /// Mapped DEI value
    ///
    /// n: New DEI value
    pub fn dei_val(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_dei_val(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Mapped DSCP value
    ///
    /// n: New DSCP value
    pub fn dscp_val(&self) -> u32 {
        (self.0 & 0x3f0) >> 4
    }
    pub fn set_dscp_val(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x3f0);
        self.0 &= !0x3f0;
        self.0 |= value;
    }
    /// Mapped OAM COLOR value
    ///
    /// n: New OAM COLOR value
    pub fn oam_color(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_oam_color(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Mapped OAM COSID value
    ///
    /// n: New OAM COSID value
    pub fn oam_cosid(&self) -> u32 {
        (self.0 & 0xe000) >> 13
    }
    pub fn set_oam_cosid(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0xe000);
        self.0 &= !0xe000;
        self.0 |= value;
    }
    /// Mapped PCP value
    ///
    /// n: New PCP value
    pub fn pcp_val(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_pcp_val(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Mapped TC value
    ///
    /// n: New TC value
    pub fn tc_val(&self) -> u32 {
        (self.0 & 0x1c00) >> 10
    }
    pub fn set_tc_val(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x1c00);
        self.0 &= !0x1c00;
        self.0 |= value;
    }
}
/// MIP sticky bit register
///
/// Event register common for all MIPs
#[derive(From, Into)]
pub struct MIP_STICKY_EVENT(u32);
impl MIP_STICKY_EVENT {
    /// This bit is set if a CCM CPU is copied to CPU
    ///
    /// 0: No event 1: Event Bit is cleared by writing a 1 to this position.
    pub fn mip_ccm_copy_sticky(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_mip_ccm_copy_sticky(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// This bit is set if a Generic PDU has been handled
    ///
    /// 0: No event 1: Event Bit is cleared by writing a 1 to this position.
    pub fn mip_generic_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_mip_generic_sticky(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// This bit is set if a destination MAC address check has failed for LBM frame
    ///
    /// 0: No event 1: Event Bit is cleared by writing a 1 to this position.
    pub fn mip_lbm_da_chk_fail_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_mip_lbm_da_chk_fail_sticky(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// This bit is set if a LBM PDU has been redirected to the CPU
    ///
    /// 0: No event 1: Event Bit is cleared by writing a 1 to this position.
    pub fn mip_lbm_redir_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_mip_lbm_redir_sticky(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// This bit is set if a LTM PDU has been redirected to the CPU
    ///
    /// 0: No event 1: Event Bit is cleared by writing a 1 to this position.
    pub fn mip_ltm_redir_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_mip_ltm_redir_sticky(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// This bit is set if a MEL check has failed for enabled OAM frames
    ///
    /// 0: No event 1: Event Bit is cleared by writing a 1 to this position.
    pub fn mip_mel_chk_fail_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_mip_mel_chk_fail_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// This bit is set if a Ring APS PDU has been handled
    ///
    /// 0: No event 1: Event Bit is cleared by writing a 1 to this position.
    pub fn mip_raps_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_mip_raps_sticky(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
}
