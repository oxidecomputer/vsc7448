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

/// Register `RX_ALIGNMENT_LOST_CNT`
///
/// Counter to track the dribble-nibble (extra nibble) errors in frames.
#[derive(From, Into)]
pub struct RX_ALIGNMENT_LOST_CNT(u32);
impl RX_ALIGNMENT_LOST_CNT {    ///
    /// The number of frames received with Alignment (dribble-nibble) error.
    ///
    /// Counter can be written by SW.
    pub fn rx_alignment_lost_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_alignment_lost_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_BAD_BYTES_CNT`
///
/// Rx Bad Byte Counter
#[derive(From, Into)]
pub struct RX_BAD_BYTES_CNT(u32);
impl RX_BAD_BYTES_CNT {    ///
    /// The number of received bytes in bad frames.
    ///
    /// Counter can be written by SW.
    pub fn rx_bad_bytes_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_bad_bytes_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_BAD_BYTES_MSB_CNT`
///
/// MSB of RX bad byte Counter
///
/// Register allowing to access the upper 4 bits of RX_IN_BYTE counter. Please note: When writing to RX_BAD_BYTES counter RX_BAD_BYTES_MSB_CNT has to be written before RX_BAD_BYTES_CNT is written. When reading RX_BAD_BYTES counter RX_BAD_BYTES_CNT has to be read before RX_BAD_BYTES_MSB_CNT is read. Accessing both counters must not be interfered by other register accesses.
#[derive(From, Into)]
pub struct RX_BAD_BYTES_MSB_CNT(u32);
impl RX_BAD_BYTES_MSB_CNT {    ///
    /// Upper 4 bits of RX_BAD_BYTES_CNT.
    ///
    /// Counter can be written by SW.
    pub fn rx_bad_bytes_msb_cnt(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_rx_bad_bytes_msb_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}

/// Register `RX_BC_CNT`
///
/// Rx Broadcast Frame Counter
#[derive(From, Into)]
pub struct RX_BC_CNT(u32);
impl RX_BC_CNT {    ///
    /// The number of good broadcast frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_bc_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_bc_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_CRC_ERR_CNT`
///
/// Rx CRC Error Counter
#[derive(From, Into)]
pub struct RX_CRC_ERR_CNT(u32);
impl RX_CRC_ERR_CNT {    ///
    /// The number of frames received with CRC error only.
    ///
    /// Counter can be written by SW.
    pub fn rx_crc_err_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_crc_err_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_FRAGMENTS_CNT`
///
/// Rx Undersize Counter (CRC error)
#[derive(From, Into)]
pub struct RX_FRAGMENTS_CNT(u32);
impl RX_FRAGMENTS_CNT {    ///
    /// The number of undersize frames with CRC error received.
    ///
    /// Counter can be written by SW.
    pub fn rx_fragments_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_fragments_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_IN_BYTES_CNT`
///
/// Rx Byte Counter
#[derive(From, Into)]
pub struct RX_IN_BYTES_CNT(u32);
impl RX_IN_BYTES_CNT {    ///
    /// The number of bytes received (good, bad, and framing).
    ///
    /// Counter can be written by SW.
    pub fn rx_in_bytes_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_in_bytes_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_IN_BYTES_MSB_CNT`
///
/// MSB of RX in byte Counter
///
/// Register allowing to access the upper 4 bits of RX_IN_BYTE counter. Please note: When writing to RX_IN_BYTES counter RX_IN_BYTES_MSB_CNT has to be written before RX_IN_BYTES_CNT is written. When reading RX_IN_BYTES counter RX_IN_BYTES_CNT has to be read before RX_IN_BYTES_MSB_CNT is read. Accessing both counters must not be interfered by other register accesses.
#[derive(From, Into)]
pub struct RX_IN_BYTES_MSB_CNT(u32);
impl RX_IN_BYTES_MSB_CNT {    ///
    /// Upper 4 bits of RX_IN_BYTES_CNT.
    ///
    /// Counter can be written by SW.
    pub fn rx_in_bytes_msb_cnt(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_rx_in_bytes_msb_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}

/// Register `RX_IN_RANGE_LEN_ERR_CNT`
///
/// Rx In-range Length Error Counter
#[derive(From, Into)]
pub struct RX_IN_RANGE_LEN_ERR_CNT(u32);
impl RX_IN_RANGE_LEN_ERR_CNT {    ///
    /// The number of frames with legal length field that doesn't match length of MAC client data.
    ///
    /// Counter can be written by SW.
    pub fn rx_in_range_len_err_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_in_range_len_err_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_IPG_SHRINK_CNT`
///
/// Rx Inter Packet Gap Shrink Counter
#[derive(From, Into)]
pub struct RX_IPG_SHRINK_CNT(u32);
impl RX_IPG_SHRINK_CNT {    ///
    /// Number of inter packet gap shrinks detected (IPG < 12 bytes).
    ///
    /// Counter can be written by SW.
    pub fn rx_ipg_shrink_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_ipg_shrink_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_JABBERS_CNT`
///
/// Rx Jabbers Counter
#[derive(From, Into)]
pub struct RX_JABBERS_CNT(u32);
impl RX_JABBERS_CNT {    ///
    /// The number of oversize frames with CRC error received.
    ///
    /// Counter can be written by SW.
    pub fn rx_jabbers_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_jabbers_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_MC_CNT`
///
/// Rx Multicast Frame Counter
#[derive(From, Into)]
pub struct RX_MC_CNT(u32);
impl RX_MC_CNT {    ///
    /// The number of good multicast frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_mc_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_mc_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_OK_BYTES_CNT`
///
/// Rx OK Byte Counter
#[derive(From, Into)]
pub struct RX_OK_BYTES_CNT(u32);
impl RX_OK_BYTES_CNT {    ///
    /// The number of received bytes in good frames.
    ///
    /// Counter can be written by SW.
    pub fn rx_ok_bytes_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_ok_bytes_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_OK_BYTES_MSB_CNT`
///
/// MSB of RX ok byte Counter
///
/// Register allowing to access the upper 4 bits of RX_IN_BYTE counter. Please note: When writing to RX_OK_BYTES counter RX_OK_BYTES_MSB_CNT has to be written before RX_OK_BYTES_CNT is written. When reading RX_OK_BYTES counter RX_OK_BYTES_CNT has to be read before RX_OK_BYTES_MSB_CNT is read. Accessing both counters must not be interfered by other register accesses.
#[derive(From, Into)]
pub struct RX_OK_BYTES_MSB_CNT(u32);
impl RX_OK_BYTES_MSB_CNT {    ///
    /// Upper 4 bits of RX_OK_BYTES_CNT.
    ///
    /// Counter can be written by SW.
    pub fn rx_ok_bytes_msb_cnt(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_rx_ok_bytes_msb_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}

/// Register `RX_OUT_OF_RANGE_LEN_ERR_CNT`
///
/// Rx Out-Of-Range Length Error Counter
#[derive(From, Into)]
pub struct RX_OUT_OF_RANGE_LEN_ERR_CNT(u32);
impl RX_OUT_OF_RANGE_LEN_ERR_CNT {    ///
    /// The number of frames with illegal length field (frames using type field are not counted here).
    ///
    /// Counter can be written by SW.
    pub fn rx_out_of_range_len_err_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_out_of_range_len_err_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_OVERSIZE_CNT`
///
/// Rx Oversize Counter (valid frame format)
#[derive(From, Into)]
pub struct RX_OVERSIZE_CNT(u32);
impl RX_OVERSIZE_CNT {    ///
    /// The number of oversize well-formed frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_oversize_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_oversize_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_PAUSE_CNT`
///
/// Rx Pause Frame Counter
#[derive(From, Into)]
pub struct RX_PAUSE_CNT(u32);
impl RX_PAUSE_CNT {    ///
    /// Number of pause control frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_pause_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_pause_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_SIZE1024TO1518_CNT`
///
/// Rx 1024-1518 Byte Frame Counter
#[derive(From, Into)]
pub struct RX_SIZE1024TO1518_CNT(u32);
impl RX_SIZE1024TO1518_CNT {    ///
    /// The number of 1024 to 1518 bytes frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_size1024to1518_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_size1024to1518_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_SIZE128TO255_CNT`
///
/// Rx 128-255 Byte Frame Counter
#[derive(From, Into)]
pub struct RX_SIZE128TO255_CNT(u32);
impl RX_SIZE128TO255_CNT {    ///
    /// The number of 128 to 255 bytes frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_size128to255_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_size128to255_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_SIZE1519TOMAX_CNT`
///
/// Rx 1519 To Max. Length Byte Frame Counter
#[derive(From, Into)]
pub struct RX_SIZE1519TOMAX_CNT(u32);
impl RX_SIZE1519TOMAX_CNT {    ///
    /// The number of frames received longer than 1518 bytes and not longer than Maximum Length Register (Maximum Length Register + 4 if the frame is VLAN tagged).
    ///
    /// Counter can be written by SW.
    pub fn rx_size1519tomax_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_size1519tomax_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_SIZE256TO511_CNT`
///
/// Rx 256-511 Byte Frame Counter
#[derive(From, Into)]
pub struct RX_SIZE256TO511_CNT(u32);
impl RX_SIZE256TO511_CNT {    ///
    /// The number of 256 to 511 bytes frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_size256to511_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_size256to511_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_SIZE512TO1023_CNT`
///
/// Rx 512-1023 Byte Frame Counter
#[derive(From, Into)]
pub struct RX_SIZE512TO1023_CNT(u32);
impl RX_SIZE512TO1023_CNT {    ///
    /// The number of 512 to 1023 bytes frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_size512to1023_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_size512to1023_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_SIZE64_CNT`
///
/// Rx 64 Byte Frame Counter
#[derive(From, Into)]
pub struct RX_SIZE64_CNT(u32);
impl RX_SIZE64_CNT {    ///
    /// The number of 64 bytes frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_size64_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_size64_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_SIZE65TO127_CNT`
///
/// Rx 65-127 Byte Frame Counter
#[derive(From, Into)]
pub struct RX_SIZE65TO127_CNT(u32);
impl RX_SIZE65TO127_CNT {    ///
    /// The number of 65 to 127 bytes frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_size65to127_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_size65to127_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_SYMBOL_ERR_CNT`
///
/// Rx Symbol Carrier Error Counter
#[derive(From, Into)]
pub struct RX_SYMBOL_ERR_CNT(u32);
impl RX_SYMBOL_ERR_CNT {    ///
    /// The number of frames received with one or more symbol errors.
    ///
    /// Counter can be written by SW.
    pub fn rx_symbol_err_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_symbol_err_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_TAGGED_FRMS_CNT`
///
/// Counts frames that are tagged (C-Tagged or S-Tagged).
#[derive(From, Into)]
pub struct RX_TAGGED_FRMS_CNT(u32);
impl RX_TAGGED_FRMS_CNT {    ///
    /// The number of frames received with C-Tag or S-Tag information
    ///
    /// Counter can be written by SW.
    pub fn rx_tagged_frms_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_tagged_frms_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_UC_CNT`
///
/// Rx Unicast Frame Counter
#[derive(From, Into)]
pub struct RX_UC_CNT(u32);
impl RX_UC_CNT {    ///
    /// The number of good unicast frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_uc_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_uc_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_UNDERSIZE_CNT`
///
/// Rx Undersize Counter (valid frame format)
#[derive(From, Into)]
pub struct RX_UNDERSIZE_CNT(u32);
impl RX_UNDERSIZE_CNT {    ///
    /// The number of undersize well-formed frames received.
    ///
    /// Counter can be written by SW.
    pub fn rx_undersize_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_undersize_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_UNSUP_OPCODE_CNT`
///
/// Rx Control Frame Counter
#[derive(From, Into)]
pub struct RX_UNSUP_OPCODE_CNT(u32);
impl RX_UNSUP_OPCODE_CNT {    ///
    /// Number of control frames with unsupported opcode received.
    ///
    /// Counter can be written by SW.
    pub fn rx_unsup_opcode_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_unsup_opcode_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `RX_UNTAGGED_FRMS_CNT`
///
/// Counts frames that are Not tagged  (neither C-Tagged nor S-Tagged).
#[derive(From, Into)]
pub struct RX_UNTAGGED_FRMS_CNT(u32);
impl RX_UNTAGGED_FRMS_CNT {    ///
    /// The number of frames received without C-Tag and S-Tag information.
    ///
    /// Counter can be written by SW.
    pub fn rx_untagged_frms_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_rx_untagged_frms_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_BACKOFF1_CNT`
///
/// Tx 1 Backoff Counter
///
/// Counter collecting the number of frames sent successfully after 1 backoff/collision.
#[derive(From, Into)]
pub struct TX_BACKOFF1_CNT(u32);
impl TX_BACKOFF1_CNT {    ///
    /// Number of frames sent successfully after 1 backoff/collision.
    ///
    /// Counter can be written by SW.
    pub fn tx_backoff1_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_backoff1_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_BC_CNT`
///
/// Tx Broadcast Frame Counter
#[derive(From, Into)]
pub struct TX_BC_CNT(u32);
impl TX_BC_CNT {    ///
    /// The number of broadcast frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_bc_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_bc_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_CSENSE_CNT`
///
/// Tx Carrier Sense Error Counter
///
/// Counter collecting the number of times CarrierSenseError is true at the end of a frame transmission.
#[derive(From, Into)]
pub struct TX_CSENSE_CNT(u32);
impl TX_CSENSE_CNT {    ///
    /// The number of times CarrierSenseError is true at the end of a frame transmission.
    ///
    /// Counter can be written by SW.
    pub fn tx_csense_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_csense_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_DEFER_CNT`
///
/// Tx First Defer Counter
///
/// Counter collecting the number of frames being deferred on first transmission attempt.
#[derive(From, Into)]
pub struct TX_DEFER_CNT(u32);
impl TX_DEFER_CNT {    ///
    /// The number of frames being deferred on first transmission attempt. Note: This counter always counts when a defer event is present, even when it is an excessive defer (TX_XDEFER_CNT).
    ///
    /// Counter can be written by SW.
    pub fn tx_defer_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_defer_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_LATE_COLL_CNT`
///
/// Tx Late Collision Counter
///
/// Counter collecting the number of late collisions.
#[derive(From, Into)]
pub struct TX_LATE_COLL_CNT(u32);
impl TX_LATE_COLL_CNT {    ///
    /// The number of late collisions detected.
    ///
    /// Counter can be written by SW.
    pub fn tx_late_coll_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_late_coll_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_MC_CNT`
///
/// Tx Multicast Frame Counter
#[derive(From, Into)]
pub struct TX_MC_CNT(u32);
impl TX_MC_CNT {    ///
    /// The number of multicast frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_mc_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_mc_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_MULTI_COLL_CNT`
///
/// Tx Multi Collision Counter
///
/// Counter collecting the number of frames transmitted without errors after multiple collisions.
#[derive(From, Into)]
pub struct TX_MULTI_COLL_CNT(u32);
impl TX_MULTI_COLL_CNT {    ///
    /// The number of frames transmitted without errors after multiple collisions.
    ///
    /// Counter can be written by SW.
    pub fn tx_multi_coll_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_multi_coll_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_OK_BYTES_CNT`
///
/// Tx OK Byte Counter
#[derive(From, Into)]
pub struct TX_OK_BYTES_CNT(u32);
impl TX_OK_BYTES_CNT {    ///
    /// The number of bytes transmitted successfully.
    ///
    /// Counter can be written by SW.
    pub fn tx_ok_bytes_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_ok_bytes_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_OK_BYTES_MSB_CNT`
///
/// MSB of TX ok byte Counter
///
/// Register allowing to access the upper 4 bits of RX_IN_BYTE counter. Please note: When writing to TX_OK_BYTES counter TX_OK_BYTES_MSB_CNT has to be written before TX_OK_BYTES_CNT is written. When reading TX_OK_BYTES counter TX_OK_BYTES_CNT has to be read before TX_OK_BYTES_MSB_CNT is read. Accessing both counters must not be interfered by other register accesses.
#[derive(From, Into)]
pub struct TX_OK_BYTES_MSB_CNT(u32);
impl TX_OK_BYTES_MSB_CNT {    ///
    /// Upper 4 bits of TX_OK_BYTES_CNT.
    ///
    /// Counter can be written by SW.
    pub fn tx_ok_bytes_msb_cnt(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_tx_ok_bytes_msb_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}

/// Register `TX_OUT_BYTES_CNT`
///
/// Tx Byte Counter
#[derive(From, Into)]
pub struct TX_OUT_BYTES_CNT(u32);
impl TX_OUT_BYTES_CNT {    ///
    /// The number of bytes transmitted (good, bad and framing).
    ///
    /// Counter can be written by SW.
    pub fn tx_out_bytes_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_out_bytes_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_OUT_BYTES_MSB_CNT`
///
/// MSB of TX out byte Counter
///
/// Register allowing to access the upper 4 bits of RX_IN_BYTE counter. Please note: When writing to TX_OUT_BYTES counter TX_OUT_BYTES_MSB_CNT has to be written before TX_OUT_BYTES_CNT is written. When reading TX_OUT_BYTES counter TX_OUT_BYTES_CNT has to be read before TX_OUT_BYTES_MSB_CNT is read. Accessing both counters must not be interfered by other register accesses.
#[derive(From, Into)]
pub struct TX_OUT_BYTES_MSB_CNT(u32);
impl TX_OUT_BYTES_MSB_CNT {    ///
    /// Upper 4 bits of TX_OUT_BYTES_CNT.
    ///
    /// Counter can be written by SW.
    pub fn tx_out_bytes_msb_cnt(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_tx_out_bytes_msb_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}

/// Register `TX_PAUSE_CNT`
///
/// Tx Pause Frame Counter
#[derive(From, Into)]
pub struct TX_PAUSE_CNT(u32);
impl TX_PAUSE_CNT {    ///
    /// The number of pause control frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_pause_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_pause_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_SIZE1024TO1518_CNT`
///
/// Tx 1024-1518 Byte Frame Counter
#[derive(From, Into)]
pub struct TX_SIZE1024TO1518_CNT(u32);
impl TX_SIZE1024TO1518_CNT {    ///
    /// The number of 1024 to 1518 bytes frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_size1024to1518_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_size1024to1518_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_SIZE128TO255_CNT`
///
/// Tx 128-255 Byte Frame Counter
#[derive(From, Into)]
pub struct TX_SIZE128TO255_CNT(u32);
impl TX_SIZE128TO255_CNT {    ///
    /// The number of 128 to 255 bytes frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_size128to255_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_size128to255_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_SIZE1519TOMAX_CNT`
///
/// Tx 1519 To Max. Length Byte Frame Counter
#[derive(From, Into)]
pub struct TX_SIZE1519TOMAX_CNT(u32);
impl TX_SIZE1519TOMAX_CNT {    ///
    /// The number of frames transmitted longer than 1518 bytes and not longer than Maximum Length Register (Maximum Length Register + 4 if the frame is VLAN tagged).
    ///
    /// Counter can be written by SW.
    pub fn tx_size1519tomax_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_size1519tomax_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_SIZE256TO511_CNT`
///
/// Tx 256-511 Byte Frame Counter
#[derive(From, Into)]
pub struct TX_SIZE256TO511_CNT(u32);
impl TX_SIZE256TO511_CNT {    ///
    /// The number of 256 to 511 bytes frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_size256to511_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_size256to511_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_SIZE512TO1023_CNT`
///
/// Tx 512-1023 Byte Frame Counter
#[derive(From, Into)]
pub struct TX_SIZE512TO1023_CNT(u32);
impl TX_SIZE512TO1023_CNT {    ///
    /// The number of 512 to 1023 bytes frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_size512to1023_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_size512to1023_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_SIZE64_CNT`
///
/// Tx 64 Byte Frame Counter
#[derive(From, Into)]
pub struct TX_SIZE64_CNT(u32);
impl TX_SIZE64_CNT {    ///
    /// The number of 64 bytes frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_size64_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_size64_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_SIZE65TO127_CNT`
///
/// Tx 65-127 Byte Frame Counter
#[derive(From, Into)]
pub struct TX_SIZE65TO127_CNT(u32);
impl TX_SIZE65TO127_CNT {    ///
    /// The number of 65 to 127 bytes frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_size65to127_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_size65to127_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_TAGGED_FRMS_CNT`
///
/// Counts frames that are tagged (C-Tagged or S-Tagged).
#[derive(From, Into)]
pub struct TX_TAGGED_FRMS_CNT(u32);
impl TX_TAGGED_FRMS_CNT {    ///
    /// The number of frames transmitted with C-Tag or S-Tag information
    pub fn tx_tagged_frms_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_tagged_frms_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_UC_CNT`
///
/// Tx Unicast Frame Counter
#[derive(From, Into)]
pub struct TX_UC_CNT(u32);
impl TX_UC_CNT {    ///
    /// The number of unicast frames transmitted.
    ///
    /// Counter can be written by SW.
    pub fn tx_uc_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_uc_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_UNTAGGED_FRMS_CNT`
///
/// Counts frames that are Not tagged  (neither C-Tagged nor S-Tagged).
#[derive(From, Into)]
pub struct TX_UNTAGGED_FRMS_CNT(u32);
impl TX_UNTAGGED_FRMS_CNT {    ///
    /// The number of frames transmitted without C-Tag and S-Tag information.
    pub fn tx_untagged_frms_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_untagged_frms_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_XCOLL_CNT`
///
/// Tx Excessive Collision Counter
///
/// Counter collecting the number of frames due to excessive collisions.
#[derive(From, Into)]
pub struct TX_XCOLL_CNT(u32);
impl TX_XCOLL_CNT {    ///
    /// The number of frames lost due to excessive collisions.
    ///
    /// Counter can be written by SW.
    pub fn tx_xcoll_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_xcoll_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `TX_XDEFER_CNT`
///
/// Tx Excessive Defer Counter
///
/// Counter collecting the number of frames sent with excessive deferral.
#[derive(From, Into)]
pub struct TX_XDEFER_CNT(u32);
impl TX_XDEFER_CNT {    ///
    /// The number of frames sent with excessive deferral.
    ///
    /// Counter can be written by SW.
    pub fn tx_xdefer_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_xdefer_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}