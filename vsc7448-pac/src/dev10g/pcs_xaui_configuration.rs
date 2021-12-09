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
/// PCS XAUI Configuration
///
/// PCS XAUI Configuration Register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_CFG(u32);
impl PCS_XAUI_CFG {
    /// Enable Link control via Backplane Ethernet ANEG
    ///
    /// 0: Disable link control 1: Enable link control
    pub fn an_link_ctrl_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_an_link_ctrl_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// RX FIFO read pointer reset
    ///
    /// 1: Reset RX fifo read pointer
    pub fn fifo_rpt_res(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_fifo_rpt_res(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Idle sequencing mode (IPG shrink mode support). When active, the first ||I|| after ||T|| will be alternatingly ||K||, ||A|| or ||R|| instead of ||K|| or ||A|| only in normal mode
    ///
    /// 1: Modified idle sequencing for IPG shrink mode support 0: Normal idle sequencing
    pub fn idle_seq_mode(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_idle_seq_mode(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Disable RX Local Fault generation when no alignment has been reached
    ///
    /// 0: Output Local Fault symbol at XGMII when not aligned 1: Output IDLE symbols at XGMII when not aligned
    pub fn lf_gen_dis(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_lf_gen_dis(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// PCS enable
    ///
    /// 0: Disable PCS 1: Enable PCS
    pub fn pcs_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pcs_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Disable RX Pad/Truncate Mode
    ///
    /// 0: Normal operation 1: Disable pad/truncate
    pub fn pt_dis(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_pt_dis(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// RX Minimum Inter Packet Gap (Minimum Idle columns, i.e. 4 /I/ on all four lanes, to occure before Truncate may happen)
    ///
    /// 0: No complete idle column is preserved 1: At least one idle column is preserved ...
    pub fn pt_ipg_size(&self) -> u32 {
        (self.0 & 0x38) >> 3
    }
    pub fn set_pt_ipg_size(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x38);
        self.0 &= !0x38;
        self.0 |= value;
    }
    /// Testloop, if enabled (XAUI encoded) data are looped from TX path to RX path just before the SERDES
    ///
    /// 1: Enable loop 0: Disable loop
    pub fn xaui_loop_ena(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_xaui_loop_ena(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
}
/// PCS XAUI Extended Configuration
///
/// Special configuration bit groups for PCS XAUI
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_EXT_CFG(u32);
impl PCS_XAUI_EXT_CFG {
    /// Flip HM-Bus in receive direction, i.e. map lane 0 on 3, lane 1 on 2, lane 2 on 1 and lane 3 on 0
    ///
    /// 0: Normal lane order 1: Flipped lane order
    pub fn rx_flip_hmbus(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_rx_flip_hmbus(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Invert HM-Bus in receive direction, invert all data signals from SERDES
    ///
    /// 0: Normal operation 1: Invert HM-bus
    pub fn rx_inv_hmbus(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_rx_inv_hmbus(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Flip HM-Bus in transmit direction, i.e. map lane 0 on 3, lane 1 on 2, lane 2 on 1 and lane 3 on 0
    ///
    /// 0: Normal lane order 1: Flipped lane order
    pub fn tx_flip_hmbus(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_tx_flip_hmbus(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Invert HM-Bus in transmit direction, invert all data signals to SERDES
    ///
    /// 0: Normal operation 1: Invert HM-bus
    pub fn tx_inv_hmbus(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_tx_inv_hmbus(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// PCS Interleave Mode Configuration
///
/// Configuration register set for Interleave Mode (XAUI via two lanes) - also known as DDR-XAUI
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_INTERLEAVE_MODE_CFG(u32);
impl PCS_XAUI_INTERLEAVE_MODE_CFG {
    /// Comma replacement. In interleave mode (using /K/ byte orderung) one 20-bit word must have only one comma for proper alignment. Misleading commas are replaced by comma_repl in transmit direction and replaced by K28.5-commas again in receive direction. Comma_repl has to be an unused valid special code-group which does not contain a comma, i.e. K28.2, K28.6 or K23.7 are possible replacements.
    pub fn comma_repl(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_comma_repl(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// Code-group alignment in 20-bit mode
    ///
    /// 0: align comma to lower 10 bits 1: align comma to upper 10 bits
    pub fn com_align_pos(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_com_align_pos(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Dual column ||A|| alignment (||A|| are inserted on even columns only)
    ///
    /// 0: Normal insertion 1: Even column insertion only
    pub fn dc_a_align_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_dc_a_align_ena(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Interleave mode selection. In interleave mode XAUI data are sent via two 5Gbps lanes
    ///
    /// 0: Interleave mode with /K/ comma based byte re-ordering (using comma replacement) 1: Interleave mode with /A/ alignment symbol based byte re-ordering
    pub fn ilv_mode(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ilv_mode(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Interleave mode enable. In interleave mode XAUI data are sent via two 5Gbps lanes
    ///
    /// 0: Normal XAUI mode 1: Interleave mode
    pub fn ilv_mode_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ilv_mode_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Comma synchronization mode
    ///
    /// 0: Synchronize on any 7-bit comma (XAUI compliant) 1: Synchronize on K28.5 only (non XAUI compliant)
    pub fn k28_5_sync_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_k28_5_sync_ena(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Channel reordering in receive direction (swap lane 0, 1 and lane 2, 3) before 8B/10B decoder
    ///
    /// 1: Enable 0: Disable
    pub fn rxchan_reord1_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_rxchan_reord1_ena(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Channel reordering in receive direction (swap lane 0, 1 and lane 2, 3) after 8B/10B decoder
    ///
    /// 1: Enable 0: Disable
    pub fn rxchan_reord2_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rxchan_reord2_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Disable 8b10b decoding of interleaved data stream
    ///
    /// 0: Interleaved data stream is 8b10b decoded 1: Each lane is individually 8b10b decoded
    pub fn rx_8b10b_ilv_dis(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_rx_8b10b_ilv_dis(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Channel reordering in transmit direction (swap lane 0, 1 and lane 2, 3) before comma remapping
    ///
    /// 1: Enable 0: Disable
    pub fn txchan_reord1_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_txchan_reord1_ena(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Channel reordering in transmit direction (swap lane 0, 1 and lane 2, 3) after comma remapping
    ///
    /// 1: Enable 0: Disable
    pub fn txchan_reord2_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_txchan_reord2_ena(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Disable 8b10b encoding of interleaved data stream
    ///
    /// 0: Interleaved data stream is 8b10b encoded 1: Each lane is individually 8b10b encoded
    pub fn tx_8b10b_ilv_dis(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_tx_8b10b_ilv_dis(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// PCS XAUI Receiver Error Counter Configuration
///
/// Error Counter Configuration Register (if a bit in the mask field is set, the errors of that lane are not counted).
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_RX_ERR_CNT_CFG(u32);
impl PCS_XAUI_RX_ERR_CNT_CFG {
    /// Codegroup error counting mask
    ///
    /// 0000: Count errors of all lanes 1110: Count error of lane 0 only ...
    pub fn cerr_mask(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    pub fn set_cerr_mask(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xf00);
        self.0 &= !0xf00;
        self.0 |= value;
    }
    /// Disparity error counting mask
    ///
    /// 0000: Count errors of all lanes 1110: Count error of lane 0 only ...
    pub fn derr_mask(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    pub fn set_derr_mask(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xf000);
        self.0 &= !0xf000;
        self.0 |= value;
    }
    /// Fifo overflow error counting mask
    ///
    /// 0000: Count errors of all lanes 1110: Count error of lane 0 only ...
    pub fn oferr_mask(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_oferr_mask(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Fifo underflow error counting mask
    ///
    /// 0000: Count errors of all lanes 1110: Count error of lane 0 only ...
    pub fn uferr_mask(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    pub fn set_uferr_mask(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0xf0);
        self.0 &= !0xf0;
        self.0 |= value;
    }
}
/// PCS XAUI SignalDetect Configuration
///
/// PCS_XAUI signal_detect configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_SD_CFG(u32);
impl PCS_XAUI_SD_CFG {
    /// Signal Detect Enable
    ///
    /// 0: The Signal Detect input pin is ignored. The PCS assumes an active Signal Detect at all times 1: The Signal Detect input pin is used to determine if a signal is detected
    pub fn sd_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_sd_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Signal detect polarity: The signal level on signal_detect input pin must be equal to SD_POL to indicate signal detection (SD_ENA must be set)
    ///
    /// 0: Signal Detect input pin must be '0' to indicate a signal detection 1: Signal Detect input pin must be '1' to indicate a signal detection
    pub fn sd_pol(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_sd_pol(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Signal detect selection (select input for internal signal_detect line)
    ///
    /// 0: Select signal_detect line from hardmacro 1: Select external signal_detect line
    pub fn sd_sel(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_sd_sel(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
}
/// PCS Transmitter Sequence Configuration Register
///
/// Sequence Transmit Configuration Register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_TX_SEQ_CFG(u32);
impl PCS_XAUI_TX_SEQ_CFG {
    /// Transmit ||Q|| code (Sequence information, i.e. lower 24 bit of a Sequence)
    pub fn tx_q(&self) -> u32 {
        (self.0 & 0xffffff00) >> 8
    }
    pub fn set_tx_q(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xffffff00);
        self.0 &= !0xffffff00;
        self.0 |= value;
    }
    /// Disable Transmit ||Q|| code replacement
    ///
    /// 1: Disable 0: Enable
    pub fn tx_q_dis(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_tx_q_dis(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
}
/// Tx OK Bytes Counter - MSB partTx OK Bytes Counter - MSB
///
/// The number of bytes transmitted - MSBs only.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TX_OK_BYTES_MSB_CNT(u32);
impl TX_OK_BYTES_MSB_CNT {
    /// The numbe rof transmitted bytes transmitted successfully - MSBs only.
    ///
    /// Counter can be written by SW.
    pub fn tx_ok_bytes_msb_cnt(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_tx_ok_bytes_msb_cnt(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
