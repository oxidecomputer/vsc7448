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

/// Register `DTI_CTRL`
///
/// DTI control
#[derive(From, Into)]
pub struct DTI_CTRL(u32);
impl DTI_CTRL {    ///
    /// DTI bandwidth. Used to give arbitration precedence to high bandwidth DTIs.
    ///
    /// 0: <5Gbps 1: >=5Gbps
    pub fn bw(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_bw(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Enable DTI. If MODE=0 or MODE=2, then ENA is cleared by AFI when configured number of sequences have been injected. Before (re)starting a DTI the following initialization should be done: DURATION must be set to 0. NEXT_FRM_PTR should be set to FIRST_FRM_PTR. DTI_CNT_DOWN.CNT_DOWN should be set to 0. FRM_INJ_CNT should be set to 0 AFI::MISC_CTRL.AFI_ENA must be set to 1. If MODE=2, then the AFI will set ENA=1 for the DTI pointed to by DTI_NEXT once the DTI with MODE=2 completes.
    pub fn ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `TTI_FRM`
///
/// Frame pointer for TTI
#[derive(From, Into)]
pub struct TTI_FRM(u32);
impl TTI_FRM {    ///
    /// Pointer to the frame in Frame Table, which TTI shall inject.
    pub fn frm_ptr(&self) -> u32 {
        (self.0 & 0xfff) >> 0
    }
    pub fn set_frm_ptr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}

/// Register `TTI_MISC_CFG`
///
/// Miscellaneous TTI parameters
#[derive(From, Into)]
pub struct TTI_MISC_CFG(u32);
impl TTI_MISC_CFG {    ///
    /// Enable counting of injected frames in AFI:TTI_MISC:TTI_INJ_CNT.TTI_INJ_CNT.
    pub fn inj_cnt_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_inj_cnt_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `TTI_PORT_QU`
///
/// Port and queue for injected frames.
#[derive(From, Into)]
pub struct TTI_PORT_QU(u32);
impl TTI_PORT_QU {    ///
    /// Port number which injection queue transmits on. Injection queue is selected by QU_NUM. PORT_NUM must not be changed when timer is enabled.
    pub fn port_num(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_port_num(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }    ///
    /// QU_NUM selects the queue, which the frame is injected into. For details, refer to the functional description of the queue system in the datasheet. QU_NUM must not be changed when timer is enabled.
    pub fn qu_num(&self) -> u32 {
        (self.0 & 0xffff00) >> 8
    }
    pub fn set_qu_num(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xffff00);
        self.0 &= !0xffff00;
        self.0 |= value;
    }
}

/// Register `TTI_TICKS`
///
/// Current state of TTI's tick counter
#[derive(From, Into)]
pub struct TTI_TICKS(u32);
impl TTI_TICKS {    ///
    /// Ticks era value last time TTI was processed.
    pub fn last_tick_era(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_last_tick_era(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }    ///
    /// Number of ticks until next injection. Frame is injected when TICK_CNT=0. Upon injection TICK_CNT gets set to TIMER_LEN. Should be set to a random value in range 1-TIMER_LEN before starting TTI.
    pub fn tick_cnt(&self) -> u32 {
        (self.0 & 0x1ff) >> 0
    }
    pub fn set_tick_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1ff);
        self.0 &= !0x1ff;
        self.0 |= value;
    }
}

/// Register `TTI_TIMER`
///
/// TTI Timer configuration
#[derive(From, Into)]
pub struct TTI_TIMER(u32);
impl TTI_TIMER {    ///
    /// Configuration of injection time jitter for TTI.
    ///
    /// 0: No jitter 1: Timer is set to a random value in the range [TIMER_LEN*0.75; TIMER_LEN] 2: Timer is set to a random value in the range [TIMER_LEN*0.50; TIMER_LEN] 3: Timer is set to a random value in the range [1;TIMER_LEN]
    pub fn jitter(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_jitter(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }    ///
    /// Timer Tick, which TTI shall use.
    pub fn tick_idx(&self) -> u32 {
        (self.0 & 0x7) >> 0
    }
    pub fn set_tick_idx(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }    ///
    /// Enable timer. Note that disabling a timer can also be achieved by setting TIMER_LEN to 0. Before enabling a timer AFI::MISC_CTRL.AFI_ENA must be set to 1.
    pub fn timer_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_timer_ena(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }    ///
    /// Number of ticks of configured TICK_IDX between frame injections. The period between each injection becomes tick_period x TIMER_LEN Setting TIMER_LEN to non-zero value enables TTI. 0x1FF (= Inject ASAP) is intended for removal of frame from buffer memory. Upon injection, HW sets TIMER_LEN to 0 (=Disable). Before setting TIMER_LEN, TICK_CNT should be set to a random value in range 1-TIMER_LEN (unless a specific initial timer value is desirable).
    ///
    /// 0 => Disable TTI. 1 => 1 tick 2 => 2 ticks ... 0x1ff => Inject ASAP, then set to TIMER_LEN=0 by AFI.
    pub fn timer_len(&self) -> u32 {
        (self.0 & 0x1ff0000) >> 16
    }
    pub fn set_timer_len(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x1ff0000);
        self.0 &= !0x1ff0000;
        self.0 |= value;
    }
}
