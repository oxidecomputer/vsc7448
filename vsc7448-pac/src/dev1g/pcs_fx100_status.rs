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

/// Register `PCS_FX100_CFG`
///
/// PCS 100Base FX Configuration
///
/// Configuration bit groups for 100Base-FX PCS
#[derive(From, Into)]
pub struct PCS_FX100_CFG(u32);
impl PCS_FX100_CFG {    ///
    /// Far-End Fault (FEF) detection enable
    ///
    /// 0: Disable FEF detection 1 Enable FEF detection
    pub fn fefchk_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_fefchk_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Far-End Fault (FEF) generation enable
    ///
    /// 0: Disable FEF generation 1 Enable FEF generation
    pub fn fefgen_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_fefgen_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Link hysteresis timer configuration. The hysteresis time lasts [linkhysttimer] * 65536 ns + 2320 ns. If linkhysttime is set to 5, the hysteresis lasts the minimum time of 330 us as specified in IEEE802.3 - 24.3.3.4.
    pub fn linkhysttimer(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    pub fn set_linkhysttimer(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0xf0);
        self.0 &= !0xf0;
        self.0 |= value;
    }    ///
    /// Link hysteresis timer test mode. When enabled, [linkhysttimer] steps are reduced from 65536 ns to 2048 ns.
    ///
    /// 1: Enable test mode 0: Disable test mode
    pub fn linkhyst_tm_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_linkhyst_tm_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }    ///
    /// Testloop, if enabled 4B5B encoded data are looped from TX path to RX path just before the SERDES
    ///
    /// 1: Enable loop 0: Disable loop
    pub fn loopback_ena(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_loopback_ena(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }    ///
    /// PCS enable
    ///
    /// 0: Disable PCS 1: Enable PCS
    pub fn pcs_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_pcs_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Select single bit from incomming 10-bit Deserializer value. Change is only required in case CP/MD handling incorrect in Clock and Data Recovery logic
    pub fn rxbitsel(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    pub fn set_rxbitsel(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xf000);
        self.0 &= !0xf000;
        self.0 |= value;
    }    ///
    /// Signal Detect Enable
    ///
    /// 0: The Signal Detect input pin is ignored. The PCS assumes an active Signal Detect at all times 1: The Signal Detect input pin is used to determine if a signal is detected
    pub fn sd_ena(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_sd_ena(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }    ///
    /// Signal detect polarity: The signal level on signal_detect input pin must be equal to SD_POL to indicate signal detection (SD_ENA must be set). Use '1' when SD_SEL is set to hardmacro.
    ///
    /// 0: Signal Detect input pin must be '0' to indicate a signal detection 1: Signal Detect input pin must be '1' to indicate a signal detection
    pub fn sd_pol(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    pub fn set_sd_pol(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x2000000);
        self.0 &= !0x2000000;
        self.0 |= value;
    }    ///
    /// Signal detect selection (select input for internal signal_detect line)
    ///
    /// 0: Select signal_detect line from hardmacro 1: Select external signal_detect line
    pub fn sd_sel(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    pub fn set_sd_sel(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0x4000000);
        self.0 &= !0x4000000;
        self.0 |= value;
    }    ///
    /// Influence of rx toggle-rate on signal_detect. Signal detect is forced to 0 after a programable number of missing edges on rx bit-stream
    ///
    /// 0: No influence 1: Force to 0 after 50 cycles without edge 2: Force to 0 after 10 cycles without edge 3: Force to 0 after 5 cycles without edge
    pub fn sigdet_cfg(&self) -> u32 {
        (self.0 & 0x600) >> 9
    }
    pub fn set_sigdet_cfg(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x600);
        self.0 &= !0x600;
        self.0 |= value;
    }    ///
    /// Swap transmission/receive order of MII nibbles
    ///
    /// 0: Lower nibble of GMII byte is transferred/received first 1: Upper nibble of GMII byte is transferred/received first
    pub fn swap_mii_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_swap_mii_ena(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }    ///
    /// Unidirectional mode enable. Implementation 0f 802.3 clause 66. When asserted, this enables MAC to transmit data independent of the state of the receive link.
    ///
    /// 0: Unidirectional mode disabled 1: Unidirectional mode enabled
    pub fn unidir_mode_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_unidir_mode_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
}
