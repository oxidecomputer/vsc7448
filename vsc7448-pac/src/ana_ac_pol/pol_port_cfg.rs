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

/// Register `POL_PORT_THRES_CFG_0`
///
/// Port policer upper threshold configuration
///
/// The registers are index by 4 x port number + port policer index.
#[derive(From, Into)]
pub struct POL_PORT_THRES_CFG_0(u32);
impl POL_PORT_THRES_CFG_0 {    ///
    /// Threshold size for port policer (burst capacity). Related parameters: ANA_AC_POL:POL_PORT_CTRL:POL_PORT_CFG.FRAME_RATE_ENA
    ///
    /// When POL_PORT_CFG.FRAME_RATE_ENA is disabled burst capacity is configured in steps of 8192 bytes. 0: Always closed 1: Burst capacity = 8192 bytes n: Burst capacity = n x 8192 bytes 63: Burst capacity = 516096 bytes When POL_PORT_CFG.FRAME_RATE_ENA is enabled burst capacity is configured in steps of 8192/2504 frames. 0: Always closed 1: Burst capacity = 1 x 8192/2504 frames n: Burst capacity = n x 8192/2504 frames 63: Burst capacity = 206 frames
    pub fn port_thres0(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_port_thres0(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}

/// Register `POL_PORT_THRES_CFG_1`
///
/// Port policer lower threshold configuration
///
/// The registers are index by 4 x port number + port policer index.
#[derive(From, Into)]
pub struct POL_PORT_THRES_CFG_1(u32);
impl POL_PORT_THRES_CFG_1 {    ///
    /// Hysteresis size for port policer. Unit is 8192 bytes. PORT_THRES1 is used when a port policer is in flow control mode. Flow control is asserted when the bucket level exceeds PORT_THRES0. Flow control is deasserted when the bucket has leaked PORT_THRES1 bytes since the assertion. PORT_THRES1 must be programmed smaller or equal to PORT_THRES0. Related parameters: ANA_AC_POL:POL_PORT_CFG:POL_PORT_THRES_CFG_0.PORT_THRES0 ANA_AC_POL:POL_ALL_CFG:POL_PORT_FC_CFG.FC_ENA
    ///
    /// 0 : No hysteresis 1: Deassert flow control when bucket has leaked 8192 bytes ... n: Deassert flow control when bucket has leaked n * 8192 bytes
    pub fn port_thres1(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_port_thres1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}

/// Register `POL_STICKY1`
///
/// Policer diagnostic information
#[derive(From, Into)]
pub struct POL_STICKY1(u32);
impl POL_STICKY1 {    ///
    /// Set if frame has been dropped by a BDLB policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: BDLB policer drop event has occurred
    pub fn pol_bdlb_drop_sticky(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_pol_bdlb_drop_sticky(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Set if BUM policer has been active. One bit per BUM policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: BUM policer has been active.
    pub fn pol_bum_slb_active_sticky(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    pub fn set_pol_bum_slb_active_sticky(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x1c);
        self.0 &= !0x1c;
        self.0 |= value;
    }    ///
    /// Set if frame has been dropped by a BUM policer. One bit per BUM policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: BUM policer drop event has occurred
    pub fn pol_bum_slb_drop_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_pol_bum_slb_drop_sticky(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}