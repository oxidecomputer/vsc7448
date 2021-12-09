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
/// Default master
///
/// The default master is selected by the bus when no master has requested ownership. The default master is able to start bus accesses slightly faster than other masters.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct DFT_MST(u32);
impl DFT_MST {
    /// Use this field to configure default master.
    ///
    /// 0: No default master 1: Master 1 2: Master 2 3: Master 3
    pub fn dft_mst(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_dft_mst(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// Early burst termination
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct EBT(u32);
impl EBT {
    /// Set when an Early Burst Termination takes place. The register is cleared when read.
    pub fn ebt(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ebt(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Early burst termination count
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct EBT_COUNT(u32);
impl EBT_COUNT {
    /// Maximum number of cycles a transfer can take before being subject to an early burst termination.
    pub fn ebt_count(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_ebt_count(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// Early burst termination enable
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct EBT_EN(u32);
impl EBT_EN {
    /// Use this field to enable early burst termination.
    ///
    /// 0: Disabled 1: Enabled
    pub fn ebt_en(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ebt_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Master 1 arbitration priority
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PL_CPU(u32);
impl PL_CPU {
    /// Arbitration priority for the master. When multiple masters request the bus at the same time, the one with the highest priority is ganted bus access.
    ///
    /// Values 0x1 through 0xF, higher values are prioritized over lower values.
    pub fn pl1(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_pl1(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// Master 3 arbitration priority
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PL_CSR(u32);
impl PL_CSR {
    /// See SBA::PL1 for description.
    pub fn pl3(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_pl3(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// Master 2 arbitration priority
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PL_PCIE(u32);
impl PL_PCIE {
    /// See SBA::PL1 for description.
    pub fn pl2(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_pl2(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// Reserved
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct RESERVED1(u32);
impl RESERVED1 {
    pub fn reserved1(&self) -> u32 {
        self.0
    }
    pub fn set_reserved1(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Master 1 clock tokens
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct WT_CPU(u32);
impl WT_CPU {
    /// Number of tokens to grant the master at the start of each refresh period for weighted-token arbitration scheme. If configured with a value of zero, the master is considered to have infinite tokens.
    pub fn wt_cl1(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_wt_cl1(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Master 3 clock tokens
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct WT_CSR(u32);
impl WT_CSR {
    /// See SBA::WT_CL1 for description.
    pub fn wt_cl3(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_wt_cl3(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Weighted-token arbitration scheme enable
///
/// When weighted token arbitration is enabled, each master on the shared bus is granted a configurable number of tokens at the start of each refresh period. The length of each refresh period is configurable. In each clock-cycle that a master uses the bus, the token counter for that master decreases. Once all tokens are spent, the master is forced to a low priority. A master with tokens remaining, always takes priority over masters with no tokens remaining.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct WT_EN(u32);
impl WT_EN {
    /// Set this field to enable weighted-token arbitration scheme.
    pub fn wt_en(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_wt_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Master 2 clock tokens
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct WT_PCIE(u32);
impl WT_PCIE {
    /// See SBA::WT_CL1 for description.
    pub fn wt_cl2(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_wt_cl2(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Clock tokens refresh period
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct WT_TCL(u32);
impl WT_TCL {
    /// Refresh period length for the weighted-token arbitration scheme.
    pub fn wt_tcl(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_wt_tcl(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
