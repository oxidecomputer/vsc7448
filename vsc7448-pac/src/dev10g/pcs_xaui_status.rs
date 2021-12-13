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
/// PCS Receiver comma alignment Register
///
/// Receive comma alignment status Register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_CGALIGN_STATUS(u32);
impl PCS_XAUI_CGALIGN_STATUS {
    /// Delay through the lane barrelshifter. Lane N delay is stored in bits 5N+4 to 5N. Unit is line bit times
    pub fn lane_cgalign_delay(&self) -> u32 {
        self.0 & 0xfffff
    }
    pub fn set_lane_cgalign_delay(&mut self, value: u32) {
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }
}
/// PCS Receiver Deskew Register
///
/// Receive deskew status Register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_DESKEW_STATUS(u32);
impl PCS_XAUI_DESKEW_STATUS {
    /// Delay through deskew fifo for each lane. Lane N delay is stored in bits 4N+3 to 4N. Unit is PCS clock cycles
    pub fn lane_deskew_delay(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_lane_deskew_delay(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// PCS Low Power Idle Status
///
/// Status register for Low Power Idle (Energy Efficient Ethernet)
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_LPI_STATUS(u32);
impl PCS_XAUI_LPI_STATUS {
    /// Receiver Low-Pwer idle occurrence
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
    }
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
    }
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
    }
    /// Transmitter Low-Pwer idle occurrence
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
    }
    /// Transmitter Low-Power Idle mode
    ///
    /// 0: Transmitter not in low power idle mode 1: Transmitter is in low power idle mode
    pub fn tx_lpi_mode(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_tx_lpi_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
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
    /// Receiver has failed to recover from Low-Power Idle mode
    ///
    /// 0: No failure 1: Failed to recover from LPI mode
    pub fn wake_err_cnt(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_wake_err_cnt(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
/// PCS Receiver Lane Error Status
///
/// Receiver Lane Error Status register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_RX_ERROR_STATUS(u32);
impl PCS_XAUI_RX_ERROR_STATUS {
    /// Alignment lost in deskew logic
    ///
    /// 0: No misalignment occured 1: A (temporary) misalignment has been detected Bit is cleared by writing a 1 to this position.
    pub fn alignment_lost_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_alignment_lost_sticky(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Coding error detected in received 8B/10B encoded data
    ///
    /// 0: No error found 1: Coding error detected Bit is cleared by writing a 1 to this position.
    pub fn c8b10b_err_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_c8b10b_err_sticky(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Synchronization lost in lane i (i = 0...3, one bit per lane)
    ///
    /// 0: No sync lost occured 1: Synchronization lost in lane i (temporarily) Bit is cleared by writing a 1 to this position.
    pub fn sync_lost_sticky(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_sync_lost_sticky(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// PCS RX 10b8b Codegroup Error and Lane 3 Error Counter
///
/// 10b8b Decoder Codegroup error counter. In test pattern check mode, this counter counts the errors of lane 3. In the latter case the counter is incremented by one whenever at least one out of eighty received bits (eight symbols) is corrupted.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_RX_FIFO_CG_ERR_L3_CNT_STATUS(u32);
impl PCS_XAUI_RX_FIFO_CG_ERR_L3_CNT_STATUS {
    /// Number of detected codegroup errors/Number of errors in lane 3
    pub fn err_cnt_10b8b_cg_l3(&self) -> u32 {
        self.0
    }
    pub fn set_err_cnt_10b8b_cg_l3(&mut self, value: u32) {
        self.0 = value;
    }
}
/// PCS RX 10b8b Disparity Error and Lane 2 Error Counter
///
/// 10b8b Decoder Disparity error counter. In test pattern check mode, this counter counts the errors of lane 2. In the latter case the counter is incremented by one whenever at least one out of eighty received bits (eight symbols) is corrupted.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_RX_FIFO_D_ERR_L2_CNT_STATUS(u32);
impl PCS_XAUI_RX_FIFO_D_ERR_L2_CNT_STATUS {
    /// Number of detected disparity errors/Number of errors in lane 2
    pub fn err_cnt_10b8b_d_l2(&self) -> u32 {
        self.0
    }
    pub fn set_err_cnt_10b8b_d_l2(&mut self, value: u32) {
        self.0 = value;
    }
}
/// PCS RX Fifo Overflow Error and Lane 0 Error Counter
///
/// Receive Fifo Overflow error counter. In test pattern check mode, this counter counts the errors of lane 0. In the latter case the counter is incremented by one whenever at least one out of eighty received bits (eight symbols) is corrupted.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_RX_FIFO_OF_ERR_L0_CNT_STATUS(u32);
impl PCS_XAUI_RX_FIFO_OF_ERR_L0_CNT_STATUS {
    /// Number of detected fifo overflow errors/Number of errors in lane 0
    pub fn err_cnt_fifo_of_l0(&self) -> u32 {
        self.0
    }
    pub fn set_err_cnt_fifo_of_l0(&mut self, value: u32) {
        self.0 = value;
    }
}
/// PCS RX Fifo Underflow Error and Lane 1 Error Counter
///
/// Receive Fifo Underflow error counter. In test pattern check mode, this counter counts the errors of lane 1. In the latter case the counter is incremented by one whenever at least one out of eighty received bits (eight symbols) is corrupted.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_RX_FIFO_UF_ERR_L1_CNT_STATUS(u32);
impl PCS_XAUI_RX_FIFO_UF_ERR_L1_CNT_STATUS {
    /// Number of detected fifo underflow errors/Number of errors in lane 1
    pub fn err_cnt_fifo_uf_l1(&self) -> u32 {
        self.0
    }
    pub fn set_err_cnt_fifo_uf_l1(&mut self, value: u32) {
        self.0 = value;
    }
}
/// PCS Receiver Sequence Result Register
///
/// Sequence Receive Status Register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_RX_SEQ_REC_STATUS(u32);
impl PCS_XAUI_RX_SEQ_REC_STATUS {
    /// Received ||Q|| code (Sequence information, i.e. lower 24 bit of a Sequence)
    pub fn rx_q(&self) -> u32 {
        (self.0 & 0xffffff00) >> 8
    }
    pub fn set_rx_q(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xffffff00);
        self.0 &= !0xffffff00;
        self.0 |= value;
    }
    /// Received ||Q|| code changed
    ///
    /// 1: New ||Q|| has been received 0: No new ||Q|| since last read Bit is cleared by writing a 1 to this position.
    pub fn rx_q_changed_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_rx_q_changed_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PCS Receiver Status Register
///
/// Receive Lane Status Register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_RX_STATUS(u32);
impl PCS_XAUI_RX_STATUS {
    /// Status of lane alignment
    ///
    /// 1: All lanes are aligned 0: No alignment reached
    pub fn alignment_status(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_alignment_status(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Local Fault status (one or more of sync/align/fifo_of/fifo_uf/8b10b error)
    ///
    /// 1: A fault occured 0: No fault detected Bit is cleared by writing a 1 to this position.
    pub fn local_fault_sticky(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    pub fn set_local_fault_sticky(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xf00);
        self.0 &= !0xf00;
        self.0 |= value;
    }
    /// Current status of selected signal_detect input lines
    pub fn signal_detect(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    pub fn set_signal_detect(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xf000);
        self.0 &= !0xf000;
        self.0 |= value;
    }
    /// Status of code group alignment (lane independent)
    ///
    /// 1111: All lanes in sync 0001: Lane 0 is in sync ...
    pub fn sync_status(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_sync_status(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
