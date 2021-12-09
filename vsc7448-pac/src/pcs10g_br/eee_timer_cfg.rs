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

/// Register `KR_FEC_CAPABILITY`
///
/// FEC ability register
#[derive(From, Into)]
pub struct KR_FEC_CAPABILITY(u32);
impl KR_FEC_CAPABILITY {
    /// This is a status bit indicating whether PHY supports FEC or not.

    ///

    /// 0: This PHY device does not support FEC. 1: This PHY device supports FEC.
    pub fn fec_capable(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_fec_capable(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `ONE_US_TIMER_REG`
///
/// One micro-second timer configuration register
///
/// This register value is used to elaps 1us time as specified in IEEE802.3az-2010 clause 49.
#[derive(From, Into)]
pub struct ONE_US_TIMER_REG(u32);
impl ONE_US_TIMER_REG {
    /// This holds no.of 64-bit PMA clocks required to achieve 1 micro-second (1 us) time interval. As per IEEE 803.3az-2010, min and max values are as follows: MIN : 1.1 us = 178 MAX: 1.3 us = 209 Note: Default value is ~1.1 us = 178
    pub fn one_us_timer(&self) -> u32 {
        self.0 & 0xfffff
    }
    pub fn set_one_us_timer(&mut self, value: u32) {
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }
}

/// Register `RX_TQ_TIMER_REG`
///
/// EEE RX Quiet timer configuration register
///
/// This register value is used to elaps time in RX_SLEEP and RX_QUIET states of EEE RX-FSM as specified in IEEE802.3az-2010 clause 49.
#[derive(From, Into)]
pub struct RX_TQ_TIMER_REG(u32);
impl RX_TQ_TIMER_REG {
    /// This holds no.of 64-bit PMA clocks required to achieve specified sleep+quiet time interval. As per IEEE 803.3az-2010, min and max values are as follows: MIN : 2 ms = 322266 MAX: 3 ms = 483398 Note: Default value is ~2.5 ms = 402832
    pub fn rx_tq_timer(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_rx_tq_timer(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}

/// Register `RX_TW_TIMER_REG`
///
/// EEE RX Wake timer configuration register
///
/// This register value is used to elaps time in RX_WAKE state of EEE RX-FSM as specified in IEEE802.3az-2010 clause 49.
#[derive(From, Into)]
pub struct RX_TW_TIMER_REG(u32);
impl RX_TW_TIMER_REG {
    /// This holds no.of 64-bit PMA clocks required to achieve specified wake time interval. As per IEEE 803.3az-2010, this timer values are as follows: Without scrambler bypassed (or Without FEC): 11.5 us = 1853 With scrambler bypassed (or With FEC): 13.7 us = 2207 Note: Default value is ~11.5 us = 1853
    pub fn rx_tw_timer(&self) -> u32 {
        self.0 & 0x1fffff
    }
    pub fn set_rx_tw_timer(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}

/// Register `TX_TQ_TIMER_REG`
///
/// EEE TX Quiet timer configuration register
///
/// This register value is used to elaps time in TX_QUIET state of EEE TX-FSM as specified in IEEE802.3az-2010 clause 49.
#[derive(From, Into)]
pub struct TX_TQ_TIMER_REG(u32);
impl TX_TQ_TIMER_REG {
    /// This holds no.of 64-bit PMA clocks required to achieve specified quiet time interval. As per IEEE 803.3az-2010, min and max values are as follows: MIN : 1.7 ms = 273926 MAX: 1.8 ms = 290039 Note: Default value is ~1.7 ms = 273926
    pub fn tx_tq_timer(&self) -> u32 {
        self.0 & 0xfffff
    }
    pub fn set_tx_tq_timer(&mut self, value: u32) {
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }
}

/// Register `TX_TS_TIMER_REG`
///
/// EEE TX Sleep timer configuration register
///
/// This register value is used to elaps time in TX_SLEEP state of EEE TX-FSM as specified in IEEE802.3az-2010 clause 49.
#[derive(From, Into)]
pub struct TX_TS_TIMER_REG(u32);
impl TX_TS_TIMER_REG {
    /// This holds no.of 64-bit PMA clocks required to achieve specified sleep time interval. As per IEEE 803.3az-2010, min and max values are as follows: MIN : 4.9 us = 790 MAX: 5.1 us = 821 Note: Default value is ~5 us = 806
    pub fn tx_ts_timer(&self) -> u32 {
        self.0 & 0xfffff
    }
    pub fn set_tx_ts_timer(&mut self, value: u32) {
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }
}

/// Register `TX_TW_TIMER_REG`
///
/// EEE TX Wake timer configuration register
///
/// This register value is used to elaps time in TX_WAKE state of EEE TX-FSM as specified in IEEE802.3az-2010 clause 49.
#[derive(From, Into)]
pub struct TX_TW_TIMER_REG(u32);
impl TX_TW_TIMER_REG {
    /// This holds no.of 64-bit PMA clocks required to achieve specified wake time interval. As per IEEE 803.3az-2010, min and max values are as follows: MIN : 10.9 us = 1757 MAX: 11.1 us = 1788 Note: Default value is ~11 us = 1773
    pub fn tx_tw_timer(&self) -> u32 {
        self.0 & 0xfffff
    }
    pub fn set_tx_tw_timer(&mut self, value: u32) {
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }
}
