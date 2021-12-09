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
/// Byte count of all frames passing through the Port VOE (MSB)
///
/// Byte count of all frames passing through the Port VOE. Whenever this RAM is read, the value of this register will be sampled into the following register: * ANA_AC_OAM_MOD::RD_LAST_PORT_BYTE_CNT_MSB.RD_LAST_PORT_BYTE_CNT_MSB (ANA) * REW::RD_LAST_PORT_BYTE_CNT_MSB.RD_LAST_PORT_BYTE_CNT_MSB (REW)
#[derive(From, Into)]
pub struct PORT_BYTE_CNT_MSB(u32);
impl PORT_BYTE_CNT_MSB {
    /// See Register Description.
    pub fn port_byte_cnt_msb(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_port_byte_cnt_msb(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Count all the frames which pass through the Port VOE
///
/// This counter counts all the frames which pass though the Port VOE. Whenever this RAM is read, the value of this register will be sampled into the following register: * ANA_AC_OAM_MOD::RD_LAST_PORT_FRM_CNT_LSB.RD_LAST_PORT_FRM_CNT_LSB (ANA) * REW::RD_LAST_PORT_FRM_CNT_LSB.RD_LAST_PORT_FRM_CNT_LSB (REW)
#[derive(From, Into)]
pub struct PORT_FRM_CNT_LSB(u32);
impl PORT_FRM_CNT_LSB {
    /// See Register Description.
    pub fn port_frm_cnt_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_port_frm_cnt_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Count selected OAM PDU received by MEP Counter
///
/// Implements the OAM VOE LM port counters. Depending on whether the OAM_PDU_MOD block is instantiated in the REW or the ANA these counters will be either egress (REW) or ingress (ANA) counters. Y.1731 LM counters count frames and are 32 bit wide. Whenever this RAM is read, the value of this register will be sampled into the following register: * ANA_AC_OAM_MOD::RD_LAST_PORT_LM_CNT_LSB.RD_LAST_PORT_LM_CNT_LSB (ANA) * REW::RD_LAST_PORT_LM_CNT_LSB.RD_LAST_PORT_LM_CNT_LSB (REW)
#[derive(From, Into)]
pub struct PORT_LM_CNT_LSB(u32);
impl PORT_LM_CNT_LSB {
    /// See Register Description.
    pub fn port_lm_cnt_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_port_lm_cnt_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Service LM counters pr. priority
///
/// Implements the OAM VOE LM counters. Depending on whether the OAM_PDU_MOD block is instantiated in the REW or the ANA these counters will be either egress (REW) or ingress (ANA) counters. Y.1731 LM counters count frames and are 32 bit wide.
#[derive(From, Into)]
pub struct SRV_LM_CNT_LSB(u32);
impl SRV_LM_CNT_LSB {    pub fn srv_lm_cnt_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_srv_lm_cnt_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
