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
/// Equal cost UPSID destination configuration
///
/// Used for configuring equal distance to UPSID
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct STACK_LINK_EQUAL_COST_CFG(u32);
impl STACK_LINK_EQUAL_COST_CFG {
    /// Enable equal cost forwarding to UPSID. I.e. both stack ports (A and B) are used for forwarding to UPSID. If STACK_LINK_EQUAL_ENA is set, then the stack forwarding mask is calculated as one of the following two, depending on AC: Even AC: UPSID_PORT_MASK &  STACK_A_MASK Odd AC: UPSID_PORT_MASK & ~STACK_A_MASK Furthermore when STACK_LINK_EQUAL_ENA is set, the TTL value is set to VSTAX2_EQUAL_STACK_LINK_TTL_VAL (instead of REW::VSTAX_PORT_GRP_CFG.VSTAX_TTL). Related parameters: ANA_AC:UPSID:UPSID_CFG.UPSID_PORT_MASK ANA_AC:PS_COMMON:STACK_A_CFG.STACK_A_MASK ANA_AC:PS_COMMON:COMMON_EQUAL_STACK_LINK_TTL_CFG.VSTAX2_EQUAL_STACK_LINK _TTL_VAL REW:COMMON:VSTAX_PORT_GRP_CFG.VSTAX_TTL
    #[inline(always)]
    pub fn stack_link_equal_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_stack_link_equal_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// UPSID destination configuration
///
/// Configures which physical ports to be used for reaching a given UPSID.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct UPSID_CFG(u32);
impl UPSID_CFG {
    /// UPSID port mask.
    #[inline(always)]
    pub fn upsid_port_mask(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_upsid_port_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// UPSID destination configuration
///
/// Refer to UPSID_CFG.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct UPSID_CFG1(u32);
impl UPSID_CFG1 {
    /// Refer to UPSID_CFG.UPSID_PORT_MASK.
    #[inline(always)]
    pub fn upsid_port_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    #[inline(always)]
    pub fn set_upsid_port_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
