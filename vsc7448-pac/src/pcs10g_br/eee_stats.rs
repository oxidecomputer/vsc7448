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
/// 10G Base-R PCS EEE interrupt mask register
///
/// Masks for 10G Base-R PCS EEE interrupt sources and sticky bits in EEE_STATUS
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct EEE_INTR_MASK(u32);
impl EEE_INTR_MASK {
    /// Mask for the RX_LPI_RECEIVED bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn rx_lpi_received_mask(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_rx_lpi_received_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Mask for the TX_LPI_RECEIVED bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn tx_lpi_received_mask(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_tx_lpi_received_mask(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}
/// EEE Status register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct EEE_STATUS(u32);
impl EEE_STATUS {
    /// 1 = The MAC may stop the clock during LPI 0 = Clock not stoppable
    pub fn clock_stop_capable(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_clock_stop_capable(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// 1 = Rx PCS is currently receiving LPI 0 = PCS is not currently receiving LPI
    pub fn rx_lpi_indication(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_rx_lpi_indication(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// When read as a one, receive PCS has received LPI signaling one or more times since the register was last read. When read as a zero, PCS has not received LPI signaling
    ///
    /// 1 = Rx PCS has received LPI 0 = LPI not received
    pub fn rx_lpi_received(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_rx_lpi_received(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// 1 = Tx PCS is currently receiving LPI 0 = PCS is not currently receiving LPI
    pub fn tx_lpi_indication(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_tx_lpi_indication(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// When read as a one, transmit PCS has received LPI signaling one or more times since the register was last read. When read as a zero, PCS has not received LPI signaling.
    ///
    /// 1 = Tx PCS has received LPI 0 = LPI not received
    pub fn tx_lpi_received(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_tx_lpi_received(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
}
/// Wake Error Counter
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct WAKE_ERR_CNT(u32);
impl WAKE_ERR_CNT {
    /// This reflects wake_error_counter specifed in IEEE 802.3az-2010, 49.2.13.2.4 Value of this counter indicates how many times LPI RX FSM entered RX_WTF state. Note: 1. This counter is cleared when ever it is read. 2. Upon overflow its value remains at 0xFFFF.
    pub fn wake_err_cnt(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_wake_err_cnt(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
