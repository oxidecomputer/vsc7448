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
/// DFT BIST configuration register 0
///
/// BIST configuration register for SD10G65 DFT controlling 'check and wait-stable' mode. The length of a '40-bit clock cycle' is defined by 40 divided by the chosen bit rate per second.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_BIST_CFG0(u32);
impl DFT_BIST_CFG0 {
    /// BIST FSM: threshold to enter FINISHED state (refer to cfg field frame_len_cfg)
    ///
    /// N: time = (N+1) * (frame_len_cfg+1) 40-bit clock cycles
    #[inline(always)]
    pub fn max_bist_frames_cfg(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_max_bist_frames_cfg(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// BIST FSM: threshold to leave DOZE state
    ///
    /// N: time = (N+1) 40-bit clock cycles
    #[inline(always)]
    pub fn wakeup_dly_cfg(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    #[inline(always)]
    pub fn set_wakeup_dly_cfg(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 16;
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
/// DFT BIST configuration register 1
///
/// BIST configuration register for SD10G65 DFT	controlling 'stable' mode. The length of a '40-bit clock cycle' is defined by 40 divided by the chosen bit rate per second.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_BIST_CFG1(u32);
impl DFT_BIST_CFG1 {
    /// BIST FSM: threshold to iterate counter for max_stable_attempts
    ///
    /// N: attempts = (N+1)
    #[inline(always)]
    pub fn max_unstable_cyc_cfg(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    #[inline(always)]
    pub fn set_max_unstable_cyc_cfg(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 16;
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    /// BIST FSM: threshold to enter CHECK state
    ///
    /// N: time = (N+1) 40-bit clock cycles
    #[inline(always)]
    pub fn stable_thres_cfg(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_stable_thres_cfg(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// DFT BIST configuration register 2
///
/// BIST configuration register for SD10G65 DFT controlling frame length in 'check' mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_BIST_CFG2(u32);
impl DFT_BIST_CFG2 {
    /// BIST FSM: threshold to iterate counter for max_bist_frames (refer to cfg field max_bist_frames_cfg)
    ///
    /// N: multiplier = (N+1)
    #[inline(always)]
    pub fn frame_len_cfg(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_frame_len_cfg(&mut self, value: u32) {
        self.0 = value;
    }
}
/// DFT BIST configuration register 3
///
/// BIST configuration register for SD10G65 DFT controlling stable attempts in ' wait-stable' mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_BIST_CFG3(u32);
impl DFT_BIST_CFG3 {
    /// BIST FSM: threshold to enter SYNC_ERR state
    ///
    /// N: attempts = (N+1)
    #[inline(always)]
    pub fn max_stable_attempts_cfg(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_max_stable_attempts_cfg(&mut self, value: u32) {
        self.0 = value;
    }
}
/// DFT clock compare config
///
/// Configuration register for Clock Compare logic. Compared clocks are always divided by 4 before any further processing. A clock edge on tx_clk increments the counter, a clock edge on rx_clk decrements the counter. If only one clock is selected for clock comparison, the number of clock cycles within a given time can be measured.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_CLK_CMP_CFG(u32);
impl DFT_CLK_CMP_CFG {
    /// Clock compare divider for RX clock
    ///
    /// 0: rx clk 1: rx_clk/2 2: rx_clk/4 3: rx_clk/8
    #[inline(always)]
    pub fn clk_cmp_div_rx(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    #[inline(always)]
    pub fn set_clk_cmp_div_rx(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 4;
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// Clock compare divider for TX clock
    ///
    /// 0: tx clk 1: tx_clk/2 2: tx_clk/4 3: tx_clk/8
    #[inline(always)]
    pub fn clk_cmp_div_tx(&self) -> u32 {
        (self.0 & 0xc0) >> 6
    }
    #[inline(always)]
    pub fn set_clk_cmp_div_tx(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 6;
        self.0 &= !0xc0;
        self.0 |= value;
    }
    /// Enable clock comparison (enabling automatically clears comparison counter)
    #[inline(always)]
    pub fn clk_cmp_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_clk_cmp_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Clock comparison mode
    ///
    /// 0: single shot 1: continuous
    #[inline(always)]
    pub fn clk_cmp_mode(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_clk_cmp_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Clock compare selection
    ///
    /// 0: rx_clk vs. tx_clk 1: rx_clk 2: tx_clk 3: Reserved
    #[inline(always)]
    pub fn clk_cmp_sel(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    #[inline(always)]
    pub fn set_clk_cmp_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 2;
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Clock compare value updated toggle bit. Toggles on each update of CLK_CMP_VALUE
    #[inline(always)]
    pub fn clk_cmp_updtog(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_clk_cmp_updtog(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Enable clock comparison counter wrap
    ///
    /// 0: counter saturates 1: counter wraps
    #[inline(always)]
    pub fn clk_cmp_wrap_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_clk_cmp_wrap_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
}
/// DFT clock comparison maximum value
///
/// Clock comparison max result. Can be used to judge e.g. SSC clock deviation. This register is updated after clock comparison timer has expired. In continuous mode this register is periodically updated.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_CLK_CMP_MAXVAL(u32);
impl DFT_CLK_CMP_MAXVAL {
    /// Clock comparison max value (maximum measured difference between clk0 and clk1)
    #[inline(always)]
    pub fn clk_cmp_maxval(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_clk_cmp_maxval(&mut self, value: u32) {
        self.0 = value;
    }
}
/// DFT clock compare timer
///
/// Clock comparison timer. After timer has expired, current clock comparison value is stored. The timer is clocked with core_clk (typically f=250MHz or 4ns period)
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_CLK_CMP_TIMER(u32);
impl DFT_CLK_CMP_TIMER {
    /// Clock comparison timer. Counter interval is N + 1 core clock cycles.
    #[inline(always)]
    pub fn clk_cmp_timer(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_clk_cmp_timer(&mut self, value: u32) {
        self.0 = value;
    }
}
/// DFT clock comparison value
///
/// Clock comparison result. This register is updated after clock comparison timer has expired. In continuous mode this register is periodically updated.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_CLK_CMP_VALUE(u32);
impl DFT_CLK_CMP_VALUE {
    /// Clock comparison value (difference between clk0 and clk1)
    #[inline(always)]
    pub fn clk_cmp_value(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_clk_cmp_value(&mut self, value: u32) {
        self.0 = value;
    }
}
/// DFT clock generator configuration register
///
/// Configuration register for clock generator to build a low speed clock signal of variable length and variable duty cycle provided on all data bits simultaniously
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_CLK_GEN_CFG(u32);
impl DFT_CLK_GEN_CFG {
    /// Duty cycle distortion: Refer to configuration fields 'cg_per_cfg' and 'cg_per_jump_cfg' for encoding description
    #[inline(always)]
    pub fn cg_dcd_cfg(&self) -> u32 {
        (self.0 & 0xffc) >> 2
    }
    #[inline(always)]
    pub fn set_cg_dcd_cfg(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 2;
        self.0 &= !0xffc;
        self.0 |= value;
    }
    /// clock generator mode
    ///
    /// 0: normal operation; cg_per_cfg controls period 0->1 transition: after current period has finished (only) the next period is controlled by cg_per_jump_cfg afterwards normal operation 2: every N'th period the high value is replaced by a low value N is defined by cg_timer_cfg 3: every N'th period the low value is replaced by a high value N is defined by cg_timer_cfg
    #[inline(always)]
    pub fn cg_mode_cfg(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_cg_mode_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// (Half) clock period configuration in normal mode (refer also to configuration field cg_mode_cfg):
    ///
    /// high period = cg_per_cfg + cg_dcd_cfg low period = cg_per_cfg - cg_dcd_cfg
    #[inline(always)]
    pub fn cg_per_cfg(&self) -> u32 {
        (self.0 & 0xffc00000) >> 22
    }
    #[inline(always)]
    pub fn set_cg_per_cfg(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 22;
        self.0 &= !0xffc00000;
        self.0 |= value;
    }
    /// (Half) clock period configuration in jump mode (refer also to configuration field cg_mode_cfg):
    ///
    /// high period = cg_per_jump_cfg + cg_dcd_cfg low period = cg_per_jump_cfg - cg_dcd_cfg
    #[inline(always)]
    pub fn cg_per_jump_cfg(&self) -> u32 {
        (self.0 & 0x3ff000) >> 12
    }
    #[inline(always)]
    pub fn set_cg_per_jump_cfg(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 12;
        self.0 &= !0x3ff000;
        self.0 |= value;
    }
}
/// DFT error status register
///
/// Status register for SD10G65 DFT containing the error counter value
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_ERR_STAT(u32);
impl DFT_ERR_STAT {
    /// Counter output depending on cnt_cfg
    #[inline(always)]
    pub fn err_cnt(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_err_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// DFT miscellaneous status register
///
/// Status register for SD10G65 DFT
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_MAIN_STAT(u32);
impl DFT_MAIN_STAT {
    /// BIST is active (i.e. left DOZE but did not enter a final state)
    #[inline(always)]
    pub fn active(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_active(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// 10 bits data word at address 'read_addr_cfg' used for further observation by SW
    #[inline(always)]
    pub fn cmp_data_stat(&self) -> u32 {
        (self.0 & 0x3ff00) >> 8
    }
    #[inline(always)]
    pub fn set_cmp_data_stat(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 8;
        self.0 &= !0x3ff00;
        self.0 |= value;
    }
    /// BIST not complete (i.e. not reached stable state or following)
    #[inline(always)]
    pub fn incomplete(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_incomplete(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// BIST: input data not stable
    #[inline(always)]
    pub fn instable(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_instable(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// BIST: no sync found since BIST enabled
    #[inline(always)]
    pub fn no_sync(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_no_sync(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Data input is constantly 0 or constantly 1 for all 40 parallel bits for at least 7 clock cycles (defined by c_STCK_CNT_THRES)
    #[inline(always)]
    pub fn stuck_at_01(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_stuck_at_01(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Data input is unchanged for all 40 parallel bits for at least 7 clock cycles (defined by c_STCK_CNT_THRES)
    #[inline(always)]
    pub fn stuck_at_par(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_stuck_at_par(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
}
/// DFT PRBS status register
///
/// Status register for SD10G65 DFT containing the PRBS data related to 1st sync lost event
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_PRBS_STAT(u32);
impl DFT_PRBS_STAT {
    /// PRBS data after first sync lost
    #[inline(always)]
    pub fn prbs_data_stat(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_prbs_data_stat(&mut self, value: u32) {
        self.0 = value;
    }
}
/// DFT Main configuration register
///
/// Main configuration register for SD10G65 DFT.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_RX_CFG(u32);
impl DFT_RX_CFG {
    /// States in which error counting is enabled
    ///
    /// 3:all but IDLE; 2:check 1:stable+check 0:wait_stable+stable+check
    #[inline(always)]
    pub fn bist_cnt_cfg(&self) -> u32 {
        (self.0 & 0x300000) >> 20
    }
    #[inline(always)]
    pub fn set_bist_cnt_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 20;
        self.0 &= !0x300000;
        self.0 |= value;
    }
    /// BIST mode
    ///
    /// 0: off 1: BIST 2: BER 3:CONT (infinite mode)
    #[inline(always)]
    pub fn bist_mode_cfg(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_bist_mode_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
    /// Selects pattern to check
    ///
    /// 0: PRBS pattern 1: constant pattern
    #[inline(always)]
    pub fn chk_mode_cfg(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_chk_mode_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Selects compare mode
    ///
    /// 0: compare mode possible 1 learn mode is forced
    #[inline(always)]
    pub fn cmp_mode_cfg(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_cmp_mode_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Selects modes in which error counter is active
    ///
    /// 0:learn and compare mode 1:transition between modes 2:learn mode 3:compare mode
    #[inline(always)]
    pub fn cnt_cfg(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    #[inline(always)]
    pub fn set_cnt_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 3;
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// SW reset of error counter; rising edge activates reset
    #[inline(always)]
    pub fn cnt_rst(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_cnt_rst(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Enable RX DFT capability
    ///
    /// 0: Disable DFT 1: Enable DFT
    #[inline(always)]
    pub fn dft_rx_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_dft_rx_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enables data through from gearbox to gearbox
    #[inline(always)]
    pub fn direct_through_ena_cfg(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    #[inline(always)]
    pub fn set_direct_through_ena_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 25;
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    /// Captures data from error counter to allow reading of stable data
    #[inline(always)]
    pub fn err_cnt_capt_cfg(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    #[inline(always)]
    pub fn set_err_cnt_capt_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 24;
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    /// Disable change of stored patterns (e.g. to avoid changes during read-out)
    #[inline(always)]
    pub fn freeze_pattern_cfg(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    #[inline(always)]
    pub fn set_freeze_pattern_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Enables PRBS checker input inversion
    #[inline(always)]
    pub fn inv_ena_cfg(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_inv_ena_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Number of consecutive errors/non-errors before transitioning to respective state
    ///
    /// value = num-40-bits-words + 1
    #[inline(always)]
    pub fn lrn_cnt_cfg(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    #[inline(always)]
    pub fn set_lrn_cnt_cfg(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 6;
        self.0 &= !0x1c0;
        self.0 |= value;
    }
    /// Data source selection
    ///
    /// 0: main path 1: vscope high path 2: vscope low path
    #[inline(always)]
    pub fn rx_data_src_sel(&self) -> u32 {
        (self.0 & 0xc00000) >> 22
    }
    #[inline(always)]
    pub fn set_rx_data_src_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 22;
        self.0 &= !0xc00000;
        self.0 |= value;
    }
    /// Selects PRBS check
    ///
    /// 0: prbs7 1: prbs15 2: prbs23 3: prbs11 4: prbs31 (default) 5: prbs9
    #[inline(always)]
    pub fn rx_prbs_sel_cfg(&self) -> u32 {
        (self.0 & 0x3800) >> 11
    }
    #[inline(always)]
    pub fn set_rx_prbs_sel_cfg(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 11;
        self.0 &= !0x3800;
        self.0 |= value;
    }
    /// Selects DES interface width
    ///
    /// 0:8 1:10 2:16 3:20 4:32 5:40 (default)
    #[inline(always)]
    pub fn rx_wid_sel_cfg(&self) -> u32 {
        (self.0 & 0x38000) >> 15
    }
    #[inline(always)]
    pub fn set_rx_wid_sel_cfg(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 15;
        self.0 &= !0x38000;
        self.0 |= value;
    }
    /// Pattern generator: 0:bytes mode; 1:10-bits word mode
    #[inline(always)]
    pub fn rx_word_mode_cfg(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    #[inline(always)]
    pub fn set_rx_word_mode_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 14;
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Disables error generation based on stuck_at_01 errors,
    ///
    /// 0: stuck_at_01 error generates 63 errors per clock cycle (in PRBS mode only) 1: stuck_at_01 error does not generate errors
    #[inline(always)]
    pub fn stuck_at_01_mask_cfg(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    #[inline(always)]
    pub fn set_stuck_at_01_mask_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 26;
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    /// Disables error generation based on stuck_at_par errors,
    ///
    /// 0: stuck_at_par error generates 63 errors per clock cycle (in PRBS mode only) 1: stuck_at_par error does not generate errors
    #[inline(always)]
    pub fn stuck_at_par_mask_cfg(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    #[inline(always)]
    pub fn set_stuck_at_par_mask_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 27;
        self.0 &= !0x8000000;
        self.0 |= value;
    }
}
/// DFT pattern mask configuration register
///
/// Configuration register for SD10G65 DFT to mask data bits preventing error counting for these bits.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_RX_MASK_CFG(u32);
impl DFT_RX_MASK_CFG {
    /// Mask out (active high) errors in 32 LSB data bits
    #[inline(always)]
    pub fn lsb_mask_cfg(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_lsb_mask_cfg(&mut self, value: u32) {
        self.0 = value;
    }
}
/// DFT Pattern checker configuration register
///
/// Pattern checker configuration register for SD10G65 DFT.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_RX_PAT_CFG(u32);
impl DFT_RX_PAT_CFG {
    /// Maximum address in Checker (before continuing with address 0)
    #[inline(always)]
    pub fn max_addr_chk_cfg(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    #[inline(always)]
    pub fn set_max_addr_chk_cfg(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 8;
        self.0 &= !0xf00;
        self.0 |= value;
    }
    /// Mask out (active high) errors in 8 MSB data bits
    #[inline(always)]
    pub fn msb_mask_cfg(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    #[inline(always)]
    pub fn set_msb_mask_cfg(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 24;
        self.0 &= !0xff000000;
        self.0 |= value;
    }
    /// Pattern read enable
    #[inline(always)]
    pub fn pat_read_cfg(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_pat_read_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Address to read patterns from used by SW
    #[inline(always)]
    pub fn read_addr_cfg(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_read_addr_cfg(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// DFT Main configuration register
///
/// Main configuration register for SD10G65 DFT.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_TX_CFG(u32);
impl DFT_TX_CFG {
    /// Enable TX DFT capability
    ///
    /// 0: Disable DFT 1: Enable DFT
    #[inline(always)]
    pub fn dft_tx_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_dft_tx_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Selects PRBS generator input
    ///
    /// 0:pat-gen 1:core
    #[inline(always)]
    pub fn ipath_cfg(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_ipath_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Selects DFT-TX output
    ///
    /// 0:PRBS/scrambler (default) 1:bypass 2:clock pattern generator
    #[inline(always)]
    pub fn opath_cfg(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    #[inline(always)]
    pub fn set_opath_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 2;
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Enables (1) reset of PRBS generator in case of unchanged data ('stuck-at') for at least 511 clock cycles. Can be disabled (0) e.g. in scrambler mode to avoid the very rare case that input patterns allow to keep the generator's shift register filled with a constant value.
    #[inline(always)]
    pub fn rst_on_stuck_at_cfg(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_rst_on_stuck_at_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Inverts the scrambler output
    #[inline(always)]
    pub fn scram_inv_cfg(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_scram_inv_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Selects PRBS generator
    ///
    /// 0: prbs7 1: prbs15 2: prbs23 3: prbs11 4: prbs31 (default) 5: prbs9
    #[inline(always)]
    pub fn tx_prbs_sel_cfg(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    #[inline(always)]
    pub fn set_tx_prbs_sel_cfg(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 6;
        self.0 &= !0x1c0;
        self.0 |= value;
    }
    /// Selects SER interface width
    ///
    /// 0:8 1:10 2:16 3:20 4:32 5:40 (default)
    #[inline(always)]
    pub fn tx_wid_sel_cfg(&self) -> u32 {
        (self.0 & 0xe00) >> 9
    }
    #[inline(always)]
    pub fn set_tx_wid_sel_cfg(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 9;
        self.0 &= !0xe00;
        self.0 |= value;
    }
    /// Word width of constant pattern generator
    ///
    /// 0:bytes mode; 1:10-bits word mode
    #[inline(always)]
    pub fn tx_word_mode_cfg(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_tx_word_mode_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// DFT TX constant pattern status register
///
/// Status register for SD10G65 DFT containing the constant patterns used for comparison (last in LEARN mode)
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_TX_CMP_DAT_STAT(u32);
impl DFT_TX_CMP_DAT_STAT {
    /// 10 bits data word at address 'store_addr_cfg' used for further observation by SW
    #[inline(always)]
    pub fn pat_stat(&self) -> u32 {
        self.0 & 0x3ff
    }
    #[inline(always)]
    pub fn set_pat_stat(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
    /// Scrambler/PRBS generator output unchanged for at least 511 clock cycles
    #[inline(always)]
    pub fn tx_stuck_at_sticky(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_tx_stuck_at_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
}
/// DFT TX Error insertion configuration register
///
/// Configuration register for explicit error insertion into DFT driven data stream. Allows to insert expected errors to check e.g. TX/RX connectivity
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_TX_ERR_INSERT_CFG(u32);
impl DFT_TX_ERR_INSERT_CFG {
    /// Preload value for clock generator timer (refer also to configuration field cg_mode_cfg).
    #[inline(always)]
    pub fn cg_timer_cfg(&self) -> u32 {
        (self.0 & 0xffc00000) >> 22
    }
    #[inline(always)]
    pub fn set_cg_timer_cfg(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        let value = value << 22;
        self.0 &= !0xffc00000;
        self.0 |= value;
    }
    /// Frequency of continous/limited error insertion in steps of 40 bits (refer also to err_posit_offs_cfg)
    ///
    /// 0: disable continous insertion 1-15: step between 2 errors = 2^(err_freq_cfg + 5) 40 bit words
    #[inline(always)]
    pub fn err_freq_cfg(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    #[inline(always)]
    pub fn set_err_freq_cfg(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 16;
        self.0 &= !0xf0000;
        self.0 |= value;
    }
    /// Position within 40 bit word where an error is inserted by inverting the bit value
    ///
    /// 0: LSB 39: MSB 40-63: reserved
    #[inline(always)]
    pub fn err_posit_cfg(&self) -> u32 {
        (self.0 & 0xfc00) >> 10
    }
    #[inline(always)]
    pub fn set_err_posit_cfg(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 10;
        self.0 &= !0xfc00;
        self.0 |= value;
    }
    /// Offset of bit position increased per inserted error; allows 'walking' error. Offset is reset when continous/limited error insertion is disabled or burst mode is enabled and burst insertion is finished or err_posit_offs_cfg = 0
    ///
    /// 0: disabled 1: move 1 bit (from LSB to MSB) ... 39: move 39 bit (from LSB to MSB) 40-63: reserved
    #[inline(always)]
    pub fn err_posit_offs_cfg(&self) -> u32 {
        (self.0 & 0x3f0) >> 4
    }
    #[inline(always)]
    pub fn set_err_posit_offs_cfg(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 4;
        self.0 &= !0x3f0;
        self.0 |= value;
    }
    /// Trigger a single error or a burst of errors (refer to num_err_cfg)
    ///
    /// 0 to 1 (edge) activates this function
    #[inline(always)]
    pub fn err_trig_oneshot_cfg(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_err_trig_oneshot_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Limited error insertion: burst mode (err_freq_cfg must be > 0; each burst is triggered by a 0 to 1 transition of configuration field err_trig_oneshot_cfg)
    ///
    /// 0: burst mode is disabled 1-15: number of errors per burst = 2^(num_err_cfg + 5)
    #[inline(always)]
    pub fn num_err_cfg(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_num_err_cfg(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// DFT TX Constant pattern configuration register
///
/// TX Constant MSB pattern configuration register for SD10G65 DFT.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DFT_TX_PAT_CFG(u32);
impl DFT_TX_PAT_CFG {
    /// Maximum address in generator (before continuing with address 0)
    #[inline(always)]
    pub fn max_addr_gen_cfg(&self) -> u32 {
        (self.0 & 0x3c0000) >> 18
    }
    #[inline(always)]
    pub fn set_max_addr_gen_cfg(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 18;
        self.0 &= !0x3c0000;
        self.0 |= value;
    }
    /// 10 bits word of constant patterns for transmission
    #[inline(always)]
    pub fn pattern_cfg(&self) -> u32 {
        self.0 & 0x3ff
    }
    #[inline(always)]
    pub fn set_pattern_cfg(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
    /// Constant patterns are valid to store
    #[inline(always)]
    pub fn pat_vld_cfg(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    #[inline(always)]
    pub fn set_pat_vld_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 26;
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    /// Current storage address for patterns in generator
    #[inline(always)]
    pub fn store_addr_cfg(&self) -> u32 {
        (self.0 & 0x3c00) >> 10
    }
    #[inline(always)]
    pub fn set_store_addr_cfg(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 10;
        self.0 &= !0x3c00;
        self.0 |= value;
    }
}
