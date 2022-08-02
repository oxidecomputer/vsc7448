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
/// Timer control
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TIMER_CTRL(u32);
impl TIMER_CTRL {
    /// Set this field to force the reload of the timer, this will set the TIMER_VALUE to TIMER_RELOAD_VALUE for the corresponding timer. This field can be set at the same time as enabeling the counter, in that case the counter will be reloaded and then enabled for counting.
    #[inline(always)]
    pub fn force_reload(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_force_reload(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// When set the timer will count continously (increment in each clock cycle) with out regard to the timer-tick generator. This feature is used for timing validation of the VCore system.
    #[inline(always)]
    pub fn max_freq_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_max_freq_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// When set the timer will automatically disable itself after it has generated interrupt.
    #[inline(always)]
    pub fn one_shot_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_one_shot_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// When enabled, the correponding timer decrements at each timer-tick. If TIMER_CTRL.ONE_SHOT_ENA is set this field is cleared when the timer reach 0 and interrupt is generated.
    ///
    /// 0: Timer is disabled 1: Timer is enabled
    #[inline(always)]
    pub fn timer_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_timer_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Timer reload value
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TIMER_RELOAD_VALUE(u32);
impl TIMER_RELOAD_VALUE {
    /// The contents of this field are loaded into the corresponding timer (TIMER_VALUE) when it wraps (decrements a zero).
    #[inline(always)]
    pub fn reload_val(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_reload_val(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Timer tick divider
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TIMER_TICK_DIV(u32);
impl TIMER_TICK_DIV {
    /// The timer tick generator runs from the VCore System frequency. By default, the divider value generates a timer tick every 100 us (10KHz). The timer tick is used for all of the timers (except the WDT). This field must not be set to generate a timer tick of less than 0.1 us (higher than 10MHz). If this field is changed, it may take up to 2ms before the timers are running stable at the new frequency.
    ///
    /// The timer tick frequency is: 250MHz/(TIMER_TICK_DIV+1).
    #[inline(always)]
    pub fn timer_tick_div(&self) -> u32 {
        self.0 & 0x3ffff
    }
    #[inline(always)]
    pub fn set_timer_tick_div(&mut self, value: u32) {
        debug_assert!(value <= 0x3ffff);
        self.0 &= !0x3ffff;
        self.0 |= value;
    }
}
/// Timer value
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TIMER_VALUE(u32);
impl TIMER_VALUE {
    /// The current value of the timer. When enabled via TIMER_CTRL.TIMER_ENA the timer decrements at every timer tick (see TIMER_TICK_DIV for more info on timer tick frequency). When the timer has reached 0, and a timer-tick is received, then an interrupt is generated. For example; If a periodic interrupt is needed every 1ms, and the timer tick is generated every 100us then the TIMER_VALUE (and TIMER_RELOAD_VALUE) must be configured to 9. By default the timer will reload from the TIMER_RELOAD_VALUE when interrupt is generated, and then continue decrementing from the reloaded value. It is possible to make the timer stop after generating interrupt by setting TIMER_CTRL.ONE_SHOT_ENA.
    #[inline(always)]
    pub fn timer_val(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_timer_val(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Watchdog timer
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct WDT(u32);
impl WDT {
    /// Use this field to enable or disable the watchdog timer. When the WDT is enabled, it causes a reset after 2 seconds if it is not periodically reset. This field is only read by the WDT after a sucessful lock sequence (see ICPU_CFG::WDT.WDT_LOCK).
    ///
    /// 0: WDT is disabled 1: WDT is enabled
    #[inline(always)]
    pub fn wdt_enable(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_wdt_enable(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Use this field to configure and reset the WDT. When writing 0xBE to this field immediately followed by writing 0xEF, the WDT resets and configurations are read from this register (as provided when the 0xEF is written). When the WDT is enabled, writing any value other than 0xBE or 0xEF after 0xBE is written, causes a WDT reset as if the timer had run out.
    #[inline(always)]
    pub fn wdt_lock(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_wdt_lock(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// Shows whether the last reset was caused by a watchdog timer reset. This field is updated during reset, therefore it is always valid.
    ///
    /// 0: Reset was not caused by WDT 1: Reset was caused by WDT timeout
    #[inline(always)]
    pub fn wdt_status(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_wdt_status(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
}
