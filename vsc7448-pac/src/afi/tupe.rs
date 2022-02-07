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
/// Address range for TUPE to process
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TUPE_ADDR(u32);
impl TUPE_ADDR {
    /// Last address in TTI Table for TUPE to process. Must be >= TUPE_START_ADDR.
    #[inline(always)]
    pub fn tupe_end_addr(&self) -> u32 {
        (self.0 & 0xfff0000) >> 16
    }
    #[inline(always)]
    pub fn set_tupe_end_addr(&mut self, value: u32) {
        assert!(value <= 0xfff);
        let value = value << 16;
        self.0 &= !0xfff0000;
        self.0 |= value;
    }
    /// First address in TTI Table for TUPE to process. Must be <= TUPE_START_ADDR.
    #[inline(always)]
    pub fn tupe_start_addr(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_tupe_start_addr(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// TUPE Command Parameters
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TUPE_CMD1(u32);
impl TUPE_CMD1 {
    /// New value for PORT_NUM for any TTIs matching TUPE criterias. Must be enabled by AFI:TUPE:TUPE_MISC.CMD_PORT_NUM_ENA.
    #[inline(always)]
    pub fn cmd_port_num_val(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_cmd_port_num_val(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// New value for QU_NUM for any TTIs matching TUPE criterias. Must be enabled by AFI:TUPE:TUPE_MISC.CMD_QU_NUM_ENA.
    #[inline(always)]
    pub fn cmd_qu_num_val(&self) -> u32 {
        (self.0 & 0xffff00) >> 8
    }
    #[inline(always)]
    pub fn set_cmd_qu_num_val(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 8;
        self.0 &= !0xffff00;
        self.0 |= value;
    }
}
/// TUPE Criterias
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TUPE_CRIT1(u32);
impl TUPE_CRIT1 {
    /// If enabled by AFI:TUPE:TUPE_MISC.CRIT_PORT_NUM_ENA, then PORT_NUM in TTIs must match this value in order to be processed by TUPE.
    #[inline(always)]
    pub fn crit_port_num_val(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_crit_port_num_val(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// If enabled by AFI:TUPE:TUPE_MISC.CRIT_QU_NUM_ENA, then QU_NUM in TTIs must match this value in order to be processed by TUPE.
    #[inline(always)]
    pub fn crit_qu_num_val(&self) -> u32 {
        (self.0 & 0xffff00) >> 8
    }
    #[inline(always)]
    pub fn set_crit_qu_num_val(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 8;
        self.0 &= !0xffff00;
        self.0 |= value;
    }
}
/// TUPE Criterias
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TUPE_CRIT2(u32);
impl TUPE_CRIT2 {
    /// Refer to AFI:TUPE:TUPE_CRIT3.CRIT_TUPE_CTRL_VAL.
    #[inline(always)]
    pub fn crit_tupe_ctrl_mask(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_crit_tupe_ctrl_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// TUPE Criterias
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TUPE_CRIT3(u32);
impl TUPE_CRIT3 {
    /// For one or more of the configured CRIT_TUPE_CTRL_VALs, TUPE_CTRL in TTIs must match the following criterias in order to be processed by TUPE: (TTI.TUPE_CTRL & CRIT_TUPE_CTRL_MASK) == CRIT_TUPE_CTRL_VAL[i]
    #[inline(always)]
    pub fn crit_tupe_ctrl_val(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_crit_tupe_ctrl_val(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Miscellaneous TUPE parameters
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TUPE_MISC(u32);
impl TUPE_MISC {
    /// Enable use of CMD_PORT_NUM_VAL. For further information refer to AFI:TUPE:TUPE_CMD1.CMD_PORT_NUM_VAL
    #[inline(always)]
    pub fn cmd_port_num_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_cmd_port_num_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Enable use of CMD_QU_NUM_VAL. For further information refer to AFI:TUPE:TUPE_CMD1.CMD_QU_NUM_VAL
    #[inline(always)]
    pub fn cmd_qu_num_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_cmd_qu_num_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Enable use of CMD_TIMER_ENA_VAL. For further information refer to AFI:TUPE:TUPE_MISC.CMD_TIMER_ENA_VAL
    #[inline(always)]
    pub fn cmd_timer_ena_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_cmd_timer_ena_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// New value for TIMER_ENA for any TTIs matching TUPE criterias. Must be enabled by AFI:TUPE:TUPE_MISC.CMD_TIMER_ENA_ENA.
    #[inline(always)]
    pub fn cmd_timer_ena_val(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_cmd_timer_ena_val(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Enable use of CRIT_PORT_NUM_VAL. For further information refer to AFI:TUPE:TUPE_CRIT1.CRIT_PORT_NUM_VAL
    #[inline(always)]
    pub fn crit_port_num_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_crit_port_num_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enable use of CRIT_QU_NUM_ENA. For further information refer to AFI:TUPE:TUPE_CRIT1.CRIT_QU_NUM_VAL
    #[inline(always)]
    pub fn crit_qu_num_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_crit_qu_num_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Start TUPE. Write 1 to start TUPE. Set to 0 by TUPE when done. Before running TUPE, AFI::MISC_CTRL.AFI_ENA must be set to 1. Note: While TUPE is running (i.e. TUPE_START=1) CPU must not write to TTI Table.
    #[inline(always)]
    pub fn tupe_start(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_tupe_start(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
