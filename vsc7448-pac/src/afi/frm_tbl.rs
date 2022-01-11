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
/// Frame Table entry configuration
///
/// Note: Write operations to entries in the frame table, which are in the process of being removed (FRM_RM=1, see FRM_ENTRY_PART0) are not allowed.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FRM_ENTRY_PART0(u32);
impl FRM_ENTRY_PART0 {
    /// Configuration of frame or delay entry in Frame Table. Delay entries are only used for DTI. Delay entry fields: DELAY: Delay between injection of start of frames. Unit: One system clock cycle. Frame entry fields: INJ_CNT: Injection count. Number times to inject frame. Frame is ignored if INJ_CNT=0 or FRM_RM=1. Only applicable for DTI. FRM_RM: When set, next frame injection causes frame to be removed from buffer memory. This injection will not be transmitted on the destination port. Once removed, HW sets FRM_GONE=1. FRM_GONE: Set by AFI when frame has been removed from buffer memory. FRM_INFO: Frame information, ref. AFI:MISC:NEW_FRM_INFO.FRM_INFO.
    ///
    /// Delay entry type: Bit 0-29: DELAY Frame entry type: Bit 0-7: INJ_CNT Bit 8-10: Reserved, must be set to 0 Bit 11: FRM_RM Bit 12: FRM_GONE Bit 13-29: FRM_INFO
    pub fn part0(&self) -> u32 {
        self.0 & 0x3fffffff
    }
    pub fn set_part0(&mut self, value: u32) {
        assert!(value <= 0x3fffffff);
        self.0 &= !0x3fffffff;
        self.0 |= value;
    }
}
/// Entry type and pointer to next Frame Table entry.
///
/// Note: Write operations to entries in the frame table, which are in the process of being removed (FRM_RM=1, see FRM_ENTRY_PART0) are not allowed.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FRM_NEXT_AND_TYPE(u32);
impl FRM_NEXT_AND_TYPE {
    /// Entry Type. Delay entries are only applicable to DTI. The Entry Type controls the use of FRM_ENTRY_PART0.PART0.
    ///
    /// 0: Frame 1: Delay
    pub fn entry_type(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_entry_type(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Pointer to next Frame Table entry. Only applicable for frames used for DTI.
    pub fn next_ptr(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_next_ptr(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
