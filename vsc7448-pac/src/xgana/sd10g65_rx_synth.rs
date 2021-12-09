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
/// SD10G65 RX RCPLL Status register 1
///
/// Status register 1 for SD10G65 RX RCPLL.
#[derive(From, Into)]
pub struct SD10G65_RX_RCPLL_STAT1(u32);
impl SD10G65_RX_RCPLL_STAT1 {
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
/// SD10G65 RX Synthesizer Register CDR loopfilter control
///
/// Register for CDR loopfilter control for SD10G65 RX SYNTH.
#[derive(From, Into)]
pub struct SD10G65_RX_SYNTH_CDRLF(u32);
impl SD10G65_RX_SYNTH_CDRLF {
    /// frequency select of integrator 1
    pub fn synth_integ1_fsel(&self) -> u32 {
        (self.0 & 0x7c0) >> 6
    }
    pub fn set_synth_integ1_fsel(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x7c0);
        self.0 &= !0x7c0;
        self.0 |= value;
    }
    /// limit of integrator 1
    pub fn synth_integ1_lim(&self) -> u32 {
        (self.0 & 0xf800) >> 11
    }
    pub fn set_synth_integ1_lim(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0xf800);
        self.0 &= !0xf800;
        self.0 |= value;
    }
    /// max value of integrator 1 during init phase
    pub fn synth_integ1_max0(&self) -> u32 {
        (self.0 & 0x1f0000) >> 16
    }
    pub fn set_synth_integ1_max0(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x1f0000);
        self.0 &= !0x1f0000;
        self.0 |= value;
    }
    /// max value of integrator 1 during normal operation
    pub fn synth_integ1_max1(&self) -> u32 {
        (self.0 & 0x3e00000) >> 21
    }
    pub fn set_synth_integ1_max1(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x3e00000);
        self.0 &= !0x3e00000;
        self.0 |= value;
    }
    /// frequency select of integrator 2
    pub fn synth_integ2_fsel(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_synth_integ2_fsel(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}
/// SD10G65 RX Synthesizer Configuration register 0
///
/// Configuration register 0 for SD10G65 RX SYNTH.
#[derive(From, Into)]
pub struct SD10G65_RX_SYNTH_CFG0(u32);
impl SD10G65_RX_SYNTH_CFG0 {
    /// enables CML2CMOS converter (low speed part of synthesizer)
    pub fn synth_conv_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_synth_conv_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// synthesizer enable
    pub fn synth_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_synth_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// selects feedback divider setting.
    ///
    /// 0: divide by 1 1: divide by 2 2: divide by 4 3: reserved
    pub fn synth_fbdiv_sel(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    pub fn set_synth_fbdiv_sel(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x30000);
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// inverts direction of sync out part
    pub fn synth_fb_dir(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_synth_fb_dir(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// selects step width for sync output
    pub fn synth_fb_step(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    pub fn set_synth_fb_step(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0xc000);
        self.0 &= !0xc000;
        self.0 |= value;
    }
    /// enables half rate mode
    pub fn synth_hrate_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_synth_hrate_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// inverts direction of integral1 part
    pub fn synth_i1_dir(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_synth_i1_dir(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// selects step width for integrator1
    pub fn synth_i1_step(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_synth_i1_step(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// inverts direction of integral2 part
    pub fn synth_i2_dir(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_synth_i2_dir(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// enable contribution of integral2 part
    pub fn synth_i2_ena(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_synth_i2_ena(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// selects step width for integrator2
    pub fn synth_i2_step(&self) -> u32 {
        (self.0 & 0x1800) >> 11
    }
    pub fn set_synth_i2_step(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x1800);
        self.0 &= !0x1800;
        self.0 |= value;
    }
    /// Enable for different offset compensation stages
    ///
    /// bit 0: Synthesizer main rotator bit 1: Feedback buffer bit 2: CDR rotator bit 3: VCO buffer
    pub fn synth_off_comp_ena(&self) -> u32 {
        (self.0 & 0x3c0000) >> 18
    }
    pub fn set_synth_off_comp_ena(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x3c0000);
        self.0 &= !0x3c0000;
        self.0 |= value;
    }
    /// inverts direction of propotional part
    pub fn synth_p_dir(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_synth_p_dir(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// selects step width for propotional
    pub fn synth_p_step(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_synth_p_step(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// reg. pool for late changes/fixes. Used bits: Bit3-1: Synthesizer BIAS adjust in steps of ~3%.
    ///
    /// 0: 100% 7: 121%
    pub fn synth_spare_pool(&self) -> u32 {
        (self.0 & 0x3fc00000) >> 22
    }
    pub fn set_synth_spare_pool(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x3fc00000);
        self.0 &= !0x3fc00000;
        self.0 |= value;
    }
    /// Selects circuit speed.
    ///
    /// 0: for settings with synth_fbdiv_sel = 2 1: for setting with synth_fbdiv_sel less than 2
    pub fn synth_speed_sel(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_synth_speed_sel(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}
/// SD10G65 RX Synthesizer Configuration register 1
///
/// Configuration register 1 for SD10G65 RX SYNTH.
#[derive(From, Into)]
pub struct SD10G65_RX_SYNTH_CFG1(u32);
impl SD10G65_RX_SYNTH_CFG1 {
    /// frequency m setting bits 35:32
    pub fn synth_freqm_1(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    pub fn set_synth_freqm_1(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0xf0);
        self.0 &= !0xf0;
        self.0 |= value;
    }
    /// frequency n setting bits 35:32
    pub fn synth_freqn_1(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_synth_freqn_1(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// frequency multiplier
    pub fn synth_freq_mult(&self) -> u32 {
        (self.0 & 0x3fff00) >> 8
    }
    pub fn set_synth_freq_mult(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x3fff00);
        self.0 &= !0x3fff00;
        self.0 |= value;
    }
    /// frequency multiplier decoder bypass
    pub fn synth_freq_mult_byp(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    pub fn set_synth_freq_mult_byp(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0x4000000);
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    /// frequency multiplier MSBits in bypass mode
    pub fn synth_freq_mult_hi(&self) -> u32 {
        (self.0 & 0x3c00000) >> 22
    }
    pub fn set_synth_freq_mult_hi(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x3c00000);
        self.0 &= !0x3c00000;
        self.0 |= value;
    }
}
/// SD10G65 RX Synthesizer Configuration register 2
///
/// Configuration register 2 for SD10G65 RX SYNTH.
#[derive(From, Into)]
pub struct SD10G65_RX_SYNTH_CFG2(u32);
impl SD10G65_RX_SYNTH_CFG2 {
    /// enables clock for VScope / APC auxillary data chanels
    pub fn synth_aux_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_synth_aux_ena(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// uses cp/md selected via synth_cpmd_dig_sel instead of cp/md from sample stage
    pub fn synth_cpmd_dig_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_synth_cpmd_dig_ena(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Cp/md dig select. Coding 0: select Bit 0/5 as cp/md (FX100 mode); 1: use cp/md from core
    pub fn synth_cpmd_dig_sel(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_synth_cpmd_dig_sel(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Controls the data valid behavior for the CDRLF I1 enable function: b0 = 0 => external signal controls, 1 => b1 controls
    pub fn synth_dv_ctrl_i1e(&self) -> u32 {
        (self.0 & 0xc00000) >> 22
    }
    pub fn set_synth_dv_ctrl_i1e(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0xc00000);
        self.0 &= !0xc00000;
        self.0 |= value;
    }
    /// Controls the data valid behavior for the CDRLF I1 max function: b0 = 0 => external signal controls, 1 => b1 controls
    pub fn synth_dv_ctrl_i1m(&self) -> u32 {
        (self.0 & 0x3000000) >> 24
    }
    pub fn set_synth_dv_ctrl_i1m(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x3000000);
        self.0 &= !0x3000000;
        self.0 |= value;
    }
    /// Controls the data valid behavior for the CDRLF I2 enable function: b0 = 0 => external signal controls, 1 => b1 controls
    pub fn synth_dv_ctrl_i2e(&self) -> u32 {
        (self.0 & 0xc000000) >> 26
    }
    pub fn set_synth_dv_ctrl_i2e(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0xc000000);
        self.0 &= !0xc000000;
        self.0 |= value;
    }
    /// Controls the data valid behavior for the moebdiv select function: b0 = 0 => external signal controls, 1 => b1 controls
    pub fn synth_dv_ctrl_md(&self) -> u32 {
        (self.0 & 0x300000) >> 20
    }
    pub fn set_synth_dv_ctrl_md(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x300000);
        self.0 &= !0x300000;
        self.0 |= value;
    }
    /// Enable ET-Serdes test mode. In this test mode the data_valid input port of SD10G65_RX switches between lock2data mode (data_valid = 0) and lock2ref mode (data_valid = 1). Do not use in mission mode.
    pub fn synth_etsd_mode_ena(&self) -> u32 {
        (self.0 & 0x10000000) >> 28
    }
    pub fn set_synth_etsd_mode_ena(&mut self, value: u32) {
        let value = value << 28;
        assert!(value <= 0x10000000);
        self.0 &= !0x10000000;
        self.0 |= value;
    }
    /// relationship phase center/aux
    pub fn synth_phase_aux(&self) -> u32 {
        self.0 & 0x7f
    }
    pub fn set_synth_phase_aux(&mut self, value: u32) {
        assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }
    /// relationship phase center/edge
    pub fn synth_phase_data(&self) -> u32 {
        (self.0 & 0x7f00) >> 8
    }
    pub fn set_synth_phase_data(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x7f00);
        self.0 &= !0x7f00;
        self.0 |= value;
    }
}
/// SD10G65 RX Synthesizer Configuration register 3
///
/// Configuration register 3 for SD10G65 RX SYNTH.
#[derive(From, Into)]
pub struct SD10G65_RX_SYNTH_CFG3(u32);
impl SD10G65_RX_SYNTH_CFG3 {
    /// frequency m setting bits 31:0
    pub fn synth_freqm_0(&self) -> u32 {
        self.0
    }
    pub fn set_synth_freqm_0(&mut self, value: u32) {
        self.0 = value;
    }
}
/// SD10G65 RX Synthesizer Configuration register 4
///
/// Configuration register 4 for SD10G65 RX SYNTH.
#[derive(From, Into)]
pub struct SD10G65_RX_SYNTH_CFG4(u32);
impl SD10G65_RX_SYNTH_CFG4 {
    /// frequency n setting bits 31:0
    pub fn synth_freqn_0(&self) -> u32 {
        self.0
    }
    pub fn set_synth_freqn_0(&mut self, value: u32) {
        self.0 = value;
    }
}
/// SD10G65 RX Synthesizer Register 0 for qualifier access
///
/// Register 0 for qualifier access for SD10G65 RX SYNTH.
#[derive(From, Into)]
pub struct SD10G65_RX_SYNTH_QUALIFIER0(u32);
impl SD10G65_RX_SYNTH_QUALIFIER0 {
    /// Rising edge captures qualifier for readback
    pub fn synth_capture_qual(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_synth_capture_qual(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Sticky flag to indicate saturating of Integrator1
    pub fn synth_i1_sat_det(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_synth_i1_sat_det(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    /// Clear for sticky flag "synth_i1_sat_det"
    pub fn synth_i1_sat_det_clr(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    pub fn set_synth_i1_sat_det_clr(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x2000000);
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    /// Sticky flag to indicate a wrap/saturating of Integrator2
    pub fn synth_i2_wrap_det(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_synth_i2_wrap_det(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// Clear for sticky flag "synth_I2_wrap_det"
    pub fn synth_i2_wrap_det_clr(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_synth_i2_wrap_det_clr(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Controls Integrator2 behavior: '0': wrapping; '1': saturating.
    pub fn synth_i2_wrap_inhibit(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_synth_i2_wrap_inhibit(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }
    /// Captured integrator 1 value
    pub fn synth_qual_i1(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_synth_qual_i1(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// MS Bits of captured integrator 2
    pub fn synth_qual_i2_msb(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    pub fn set_synth_qual_i2_msb(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xf0000);
        self.0 &= !0xf0000;
        self.0 |= value;
    }
}
/// SD10G65 RX Synthesizer Register 1 for qualifier access
///
/// Register 1 for qualifier access for SD10G65 RX SYNTH.
#[derive(From, Into)]
pub struct SD10G65_RX_SYNTH_QUALIFIER1(u32);
impl SD10G65_RX_SYNTH_QUALIFIER1 {
    /// LS Bits of captured integrator 2
    pub fn synth_qual_i2_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_synth_qual_i2_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// SD10G65 RX Synthesizer Register for sync control data
///
/// Register 0 for sync control data for SD10G65 RX SYNTH.
#[derive(From, Into)]
pub struct SD10G65_RX_SYNTH_SYNC_CTRL(u32);
impl SD10G65_RX_SYNTH_SYNC_CTRL {
    /// Sticky bit that indicates a sync control protocol error.
    pub fn synth_sc_protocol_err(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_synth_sc_protocol_err(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Clear for synth_sc_protocol_err sticky bit. Rising edge causes the clearing and a concurrent error event has higher priority.
    pub fn synth_sc_protocol_err_clr(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_synth_sc_protocol_err_clr(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Selects the synchronization period for the I2 value via sync control bus. Must be disabled (0) when sync control test generator is used. Coding in 312.5MHz clock cycles: 0: disabled, 1: 2^6, 2: 2^7, .., 15: 2^20.
    pub fn synth_sc_sync_timer_sel(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_synth_sc_sync_timer_sel(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Selects sync control test generator period. Test generator period, i.e. number of INC resp. DEC steps, equal programmed value + 1.
    pub fn synth_sc_test_count(&self) -> u32 {
        (self.0 & 0x3ff0) >> 4
    }
    pub fn set_synth_sc_test_count(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x3ff0);
        self.0 &= !0x3ff0;
        self.0 |= value;
    }
    /// Enables sync control test generator. Before enabling the test generator the sync timer must be disabled (synth_sc_sync_timer_sel = 0) and the integrator 2 has to be cleared (synth_dv_ctrl_i2e = 1).
    pub fn synth_sc_test_enable(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_synth_sc_test_enable(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Selects sync control test generator mode: 0: INC, 1: DEC, 2: WOBBLE
    pub fn synth_sc_test_mode(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    pub fn set_synth_sc_test_mode(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0xc000);
        self.0 &= !0xc000;
        self.0 |= value;
    }
    /// Triggers sync control test generator. In INC resp. DEC mode both edges act as trigger event. In WOBBLE mode a 1 enables wobbling. Disabling in WOBBEL mode lets current wobble cycle finish before stopping. I.e. the I2 values reaches its value prior wobbling again.
    pub fn synth_sc_test_trigger(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_synth_sc_test_trigger(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
}
