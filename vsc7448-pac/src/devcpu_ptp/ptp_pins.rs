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
/// Waveform programming
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PIN_WF_HIGH_PERIOD(u32);
impl PIN_WF_HIGH_PERIOD {
    /// Configure waveform. Unit is nanoseconds. EX. 25MHz 60/40 clock: PIN_ACTION=CLOCK, PIN_SYNC=0, PIN_WFH=24, PIN_WFL=16 EX. 1 us pulse after 150 ns PIN_ACTION=CLOCK, PIN_SYNC=1, PIN_WFH=1000, PIN_WFL=150
    pub fn pin_wfh(&self) -> u32 {
        self.0 & 0x3fffffff
    }
    pub fn set_pin_wfh(&mut self, value: u32) {
        assert!(value <= 0x3fffffff);
        self.0 &= !0x3fffffff;
        self.0 |= value;
    }
}
/// Waveform programming
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PIN_WF_LOW_PERIOD(u32);
impl PIN_WF_LOW_PERIOD {
    /// Configure waveform
    pub fn pin_wfl(&self) -> u32 {
        self.0 & 0x3fffffff
    }
    pub fn set_pin_wfl(&mut self, value: u32) {
        assert!(value <= 0x3fffffff);
        self.0 &= !0x3fffffff;
        self.0 |= value;
    }
}
/// Current time of day
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PTP_CUR_SEC_MSB(u32);
impl PTP_CUR_SEC_MSB {
    /// Current time of day, seconds part, latched when NSF was read
    pub fn ptp_cur_sec_msb(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_ptp_cur_sec_msb(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Stamper clock
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PTP_NSF(u32);
impl PTP_NSF {
    /// Value of NSF counter when load/save action was executed. This value will not be loaded into the timers upon a LOAD operation.
    pub fn ptp_nsf(&self) -> u32 {
        self.0
    }
    pub fn set_ptp_nsf(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Configuration of use of the register group
///
/// Select use of the ptp i/o pin. Ptp pin 4 is not attached to any physical I/O.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PTP_PIN_CFG(u32);
impl PTP_PIN_CFG {
    /// Defined actions are: IDLE: No operation LOAD:   Load TimeOfDay with configured values STORE:  Store TimeOfDay and NSF of selected time domain CLOCK:  Generate a clock output DELTA:  Add PTP_TOD_NSEC field as a signed integer to TimeOfDay When the sync option is set, the action will be done when the pin sees an active edge. The action will automatically return to IDLE when complete.
    ///
    /// 0: IDLE 1: LOAD 2: STORE 3: CLOCK 4: DELTA 5-7: reserved
    pub fn ptp_pin_action(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_ptp_pin_action(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }
    /// Configures the time domain the pin is connected to.
    pub fn ptp_pin_dom(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_ptp_pin_dom(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Polarity of the PTP pin.
    ///
    /// 0: Active high 1: Active low
    pub fn ptp_pin_inv_pol(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_ptp_pin_inv_pol(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// For LOAD/STORE/DELTA actions, setting this option will suspend the action until an active edge is seen on the pin. Otherwise it will be done immidiately. For the CLOCK action, the sync option makes the pin generate a single pulse, <WAFEFORM_LOW> nanoseconds after the timeof day has increased the seconds. The pulse will get a width of <WAVEFORM_HIGH> nanoseconds.
    pub fn ptp_pin_sync(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_ptp_pin_sync(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// Time Of Day nanosecs part
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PTP_TOD_NSEC(u32);
impl PTP_TOD_NSEC {
    /// Time Of Day nanoseconds loaded or stored into TimeOfDay. A synced store operation may return a value between -2 and 999.999.999 in this field. To normalize the complete TOD, in case field is read 0x3ffffffe/f, software must subtract one from the SEC part, and add 1.000.000.000 to the NSEC part.
    pub fn ptp_tod_nsec(&self) -> u32 {
        self.0 & 0x3fffffff
    }
    pub fn set_ptp_tod_nsec(&mut self, value: u32) {
        assert!(value <= 0x3fffffff);
        self.0 &= !0x3fffffff;
        self.0 |= value;
    }
}
/// Time Of Day LSB
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PTP_TOD_SEC_LSB(u32);
impl PTP_TOD_SEC_LSB {
    /// Bits 31:0 of the time-of-day seconds
    pub fn ptp_tod_sec_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_ptp_tod_sec_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Time Of Day MSB
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PTP_TOD_SEC_MSB(u32);
impl PTP_TOD_SEC_MSB {
    /// Bits 47:32 of the time-of-day seconds
    pub fn ptp_tod_sec_msb(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_ptp_tod_sec_msb(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
