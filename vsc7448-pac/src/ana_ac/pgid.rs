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

/// Register `MBR_CNT_CFG`
///
/// GLAG member count configuration
#[derive(From, Into)]
pub struct MBR_CNT_CFG(u32);
impl MBR_CNT_CFG {    ///
    /// GLAG member count. This is used to select the GLAG port mask within the GLAG part of ANA_AC:PGID. Using GLAG_MBR_CNT a GLAG_MBR_IDX is calculated as follows: GLAG_MBR_IDX = frame.ac % GLAG_MBR_CNT The frame's GLAGID and GLAG_MBR_IDX are then used for lookup in ANA_AC:PGID.
    ///
    /// 0: One member 1: Two members ... 7: Eight members
    pub fn glag_mbr_cnt(&self) -> u32 {
        (self.0 & 0x70000) >> 16
    }
    pub fn set_glag_mbr_cnt(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x70000);
        self.0 &= !0x70000;
        self.0 |= value;
    }
}

/// Register `PGID_CFG`
///
/// PGID port mask / destination configuration
#[derive(From, Into)]
pub struct PGID_CFG(u32);
impl PGID_CFG {    ///
    /// PGID port mask or stack forwarding information, depending on STACK_TYPE_ENA. Related parameters: ANA_AC:PGID:PGID_MISC_CFG.STACK_TYPE_ENA
    ///
    /// PGID_MISC_CFG.STACK_TYPE_ENA=0: Destination port mask. PGID_MISC_CFG.STACK_TYPE_ENA=1: Bit 4:0 VStaX destination UPSPN. Bit 5 VStaX destination port type. 0=Normal UPSPN, 1=Internal port (advanced use). Bit 10:6 VStaX destination UPSID Bit 13:11 VStaX forwarding mode. Only used if PGID origins from multicast index in MAC table. In all other cases bit 13:11 must be 0. Encoding: '001': fwd_logical. Forward to logical front port at specific UPS, using (UPSID, UPSPN). '010': fwd_physical. Forward to physical front port at specific UPS, using (UPSID, UPSPN). '101': fwd_gcpu_ups. Forward to GCPU of specific UPS (identified by UPSID). Other: Reserved. If bit 10:6 is a remote UPSID, then 20:16 must be set to 0. If bit 10:6 is the a UPSID, then 20:16 must be set to same value as bit 4:0.
    pub fn port_mask(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_port_mask(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `PGID_CFG1`
///
/// PGID port mask / destination configuration
#[derive(From, Into)]
pub struct PGID_CFG1(u32);
impl PGID_CFG1 {    ///
    /// Refer to PGID_CFG.PORT_MASK description.
    pub fn port_mask1(&self) -> u32 {
        (self.0 & 0x1fffff) >> 0
    }
    pub fn set_port_mask1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
