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
/// Various switch port mode settings
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PFC_CFG(u32);
impl PFC_CFG {
    /// When set the MAC sends PRIO pause control frames in the Tx direction when congested.
    pub fn tx_pfc_ena(&self) -> u32 {
        (self.0 & 0x1fe) >> 1
    }
    pub fn set_tx_pfc_ena(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 1;
        self.0 &= !0x1fe;
        self.0 |= value;
    }
    /// When set, a congested priority request pause of all lower priorities as well.
    pub fn tx_pfc_mode(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_tx_pfc_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// DLB shaping offset
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct RES_DLB_OFFSET(u32);
impl RES_DLB_OFFSET {
    /// The watermarks for enabling DLB rate will be offset this value compared to the sensed resource. Ie. if shared priority 0 watermark is set to 40000 cells, the default value will allow higher rate shaping when 39950 cells has been used.
    pub fn res_dlb_offs_val(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_res_dlb_offs_val(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// Shared QOS resource mode
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct RES_QOS_MODE(u32);
impl RES_QOS_MODE {
    /// When a qos class is enabled in this mask, the class will have guaranteed shared space. The watermarks found in RES_CFG are used for setting the amount of space set aside.
    pub fn res_qos_rsrvd(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_res_qos_rsrvd(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Random Early Discard (WRED) configuration
///
/// Configured the WRED group per port.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct WRED_GROUP(u32);
impl WRED_GROUP {
    /// Frames towards a port is WRED discarded by the profiles for the group configured.
    pub fn wred_group(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_wred_group(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
