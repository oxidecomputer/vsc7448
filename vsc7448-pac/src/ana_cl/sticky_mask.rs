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
/// Sticky counter mask register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CAT_STICKY_MASK(u32);
impl CAT_STICKY_MASK {
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn ag_sticky_mask(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_ag_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn bpdu_sticky_mask(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_bpdu_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn capture_tpid_dis_sticky_mask(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_capture_tpid_dis_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn gxrp_sticky_mask(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_gxrp_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn igmp_sticky_mask(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_igmp_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn ip4_mc_ctrl_sticky_mask(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_ip4_mc_ctrl_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn ip6_hop_by_hop_sticky_mask(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_ip6_hop_by_hop_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn ip6_icmp_hop_by_hop_sticky_mask(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_ip6_icmp_hop_by_hop_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn ip6_mc_ctrl_sticky_mask(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_ip6_mc_ctrl_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn mld_sticky_mask(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_mld_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn vrap_sticky_mask(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_vrap_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn vstax2_ttl_zero_sticky_mask(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_vstax2_ttl_zero_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
}
/// Sticky counter mask register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CLASS_STICKY_MASK(u32);
impl CLASS_STICKY_MASK {
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn dscp_qos_rewr_sticky_mask(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_dscp_qos_rewr_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn dscp_translate_sticky_mask(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_dscp_translate_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn qos_default_sticky_mask(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_qos_default_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn qos_dscp_sticky_mask(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_qos_dscp_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn qos_pcp_dei_sticky_mask(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_qos_pcp_dei_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn qos_stack_tag_sticky_mask(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_qos_stack_tag_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn vid_port_sticky_mask(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_vid_port_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn vid_stack_sticky_mask(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_vid_stack_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn vid_tag_sticky_mask(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_vid_tag_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
}
/// Sticky counter mask register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FILTER_STICKY_MASK(u32);
impl FILTER_STICKY_MASK {
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn bad_macs_sticky_mask(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bad_macs_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn stacking_filter_sticky_mask(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_stacking_filter_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Sticky counter mask register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct IP_HDR_CHK_STICKY_MASK(u32);
impl IP_HDR_CHK_STICKY_MASK {
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn ip4_chksum_sticky_mask(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_ip4_chksum_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn ip4_fragment_sticky_mask(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_ip4_fragment_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn ip4_len_sticky_mask(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_ip4_len_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn ip_options_sticky_mask(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_ip_options_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// Sticky counter mask register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MIP_STICKY_MASK(u32);
impl MIP_STICKY_MASK {
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn mip_ccm_copy_sticky_mask(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_mip_ccm_copy_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn mip_generic_sticky_mask(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_mip_generic_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn mip_lbm_da_chk_fail_sticky_mask(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_mip_lbm_da_chk_fail_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn mip_lbm_redir_sticky_mask(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_mip_lbm_redir_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn mip_ltm_redir_sticky_mask(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_mip_ltm_redir_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn mip_mel_chk_fail_sticky_mask(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_mip_mel_chk_fail_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn mip_raps_sticky_mask(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_mip_raps_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// Sticky counter mask register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VLAN_FILTER_STICKY_MASK(u32);
impl VLAN_FILTER_STICKY_MASK {
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn filter_ctag_sticky_mask(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_filter_ctag_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn filter_prio_ctag_sticky_mask(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_filter_prio_ctag_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enabale event count
    #[inline(always)]
    pub fn filter_prio_stag_sticky_mask(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_filter_prio_stag_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enable event count
    #[inline(always)]
    pub fn filter_required_tag_sticky_mask(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_filter_required_tag_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Mask count of sticky event.
    ///
    /// 0: Disable event count 1: Enabale event count
    #[inline(always)]
    pub fn filter_stag_sticky_mask(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_filter_stag_sticky_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
