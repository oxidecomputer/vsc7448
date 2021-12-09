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
/// PCS2X6G Configuration
///
/// Configuration register
#[derive(From, Into)]
pub struct PCS2X6G_CFG(u32);
impl PCS2X6G_CFG {
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
    /// PCS2X6G enable

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
    /// Enable rate adaption (use in parallel loop mode only)

    ///

    /// 0: Rate adaption disabled 1: Rate adaption enabled
    pub fn ploop_ra_ena(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_ploop_ra_ena(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// Force re-synchronization of receive logic

    ///

    /// 0: Normal operation 1: Reset Synchronization
    pub fn resync_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_resync_ena(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Disable Scrambler in 64B/66B codec (to be used only for test purposes)

    ///

    /// 0: Enable scrambler 1: Disable scrambler
    pub fn scram_dis(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_scram_dis(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Signal Detect Enable

    ///

    /// 0: The Signal Detect input pin is ignored. The PCS2X6G assumes an active Signal Detect at all times 1: The Signal Detect input pin is used to determine if a signal is detected
    pub fn sd_ena(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_sd_ena(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    /// Signal detect polarity: The signal level on signal_detect input pin must be equal to SD_POL to indicate signal detection (SD_ENA must be set)

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
    }
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
    }
    /// Number of sync headers required for block lock plus one (used in 10Gb-R type SSM)
    pub fn sh_cnt_max(&self) -> u32 {
        (self.0 & 0xfc00) >> 10
    }
    pub fn set_sh_cnt_max(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xfc00);
        self.0 &= !0xfc00;
        self.0 |= value;
    }
    /// Testloop, if active data are looped from TX path to RX path without using SERDES loops

    ///

    /// 0: Normal operation 1: Testloop enable
    pub fn sloop_ena(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_sloop_ena(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Select type of Synchronization State Machine

    ///

    /// 0: Use 10Gb-R (Cl. 49) type 1: Use 10Gb-X (Cl. 48) type Others: Reserved
    pub fn sync_type_sel(&self) -> u32 {
        (self.0 & 0x300) >> 8
    }
    pub fn set_sync_type_sel(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x300);
        self.0 &= !0x300;
        self.0 |= value;
    }
}
/// PCS2X6G Error Status
///
/// Error indication of 64B/66B PCS2X6G logic
#[derive(From, Into)]
pub struct PCS2X6G_ERR_STATUS(u32);
impl PCS2X6G_ERR_STATUS {
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
    /// Coding error detected in received 64B/66B encoded data

    ///

    /// 0: No error found 1: Coding error detected Bit is cleared by writing a 1 to this position.
    pub fn c64b66b_err_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_c64b66b_err_sticky(&mut self, value: u32) {
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
    /// Coding error detected in xgmii data to be transmitted

    ///

    /// 0: No error found 1: Coding error detected Bit is cleared by writing a 1 to this position.
    pub fn xgmii_err_sticky(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_xgmii_err_sticky(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
}
/// PCS2X6G Status
///
/// Status of PCS2X6G logic
#[derive(From, Into)]
pub struct PCS2X6G_STATUS(u32);
impl PCS2X6G_STATUS {
    /// Status of deskew logic

    ///

    /// 0: Lanes not aligned 1: All lanes are aligned
    pub fn rx_alignment_status(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rx_alignment_status(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Status of synchronization in lane i (i = 0...3, one bit per lane)

    ///

    /// 0: Lane i out of sync 1: Lane i is in sync
    pub fn rx_sync_status(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_rx_sync_status(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
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
}
