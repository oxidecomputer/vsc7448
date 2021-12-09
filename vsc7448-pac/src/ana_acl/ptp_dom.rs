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
/// VCAP_IS2 counter
///
/// The CNT_TBL counters count number of hits in VCAP_IS2. For each of the two VCAP_IS2 lookups, a VCAP match results in the associated counter being incremented using the VCAP_IS2 action CNT_ID as index.
#[derive(From, Into)]
pub struct CNT(u32);
impl CNT {
    /// VCAP_IS2 counter value.
    pub fn cnt(&self) -> u32 {
        self.0
    }
    pub fn set_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// PTP clock identifier LSB
#[derive(From, Into)]
pub struct PTP_CLOCK_ID_LSB(u32);
impl PTP_CLOCK_ID_LSB {
    /// Bits 31:0 of clockIdentifier used in portIdentity.
    pub fn clock_id_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_clock_id_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// PTP clock identifier MSB
#[derive(From, Into)]
pub struct PTP_CLOCK_ID_MSB(u32);
impl PTP_CLOCK_ID_MSB {
    /// Bits 63:32 of clockIdentifier used in portIdentity.
    pub fn clock_id_msb(&self) -> u32 {
        self.0
    }
    pub fn set_clock_id_msb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Miscellaneous PTP domain configuration
#[derive(From, Into)]
pub struct PTP_MISC_CFG(u32);
impl PTP_MISC_CFG {
    /// New values for byte 0 in flagField. Only bits with the corresponding bits set in FLAG_FIELD_MASK, are used.
    pub fn flag_field(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_flag_field(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// Mask used to configure which bits in flagField, byte 0, are overwriteen with value configured in PTP_MISC_CFG.FLAG_FIELD.

    ///

    /// Bit x=0: Do not overwrite bit x in flagField, byte 0. Bit x=1: Overwrite bit x in flagField, byte 0, with FLAG_FIELD, bit x.
    pub fn flag_field_mask(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_flag_field_mask(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
}
/// PTP domain configuration used in sourcePortIdentity
#[derive(From, Into)]
pub struct PTP_SRC_PORT_CFG(u32);
impl PTP_SRC_PORT_CFG {
    /// Port number used in portIdentity.
    pub fn port_num(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_port_num(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// If set, lowest 6 bits in portIdentity is taken from ANA_ACL:PORT:PTP_CFG.PTP_PORT_NUM. Otherwise, portIdentity is taken from PTP_SRC_PORT_CFG.PORT_NUM.
    pub fn port_num_sel(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_port_num_sel(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
}
