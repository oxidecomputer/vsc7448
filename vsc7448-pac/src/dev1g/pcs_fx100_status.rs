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
/// PCS 100Base FX Status
///
/// Status bit groups for 100Base-FX PCS. Note: If sigdet_cfg != "00" is selected status signal "signal_detect" shows the internal signal_detect value is gated with the status of rx toggle-rate control circuitry.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCS_FX100_STATUS(u32);
impl PCS_FX100_STATUS {
    /// Data change position in the 10bit words received. Must be used for adjusting PTP ingress delays.
    #[inline(always)]
    pub fn edge_pos_ptp(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    #[inline(always)]
    pub fn set_edge_pos_ptp(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        let value = value << 8;
        self.0 &= !0xf00;
        self.0 |= value;
    }
    /// Far-end Fault state has occurred
    ///
    /// 1: A Far-End Fault has been detected 0: No Far-End Fault occurred Bit is cleared by writing a 1 to this position.
    #[inline(always)]
    pub fn fef_found_sticky(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_fef_found_sticky(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Current status of Far-end Fault detection state
    ///
    /// 1: Link currently in fault state 0: Link is in normal state
    #[inline(always)]
    pub fn fef_status(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_fef_status(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// PCS error has occurred
    ///
    /// 1: RX_ER was high while RX_DV active 0: No RX_ER indication found while RX_DV active Bit is cleared by writing a 1 to this position.
    #[inline(always)]
    pub fn pcs_error_sticky(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_pcs_error_sticky(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Current status of selected signal_detect input line
    ///
    /// 1: Proper signal detected 0: No proper signal found
    #[inline(always)]
    pub fn signal_detect(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_signal_detect(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Stream Start Delimiter error occurred
    ///
    /// 1: A Start-of-Stream Delimiter error has been detected 0: No SSD error occurred Bit is cleared by writing a 1 to this position.
    #[inline(always)]
    pub fn ssd_error_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_ssd_error_sticky(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Synchronization lost
    ///
    /// 1: Synchronization lost 0: No sync lost occurred Bit is cleared by writing a 1 to this position.
    #[inline(always)]
    pub fn sync_lost_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_sync_lost_sticky(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Status of synchronization
    ///
    /// 1: Link established 0: No link found
    #[inline(always)]
    pub fn sync_status(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_sync_status(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
