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
/// LSB of Rx Port VOE mapping table (ANA).
///
/// This register contains the lower 32 bits of the Port VOE Rx (ANA) COSID mapping table. The mapping in this register is used when Port DEI = 0.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_RX_COSID_MAP(u32);
impl PORT_RX_COSID_MAP {
    /// See register description.
    pub fn port_rx_cosid_map(&self) -> u32 {
        self.0
    }
    pub fn set_port_rx_cosid_map(&mut self, value: u32) {
        self.0 = value;
    }
}
/// MSB of Rx Port VOE mapping table (ANA).
///
/// This register contains the upper 32 bits of the Port VOE Rx (ANA) COSID mapping table. This mapping in this register is used when Port DEI = 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_RX_COSID_MAP1(u32);
impl PORT_RX_COSID_MAP1 {
    /// See register description.
    pub fn port_rx_cosid_map1(&self) -> u32 {
        self.0
    }
    pub fn set_port_rx_cosid_map1(&mut self, value: u32) {
        self.0 = value;
    }
}
/// LSB of Tx Port VOE mapping table (REW).
///
/// This register contains the lower 32 bits of the Port VOE Tx (REW) COSID mapping table. This mapping in this register is used when Port DEI = 0
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_TX_COSID_MAP(u32);
impl PORT_TX_COSID_MAP {
    /// See register description.
    pub fn port_tx_cosid_map(&self) -> u32 {
        self.0
    }
    pub fn set_port_tx_cosid_map(&mut self, value: u32) {
        self.0 = value;
    }
}
/// MSB of Tx Port VOE mapping table (REW).
///
/// This register contains the upper 32 bits of the Port VOE Tx (REW) COSID mapping table. This mapping in this register is used when Port DEI = 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_TX_COSID_MAP1(u32);
impl PORT_TX_COSID_MAP1 {
    /// See register description.
    pub fn port_tx_cosid_map1(&self) -> u32 {
        self.0
    }
    pub fn set_port_tx_cosid_map1(&mut self, value: u32) {
        self.0 = value;
    }
}
