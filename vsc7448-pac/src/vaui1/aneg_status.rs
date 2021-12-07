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

/// Register `ANEG_LP_ADV_ABILITY_0`
///
/// ANEG Link Partner Advertised Ability 0
///
/// 48 bits that contain the link partner's advertised abilities / next page information (received link code word, lower 32 bits, received during auto-negotiation). The bit groups are only valid for base pages; for next page data exchange a different bit group coding has to be applied.
#[derive(From, Into)]
pub struct ANEG_LP_ADV_ABILITY_0(u32);
impl ANEG_LP_ADV_ABILITY_0 {    ///
    /// Acknowledge bit (this bit is automatically overwritten by ANEG)
    pub fn ackn(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_ackn(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }    ///
    /// Technology Ability advertised by LP (here: 10GBase-KR)
    ///
    /// 0: LP is not 10GB-KR capable 1: LP is 10GB-KR capable
    pub fn cap_10gkr(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_cap_10gkr(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }    ///
    /// Technology Ability advertised by LP (here: 10GBase-KX4)
    ///
    /// 0: LP is not 10GB-KX4 capable 1: LP is 10GB-KX4 capable
    pub fn cap_10gkx4(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_cap_10gkx4(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }    ///
    /// Technology Ability advertised by LP (here: 1000Base-KX)
    ///
    /// 0: LP is not 1GB-KX capable 1: LP is 1GB-KX capable
    pub fn cap_1gkx(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_cap_1gkx(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }    ///
    /// Echoed nonce field
    pub fn echoed_nonce(&self) -> u32 {
        (self.0 & 0x3e0) >> 5
    }
    pub fn set_echoed_nonce(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x3e0);
        self.0 &= !0x3e0;
        self.0 |= value;
    }    ///
    /// Bits 31 down to 24 of link code word received from link partner.
    pub fn lp_adv_abil_lsb(&self) -> u32 {
        (self.0 & 0xffffff) >> 24
    }
    pub fn set_lp_adv_abil_lsb(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }    ///
    /// Next page exchange desired by LP
    ///
    /// 0: No NP exchange desired 1: NP exchange desired
    pub fn np(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_np(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }    ///
    /// Pause field
    pub fn pause(&self) -> u32 {
        (self.0 & 0x1c00) >> 10
    }
    pub fn set_pause(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x1c00);
        self.0 &= !0x1c00;
        self.0 |= value;
    }    ///
    /// RF bit
    pub fn rf(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_rf(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }    ///
    /// Selector field
    pub fn sel_field(&self) -> u32 {
        (self.0 & 0x1f) >> 0
    }
    pub fn set_sel_field(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }    ///
    /// Transmit-Nonce field (received from LinkPartner)
    pub fn tx_nonce(&self) -> u32 {
        (self.0 & 0x1f0000) >> 16
    }
    pub fn set_tx_nonce(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x1f0000);
        self.0 &= !0x1f0000;
        self.0 |= value;
    }
}

/// Register `ANEG_LP_ADV_ABILITY_1`
///
/// ANEG Link Partner Advertised Ability 1
///
/// 48 bits that contain the link partner's advertised abilities or next page information (received link code word, upper 16 bits, received during auto-negotiation). The bit groups are only valid for base pages; for next page data exchange a different bit group coding has to be applied.
#[derive(From, Into)]
pub struct ANEG_LP_ADV_ABILITY_1(u32);
impl ANEG_LP_ADV_ABILITY_1 {    ///
    /// FEC capability (bit 14: FEC ability, bit 15: FEC requested) - Only used with 10GBase-KR
    pub fn fec(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    pub fn set_fec(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0xc000);
        self.0 &= !0xc000;
        self.0 |= value;
    }    ///
    /// Bits 45 down to 32 of link code word received from link partner.
    pub fn lp_adv_abil_msb(&self) -> u32 {
        (self.0 & 0x3fff) >> 0
    }
    pub fn set_lp_adv_abil_msb(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3fff);
        self.0 &= !0x3fff;
        self.0 |= value;
    }
}

/// Register `ANEG_NEXT_PAGE_1`
///
/// ANEG Next Page 1
///
/// 48 bits that contain the new next page to transmit during auto-negotiation (here: upper 16 bits).
#[derive(From, Into)]
pub struct ANEG_NEXT_PAGE_1(u32);
impl ANEG_NEXT_PAGE_1 {    ///
    /// Must be set when a new next page is programmed (self-clearing)
    pub fn next_page_loaded_one_shot(&self) -> u32 {
        (self.0 & 0x7fffffff) >> 31
    }
    pub fn set_next_page_loaded_one_shot(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x7fffffff);
        self.0 &= !0x7fffffff;
        self.0 |= value;
    }    ///
    /// Upper 16 bits of next page link code word
    pub fn np_tx_msb(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_np_tx_msb(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `ANEG_STATUS`
///
/// ANEG Status
///
/// Auto negotiation status register
#[derive(From, Into)]
pub struct ANEG_STATUS(u32);
impl ANEG_STATUS {    ///
    /// Error condition indicating an Arbitration state machine error.
    ///
    /// Bit is cleared by writing a 1 to this position.
    pub fn aneg_arb_fsm_err_sticky(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_aneg_arb_fsm_err_sticky(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }    ///
    /// Status indicating whether auto-negotiation has completed.
    pub fn aneg_complete(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_aneg_complete(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Error condition indicating a Receive state machine error.
    ///
    /// Bit is cleared by writing a 1 to this position.
    pub fn aneg_rx_fsm_err_sticky(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_aneg_rx_fsm_err_sticky(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }    ///
    /// Error condition indicating a Transmit state machine error.
    ///
    /// Bit is cleared by writing a 1 to this position.
    pub fn aneg_tx_fsm_err_sticky(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_aneg_tx_fsm_err_sticky(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }    ///
    /// Current state of Arbiter State Machine
    ///
    /// 0000: AUTO_NEG_ENA 0001: TX_DISABLE 0010: ABILITY_DETECT 0011: ACKN_DETECT 0100: COMPLETE_ACKN 0101: AN_GOOD_CHECK 0110: AN_GOOD 0111: NEXT_PAGE_WAIT 1000: LINK_STATUS_CHECK 1001: PARALLEL_DET_FAULT 1010: PD_CHECK10GKR 1011: PD_CHECK2G5 1100: PD_CHECK1GKX 1101: PD_CHECK10GKX4
    pub fn arbiter_state(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    pub fn set_arbiter_state(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xf00);
        self.0 &= !0xf00;
        self.0 |= value;
    }    ///
    /// Error condition indicating that no compatible link was found.
    pub fn incompatible_link(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_incompatible_link(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }    ///
    /// Link control information for 10G quad lane mode
    ///
    /// 00: Disabled 01: Enabled 11: Scan for carrier
    pub fn link_ctrl_10gkx4(&self) -> u32 {
        (self.0 & 0x30000000) >> 28
    }
    pub fn set_link_ctrl_10gkx4(&mut self, value: u32) {
        let value = value << 28;
        assert!(value <= 0x30000000);
        self.0 &= !0x30000000;
        self.0 |= value;
    }    ///
    /// Link control information for 1G single lane mode
    ///
    /// 00: Disabled 01: Enabled 11: Scan for carrier
    pub fn link_ctrl_1gkx(&self) -> u32 {
        (self.0 & 0xc000000) >> 26
    }
    pub fn set_link_ctrl_1gkx(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0xc000000);
        self.0 &= !0xc000000;
        self.0 |= value;
    }    ///
    /// Link control information for 2.5G single lane mode
    ///
    /// 00: Disabled 01: Enabled 11: Scan for carrier
    pub fn link_ctrl_2g5(&self) -> u32 {
        (self.0 & 0x3000000) >> 24
    }
    pub fn set_link_ctrl_2g5(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x3000000);
        self.0 &= !0x3000000;
        self.0 |= value;
    }    ///
    /// Status indicating whether the link partner supports auto-negotiation.
    pub fn lp_aneg_able(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_lp_aneg_able(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Status indicating whether a new page has been received.
    pub fn page_rx(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_page_rx(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Error condition indicating errors during parallel detection.
    ///
    /// Bit is cleared by writing a 1 to this position.
    pub fn par_detect_fault_sticky(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_par_detect_fault_sticky(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
}
