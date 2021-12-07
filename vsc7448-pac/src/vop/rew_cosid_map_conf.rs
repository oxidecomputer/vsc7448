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

/// Register `COSID_MAP_CFG_ANA`
///
/// COSID / Color control signals
///
/// The bit fields in this register determines the source of the COSID mapping / COLOR of frames not processed by the VOE.
#[derive(From, Into)]
pub struct COSID_MAP_CFG_ANA(u32);
impl COSID_MAP_CFG_ANA {    ///
    /// Determines if the VOE LM counters counts all frames or only GREEN frames.
    ///
    /// '0': do not include yellow frames in the LM count. '1': include yellow frames in the LM count.
    pub fn cnt_yellow_ana(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_cnt_yellow_ana(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Determines which internal signal carries color for the current VOE. This configuration is only used in ANA, the corresponding configuration in the REW is not used.
    ///
    /// "00": ifh.dp_color "01": ifh.cl_dei "10": ANA_CL:MAP_TBL:MAP_ENTRY.PATH_COLOR_VAL (Output from mapping table. Do not use for Up-MEP) "11": reserved for future use (do not use)
    pub fn color_src_sel_ana(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_color_src_sel_ana(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x6);
        self.0 &= !0x6;
        self.0 |= value;
    }    ///
    /// Selects the source of the COSID mapping.
    ///
    /// "00": ifh.cosid "01": ifh.tc "10": ifh_iprio "11": ANA_CL:MAP_TBL:MAP_ENTRY.PATH_COSID_VAL (Output from mapping table. Do not use for Up-MEP)
    pub fn cosid_src_sel_ana(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    pub fn set_cosid_src_sel_ana(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x18);
        self.0 &= !0x18;
        self.0 |= value;
    }
}

/// Register `COSID_MAP_TABLE_REW`
///
/// COSID mapping table
///
/// COSID mapping table used for mapping the selected COSID values. A single mapping table is available for each of the Service/Path VOEs.
#[derive(From, Into)]
pub struct COSID_MAP_TABLE_REW(u32);
impl COSID_MAP_TABLE_REW {    ///
    /// The table is used to map the choosen COSID in the REW. bit(2:0) will be used to map COSID = 0 bit(5:3) will be used to map COSID = 1 ... bit(23:21) will be used to map COSID = 7 When mapping a COSID, the following procedure is followed: 1) Use COSID_SRC_SEL_REW to select the source of the COSID mapping. I.e. if COSID_SRC_SEL_REW = 1 (TC) the input to the mapping table is set to the IFH.TC. 2) Map the selected value. If IFH.TC = 3, the mapped COSID will be set to COSID_MAP_TABLE_REW[11:9]
    pub fn cosid_map_table_rew(&self) -> u32 {
        (self.0 & 0xffffff) >> 0
    }
    pub fn set_cosid_map_table_rew(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
