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

/// Register `AN_SM`
///
/// VS AN arb state machine
#[derive(From, Into)]
pub struct AN_SM(u32);
impl AN_SM {    ///
    /// ABILITY_DETECT state counter
    pub fn abdet_cnt(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    pub fn set_abdet_cnt(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0xf0);
        self.0 &= !0xf0;
        self.0 |= value;
    }    ///
    /// AN state machine
    pub fn an_sm(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_an_sm(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}

/// Register `AN_STS0`
///
/// VS AN status 0
#[derive(From, Into)]
pub struct AN_STS0(u32);
impl AN_STS0 {    ///
    /// Incompatible link (LH)
    pub fn incp_link(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_incp_link(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }    ///
    /// speed setting
    ///
    /// 0: 10G 1: 1G 2: 3G
    pub fn line_rate(&self) -> u32 {
        (self.0 & 0x3) >> 0
    }
    pub fn set_line_rate(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }    ///
    /// AN link_control variable
    ///
    /// 0: ENABLE 1: DISABLE 2: SCAN_FOR_CARRIER
    pub fn link_ctl(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_link_ctl(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }    ///
    /// Negotiated HCD
    ///
    /// 0: KX_1G 1: KX4_10G 2: KR_10G 3: KR4_40G 4: CR4_40G 5: CR10_100G
    pub fn link_hcd(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_link_hcd(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }    ///
    /// Nonce match (LH)
    pub fn nonce_match(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_nonce_match(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }    ///
    /// 10G sync status of local detector
    pub fn sync10g(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_sync10g(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }    ///
    /// 1G or 3G sync status of local detector
    pub fn sync8b10b(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_sync8b10b(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
}
