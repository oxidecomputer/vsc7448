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
/// Base Tick configuration.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TTI_TICK_BASE(u32);
impl TTI_TICK_BASE {
    /// Current value of Base Tick counter.
    pub fn base_cnt(&self) -> u32 {
        (self.0 & 0x3fff0000) >> 16
    }
    pub fn set_base_cnt(&mut self, value: u32) {
        assert!(value <= 0x3fff);
        let value = value << 16;
        self.0 &= !0x3fff0000;
        self.0 |= value;
    }
    /// Length of TTI Base Tick. Unit: One system clock cycle. In default configuration and a clock cycle of 6.4 ns, the tick length corresponds to 52us. If the device is uses a longer clock cycle, then the value of BASE_LEN must be reconfigured accordingly.
    pub fn base_len(&self) -> u32 {
        self.0 & 0x3fff
    }
    pub fn set_base_len(&mut self, value: u32) {
        assert!(value <= 0x3fff);
        self.0 &= !0x3fff;
        self.0 |= value;
    }
}
/// Length of TTI Ticks 0-3
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TTI_TICK_LEN_0_3(u32);
impl TTI_TICK_LEN_0_3 {
    /// Length of TTI Tick 0. Unit: Base Ticks, as configured in TTI_TICK_BASE.BASE_LEN. In default configuration the tick length corresponds to 52us.
    pub fn len0(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_len0(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// Length of TTI Tick 1. Unit: TTI Tick 0, as configured in TTI_TICK_LEN_0_3.LEN0. In default configuration the tick length corresponds to 416us.
    pub fn len1(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_len1(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// Length of TTI Tick 2. Unit: TTI Tick 1, as configured in TTI_TICK_LEN_0_3.LEN1. In default configuration the tick length corresponds to 3.3ms.
    pub fn len2(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_len2(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 16;
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    /// Length of TTI Tick 3. Unit: TTI Tick 2, as configured in TTI_TICK_LEN_0_3.LEN2. In default configuration the tick length corresponds to 10ms.
    pub fn len3(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_len3(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 24;
        self.0 &= !0xff000000;
        self.0 |= value;
    }
}
/// Length of TTI Ticks 4-7
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TTI_TICK_LEN_4_7(u32);
impl TTI_TICK_LEN_4_7 {
    /// Length of TTI Tick 4. Unit: TTI Tick 3, as configured in TTI_TICK_LEN_0_3.LEN3. In default configuration the tick length corresponds to 100ms.
    pub fn len4(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_len4(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// Length of TTI Tick 5. Unit: TTI Tick 4, as configured in TTI_TICK_LEN_4_7.LEN4. In default configuration the tick length corresponds to 1s.
    pub fn len5(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_len5(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// Length of TTI Tick 6. Unit: TTI Tick 5, as configured in TTI_TICK_LEN_4_7.LEN5. In default configuration the tick length corresponds to 10s.
    pub fn len6(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_len6(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 16;
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    /// Length of TTI Tick 7. Unit: TTI Tick 6, as configured in TTI_TICK_LEN_4_7.LEN6. In default configuration the tick length corresponds to 1min.
    pub fn len7(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_len7(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 24;
        self.0 &= !0xff000000;
        self.0 |= value;
    }
}
/// Current state of TTI Tick counters
///
/// The TTI Tick counters are permanently running. Their current state (CNT and ERA) can be inspected and written to through these registers.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TTI_TICK_STATE(u32);
impl TTI_TICK_STATE {
    /// Tick's current counter value.
    pub fn tick_cnt(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_tick_cnt(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// Tick's current era. Each tick counts up to its configured LEN. When LEN is reached TICK_ERA toggles and the tick restarts counting from 0. When a TTI in TTI_TBL is processed, then the LAST_TICK_ERA of the TTI is compared with the TICK_ERA of the tick used by the TTI and if they differ the TTI's TICK_CNT is decremented.
    pub fn tick_era(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_tick_era(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
}
