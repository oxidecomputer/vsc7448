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
/// Priority-based flow control configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PFC_CFG(u32);
impl PFC_CFG {
    /// Configures the link speed. This is used to evaluate the time specifications in incoming pause frames.
    ///
    /// 0: 12000 Mbps 1: 10000 Mbps 2: 2500 Mbps 3: 1000 Mbps 4: 100 Mbps 5: 10 Mbps
    #[inline]
    pub fn fc_link_speed(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline]
    pub fn set_fc_link_speed(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Enable PFC per priority. Bit n enables PFC on priority n.
    #[inline]
    pub fn rx_pfc_ena(&self) -> u32 {
        (self.0 & 0x7f8) >> 3
    }
    #[inline]
    pub fn set_rx_pfc_ena(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 3;
        self.0 &= !0x7f8;
        self.0 |= value;
    }
}
/// Current timer per priority
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PFC_TIMER(u32);
impl PFC_TIMER {
    /// The current timer value per priority. Value >0 indicates that the priority is paused.
    ///
    /// Unit is 1024 bit times.
    #[inline]
    pub fn pfc_timer_val(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline]
    pub fn set_pfc_timer_val(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
