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
/// Count the number of LBR and TST CRC errors received.
///
/// The VOE can verify the CRC-32 of Test TLVs in incoming LBR and TST PDUs. This functionality is enabled using one of the following bit fields: * VOP:VOE_CONF:OAM_HW_CTRL.LBR_TLV_CRC_VERIFY_ENA * VOP:VOE_CONF:OAM_HW_CTRL.TST_TLV_CRC_VERIFY_ENA When enabled the VOE examines the TLV field of valid LBR and TST PDUs in the Rx direction. If the first TLV following the LBR or TST PDU is a Test TLV including a CRC-32 across the Data Pattern, the VOE will calculate the CRC across the Data Pattern and verify the CRC-32. This register will count the number of CRC errors received by the VOE. The CRC counters are indexed as follows: * Service (/Path) VOEs are indexed: 0 - 255 * Port VOEs are indexed: 256 (Port 0) - 266 (Port 10)
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct LBR_CRC_ERR_CNT(u32);
impl LBR_CRC_ERR_CNT {
    /// See Register Description.
    #[inline(always)]
    pub fn lbr_crc_err_cnt(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_lbr_crc_err_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
