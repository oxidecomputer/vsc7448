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
/// SD10G65 SSC generator Configuration register 1
///
/// Configuration register 1 for SD10G65 SSC generator.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_SSC_CFG1(u32);
impl SD10G65_SSC_CFG1 {
    /// Select the MLD clock source for the internal MLD phase detector
    pub fn mld_sync_clk_sel(&self) -> u32 {
        (self.0 & 0x1800000) >> 23
    }
    pub fn set_mld_sync_clk_sel(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x1800000);
        self.0 &= !0x1800000;
        self.0 |= value;
    }
    /// Control of the internal MLD phase detector: b0: enable; b1: enable hyst. b2: enable window function; b3: select window size
    pub fn mld_sync_ctrl(&self) -> u32 {
        (self.0 & 0x1e000000) >> 25
    }
    pub fn set_mld_sync_ctrl(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x1e000000);
        self.0 &= !0x1e000000;
        self.0 |= value;
    }
    /// Select between the internal and external MLD phase detector: 0: internal; 1: external
    pub fn mld_sync_src_sel(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    pub fn set_mld_sync_src_sel(&mut self, value: u32) {
        let value = value << 29;
        assert!(value <= 0x20000000);
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    /// Enables Smooth generator
    pub fn smooth_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_smooth_ena(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// SSC modulation period multiplier encoded 2**n: 0 => 1; 1 => 2; 2 => 4, 3 => 8 ...
    pub fn ssc_mod_mul(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_ssc_mod_mul(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// SSC sigma delta gain.
    pub fn ssc_sd_gain(&self) -> u32 {
        (self.0 & 0x3e0) >> 5
    }
    pub fn set_ssc_sd_gain(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x3e0);
        self.0 &= !0x3e0;
        self.0 |= value;
    }
    /// SSC modulation start position on synchronization trigger
    pub fn ssc_sync_pos(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    pub fn set_ssc_sync_pos(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x18);
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// Frequency select of integrator 2 replica used for lane sync.
    pub fn sync_ctrl_fsel(&self) -> u32 {
        (self.0 & 0x3f0000) >> 16
    }
    pub fn set_sync_ctrl_fsel(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x3f0000);
        self.0 &= !0x3f0000;
        self.0 |= value;
    }
    /// Sticky bit that indicates a sync control protocol error.
    pub fn sync_ctrl_protocol_err(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_sync_ctrl_protocol_err(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Clear for synth_sc_protocol_err sticky bit. Rising edge causes the clearing and a concurrent error event has higher priority.
    pub fn sync_ctrl_protocol_err_clr(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_sync_ctrl_protocol_err_clr(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    /// Controls integrator 2 replica behavior: '0': wrapping; '1': saturating.
    pub fn sync_ctrl_wrap_inhibit(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_sync_ctrl_wrap_inhibit(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
}
/// SD10G65 TX RCPLL Configuration register 0
///
/// Configuration register 0 for SD10G65 TX RCPLL.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_RCPLL_CFG0(u32);
impl SD10G65_TX_RCPLL_CFG0 {
    /// Enable RCPLL FSM
    pub fn pllf_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pllf_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable predivider for feedback clock to allow for faster clocks,
    ///
    /// 0: disable 1: enable
    pub fn pllf_fbdiv_pre(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_pllf_fbdiv_pre(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Enable PLL loop ctrl by pllf_loop_ena,
    ///
    /// 0: enable loop ctrl by FSM 1: enable loop ctrl by pllf_loop_ena
    pub fn pllf_loop_ctrl_ena(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_pllf_loop_ctrl_ena(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Enable closed loop, selected with pllf_loop_ctrl_ena=1
    pub fn pllf_loop_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_pllf_loop_ena(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Enable VCO control signal out of range recalibration
    pub fn pllf_oor_recal_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_pllf_oor_recal_ena(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Sets the ramp characteristic of the FSM, higher values give faster ramp up but less accuracy,
    ///
    /// 0: normal (default) ramping 1: faster ramping 2: fastest ramping 3: slow ramping uses all possible values of r_ctrl
    pub fn pllf_ramp_mode_sel(&self) -> u32 {
        (self.0 & 0x380) >> 7
    }
    pub fn set_pllf_ramp_mode_sel(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x380);
        self.0 &= !0x380;
        self.0 |= value;
    }
    /// Select output on pllf_ref_cnt_stat,
    ///
    /// 0: ref_cnt 1: pll_cnt_diff 2: syn_cnt 3: pll_cnt
    pub fn pllf_ref_cnt_sel(&self) -> u32 {
        (self.0 & 0x3000) >> 12
    }
    pub fn set_pllf_ref_cnt_sel(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x3000);
        self.0 &= !0x3000;
        self.0 |= value;
    }
    /// Enable restart of FSM from frequency deviation/unlock condition
    pub fn pllf_rst_frqdet_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_pllf_rst_frqdet_ena(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Enable restart of FSM from overrun
    pub fn pllf_rst_overrun_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_pllf_rst_overrun_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Preload value of the ramp up counter, reduces ramp up time for higher frequencies
    pub fn pllf_start_cnt(&self) -> u32 {
        (self.0 & 0x3ff0000) >> 16
    }
    pub fn set_pllf_start_cnt(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x3ff0000);
        self.0 &= !0x3ff0000;
        self.0 |= value;
    }
    /// Enable feedback clock usage instead of DES/SER par. clock,
    ///
    /// 0: disable 1: enable
    pub fn pllf_syn_clk_ena(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_pllf_syn_clk_ena(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Enable vco load by pllf_vco_fast,
    ///
    /// 0: enable vco fast by FSM 1: enable vco load by pllf_vco_fast
    pub fn pllf_vco_fast_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_pllf_vco_fast_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable vco load by pllf_vco_load,
    ///
    /// 0: enable vco load by FSM 1: enable vco load by pllf_vco_load
    pub fn pllf_vco_load_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_pllf_vco_load_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Enable vco load by pllf_vco_slow,
    ///
    /// 0: enable vco slow by FSM 1: enable vco load by pllf_vco_slow
    pub fn pllf_vco_slow_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_pllf_vco_slow_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// SD10G65 TX RCPLL Configuration register 1
///
/// Configuration register 1 for SD10G65 TX RCPLL.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_RCPLL_CFG1(u32);
impl SD10G65_TX_RCPLL_CFG1 {
    /// enable special BIST settings
    pub fn pllf_bist_ena(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_pllf_bist_ena(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Target value: 1/vco_frq * par.bit.width * 512 * ref_clk_frq
    pub fn pllf_ref_cnt_end(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_pllf_ref_cnt_end(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    /// Select vco static frequency by feedforward (higher freq.),
    ///
    /// 0: min. freq 3: max. freq.
    pub fn pllf_vco_fast(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_pllf_vco_fast(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Select vco static frequency by load,
    ///
    /// 0: VCO off
    pub fn pllf_vco_load(&self) -> u32 {
        (self.0 & 0x3ff0) >> 4
    }
    pub fn set_pllf_vco_load(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x3ff0);
        self.0 &= !0x3ff0;
        self.0 |= value;
    }
    /// Select vco static frequency by feedback (lower freq.),
    ///
    /// 0: max. freq 3: min. freq.
    pub fn pllf_vco_slow(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_pllf_vco_slow(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
/// SD10G65 TX RCPLL Configuration register 2
///
/// Configuration register 2 for SD10G65 TX RCPLL.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_RCPLL_CFG2(u32);
impl SD10G65_TX_RCPLL_CFG2 {
    /// Not used
    pub fn pll_cal_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_pll_cal_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enable analog RCPLL part
    pub fn pll_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pll_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Select VCO regulated supply (820mV),
    ///
    /// 0: 820mv 4: 860mV 6: 950mV 7: short to VDDA all others reserved
    pub fn pll_fbsel820(&self) -> u32 {
        (self.0 & 0x7000000) >> 24
    }
    pub fn set_pll_fbsel820(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x7000000);
        self.0 &= !0x7000000;
        self.0 |= value;
    }
    /// Select chargepump current,
    ///
    /// 0: 50uA 1: 100uA 2: 150uA 3: 200uA
    pub fn pll_lpf_cur(&self) -> u32 {
        (self.0 & 0x1800) >> 11
    }
    pub fn set_pll_lpf_cur(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x1800);
        self.0 &= !0x1800;
        self.0 |= value;
    }
    /// Select loop filter resistor value,
    ///
    /// 0: not allowed 1: 2400 2: 1600 3: 960 4: 1200 5: 800 6: 685 7: 533 8: 800 9: 600 10: 533 11: 436 12: 480 13: 400 14: 369 15: 320
    pub fn pll_lpf_res(&self) -> u32 {
        (self.0 & 0x780) >> 7
    }
    pub fn set_pll_lpf_res(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x780);
        self.0 &= !0x780;
        self.0 |= value;
    }
    /// Debug, short N bulk voltage of OpAmps,
    ///
    /// 0: N bulk unequal 0 1: short to VSSA
    pub fn pll_short_bulkhn_ena(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_pll_short_bulkhn_ena(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Debug, short P bulk voltage of OpAmps,
    ///
    /// 0: P bulk unequal supply 1: short to supply
    pub fn pll_short_bulkhp_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_pll_short_bulkhp_ena(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Debug, short N bulk voltage of VCO,
    ///
    /// 0: N bulk unequal 0 1: short to VSSA
    pub fn pll_short_bulkn_ena(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_pll_short_bulkn_ena(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Debug, short P bulk voltage of VCO,
    ///
    /// 0: P bulk unequal supply 1: short to supply
    pub fn pll_short_bulkp_ena(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_pll_short_bulkp_ena(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Select vco current,
    ///
    /// 0: lowest current 31: highest current
    pub fn pll_vco_cur(&self) -> u32 {
        (self.0 & 0x7c) >> 2
    }
    pub fn set_pll_vco_cur(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x7c);
        self.0 &= !0x7c;
        self.0 |= value;
    }
    /// Select VCO load switch gate voltage,
    ///
    /// 0: 1.4V 1: 1.5V 2: 1.6V 3: 1.7V 15: max.
    pub fn pll_vreg18(&self) -> u32 {
        (self.0 & 0xf00000) >> 20
    }
    pub fn set_pll_vreg18(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0xf00000);
        self.0 &= !0xf00000;
        self.0 |= value;
    }
}
/// SD10G65 TX RCPLL Status register 0
///
/// Status register 0 for SD10G65 TX RCPLL.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_RCPLL_STAT0(u32);
impl SD10G65_TX_RCPLL_STAT0 {
    /// PLL lock status,
    ///
    /// 0: not locked 1: locked
    pub fn pllf_lock_stat(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_pllf_lock_stat(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Actual value of VCO fast portion, set by FSM
    pub fn pllf_vco_fast_stat(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_pllf_vco_fast_stat(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Actual value of VCO load, set by FSM
    pub fn pllf_vco_load_stat(&self) -> u32 {
        (self.0 & 0x3ff0) >> 4
    }
    pub fn set_pllf_vco_load_stat(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x3ff0);
        self.0 &= !0x3ff0;
        self.0 |= value;
    }
    /// Actual value of VCO slow portion, set by FSM
    pub fn pllf_vco_slow_stat(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_pllf_vco_slow_stat(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Out of range status bit,
    ///
    /// 0: within range 1: out of range
    pub fn pll_range_limit(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_pll_range_limit(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Out of range status bit, sticky bit,
    ///
    /// 0: within range 1: out of range
    pub fn pll_range_limit_sty(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_pll_range_limit_sty(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
}
/// SD10G65 TX RCPLL Status register 1
///
/// Status register 1 for SD10G65 TX RCPLL.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_RCPLL_STAT1(u32);
impl SD10G65_TX_RCPLL_STAT1 {
    /// Actual value of step up counter
    pub fn pllf_fsm_cnt_stat(&self) -> u32 {
        (self.0 & 0x7ff0) >> 4
    }
    pub fn set_pllf_fsm_cnt_stat(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x7ff0);
        self.0 &= !0x7ff0;
        self.0 |= value;
    }
    /// Actual value of the FSM stage,
    ///
    /// 0: reset state 1: init state after reset 3: ramp up state checks for the counters and ramps up the frequency 6: additional wait state for internal BIAS settling 8: additional wait state 1 9: additional wait state 2 10; additional wait state 3 11: additional wait state 4 12: 1st locking state enables dynamic locking 13: final locking state checks for out of lock and overrun condition 14: error state low frequency 15: error state high frequency
    pub fn pllf_fsm_stat(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_pllf_fsm_stat(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Internal FSM values selected by pllf_ref_cnt_sel
    pub fn pllf_ref_cnt_stat(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_pllf_ref_cnt_stat(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}
