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

/// Register `PFC_CFG`
///
/// Priority-based flow control configuration
#[derive(From, Into)]
pub struct PFC_CFG(u32);
impl PFC_CFG {
    /// Configures the link speed. This is used to evaluate the time specifications in incoming pause frames.

    ///

    /// 0: 12000 Mbps 1: 10000 Mbps 2: 2500 Mbps 3: 1000 Mbps 4: 100 Mbps 5: 10 Mbps
    pub fn fc_link_speed(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_fc_link_speed(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Enable PFC per priority. Bit n enables PFC on priority n.
    pub fn rx_pfc_ena(&self) -> u32 {
        (self.0 & 0x7f8) >> 3
    }
    pub fn set_rx_pfc_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x7f8);
        self.0 &= !0x7f8;
        self.0 |= value;
    }
}

/// Register `PORT_STICKY`
///
/// ASM port sticky bits
///
/// This register holds all the sticky bits that exists for each port.
#[derive(From, Into)]
pub struct PORT_STICKY(u32);
impl PORT_STICKY {
    /// This field indicates if one or more Ethernet frames have been discarded due to aging.

    ///

    /// '0': No Ethernet frames have been discarded due to aging. '1': One or more Ethernet frames have been discarded due to aging. Bit is cleared by writing a '1' to this position.
    pub fn frm_aging_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_frm_aging_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// This field is set if the PORT_CFG.INJ_FORMAT_CFG field is set to one of the IFH modes and the incoming frame's format does not comply with the configured prefix.
    pub fn ifh_prefix_err_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ifh_prefix_err_sticky(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
