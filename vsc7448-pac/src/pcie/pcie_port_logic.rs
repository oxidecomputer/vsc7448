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
/// Ack Frequency and L0-L1 ASPM Control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ACK_F_ASPM_CTRL(u32);
impl ACK_F_ASPM_CTRL {
    pub fn ack_freq(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_ack_freq(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    pub fn ack_n_fts(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_ack_n_fts(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
    pub fn common_clk_n_fts(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_common_clk_n_fts(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xff0000);
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    pub fn enter_aspm(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_enter_aspm(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    pub fn l0_entrance_latency(&self) -> u32 {
        (self.0 & 0x7000000) >> 24
    }
    pub fn set_l0_entrance_latency(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x7000000);
        self.0 &= !0x7000000;
        self.0 |= value;
    }
    pub fn l1_entrance_latency(&self) -> u32 {
        (self.0 & 0x38000000) >> 27
    }
    pub fn set_l1_entrance_latency(&mut self, value: u32) {
        let value = value << 27;
        assert!(value <= 0x38000000);
        self.0 &= !0x38000000;
        self.0 |= value;
    }
}
/// Ack Latency Timer and Replay Timer
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ACK_LATENCY_TIMER(u32);
impl ACK_LATENCY_TIMER {
    pub fn replay_time_limit(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_replay_time_limit(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    pub fn round_trip_latency_time_limit(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_round_trip_latency_time_limit(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Amba Error Response Default
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct AMBA_ERROR_RESPONSE_DEFAULT(u32);
impl AMBA_ERROR_RESPONSE_DEFAULT {
    pub fn amba_error_response_default(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_amba_error_response_default(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// Outbound Decomponsed Subrequests Control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct AMBA_MUL_OB_DECOMP_NP_SUB_REQ_CTRL(u32);
impl AMBA_MUL_OB_DECOMP_NP_SUB_REQ_CTRL {
    pub fn ib_ob_rd_split_burst_en(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_ib_ob_rd_split_burst_en(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
/// Address translation upper base address
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ATU_BASE_ADDR_HIGH(u32);
impl ATU_BASE_ADDR_HIGH {
    /// Outbound: Not used. Inbound: Bits 63:32 of the starting address of the address region to be translated.
    pub fn atu_base_addr_high(&self) -> u32 {
        self.0
    }
    pub fn set_atu_base_addr_high(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Address translation lower base address
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ATU_BASE_ADDR_LOW(u32);
impl ATU_BASE_ADDR_LOW {
    /// Bits 31:16 of the starting address of the address region to be translated.
    pub fn atu_base_addr_low(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_atu_base_addr_low(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
/// Address translation configuration register 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ATU_CFG1(u32);
impl ATU_CFG1 {
    pub fn atu_at(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    pub fn set_atu_at(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x30000);
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// When the address of an outbound TLP is matched to this region, then the ATTR field of the TLP is changed to the value in this register.
    pub fn atu_attr(&self) -> u32 {
        (self.0 & 0x600) >> 9
    }
    pub fn set_atu_attr(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x600);
        self.0 &= !0x600;
        self.0 |= value;
    }
    /// Must be 0.
    pub fn atu_fn(&self) -> u32 {
        (self.0 & 0x1f00000) >> 20
    }
    pub fn set_atu_fn(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x1f00000);
        self.0 &= !0x1f00000;
        self.0 |= value;
    }
    /// When the address of an outbound TLP is matched to this region, then the TC field of the TLP is changed to the value in this register.
    pub fn atu_tc(&self) -> u32 {
        (self.0 & 0xe0) >> 5
    }
    pub fn set_atu_tc(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0xe0);
        self.0 &= !0xe0;
        self.0 |= value;
    }
    /// When the address of an outbound TLP is matched to this region, then the TD field of the TLP is changed to the value in this register.
    pub fn atu_td(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_atu_td(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// When the address of an outbound TLP is matched to this region, then the TYPE field of the TLP is changed to the value in this register.
    ///
    /// 0: MRd/MWr 1: MRdLk 2: IORd/IOWr 4: CfgRd0/CfgWr0 5: CfgRd1/CfgWr1 16-23: Msg/MsgD
    pub fn atu_type(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_atu_type(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}
/// Address translation configuration register 2
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ATU_CFG2(u32);
impl ATU_CFG2 {
    pub fn atu_cfg_shift_ena(&self) -> u32 {
        (self.0 & 0x10000000) >> 28
    }
    pub fn set_atu_cfg_shift_ena(&mut self, value: u32) {
        let value = value << 28;
        assert!(value <= 0x10000000);
        self.0 &= !0x10000000;
        self.0 |= value;
    }
    /// Must be 0.
    pub fn atu_fn_match_ena(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_atu_fn_match_ena(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    pub fn atu_invert_ena(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    pub fn set_atu_invert_ena(&mut self, value: u32) {
        let value = value << 29;
        assert!(value <= 0x20000000);
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    /// When the address of an outbound TLP is matched to this region, and the translated TLP TYPE field is Msg or MsgD; then the Message field of the TLP is changed to the value in this register.
    pub fn atu_msg_code(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_atu_msg_code(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// This bit must be set for address translation to take place.
    pub fn atu_region_ena(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_atu_region_ena(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
}
/// Address translation configuration register 3
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ATU_CFG3(u32);
impl ATU_CFG3 {
    pub fn vf_active(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_vf_active(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    pub fn vf_number(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vf_number(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Address translation limit address
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ATU_LIMIT_ADDR(u32);
impl ATU_LIMIT_ADDR {
    /// Bits 31:16 of the ending address of the address region to be translated.
    pub fn atu_limit_addr(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_atu_limit_addr(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    pub fn reserved_3(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_reserved_3(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Address translation region
///
/// The address translation unit supports 2 outbound regions. The registers PCIE::ATU_CFG1, PCIE::ATU_CFG2, PCIE::ATU_BASE_ADDR_LOW, PCIE::ATU_BASE_ADDR_HIGH, PCIE::ATU_LIMIT_ADDR, PCIE::ATU_TGT_ADDR_LOW, and PCIE::ATU_TGT_ADDR_HIGH all maps to the currently configured region (as configured in this register).
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ATU_REGION(u32);
impl ATU_REGION {
    /// Selects either inbound or outbound regions.
    ///
    /// 0: Outbound. 1: Inbound.
    pub fn atu_direction(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_atu_direction(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Selects region index, set to 0 or 1.
    pub fn atu_idx(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_atu_idx(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Address translation upper target address
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ATU_TGT_ADDR_HIGH(u32);
impl ATU_TGT_ADDR_HIGH {
    /// Bits 63:32 of the new address of the translated region. Set to 0 to force new address into 32bit PCIe memory space.
    pub fn atu_tgt_addr_high(&self) -> u32 {
        self.0
    }
    pub fn set_atu_tgt_addr_high(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Address translation lower target address
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct ATU_TGT_ADDR_LOW(u32);
impl ATU_TGT_ADDR_LOW {
    /// Bits 31:16 of the new address of the translated region.
    pub fn atu_tgt_addr_low(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_atu_tgt_addr_low(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
/// Auxillary Clock Frequency
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct AUX_CLK_FREQ(u32);
impl AUX_CLK_FREQ {
    pub fn aux_clk_freq(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_aux_clk_freq(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// AXI Master Control 0
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct AXI_MASTER_CTRL_REG_0(u32);
impl AXI_MASTER_CTRL_REG_0 {
    pub fn remote_max_bridge_tag(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_remote_max_bridge_tag(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
    pub fn remote_read_req_size(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_remote_read_req_size(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// AXI Master Control 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct AXI_MASTER_CTRL_REG_1(u32);
impl AXI_MASTER_CTRL_REG_1 {
    pub fn resize_master_response(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_resize_master_response(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Debug 0
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct DEBUG_REG_0(u32);
impl DEBUG_REG_0 {
    pub fn deb_reg_0(&self) -> u32 {
        self.0
    }
    pub fn set_deb_reg_0(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Debug 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct DEBUG_REG_1(u32);
impl DEBUG_REG_1 {
    pub fn deb_reg_1(&self) -> u32 {
        self.0
    }
    pub fn set_deb_reg_1(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Filter Mask 2
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FILTER_MASK_REG_2(u32);
impl FILTER_MASK_REG_2 {
    pub fn mask_radm_2(&self) -> u32 {
        self.0
    }
    pub fn set_mask_radm_2(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Link Width and Speed Change Control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct GEN2_CTRL(u32);
impl GEN2_CTRL {
    pub fn gen1_ei_inference(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_gen1_ei_inference(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }
    pub fn num_of_lanes(&self) -> u32 {
        (self.0 & 0x1ff00) >> 8
    }
    pub fn set_num_of_lanes(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x1ff00);
        self.0 &= !0x1ff00;
        self.0 |= value;
    }
}
/// Lane Skew
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct LANE_SKEW(u32);
impl LANE_SKEW {
    pub fn ack_nak_disable(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    pub fn set_ack_nak_disable(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x2000000);
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    pub fn disable_lane_to_lane_deskew(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_disable_lane_to_lane_deskew(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    pub fn flow_ctrl_disable(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_flow_ctrl_disable(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    pub fn insert_lane_skew(&self) -> u32 {
        self.0 & 0xffffff
    }
    pub fn set_insert_lane_skew(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// Misc Control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MISC_CONTROL_1(u32);
impl MISC_CONTROL_1 {
    pub fn dbi_ro_wr_en(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_dbi_ro_wr_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PHY Contol
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PHY_CONTROL(u32);
impl PHY_CONTROL {
    pub fn phy_control(&self) -> u32 {
        self.0
    }
    pub fn set_phy_control(&mut self, value: u32) {
        self.0 = value;
    }
}
/// PHY Status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PHY_STATUS(u32);
impl PHY_STATUS {
    pub fn phy_status(&self) -> u32 {
        self.0
    }
    pub fn set_phy_status(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Port Force Link
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_FORCE(u32);
impl PORT_FORCE {
    pub fn cpl_sent_count(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_cpl_sent_count(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xff000000);
        self.0 &= !0xff000000;
        self.0 |= value;
    }
    pub fn forced_ltssm(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    pub fn set_forced_ltssm(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xf00);
        self.0 &= !0xf00;
        self.0 |= value;
    }
    pub fn link_num(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_link_num(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    pub fn link_state(&self) -> u32 {
        (self.0 & 0x3f0000) >> 16
    }
    pub fn set_link_state(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x3f0000);
        self.0 &= !0x3f0000;
        self.0 |= value;
    }
}
/// Port Link Control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_LINK_CTRL(u32);
impl PORT_LINK_CTRL {
    pub fn beacon_enable(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_beacon_enable(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    pub fn corrupt_lcrc_enable(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    pub fn set_corrupt_lcrc_enable(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x2000000);
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    pub fn dll_link_en(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_dll_link_en(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    pub fn extended_synch(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    pub fn set_extended_synch(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0x4000000);
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    pub fn fast_link_mode(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_fast_link_mode(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    pub fn link_capable(&self) -> u32 {
        (self.0 & 0x3f0000) >> 16
    }
    pub fn set_link_capable(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x3f0000);
        self.0 &= !0x3f0000;
        self.0 |= value;
    }
    pub fn link_disable(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_link_disable(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    pub fn link_rate(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    pub fn set_link_rate(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xf00);
        self.0 &= !0xf00;
        self.0 |= value;
    }
    pub fn loopback_enable(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_loopback_enable(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    pub fn reserved0(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_reserved0(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    pub fn reserved1(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    pub fn set_reserved1(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xf000);
        self.0 &= !0xf000;
        self.0 |= value;
    }
    pub fn reserved2(&self) -> u32 {
        (self.0 & 0xc00000) >> 22
    }
    pub fn set_reserved2(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0xc00000);
        self.0 &= !0xc00000;
        self.0 |= value;
    }
    pub fn reserved3(&self) -> u32 {
        (self.0 & 0xf0000000) >> 28
    }
    pub fn set_reserved3(&mut self, value: u32) {
        let value = value << 28;
        assert!(value <= 0xf0000000);
        self.0 &= !0xf0000000;
        self.0 |= value;
    }
    pub fn reset_assert(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_reset_assert(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    pub fn scramble_disable(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_scramble_disable(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    pub fn transmit_lane_reversale_enable(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    pub fn set_transmit_lane_reversale_enable(&mut self, value: u32) {
        let value = value << 27;
        assert!(value <= 0x8000000);
        self.0 &= !0x8000000;
        self.0 |= value;
    }
    pub fn vendor_specific_dllp_req(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vendor_specific_dllp_req(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Queue Status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct Q_STATUS(u32);
impl Q_STATUS {
    pub fn rx_queue_non_empty(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_rx_queue_non_empty(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    pub fn rx_tlp_fc_credit_non_return(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_rx_tlp_fc_credit_non_return(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    pub fn timer_mod_flow_control(&self) -> u32 {
        (self.0 & 0x1fff0000) >> 16
    }
    pub fn set_timer_mod_flow_control(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x1fff0000);
        self.0 &= !0x1fff0000;
        self.0 |= value;
    }
    pub fn timer_mod_flow_control_en(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_timer_mod_flow_control_en(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    pub fn tx_retry_buffer_ne(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_tx_retry_buffer_ne(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Symbol Timer and Filter Mask 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SYMBOL_TIMER_FILTER_1(u32);
impl SYMBOL_TIMER_FILTER_1 {
    pub fn disable_fc_wd_timer(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_disable_fc_wd_timer(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    pub fn eidle_timer(&self) -> u32 {
        (self.0 & 0x7800) >> 11
    }
    pub fn set_eidle_timer(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x7800);
        self.0 &= !0x7800;
        self.0 |= value;
    }
    pub fn mask_radm_1(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_mask_radm_1(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    pub fn skp_int_val(&self) -> u32 {
        self.0 & 0x7ff
    }
    pub fn set_skp_int_val(&mut self, value: u32) {
        assert!(value <= 0x7ff);
        self.0 &= !0x7ff;
        self.0 |= value;
    }
}
/// Timer Control and Max Function Number
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TIMER_CTRL_MAX_FUNC_NUM(u32);
impl TIMER_CTRL_MAX_FUNC_NUM {
    pub fn max_func_num(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_max_func_num(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    pub fn timer_mod_ack_nak(&self) -> u32 {
        (self.0 & 0xf80000) >> 19
    }
    pub fn set_timer_mod_ack_nak(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0xf80000);
        self.0 &= !0xf80000;
        self.0 |= value;
    }
    pub fn timer_mod_replay_timer(&self) -> u32 {
        (self.0 & 0x7c000) >> 14
    }
    pub fn set_timer_mod_replay_timer(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x7c000);
        self.0 &= !0x7c000;
        self.0 |= value;
    }
    pub fn update_freq_timer(&self) -> u32 {
        (self.0 & 0x1f000000) >> 24
    }
    pub fn set_update_freq_timer(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1f000000);
        self.0 &= !0x1f000000;
        self.0 |= value;
    }
}
/// Transmit Completion FC Credit Status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TX_CPL_FC_CREDIT_STATUS(u32);
impl TX_CPL_FC_CREDIT_STATUS {
    pub fn tx_cpl_data_fc_credit(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_tx_cpl_data_fc_credit(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    pub fn tx_cpl_header_fc_credit(&self) -> u32 {
        (self.0 & 0xff000) >> 12
    }
    pub fn set_tx_cpl_header_fc_credit(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xff000);
        self.0 &= !0xff000;
        self.0 |= value;
    }
}
/// Transmit Non-Posted FC Credit Status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TX_NP_FC_CREDIT_STATUS(u32);
impl TX_NP_FC_CREDIT_STATUS {
    pub fn tx_np_data_fc_credit(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_tx_np_data_fc_credit(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    pub fn tx_np_header_fc_credit(&self) -> u32 {
        (self.0 & 0xff000) >> 12
    }
    pub fn set_tx_np_header_fc_credit(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xff000);
        self.0 &= !0xff000;
        self.0 |= value;
    }
}
/// Transmit Posted FC Credit Status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TX_P_FC_CREDIT_STATUS(u32);
impl TX_P_FC_CREDIT_STATUS {
    pub fn tx_p_data_fc_credit(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_tx_p_data_fc_credit(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    pub fn tx_p_header_fc_credit(&self) -> u32 {
        (self.0 & 0xff000) >> 12
    }
    pub fn set_tx_p_header_fc_credit(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xff000);
        self.0 &= !0xff000;
        self.0 |= value;
    }
}
/// Completion Receive Queue Control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VC0_CPL_RX_Q_CTRL(u32);
impl VC0_CPL_RX_Q_CTRL {
    pub fn reserved8(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_reserved8(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    pub fn reserved9(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_reserved9(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xff000000);
        self.0 &= !0xff000000;
        self.0 |= value;
    }
    pub fn vc0_cpl_data_credit(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_vc0_cpl_data_credit(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    pub fn vc0_cpl_header_credit(&self) -> u32 {
        (self.0 & 0xff000) >> 12
    }
    pub fn set_vc0_cpl_header_credit(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xff000);
        self.0 &= !0xff000;
        self.0 |= value;
    }
    pub fn vc0_cpl_tlp_q_mode(&self) -> u32 {
        (self.0 & 0xe00000) >> 21
    }
    pub fn set_vc0_cpl_tlp_q_mode(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0xe00000);
        self.0 &= !0xe00000;
        self.0 |= value;
    }
}
/// Non-Posted Receive Queue Control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VC0_NP_RX_Q_CTRL(u32);
impl VC0_NP_RX_Q_CTRL {
    pub fn reserved6(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_reserved6(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    pub fn reserved7(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_reserved7(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xff000000);
        self.0 &= !0xff000000;
        self.0 |= value;
    }
    pub fn vc0_np_data_credit(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_vc0_np_data_credit(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    pub fn vc0_np_header_credit(&self) -> u32 {
        (self.0 & 0xff000) >> 12
    }
    pub fn set_vc0_np_header_credit(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xff000);
        self.0 &= !0xff000;
        self.0 |= value;
    }
    pub fn vc0_np_tlp_q_mode(&self) -> u32 {
        (self.0 & 0xe00000) >> 21
    }
    pub fn set_vc0_np_tlp_q_mode(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0xe00000);
        self.0 &= !0xe00000;
        self.0 |= value;
    }
}
/// Posted Receive Queue Control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VC0_P_RX_Q_CTRL(u32);
impl VC0_P_RX_Q_CTRL {
    pub fn reserved4(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_reserved4(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    pub fn reserved5(&self) -> u32 {
        (self.0 & 0x3f000000) >> 24
    }
    pub fn set_reserved5(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x3f000000);
        self.0 &= !0x3f000000;
        self.0 |= value;
    }
    pub fn tlp_type_ordering_vc0(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_tlp_type_ordering_vc0(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    pub fn vc0_p_data_credit(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_vc0_p_data_credit(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
    pub fn vc0_p_header_credit(&self) -> u32 {
        (self.0 & 0xff000) >> 12
    }
    pub fn set_vc0_p_header_credit(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xff000);
        self.0 &= !0xff000;
        self.0 |= value;
    }
    pub fn vc0_p_tlp_q_mode(&self) -> u32 {
        (self.0 & 0xe00000) >> 21
    }
    pub fn set_vc0_p_tlp_q_mode(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0xe00000);
        self.0 &= !0xe00000;
        self.0 |= value;
    }
    pub fn vc_ordering_rx_q(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_vc_ordering_rx_q(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
}
/// Transmit Arbitration 1
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VC_TX_ARBI_REG_1(u32);
impl VC_TX_ARBI_REG_1 {
    pub fn wrr_weight_vc_0(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_wrr_weight_vc_0(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    pub fn wrr_weight_vc_1(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_wrr_weight_vc_1(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
    pub fn wrr_weight_vc_2(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_wrr_weight_vc_2(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xff0000);
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    pub fn wrr_weight_vc_3(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_wrr_weight_vc_3(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xff000000);
        self.0 &= !0xff000000;
        self.0 |= value;
    }
}
/// Transmit Arbitration 2
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VC_TX_ARBI_REG_2(u32);
impl VC_TX_ARBI_REG_2 {
    pub fn wrr_weight_vc_4(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_wrr_weight_vc_4(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    pub fn wrr_weight_vc_5(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_wrr_weight_vc_5(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
    pub fn wrr_weight_vc_6(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_wrr_weight_vc_6(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xff0000);
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    pub fn wrr_weight_vc_7(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_wrr_weight_vc_7(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xff000000);
        self.0 &= !0xff000000;
        self.0 |= value;
    }
}
/// Vendor Specific DLLP
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VENDOR_SPEC_DLLP(u32);
impl VENDOR_SPEC_DLLP {
    pub fn vendor_spec_dllp(&self) -> u32 {
        self.0
    }
    pub fn set_vendor_spec_dllp(&mut self, value: u32) {
        self.0 = value;
    }
}
