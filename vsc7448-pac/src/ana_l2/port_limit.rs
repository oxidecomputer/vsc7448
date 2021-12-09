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
/// Controls automatic learn limits per FID
///
/// Per FID configuration of MAC table learn limits
#[derive(From, Into)]
pub struct FID_LIMIT_CTRL(u32);
impl FID_LIMIT_CTRL {
    /// Allow setting FID_LIMIT_INTR when exceeding limit on learning (happens when MAC address are supposed to be installed in the MAC table.

    ///

    /// 0: Disable 1: allow FID_LIMIT_INTR to be set upon trying to learn a MAC address that causes learn limit to be exeeded
    pub fn fid_limit_exceed_irq_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_fid_limit_exceed_irq_ena(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Action for traffic when learn limit is exceeded.

    ///

    /// 00: Normal forward 01: Enable redirection to CPU queue 10: Enable copy to CPU queue 11: Discard the frame
    pub fn fid_limit_exceed_sel(&self) -> u32 {
        (self.0 & 0x18000) >> 15
    }
    pub fn set_fid_limit_exceed_sel(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x18000);
        self.0 &= !0x18000;
        self.0 |= value;
    }
    /// Configures the number of MAC table entries that can be used for a given FID (through Automatic learning and CPU based learning with LOCK bit cleared and not multicast).

    ///

    /// 0: Disable i.e. no learn limit for the FID 1: Only learning of one MAC address allowed for this FID ... n: Learning of n MAC address allowed for this FID
    pub fn fid_lrn_cnt_limit(&self) -> u32 {
        self.0 & 0x7fff
    }
    pub fn set_fid_lrn_cnt_limit(&mut self, value: u32) {
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}
/// Controls automatic learn limits
///
/// Per port configuration of autolearn limits
#[derive(From, Into)]
pub struct PORT_LIMIT_STATUS(u32);
impl PORT_LIMIT_STATUS {
    /// Contains the number of MAC table entries currently learned associated with a given logical PORT or GLAG.

    ///

    /// 0 : no entries
    pub fn port_lrn_cnt(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_port_lrn_cnt(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// Set if specified MAX learn cnt limit is exceeded and max learn cnt was enabled. Write '1' to clear this field.

    ///

    /// 0: Learn cnt not exceeded. 1: Learning operation has failed due to PORT max learn cnt exceeded. Write '1' to clear this field.
    pub fn port_lrn_limit_exceeded_sticky(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_port_lrn_limit_exceeded_sticky(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
}
