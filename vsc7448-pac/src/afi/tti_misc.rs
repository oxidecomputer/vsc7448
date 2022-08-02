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
/// Number of TTIs to service in slot before moving to next TTI Calendar slot.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TTI_CAL_SLOT_CNT(u32);
impl TTI_CAL_SLOT_CNT {
    /// Number of TTIs to service in slot before moving to next TTI Calendar Slot.
    ///
    /// 0: 1 1: 1 2: 2 ...
    #[inline(always)]
    pub fn slot_cnt(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_slot_cnt(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Start and end Frame Table pointers for Calendar Slot
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TTI_CAL_SLOT_PTRS(u32);
impl TTI_CAL_SLOT_PTRS {
    /// Calendar Slot's Frame Table end pointer.
    #[inline(always)]
    pub fn slot_end_ptr(&self) -> u32 {
        (self.0 & 0xfff0000) >> 16
    }
    #[inline(always)]
    pub fn set_slot_end_ptr(&mut self, value: u32) {
        debug_assert!(value <= 0xfff);
        let value = value << 16;
        self.0 &= !0xfff0000;
        self.0 |= value;
    }
    /// Calendar Slot's Frame Table start pointer.
    #[inline(always)]
    pub fn slot_start_ptr(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_slot_start_ptr(&mut self, value: u32) {
        debug_assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// TTI Calendar state information
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TTI_CAL_STATE(u32);
impl TTI_CAL_STATE {
    /// Current value of Calendar Slot's TTI Table Pointer.
    #[inline(always)]
    pub fn slot_tti_tbl_ptr(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_slot_tti_tbl_ptr(&mut self, value: u32) {
        debug_assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// TTI Control
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TTI_CTRL(u32);
impl TTI_CTRL {
    /// Number of remaining TTI Table entries to service for current Calendar Slot.
    #[inline(always)]
    pub fn tti_cal_cnt(&self) -> u32 {
        (self.0 & 0xff00000) >> 20
    }
    #[inline(always)]
    pub fn set_tti_cal_cnt(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        let value = value << 20;
        self.0 &= !0xff00000;
        self.0 |= value;
    }
    /// Length of TTI Calendar.
    ///
    /// 0: Length=1 (Slot 0) 1: Length=2 (Slot 0-1) 2: Length=3 (Slot 0-2) 3: Length=4 (Slot 0-3)
    #[inline(always)]
    pub fn tti_cal_len(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    #[inline(always)]
    pub fn set_tti_cal_len(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        let value = value << 4;
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// Current TTI Calendar slot.
    #[inline(always)]
    pub fn tti_cal_ptr(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    #[inline(always)]
    pub fn set_tti_cal_ptr(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        let value = value << 16;
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// Maximum number of clock cycles that CSR accesses have to wait before gaining access to TTI table. Note that writes need two accesses and may thus have to await 2x the configured number of clock cycles. Setting this parameter too low while doing excessive TTI Table CSR accesses may cause TTIs (in TTI Table) to be serviced too infrequently for their configured timer values.
    ///
    /// 0 => CSR accesses takes precedence. 1 => 1 clock cycle ...
    #[inline(always)]
    pub fn tti_csr_rsv(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline(always)]
    pub fn set_tti_csr_rsv(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// Enable TTIs. Before enabling TTIs, TTI_INIT should be used to initialize Calendar state.
    #[inline(always)]
    pub fn tti_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_tti_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// When set, initialize Calendar to start at Calendar Slot 0. Cleared by AFI when done.
    #[inline(always)]
    pub fn tti_init(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_tti_init(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// If set only one TTI is processed at-a-time. This imposes some TTI limitations and is only intended to be used as work around for unexpected RTL bugs.
    #[inline(always)]
    pub fn tti_serial_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_tti_serial_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// TTI Control
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TTI_CTRL2(u32);
impl TTI_CTRL2 {
    /// Minium number of clock cycles between TUPE accessing TTI Table. TUPE access to TTI Table takes precedence over both CSR accesses and normal TTI processing. TUPE will at least consume 4 clock cycles per processed TTI, so setting TTI_TUPE_RSV<4 results in same behaviour as setting it to 4.
    #[inline(always)]
    pub fn tti_tupe_rsv(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_tti_tupe_rsv(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// TTI Injection Counter
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TTI_INJ_CNT(u32);
impl TTI_INJ_CNT {
    /// Number of TTI frame injections. Enabled per TTI using AFI:TTI_TBL:TTI_MISC_CFG.INJ_CNT_ENA. Frames injected for removal are also counted.
    #[inline(always)]
    pub fn tti_inj_cnt(&self) -> u32 {
        self.0 & 0x3fffffff
    }
    #[inline(always)]
    pub fn set_tti_inj_cnt(&mut self, value: u32) {
        debug_assert!(value <= 0x3fffffff);
        self.0 &= !0x3fffffff;
        self.0 |= value;
    }
}
/// Outstanding TTI injections per port
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TTI_PORT_FRM_OUT(u32);
impl TTI_PORT_FRM_OUT {
    /// See AFI:PORT_TBL:PORT_CFG.FRM_OUT_MAX.
    #[inline(always)]
    pub fn tti_frm_out_max(&self) -> u32 {
        self.0 & 0x3ff
    }
    #[inline(always)]
    pub fn set_tti_frm_out_max(&mut self, value: u32) {
        debug_assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// State of random algorithm used for TTI jitter calculation
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TTI_RAND_STATE(u32);
impl TTI_RAND_STATE {
    /// State of random algorithm used for TTI jitter calculation. Updated by AFI for each jitter calculation. Should be initialized by SW to a random, non-zero value.
    #[inline(always)]
    pub fn tti_rand_state(&self) -> u32 {
        self.0 & 0x3ffff
    }
    #[inline(always)]
    pub fn set_tti_rand_state(&mut self, value: u32) {
        debug_assert!(value <= 0x3ffff);
        self.0 &= !0x3ffff;
        self.0 |= value;
    }
}
