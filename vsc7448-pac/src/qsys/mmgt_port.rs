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
/// Ingress queue status
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MMGT_IQ_STAT(u32);
impl MMGT_IQ_STAT {
    /// Number of frame copies pending in the ingress queue
    #[inline(always)]
    pub fn mmgt_iq_size(&self) -> u32 {
        self.0 & 0xfffff
    }
    #[inline(always)]
    pub fn set_mmgt_iq_size(&mut self, value: u32) {
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }
}
/// Total consumption per ingress port
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MMGT_PORT_USE(u32);
impl MMGT_PORT_USE {
    /// Total consumption per port in the memory manager. Unit is one cell (176 bytes).
    #[inline(always)]
    pub fn mmgt_port_use(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_mmgt_port_use(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Port to see values for
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MMGT_PORT_VIEW(u32);
impl MMGT_PORT_VIEW {
    /// Select port to see consumption values for
    #[inline(always)]
    pub fn mmgt_port_view(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_mmgt_port_view(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MMGT_TAILDROP_CNT(u32);
impl MMGT_TAILDROP_CNT {
    /// Returns the number of buffer drops due to ATOP reached, or lack of free memory. Values returned for the port mapped in MMGT_PORT_VIEW. Counter wraps when maximum is reached reached.
    #[inline(always)]
    pub fn mmgt_taildrop_cnt(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_mmgt_taildrop_cnt(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
