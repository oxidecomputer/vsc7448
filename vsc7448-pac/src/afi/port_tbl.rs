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

/// Register `PORT_FRM_OUT`
///
/// Number of outstanding injections per port
#[derive(From, Into)]
pub struct PORT_FRM_OUT(u32);
impl PORT_FRM_OUT {
    /// Current number of injections (TTI or DTI) outstanding per port. This parameter should not be written to. If the parameter is written to and a TTI injection occurs concurrently, then the written value may get overwritten by the AFI block.
    pub fn frm_out_cnt(&self) -> u32 {
        (self.0 & 0x7ff0000) >> 16
    }
    pub fn set_frm_out_cnt(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x7ff0000);
        self.0 &= !0x7ff0000;
        self.0 |= value;
    }
}

/// Register `TTI_PORT_FRM_OUT`
///
/// Outstanding TTI injections per port
#[derive(From, Into)]
pub struct TTI_PORT_FRM_OUT(u32);
impl TTI_PORT_FRM_OUT {
    /// See AFI:PORT_TBL:PORT_CFG.FRM_OUT_MAX.
    pub fn tti_frm_out_max(&self) -> u32 {
        (self.0 & 0x3ff) >> 0
    }
    pub fn set_tti_frm_out_max(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
