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
/// PCS status register
///
/// Contains status information from the PCS core
#[derive(From, Into)]
pub struct PCS_STATUS(u32);
impl PCS_STATUS {
    /// The block_lock status from the synchronization state machine
    ///
    /// 0: Not synchronized 1: Synchronized, lock obtained
    pub fn rx_block_lock(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_rx_block_lock(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set by the Rx BER state machine when a high bit-error-rate condition is detected
    ///
    /// 0: Normal BER 1: High BER
    pub fn rx_hi_ber(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_rx_hi_ber(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// When in test pattern check mode, this bit will read 1 if the test pattern checker detects a match. When 0, the test pattern does not match. The test pattern error counts should still be used along with this register bit to determine proper test match status. The bit will read back 1 only when the test pattern is matching. This may happen even while test pattern errors are counted on other clock cycles.
    ///
    /// 0: Test pattern mismatch 1: Test pattern match
    pub fn testpat_match(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_testpat_match(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}
/// ber_count
///
/// ber_count from IEEE802.3 section 49.2.14.2.
#[derive(From, Into)]
pub struct RX_BER_CNT(u32);
impl RX_BER_CNT {
    /// ber_count from the BER state machine
    pub fn rx_ber_cnt(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_rx_ber_cnt(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}
/// Invalid character counter
///
/// Counts the number of invalid control characters
#[derive(From, Into)]
pub struct RX_CHARERR_CNT(u32);
impl RX_CHARERR_CNT {
    /// Count of the number of invalid control characters
    pub fn rx_charerr_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_rx_charerr_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Rx errored block counter
///
/// Count of the Rx errored blocks
#[derive(From, Into)]
pub struct RX_ERRBLK_CNT(u32);
impl RX_ERRBLK_CNT {
    /// Count of the errored Rx blocks
    pub fn rx_errblk_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_rx_errblk_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Status of Rx signal ordered set FIFO
///
/// Contains status information for the FIFO containing captured Rx ordered sets
#[derive(From, Into)]
pub struct RX_FSET_FIFO_STAT(u32);
impl RX_FSET_FIFO_STAT {
    /// Indicates if the FIFO is full
    ///
    /// 0: FIFO not full 1: FIFO full
    pub fn rx_fset_fifo_full(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rx_fset_fifo_full(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Number of valid ordered sets in the FIFO that can be read
    ///
    /// Binary number
    pub fn rx_fset_fifo_num(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_rx_fset_fifo_num(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Rx ordered set FIFO data
///
/// The register interface to the ordered set data
#[derive(From, Into)]
pub struct RX_OSET_FIFO_DATA(u32);
impl RX_OSET_FIFO_DATA {
    /// Register interface to the FIFO containing captured ordered sets. Each read of this register pops a 24-bit ordered set off the FIFO and increments the FIFO pointer.
    pub fn rx_oset_fifo_data(&self) -> u32 {
        (self.0 & 0xffffff00) >> 8
    }
    pub fn set_rx_oset_fifo_data(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xffffff00);
        self.0 &= !0xffffff00;
        self.0 |= value;
    }
}
/// Status of Rx sequence ordered set FIFO
///
/// Contains status information for the FIFO containing captured Rx ordered sets
#[derive(From, Into)]
pub struct RX_OSET_FIFO_STAT(u32);
impl RX_OSET_FIFO_STAT {
    /// Indicates if the FIFO is full
    ///
    /// 0: FIFO not full 1: FIFO full
    pub fn rx_oset_fifo_full(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rx_oset_fifo_full(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Number of valid ordered sets in the FIFO that can be read
    ///
    /// Binary number
    pub fn rx_oset_fifo_num(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_rx_oset_fifo_num(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Test pattern mode error counts
///
/// Count of the errors detected in test pattern mode
#[derive(From, Into)]
pub struct TEST_ERR_CNT(u32);
impl TEST_ERR_CNT {
    /// Count of detected test pattern errors in Rx test pattern checker. Write 0 to clear.
    pub fn test_err_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_test_err_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Invalid character counter
///
/// Counts the number of invalid control characters
#[derive(From, Into)]
pub struct TX_CHARERR_CNT(u32);
impl TX_CHARERR_CNT {
    /// Count of the number of invalid control characters
    pub fn tx_charerr_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_tx_charerr_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Tx errored block counter
///
/// Count of the Tx errored blocks
#[derive(From, Into)]
pub struct TX_ERRBLK_CNT(u32);
impl TX_ERRBLK_CNT {
    /// Count of the errored Tx blocks
    pub fn tx_errblk_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_tx_errblk_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
