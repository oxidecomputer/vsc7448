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

/// Register `FIXED_ERROR_COUNT_THRESHOLD`
///
/// FEC fixed error count threshold value
#[derive(From, Into)]
pub struct FIXED_ERROR_COUNT_THRESHOLD(u32);
impl FIXED_ERROR_COUNT_THRESHOLD {
    /// When fixed error count exceeds or equal to this value, then FEC_FIXED_ERROR_COUNT_STICKY sticky bit is set and interrupt is generated if enabled through FEC_FIXED_ERROR_COUNT_STICKY_MASK
    pub fn fixed_error_count_threshold(&self) -> u32 {
        self.0
    }
    pub fn set_fixed_error_count_threshold(&mut self, value: u32) {
        self.0 = value;
    }
}

/// Register `KR_FEC_CFG`
///
/// KR FEC configuration regsiter
#[derive(From, Into)]
pub struct KR_FEC_CFG(u32);
impl KR_FEC_CFG {
    /// Enables enabling/disabling FEC using backplane Ethernet ANEG (Auto-Negotiation)

    ///

    /// 0: ANEG doesn't control Enable/Disable of FEC 1: ANEG controls Enable/Diable of FEC
    pub fn an_fec_ctrl_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_an_fec_ctrl_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enables FEC decoder to indicate errors to PCS by sync bits

    ///

    /// 0: FEC Decoding errors have no effect on PCS sync bits 1: Enable FEC decoder to indicate errors to PCS sync bits
    pub fn enable_error_indication(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_enable_error_indication(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Global FEC Enable/Disable configuration bit.

    ///

    /// 0 = Disable FEC 1 = Enable FEC
    pub fn fec_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_fec_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// When set FEC counters are reset.

    ///

    /// 0: no effect 1: reset FEC counters
    pub fn reset_monitor_counters(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_reset_monitor_counters(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Flip the data bus bits at PCS interface such that bit 65 is mapped to bit 0 and bit 0 to 65. i.e. the output bus (65 downto 0) is remapped to (0 to 65) and bit 65 is the first bit.

    ///

    /// 0 = No flip (LSB first) 1 = Flip bus (MSB first)
    pub fn rx_data_flip(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rx_data_flip(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Flip the data bus bits at PCS interface such that bit 65 is mapped to bit 0 and bit 0 to 65. i.e. the output bus (65 downto 0) is remapped to (0 to 65) and bit 65 is the first bit.

    ///

    /// 0 = No flip (LSB first) 1 = Flip bus (MSB first)
    pub fn tx_data_flip(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_tx_data_flip(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
}
