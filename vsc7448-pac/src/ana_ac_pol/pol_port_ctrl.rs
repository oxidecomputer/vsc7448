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

/// Register `POL_PORT_GAP`
///
/// Port policer gap and pipeline point configuration
#[derive(From, Into)]
pub struct POL_PORT_GAP(u32);
impl POL_PORT_GAP {    ///
    /// Value added to each frame before updating the bucket. Gap value range: -64 to +63 in two's complement format. Setting GAP_VALUE to 20 corresponds to a line-rate measurement, since on the line each frame will be preceded by 12 bytes of IFG and 8 bytes of preamble. Setting GAP_VALUE to 0 corresponds to a data-rate measurement.
    ///
    /// 0x40: -64 0x41: -63 ... 0x7F: -1 0x00: 0 0x01: 1 ... 0x3F: 63
    pub fn gap_value(&self) -> u32 {
        (self.0 & 0x7f) >> 0
    }
    pub fn set_gap_value(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }    ///
    /// Configures the pipeline point per port policer. When injecting or looping at a pipeline point after PORT_PIPELINE_PT will not cause port policing. When extracting at a pipeline point before PORT_PIPELINE_PT will not cause port policing.
    ///
    /// 0: NONE 1: ANA_VRAP 2: ANA_PORT_VOE 3: ANA_CL 4: ANA_CLM 5: ANA_IPT_PROT 6: ANA_OU_MIP 7: ANA_OU_SW 8: ANA_OU_PROT 9: ANA_OU_VOE 10: ANA_MID_PROT 11: ANA_IN_VOE 12: ANA_IN_PROT 13: ANA_IN_SW 14: ANA_IN_MIP 15: ANA_VLAN
    pub fn port_pipeline_pt(&self) -> u32 {
        (self.0 & 0xf80) >> 7
    }
    pub fn set_port_pipeline_pt(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0xf80);
        self.0 &= !0xf80;
        self.0 |= value;
    }
}

/// Register `POL_PORT_RATE_CFG`
///
/// Port policer rate configuration
///
/// The registers are index by 4 x port number + port policer index.
#[derive(From, Into)]
pub struct POL_PORT_RATE_CFG(u32);
impl POL_PORT_RATE_CFG {    ///
    /// Port policer leaky bucket rate. Regarding unit, refer to POL_UPD_INT. Related parameters: ANA_AC_POL:POL_ALL_CFG:POL_UPD_INT_CFG.POL_UPD_INT ANA_AC_POL:POL_PORT_CTRL:POL_PORT_CFG.FRAME_RATE_ENA
    ///
    /// When POL_PORT_CFG.FRAME_RATE_ENA is disabled, policing is performed in bits per second (bps). 0: Open until burst capacity is used, then closed. 1: Rate = 1 x <unit> bps n: Rate = n x <unit> bps When POL_PORT_CFG.FRAME_RATE_ENA is enabled, policing is performed in frames per second (fps). 0: Open until burst capacity is used, then closed. 1: Rate = <unit> fps n: Rate = n x <unit> fps
    pub fn port_rate(&self) -> u32 {
        (self.0 & 0x7ffff) >> 0
    }
    pub fn set_port_rate(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7ffff);
        self.0 &= !0x7ffff;
        self.0 |= value;
    }
}
