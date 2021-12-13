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
/// Port configuration parameters
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_CFG(u32);
impl PORT_CFG {
    /// Controls what action to take if TTI injection cannot be performed due to FRM_OUT_MAX reached or injection stop from QSYS. FC_SKIP_TTI_INJ should be set when disabling a port using FRM_OUT_MAX. See PORT_CFG.FRM_OUT_MAX.
    ///
    /// 0: Postpone injection until injection is again allowed. 1: Skip this injection.
    pub fn fc_skip_tti_inj(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_fc_skip_tti_inj(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Maximum number of injections that can be outstanding at a time per port. DTI injections are held back if FRM_OUT_MAX is exceeded. TTI injections are held back if FRM_OUT_MAX+TTI_FRM_OUT_MAX is exceeded. TTI_FRM_OUT_MAX ensures that TTI injections are still possible when a DTI flow is configured slightly above port speed. If FRM_OUT_MAX is set to 0 to disable injections for port, then it is recommended to first set FC_SKIP_TTI_INJ=1 to avoid a burst of injections when injections are later enabled. Upon setting FRM_OUT_MAX back to non-zero value, then FC_SKIP_TTI_INJ must be set back to its orginal value. Note that FRM_OUT_MAX must also be >0 when performing "removal injections" (for removing frames from buffer memory).
    ///
    /// 0: Injection disabled for port (both TTI and DTI injections, regardless of TTI_FRM_OUT_MAX value) 1: Maximum 1 outstanding injection. 2: Maximum 2 outstanding injections. ... 1022: Maximum 1022 outstanding injections. 1023: Illegal.
    pub fn frm_out_max(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_frm_out_max(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
    /// Only allow frame removal injections, i.e. normal injections are disallowed. If FRM_RM_ONLY is set, then it is recommended to first set FC_SKIP_TTI_INJ=1 to avoid a burst of injections when normal injections are later re-enabled. Upon setting FRM_RM_ONLY back to zero, then FC_SKIP_TTI_INJ must be set back to its orginal value.
    ///
    /// 0: Allow both normal and removal injections. 1: Only allow removal injections.
    pub fn frm_rm_only(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_frm_rm_only(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
}
/// Number of outstanding injections per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
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
