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
/// MSTP Forwarding Control
///
/// Configuration of forwarding state per MSTP
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MSTP_FWD_CFG(u32);
impl MSTP_FWD_CFG {
    /// Enable/disable forwarding per port. Ports in MSTP Forwarding state must be enabled in this port mask. If a port is disabled in this mask, frames received on the port are not forwarded, and frames are not forwarded to the port. Related parameters: ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.MSTP_DISCARD_STICKY ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.MSTP_FWD_ALLOWED_STICKY
    ///
    /// '0': forwarding is disabled from/to respective port '1': forwarding is enabled from/to respective port
    #[inline(always)]
    pub fn mstp_fwd_mask(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_mstp_fwd_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// MSTP Forwarding Control
///
/// Configuration of forwarding state per MSTP
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MSTP_FWD_CFG1(u32);
impl MSTP_FWD_CFG1 {
    /// Refer to MSTP_FWD_CFG.MSTP_FWD_MASK description.
    #[inline(always)]
    pub fn mstp_fwd_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    #[inline(always)]
    pub fn set_mstp_fwd_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// MSTP Learning Control
///
/// Configuration of learning state per MSTP.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MSTP_LRN_CFG(u32);
impl MSTP_LRN_CFG {
    /// Enable/disable learning per port. If a port is disabled in the mask, L2 learning of the (FID, SMAC) pair is not done. Ports in MSTP Learning and Forwarding state must be enabled in this mask. Related parameters: ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.MSTP_LRN_DENY_STICKY ANA_L3:VLAN_ARP_L3MC_STICKY:VLAN_STICKY.MSTP_LRN_ALLOWED_STICKY
    ///
    /// '0': Learning is disabled for frames from respective port '1': Learning is enabled for frames from respective port
    #[inline(always)]
    pub fn mstp_lrn_mask(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_mstp_lrn_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// MSTP Learning Control
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MSTP_LRN_CFG1(u32);
impl MSTP_LRN_CFG1 {
    /// Refer to MSTP_LRN_CFG.MSTP_LRN_MASK description.
    #[inline(always)]
    pub fn mstp_lrn_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    #[inline(always)]
    pub fn set_mstp_lrn_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
