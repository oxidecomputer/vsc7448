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
/// SD10G65 SSC generator Configuration register 0
///
/// Configuration register 0 for SD10G65 SSC generator.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SD10G65_SSC_CFG0(u32);
impl SD10G65_SSC_CFG0 {
    /// SSC generator enable.
    #[inline(always)]
    pub fn ssc_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_ssc_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// SSC modulation frequency fine tuning control
    #[inline(always)]
    pub fn ssc_mod_freq(&self) -> u32 {
        (self.0 & 0x7e) >> 1
    }
    #[inline(always)]
    pub fn set_ssc_mod_freq(&mut self, value: u32) {
        debug_assert!(value <= 0x3f);
        let value = value << 1;
        self.0 &= !0x7e;
        self.0 |= value;
    }
    /// SSC modulation amplitude limiter
    #[inline(always)]
    pub fn ssc_mod_lim(&self) -> u32 {
        (self.0 & 0xfff80000) >> 19
    }
    #[inline(always)]
    pub fn set_ssc_mod_lim(&mut self, value: u32) {
        debug_assert!(value <= 0x1fff);
        let value = value << 19;
        self.0 &= !0xfff80000;
        self.0 |= value;
    }
    /// SSC modulation period / amplitude.
    #[inline(always)]
    pub fn ssc_mod_period(&self) -> u32 {
        (self.0 & 0x7ff80) >> 7
    }
    #[inline(always)]
    pub fn set_ssc_mod_period(&mut self, value: u32) {
        debug_assert!(value <= 0xfff);
        let value = value << 7;
        self.0 &= !0x7ff80;
        self.0 |= value;
    }
}
/// SD10G65 SSC generator Configuration register 1
///
/// Configuration register 1 for SD10G65 SSC generator.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SD10G65_SSC_CFG1(u32);
impl SD10G65_SSC_CFG1 {
    /// Select the MLD clock source for the internal MLD phase detector
    #[inline(always)]
    pub fn mld_sync_clk_sel(&self) -> u32 {
        (self.0 & 0x1800000) >> 23
    }
    #[inline(always)]
    pub fn set_mld_sync_clk_sel(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        let value = value << 23;
        self.0 &= !0x1800000;
        self.0 |= value;
    }
    /// Control of the internal MLD phase detector: b0: enable; b1: enable hyst. b2: enable window function; b3: select window size
    #[inline(always)]
    pub fn mld_sync_ctrl(&self) -> u32 {
        (self.0 & 0x1e000000) >> 25
    }
    #[inline(always)]
    pub fn set_mld_sync_ctrl(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        let value = value << 25;
        self.0 &= !0x1e000000;
        self.0 |= value;
    }
    /// Select between the internal and external MLD phase detector: 0: internal; 1: external
    #[inline(always)]
    pub fn mld_sync_src_sel(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    #[inline(always)]
    pub fn set_mld_sync_src_sel(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 29;
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    /// Enables Smooth generator
    #[inline(always)]
    pub fn smooth_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_smooth_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// SSC modulation period multiplier encoded 2**n: 0 => 1; 1 => 2; 2 => 4, 3 => 8 ...
    #[inline(always)]
    pub fn ssc_mod_mul(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_ssc_mod_mul(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// SSC sigma delta gain.
    #[inline(always)]
    pub fn ssc_sd_gain(&self) -> u32 {
        (self.0 & 0x3e0) >> 5
    }
    #[inline(always)]
    pub fn set_ssc_sd_gain(&mut self, value: u32) {
        debug_assert!(value <= 0x1f);
        let value = value << 5;
        self.0 &= !0x3e0;
        self.0 |= value;
    }
    /// SSC modulation start position on synchronization trigger
    #[inline(always)]
    pub fn ssc_sync_pos(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    #[inline(always)]
    pub fn set_ssc_sync_pos(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        let value = value << 3;
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// Frequency select of integrator 2 replica used for lane sync.
    #[inline(always)]
    pub fn sync_ctrl_fsel(&self) -> u32 {
        (self.0 & 0x3f0000) >> 16
    }
    #[inline(always)]
    pub fn set_sync_ctrl_fsel(&mut self, value: u32) {
        debug_assert!(value <= 0x3f);
        let value = value << 16;
        self.0 &= !0x3f0000;
        self.0 |= value;
    }
    /// Sticky bit that indicates a sync control protocol error.
    #[inline(always)]
    pub fn sync_ctrl_protocol_err(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    #[inline(always)]
    pub fn set_sync_ctrl_protocol_err(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 31;
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Clear for synth_sc_protocol_err sticky bit. Rising edge causes the clearing and a concurrent error event has higher priority.
    #[inline(always)]
    pub fn sync_ctrl_protocol_err_clr(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    #[inline(always)]
    pub fn set_sync_ctrl_protocol_err_clr(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 30;
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    /// Controls integrator 2 replica behavior: '0': wrapping; '1': saturating.
    #[inline(always)]
    pub fn sync_ctrl_wrap_inhibit(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    #[inline(always)]
    pub fn set_sync_ctrl_wrap_inhibit(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
}
/// SD10G65 TX Synthesizer Configuration register 0
///
/// Configuration register 0 for SD10G65 TX SYNTH.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_SYNTH_CFG0(u32);
impl SD10G65_TX_SYNTH_CFG0 {
    /// Synthesizer BIAS adjust in steps of ~3%, 0: 100%, 7: 121%
    #[inline(always)]
    pub fn synth_bias_adjust(&self) -> u32 {
        (self.0 & 0x3800000) >> 23
    }
    #[inline(always)]
    pub fn set_synth_bias_adjust(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 23;
        self.0 &= !0x3800000;
        self.0 |= value;
    }
    /// enables CML2CMOS converter
    #[inline(always)]
    pub fn synth_conv_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_synth_conv_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// comon sync speed
    #[inline(always)]
    pub fn synth_cs_speed(&self) -> u32 {
        (self.0 & 0x3800) >> 11
    }
    #[inline(always)]
    pub fn set_synth_cs_speed(&mut self, value: u32) {
        debug_assert!(value <= 0x7);
        let value = value << 11;
        self.0 &= !0x3800;
        self.0 |= value;
    }
    /// dig. sync direction
    #[inline(always)]
    pub fn synth_ds_dir(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_synth_ds_dir(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// dig. sync enable
    #[inline(always)]
    pub fn synth_ds_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_synth_ds_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// dig. sync speed
    #[inline(always)]
    pub fn synth_ds_speed(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_synth_ds_speed(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// synthesizer enable
    #[inline(always)]
    pub fn synth_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_synth_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// enable sync unit
    #[inline(always)]
    pub fn synth_ena_sync_unit(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_synth_ena_sync_unit(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// selects feedback divider setting
    #[inline(always)]
    pub fn synth_fbdiv_sel(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    #[inline(always)]
    pub fn set_synth_fbdiv_sel(&mut self, value: u32) {
        debug_assert!(value <= 0x3);
        let value = value << 16;
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// half rate enable
    #[inline(always)]
    pub fn synth_hrate_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_synth_hrate_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// lane sync direction
    #[inline(always)]
    pub fn synth_ls_dir(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_synth_ls_dir(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// lane sync enable
    #[inline(always)]
    pub fn synth_ls_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_synth_ls_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// lane sync speed
    #[inline(always)]
    pub fn synth_ls_speed(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_synth_ls_speed(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Enable for different offset compensation stages
    #[inline(always)]
    pub fn synth_off_comp_ena(&self) -> u32 {
        (self.0 & 0x7c0000) >> 18
    }
    #[inline(always)]
    pub fn set_synth_off_comp_ena(&mut self, value: u32) {
        debug_assert!(value <= 0x1f);
        let value = value << 18;
        self.0 &= !0x7c0000;
        self.0 |= value;
    }
    /// reg. pool for late changes/fixes.
    #[inline(always)]
    pub fn synth_spare_pool(&self) -> u32 {
        (self.0 & 0x3c000000) >> 26
    }
    #[inline(always)]
    pub fn set_synth_spare_pool(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        let value = value << 26;
        self.0 &= !0x3c000000;
        self.0 |= value;
    }
    /// Selects circuit speed. Coding: 0 for settings with synth_fbdiv_sel = 2; 1 for setting with synth_fbdiv_sel smaller than 2.
    #[inline(always)]
    pub fn synth_speed_sel(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_synth_speed_sel(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
}
/// SD10G65 TX Synthesizer Configuration register 1
///
/// Configuration register 1 for SD10G65 TX SYNTH.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_SYNTH_CFG1(u32);
impl SD10G65_TX_SYNTH_CFG1 {
    /// frequency m setting bits 35:32
    #[inline(always)]
    pub fn synth_freqm_1(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    #[inline(always)]
    pub fn set_synth_freqm_1(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        let value = value << 4;
        self.0 &= !0xf0;
        self.0 |= value;
    }
    /// frequency n setting bits 35:32
    #[inline(always)]
    pub fn synth_freqn_1(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_synth_freqn_1(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// frequency multiplier
    #[inline(always)]
    pub fn synth_freq_mult(&self) -> u32 {
        (self.0 & 0x3fff00) >> 8
    }
    #[inline(always)]
    pub fn set_synth_freq_mult(&mut self, value: u32) {
        debug_assert!(value <= 0x3fff);
        let value = value << 8;
        self.0 &= !0x3fff00;
        self.0 |= value;
    }
    /// frequency multiplier decoder bypass
    #[inline(always)]
    pub fn synth_freq_mult_byp(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    #[inline(always)]
    pub fn set_synth_freq_mult_byp(&mut self, value: u32) {
        debug_assert!(value <= 0x1);
        let value = value << 26;
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    /// frequency multiplier MSBits in bypass mode
    #[inline(always)]
    pub fn synth_freq_mult_hi(&self) -> u32 {
        (self.0 & 0x3c00000) >> 22
    }
    #[inline(always)]
    pub fn set_synth_freq_mult_hi(&mut self, value: u32) {
        debug_assert!(value <= 0xf);
        let value = value << 22;
        self.0 &= !0x3c00000;
        self.0 |= value;
    }
}
/// SD10G65 TX Synthesizer Configuration register 3
///
/// Configuration register 3 for SD10G65 TX SYNTH.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_SYNTH_CFG3(u32);
impl SD10G65_TX_SYNTH_CFG3 {
    /// frequency m setting bits 31:0
    #[inline(always)]
    pub fn synth_freqm_0(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_synth_freqm_0(&mut self, value: u32) {
        self.0 = value;
    }
}
/// SD10G65 TX Synthesizer Configuration register 4
///
/// Configuration register 4 for SD10G65 TX SYNTH.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_SYNTH_CFG4(u32);
impl SD10G65_TX_SYNTH_CFG4 {
    /// frequency n setting bits 31:0
    #[inline(always)]
    pub fn synth_freqn_0(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_synth_freqn_0(&mut self, value: u32) {
        self.0 = value;
    }
}
