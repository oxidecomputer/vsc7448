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

/// Register `PCS1G_LPI_STATUS`
///
/// PCS1G Low Power Idle Status
///
/// Status register for Low Power Idle (Energy Efficient Ethernet)
#[derive(From, Into)]
pub struct PCS1G_LPI_STATUS(u32);
impl PCS1G_LPI_STATUS {    ///
    /// Receiver Low-Power idle occurrence
    ///
    /// 0: No LPI symbols received 1: Receiver has received LPI symbols Bit is cleared by writing a 1 to this position.
    pub fn rx_lpi_event_sticky(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_rx_lpi_event_sticky(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }    ///
    /// Receiver Low-Power Idle mode
    ///
    /// 0: Receiver not in low power idle mode 1: Receiver is in low power idle mode
    pub fn rx_lpi_mode(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_rx_lpi_mode(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }    ///
    /// Receiver Low-Power Quiet mode
    ///
    /// 0: Receiver not in quiet mode 1: Receiver is in quiet mode
    pub fn rx_quiet(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_rx_quiet(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }    ///
    /// Transmitter Low-Power idle occurrence
    ///
    /// 0: No LPI symbols transmitted 1: Transmitter has transmitted LPI symbols Bit is cleared by writing a 1 to this position.
    pub fn tx_lpi_event_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_tx_lpi_event_sticky(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Transmitter Low-Power Idle mode
    ///
    /// 0: Transmitter not in low power idle mode 1: Transmitter is in low power idle mode
    pub fn tx_lpi_mode(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_tx_lpi_mode(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Transmitter Low-Power Quiet mode
    ///
    /// 0: Transmitter not in quiet mode 1: Transmitter is in quiet mode
    pub fn tx_quiet(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_tx_quiet(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}

/// Register `PCS1G_TSTPAT_MODE_CFG`
///
/// PCS1G TSTPAT MODE CFG
///
/// PCS1G testpattern mode configuration register (Frame based pattern 4 and 5 might be not available depending on chip type)
#[derive(From, Into)]
pub struct PCS1G_TSTPAT_MODE_CFG(u32);
impl PCS1G_TSTPAT_MODE_CFG {    ///
    /// Jitter Test Pattern Select: Enables and selects the jitter test pattern to be transmitted. The jitter test patterns are according to the IEEE 802.3, Annex 36A
    ///
    /// 0: Disable transmission of test patterns 1: High frequency test pattern - repeated transmission of D21.5 code group 2: Low frequency test pattern - repeated transmission of K28.7 code group 3: Mixed frequency test pattern - repeated transmission of K28.5 code group 4: Long continuous random test pattern (packet length is 1524 bytes) 5: Short continuous random test pattern (packet length is 360 bytes)
    pub fn jtp_sel(&self) -> u32 {
        (self.0 & 0x7) >> 0
    }
    pub fn set_jtp_sel(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}