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

/// Register `QLIMIT_CONG_CNT_STAT`
///
/// Current amount of congested scheduling elements in the share
#[derive(From, Into)]
pub struct QLIMIT_CONG_CNT_STAT(u32);
impl QLIMIT_CONG_CNT_STAT {    ///
    /// Return the current number of active queues in the share.
    pub fn qlimit_act_cnt(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_qlimit_act_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }    pub fn qlimit_cong_cnt(&self) -> u32 {
        (self.0 & 0x7ff8000) >> 15
    }
    pub fn set_qlimit_cong_cnt(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x7ff8000);
        self.0 &= !0x7ff8000;
        self.0 |= value;
    }
}

/// Register `QLIMIT_DIS_CFG`
///
/// Queue Limitation Configuration
///
/// Disabling queue limitation per port.
#[derive(From, Into)]
pub struct QLIMIT_DIS_CFG(u32);
impl QLIMIT_DIS_CFG {    ///
    /// Disable queue limitation for the port. If either queue limitation is disabled for the source port, or for the destination port, the queue limitation algorithm is bypassed.
    pub fn qlimit_egr_dis(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_qlimit_egr_dis(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Disable queue limitation for the port. If either queue limitation is disabled for the source port, or for the destination port, the queue limitation algorithm is bypassed.
    pub fn qlimit_igr_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_qlimit_igr_dis(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}

/// Register `QLIMIT_QUE_ACT_CFG`
///
/// Size of an active queue
#[derive(From, Into)]
pub struct QLIMIT_QUE_ACT_CFG(u32);
impl QLIMIT_QUE_ACT_CFG {    pub fn qlimit_que_act(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_qlimit_que_act(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}

/// Register `QLIMIT_QUE_CONG_CFG`
///
/// Size of a congested queue
#[derive(From, Into)]
pub struct QLIMIT_QUE_CONG_CFG(u32);
impl QLIMIT_QUE_CONG_CFG {    pub fn qlimit_que_cong(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_qlimit_que_cong(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}

/// Register `QLIMIT_SE_CONG_CFG`
///
/// Size of congested SE
#[derive(From, Into)]
pub struct QLIMIT_SE_CONG_CFG(u32);
impl QLIMIT_SE_CONG_CFG {    ///
    /// An SE is regarded congested when its total queue size exceeds this.
    pub fn qlimit_se_cong(&self) -> u32 {
        (self.0 & 0xfffff) >> 0
    }
    pub fn set_qlimit_se_cong(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }
}

/// Register `QLIMIT_SHR_ATOP_CFG`
///
/// Maximum congested size of shared area
#[derive(From, Into)]
pub struct QLIMIT_SHR_ATOP_CFG(u32);
impl QLIMIT_SHR_ATOP_CFG {    ///
    /// When filling exceeds this level, all active queues start tail dropping.
    pub fn qlimit_shr_atop(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_qlimit_shr_atop(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}

/// Register `QLIMIT_SHR_CTOP_CFG`
///
/// Maximum congested size of shared area
#[derive(From, Into)]
pub struct QLIMIT_SHR_CTOP_CFG(u32);
impl QLIMIT_SHR_CTOP_CFG {    ///
    /// When filling exceeds this level, all congested queues start tail dropping.
    pub fn qlimit_shr_ctop(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_qlimit_shr_ctop(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}

/// Register `QLIMIT_SHR_FILL_STAT`
///
/// Current use of the shared area
#[derive(From, Into)]
pub struct QLIMIT_SHR_FILL_STAT(u32);
impl QLIMIT_SHR_FILL_STAT {    pub fn qlimit_shr_fill(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_qlimit_shr_fill(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}

/// Register `QLIMIT_SHR_QDIV_CFG`
///
/// Maximum area to distribute between large SE users
#[derive(From, Into)]
pub struct QLIMIT_SHR_QDIV_CFG(u32);
impl QLIMIT_SHR_QDIV_CFG {    ///
    /// This amount can be shared between large SEs
    pub fn qlimit_shr_qdiv(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_qlimit_shr_qdiv(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}

/// Register `QLIMIT_SHR_QLIM_CFG`
///
/// Maximum area use before queue limitation kicks in
#[derive(From, Into)]
pub struct QLIMIT_SHR_QLIM_CFG(u32);
impl QLIMIT_SHR_QLIM_CFG {    ///
    /// When filling exceeds this level, all queues are limited in size depending on number of congested queues.
    pub fn qlimit_shr_qlim(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_qlimit_shr_qlim(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}

/// Register `QLIMIT_SHR_TOP_CFG`
///
/// Maximum size of shared area
#[derive(From, Into)]
pub struct QLIMIT_SHR_TOP_CFG(u32);
impl QLIMIT_SHR_TOP_CFG {    ///
    /// When total consumption of a shared area exceeds this level, all queues belonging to the area start tail dropping
    pub fn qlimit_shr_top(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_qlimit_shr_top(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}
