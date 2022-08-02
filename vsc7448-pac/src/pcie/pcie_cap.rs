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
/// Word offset 1
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEVICE_CAPABILITIES(u32);
impl DEVICE_CAPABILITIES {
    #[inline(always)]
    pub fn pcie_cap_ep_l0s_accpt_latency(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    #[inline(always)]
    pub fn set_pcie_cap_ep_l0s_accpt_latency(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 6;
        self.0 &= !0x1c0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_ep_l1_accpt_latency(&self) -> u32 {
        (self.0 & 0xe00) >> 9
    }
    #[inline(always)]
    pub fn set_pcie_cap_ep_l1_accpt_latency(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 9;
        self.0 &= !0xe00;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_ext_tag_supp(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_pcie_cap_ext_tag_supp(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_flr_cap(&self) -> u32 {
        (self.0 & 0x10000000) >> 28
    }
    #[inline(always)]
    pub fn set_pcie_cap_flr_cap(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 28;
        self.0 &= !0x10000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_max_payload_size(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_pcie_cap_max_payload_size(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_phantom_func_support(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    #[inline(always)]
    pub fn set_pcie_cap_phantom_func_support(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        let value = value << 3;
        self.0 &= !0x18;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_role_based_err_report(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    #[inline(always)]
    pub fn set_pcie_cap_role_based_err_report(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
}
/// Word offset 9
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEVICE_CAPABILITIES2(u32);
impl DEVICE_CAPABILITIES2 {
    #[inline(always)]
    pub fn pcie_cap_128_cas_cpl_supp(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_pcie_cap_128_cas_cpl_supp(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_32_atomic_cpl_supp(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_pcie_cap_32_atomic_cpl_supp(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_64_atomic_cpl_supp(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_pcie_cap_64_atomic_cpl_supp(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_ari_forward_support(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_pcie_cap_ari_forward_support(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_atomic_routing_supp(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_pcie_cap_atomic_routing_supp(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_cpl_timeout_disable_support(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_pcie_cap_cpl_timeout_disable_support(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_cpl_timeout_range(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_pcie_cap_cpl_timeout_range(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_ltr_supp(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_pcie_cap_ltr_supp(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_no_ro_en_pr2pr_par(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_pcie_cap_no_ro_en_pr2pr_par(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_obff_support(&self) -> u32 {
        (self.0 & 0xc0000) >> 18
    }
    #[inline(always)]
    pub fn set_pcie_cap_obff_support(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        let value = value << 18;
        self.0 &= !0xc0000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_tph_cmplt_support_0(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_pcie_cap_tph_cmplt_support_0(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_tph_cmplt_support_1(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    #[inline(always)]
    pub fn set_pcie_cap_tph_cmplt_support_1(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
}
/// Word offset 10
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEVICE_CONTROL2_DEVICE_STATUS2(u32);
impl DEVICE_CONTROL2_DEVICE_STATUS2 {
    #[inline(always)]
    pub fn pcie_cap_ari_forward_support_cs(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_pcie_cap_ari_forward_support_cs(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_cpl_timeout_disable(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_pcie_cap_cpl_timeout_disable(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
}
/// Word offset 2
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEVICE_CONTROL_DEVICE_STATUS(u32);
impl DEVICE_CONTROL_DEVICE_STATUS {
    #[inline(always)]
    pub fn pcie_cap_aux_power_detected(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_pcie_cap_aux_power_detected(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_aux_power_pm_en(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_pcie_cap_aux_power_pm_en(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_corr_err_detected(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_pcie_cap_corr_err_detected(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_corr_err_report_en(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_pcie_cap_corr_err_report_en(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_en_no_snoop(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_pcie_cap_en_no_snoop(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_en_rel_order(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_pcie_cap_en_rel_order(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_ext_tag_en(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_pcie_cap_ext_tag_en(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_fatal_err_detected(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_pcie_cap_fatal_err_detected(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_fatal_err_report_en(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_pcie_cap_fatal_err_report_en(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_initiate_flr(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    #[inline(always)]
    pub fn set_pcie_cap_initiate_flr(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_max_payload_size_cs(&self) -> u32 {
        (self.0 & 0xe0) >> 5
    }
    #[inline(always)]
    pub fn set_pcie_cap_max_payload_size_cs(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 5;
        self.0 &= !0xe0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_max_read_req_size(&self) -> u32 {
        (self.0 & 0x7000) >> 12
    }
    #[inline(always)]
    pub fn set_pcie_cap_max_read_req_size(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 12;
        self.0 &= !0x7000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_non_fatal_err_detected(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline(always)]
    pub fn set_pcie_cap_non_fatal_err_detected(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_non_fatal_err_report_en(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_pcie_cap_non_fatal_err_report_en(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_phantom_func_en(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_pcie_cap_phantom_func_en(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_trans_pending(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline(always)]
    pub fn set_pcie_cap_trans_pending(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_unsupported_req_detected(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    #[inline(always)]
    pub fn set_pcie_cap_unsupported_req_detected(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_unsupport_req_rep_en(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_pcie_cap_unsupport_req_rep_en(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// Word offset 3
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct LINK_CAPABILITIES(u32);
impl LINK_CAPABILITIES {
    #[inline(always)]
    pub fn pcie_cap_active_state_link_pm_support(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    #[inline(always)]
    pub fn set_pcie_cap_active_state_link_pm_support(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        let value = value << 10;
        self.0 &= !0xc00;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_aspm_opt_compliance(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    #[inline(always)]
    pub fn set_pcie_cap_aspm_opt_compliance(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_clock_power_man(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_pcie_cap_clock_power_man(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_dll_active_rep_cap(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_pcie_cap_dll_active_rep_cap(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_l0s_exit_latency(&self) -> u32 {
        (self.0 & 0x7000) >> 12
    }
    #[inline(always)]
    pub fn set_pcie_cap_l0s_exit_latency(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 12;
        self.0 &= !0x7000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_l1_exit_latency(&self) -> u32 {
        (self.0 & 0x38000) >> 15
    }
    #[inline(always)]
    pub fn set_pcie_cap_l1_exit_latency(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 15;
        self.0 &= !0x38000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_link_bw_not_cap(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline(always)]
    pub fn set_pcie_cap_link_bw_not_cap(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_max_link_speed(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_pcie_cap_max_link_speed(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_max_link_width(&self) -> u32 {
        (self.0 & 0x3f0) >> 4
    }
    #[inline(always)]
    pub fn set_pcie_cap_max_link_width(&mut self, value: u32) {
        debug_assert!(value <= 0x3f);
        let value = value << 4;
        self.0 &= !0x3f0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_port_num(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    #[inline(always)]
    pub fn set_pcie_cap_port_num(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        let value = value << 24;
        self.0 &= !0xff000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_surprise_down_err_rep_cap(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    #[inline(always)]
    pub fn set_pcie_cap_surprise_down_err_rep_cap(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
}
/// Word offset 11
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct LINK_CAPABILITIES2(u32);
impl LINK_CAPABILITIES2 {
    #[inline(always)]
    pub fn pcie_cap_cross_link_support(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_pcie_cap_cross_link_support(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_support_link_speed_vector(&self) -> u32 {
        (self.0 & 0xfe) >> 1
    }
    #[inline(always)]
    pub fn set_pcie_cap_support_link_speed_vector(&mut self, value: u32) {
        debug_assert!(value <= 0x7f);
        let value = value << 1;
        self.0 &= !0xfe;
        self.0 |= value;
    }
}
/// Word offset 12
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct LINK_CONTROL2_LINK_STATUS2(u32);
impl LINK_CONTROL2_LINK_STATUS2 {
    #[inline(always)]
    pub fn pcie_cap_curr_deemphasis(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_pcie_cap_curr_deemphasis(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_eq_cpl(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline(always)]
    pub fn set_pcie_cap_eq_cpl(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_eq_cpl_p1(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_pcie_cap_eq_cpl_p1(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_eq_cpl_p2(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    #[inline(always)]
    pub fn set_pcie_cap_eq_cpl_p2(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_eq_cpl_p3(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_pcie_cap_eq_cpl_p3(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_link_eq_req(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline(always)]
    pub fn set_pcie_cap_link_eq_req(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
}
/// Word offset 4
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct LINK_CONTROL_LINK_STATUS(u32);
impl LINK_CONTROL_LINK_STATUS {
    #[inline(always)]
    pub fn pcie_cap_active_state_link_pm_control(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_pcie_cap_active_state_link_pm_control(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_common_clk_config(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_pcie_cap_common_clk_config(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_dll_active(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    #[inline(always)]
    pub fn set_pcie_cap_dll_active(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 29;
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_en_clk_power_man(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_pcie_cap_en_clk_power_man(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_extended_synch(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_pcie_cap_extended_synch(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_link_auot_bw_status(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    #[inline(always)]
    pub fn set_pcie_cap_link_auot_bw_status(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 31;
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_link_auto_bw_int_en(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_pcie_cap_link_auto_bw_int_en(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_link_bw_man_int_en(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_pcie_cap_link_bw_man_int_en(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_link_bw_man_status(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    #[inline(always)]
    pub fn set_pcie_cap_link_bw_man_status(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 30;
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_link_disable(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_pcie_cap_link_disable(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_link_speed(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    #[inline(always)]
    pub fn set_pcie_cap_link_speed(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        let value = value << 16;
        self.0 &= !0xf0000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_link_training(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    #[inline(always)]
    pub fn set_pcie_cap_link_training(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 27;
        self.0 &= !0x8000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_nego_link_width(&self) -> u32 {
        (self.0 & 0x1f00000) >> 20
    }
    #[inline(always)]
    pub fn set_pcie_cap_nego_link_width(&mut self, value: u32) {
        debug_assert!(value <= 0x1f);
        let value = value << 20;
        self.0 &= !0x1f00000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_rcb(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_pcie_cap_rcb(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_retrain_link(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_pcie_cap_retrain_link(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_slot_clk_config(&self) -> u32 {
        (self.0 & 0x10000000) >> 28
    }
    #[inline(always)]
    pub fn set_pcie_cap_slot_clk_config(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 28;
        self.0 &= !0x10000000;
        self.0 |= value;
    }
}
/// Word offset 0
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCIE_CAP_ID_PCIE_NEXT_CAP_PTR_PCIE_CAP(u32);
impl PCIE_CAP_ID_PCIE_NEXT_CAP_PTR_PCIE_CAP {
    #[inline(always)]
    pub fn pcie_cap(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    #[inline(always)]
    pub fn set_pcie_cap(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        let value = value << 16;
        self.0 &= !0xf0000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_id(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_pcie_cap_id(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_cap_next_ptr(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline(always)]
    pub fn set_pcie_cap_next_ptr(&mut self, value: u32) {
        debug_assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_dev_port_type(&self) -> u32 {
        (self.0 & 0xf00000) >> 20
    }
    #[inline(always)]
    pub fn set_pcie_dev_port_type(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        let value = value << 20;
        self.0 &= !0xf00000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_int_msg_num(&self) -> u32 {
        (self.0 & 0x3e000000) >> 25
    }
    #[inline(always)]
    pub fn set_pcie_int_msg_num(&mut self, value: u32) {
        debug_assert!(value <= 0x1f);
        let value = value << 25;
        self.0 &= !0x3e000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pcie_slot_imp(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    #[inline(always)]
    pub fn set_pcie_slot_imp(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 24;
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn rsvd(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    #[inline(always)]
    pub fn set_rsvd(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 30;
        self.0 &= !0x40000000;
        self.0 |= value;
    }
}
