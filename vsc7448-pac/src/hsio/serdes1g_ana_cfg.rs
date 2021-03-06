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
/// SERDES1G Common Cfg
///
/// Configuration register for common SERDES1G functions Note: When enabling the facility loop (ena_floop) also the phase alignment in the serializer has to be enabled and configured adequate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SERDES1G_COMMON_CFG(u32);
impl SERDES1G_COMMON_CFG {
    /// Enable direct line
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ena_direct(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_ena_direct(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Enable equipment loop
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ena_eloop(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_ena_eloop(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Enable facility loop
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ena_floop(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_ena_floop(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Enable input loop
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ena_iloop(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_ena_iloop(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Enable lane
    ///
    /// 0: Disable lane 1: Enable line
    #[inline(always)]
    pub fn ena_lane(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_ena_lane(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Enable pad loop
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ena_ploop(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_ena_ploop(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Enable half rate
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn hrate(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_hrate(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Interface mode
    ///
    /// 0: 8-bit mode 1: 10-bit mode
    #[inline(always)]
    pub fn if_mode(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_if_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Hidden spare bits (not connected internally yet)
    #[inline(always)]
    pub fn lane_ctrl(&self) -> u32 {
        (self.0 & 0xe000) >> 13
    }
    #[inline(always)]
    pub fn set_lane_ctrl(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 13;
        self.0 &= !0xe000;
        self.0 |= value;
    }
    /// Power-down RX-path
    ///
    /// 0: Normal mode 1: Power down mode
    #[inline(always)]
    pub fn pwd_rx(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline(always)]
    pub fn set_pwd_rx(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Power-down TX-path
    ///
    /// 0: Normal mode 1: Power down mode
    #[inline(always)]
    pub fn pwd_tx(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_pwd_tx(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Enable auto-squelching for sync. ethernet clock output: when set the clock output will stop toggling (keep its last value constantly) when PCS looses link synchrony.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn se_auto_squelch_ena(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline(always)]
    pub fn set_se_auto_squelch_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// System reset (low active)
    ///
    /// 0: Apply reset (not self-clearing) 1: Reset released
    #[inline(always)]
    pub fn sys_rst(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    #[inline(always)]
    pub fn set_sys_rst(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 31;
        self.0 &= !0x80000000;
        self.0 |= value;
    }
}
/// SERDES1G Deserializer Cfg
///
/// Configuration register for SERDES1G deserializer
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SERDES1G_DES_CFG(u32);
impl SERDES1G_DES_CFG {
    /// Bandwidth selection for proportional path of CDR loop.
    ///
    /// 0: No division 1: Divide by 2 2: Divide by 4 3: Divide by 8 4: Divide by 16 5: Divide by 32 6: Divide by 64 7: Divide by 128
    #[inline(always)]
    pub fn des_bw_ana(&self) -> u32 {
        (self.0 & 0xe0) >> 5
    }
    #[inline(always)]
    pub fn set_des_bw_ana(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 5;
        self.0 &= !0xe0;
        self.0 |= value;
    }
    /// Selection of time constant for integrative path of CDR loop.
    ///
    /// 0: Divide by 2 1: Divide by 4 2: Divide by 8 3: Divide by 16 4: Divide by 32 5: Divide by 64 6: Divide by 128 7: Divide by 256
    #[inline(always)]
    pub fn des_bw_hyst(&self) -> u32 {
        (self.0 & 0xe) >> 1
    }
    #[inline(always)]
    pub fn set_des_bw_hyst(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 1;
        self.0 &= !0xe;
        self.0 |= value;
    }
    /// Deserializer phase control, main CP/MD select
    ///
    /// 0: Directly from DES 1: Through hysteresis stage from DES 2: From core 3: Disabled
    #[inline(always)]
    pub fn des_cpmd_sel(&self) -> u32 {
        (self.0 & 0x1800) >> 11
    }
    #[inline(always)]
    pub fn set_des_cpmd_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 11;
        self.0 &= !0x1800;
        self.0 |= value;
    }
    /// Des phase control for 180 degrees deadlock block mode of operation
    ///
    /// 0: Depending on density of input pattern 1: Active until PCS has synchronized 2: Depending on density of input pattern until PCS has synchronized 3: Never 4: Always 5-7: Reserved
    #[inline(always)]
    pub fn des_mbtr_ctrl(&self) -> u32 {
        (self.0 & 0x700) >> 8
    }
    #[inline(always)]
    pub fn set_des_mbtr_ctrl(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 8;
        self.0 &= !0x700;
        self.0 |= value;
    }
    /// Control of phase regulator logic. Bit 3 must be set to 0.
    ///
    /// 0: Disabled 1: Enabled with 99 ppm limit 2: Enabled with 202 ppm limit 3: Enabled with 485 ppm limit 4: Enabled if corresponding PCS is in sync with 50 ppm limit 5: Enabled if corresponding PCS is in sync with 99 ppm limit 6: Enabled if corresponding PCS is in sync with 202 ppm limit 7: Enabled if corresponding PCS is in sync with 485 ppm limit
    #[inline(always)]
    pub fn des_phs_ctrl(&self) -> u32 {
        (self.0 & 0x1e000) >> 13
    }
    #[inline(always)]
    pub fn set_des_phs_ctrl(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 13;
        self.0 &= !0x1e000;
        self.0 |= value;
    }
    /// Swap non-hysteresis CP/MD signals.
    ///
    /// 0: No swapping 1: Swapping
    #[inline(always)]
    pub fn des_swap_ana(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_des_swap_ana(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Swap hysteresis CP/MD signals.
    ///
    /// 0: No swapping 1: Swapping
    #[inline(always)]
    pub fn des_swap_hyst(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_des_swap_hyst(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// SERDES1G Input Buffer Cfg
///
/// Configuration register for SERDES1G input buffer
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SERDES1G_IB_CFG(u32);
impl SERDES1G_IB_CFG {
    /// Hysteresis level for AC-JTAG Input
    ///
    /// 0: low 7: high
    #[inline(always)]
    pub fn acjtag_hyst(&self) -> u32 {
        (self.0 & 0x7000000) >> 24
    }
    #[inline(always)]
    pub fn set_acjtag_hyst(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 24;
        self.0 &= !0x7000000;
        self.0 |= value;
    }
    /// Detect thresholds:
    ///
    /// 0: 159-189mVppd 1: 138-164mVppd 2: 109-124mVppd 3: 74-89mVppd
    #[inline(always)]
    pub fn ib_det_lev(&self) -> u32 {
        (self.0 & 0x380000) >> 19
    }
    #[inline(always)]
    pub fn set_ib_det_lev(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 19;
        self.0 &= !0x380000;
        self.0 |= value;
    }
    /// Enable common mode voltage termination
    ///
    /// 0: Low termination (VDD_A x 0.7) 1: High termination (VDD_A)
    #[inline(always)]
    pub fn ib_ena_cmv_term(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    #[inline(always)]
    pub fn set_ib_ena_cmv_term(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Enable dc-coupling of input signal
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ib_ena_dc_coupling(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_ib_ena_dc_coupling(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Enable detect level circuit
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ib_ena_detlev(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_ib_ena_detlev(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Enable hysteresis for input signal. Hystesis can only be enabled if DC offset compensation is disabled.
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ib_ena_hyst(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_ib_ena_hyst(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Enable offset compensation of input stage. This bit must be disabled to enable hysteresis (IB_ENA_HYST).
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn ib_ena_offset_comp(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_ib_ena_offset_comp(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Selects weighting between AC and DC input path:
    ///
    /// 0: Reserved 1: Reserved 2: 0dB (recommended value) 3: 1.5dB 4: 3dB 5: 6dB 6: 9dB 12.5dB
    #[inline(always)]
    pub fn ib_eq_gain(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    #[inline(always)]
    pub fn set_ib_eq_gain(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 6;
        self.0 &= !0x1c0;
        self.0 |= value;
    }
    /// Switches signal detect circuit into low frequency mode, must be used in FX100 mode
    #[inline(always)]
    pub fn ib_fx100_ena(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    #[inline(always)]
    pub fn set_ib_fx100_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 27;
        self.0 &= !0x8000000;
        self.0 |= value;
    }
    /// Input buffer hysteresis levels:
    ///
    /// 0: 59-79mV 1: 81-124mV
    #[inline(always)]
    pub fn ib_hyst_lev(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    #[inline(always)]
    pub fn set_ib_hyst_lev(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 14;
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Resistor control. Value must be taken from RCOMP_STATUS.RCOMP. (default: -3)
    #[inline(always)]
    pub fn ib_resistor_ctrl(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_ib_resistor_ctrl(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Corner frequencies of AC path:
    ///
    /// 0: 1.3GHz 1: 1.5GHz 2: 1.6GHz 3: 1.8GHz
    #[inline(always)]
    pub fn ib_sel_corner_freq(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    #[inline(always)]
    pub fn set_ib_sel_corner_freq(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 4;
        self.0 &= !0x30;
        self.0 |= value;
    }
}
/// SERDES1G Output Buffer Cfg
///
/// Configuration register for SERDES1G output buffer
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SERDES1G_OB_CFG(u32);
impl SERDES1G_OB_CFG {
    /// Amplitude control in steps of 50mVppd.
    ///
    /// 0: 0.4Vppd 15: 1.1Vppd
    #[inline(always)]
    pub fn ob_amp_ctrl(&self) -> u32 {
        (self.0 & 0x1e000) >> 13
    }
    #[inline(always)]
    pub fn set_ob_amp_ctrl(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 13;
        self.0 &= !0x1e000;
        self.0 |= value;
    }
    /// CMM bias control
    #[inline(always)]
    pub fn ob_cmm_bias_ctrl(&self) -> u32 {
        (self.0 & 0x1c00) >> 10
    }
    #[inline(always)]
    pub fn set_ob_cmm_bias_ctrl(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 10;
        self.0 &= !0x1c00;
        self.0 |= value;
    }
    /// Disable VCM control
    #[inline(always)]
    pub fn ob_dis_vcm_ctrl(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_ob_dis_vcm_ctrl(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Enable measure vreg
    #[inline(always)]
    pub fn ob_en_meas_vreg(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_ob_en_meas_vreg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Resistor control. Value must be taken from RCOMP_STATUS.RCOMP. (default: +1)
    #[inline(always)]
    pub fn ob_resistor_ctrl(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_ob_resistor_ctrl(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Slope / slew rate control:
    ///
    /// 0: 45ps 1: 85ps 2: 105ps 3: 115ps
    #[inline(always)]
    pub fn ob_slp(&self) -> u32 {
        (self.0 & 0x60000) >> 17
    }
    #[inline(always)]
    pub fn set_ob_slp(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 17;
        self.0 &= !0x60000;
        self.0 |= value;
    }
    /// Common mode voltage control:
    ///
    /// 0: Reserved 1: 440mV 2: 480mV 3: 460mV 4: 530mV 5: 500mV 6: 570mV 7: 550mV
    #[inline(always)]
    pub fn ob_vcm_ctrl(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    #[inline(always)]
    pub fn set_ob_vcm_ctrl(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 4;
        self.0 &= !0xf0;
        self.0 |= value;
    }
}
/// SERDES1G Pll Cfg
///
/// Configuration register for SERDES1G RCPLL
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SERDES1G_PLL_CFG(u32);
impl SERDES1G_PLL_CFG {
    /// Enable feedback divider (divide by two)
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn pll_ena_fb_div2(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    #[inline(always)]
    pub fn set_pll_ena_fb_div2(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Enable reference clock divider (divide by two)
    ///
    /// 0: Disable 1: Enable
    #[inline(always)]
    pub fn pll_ena_rc_div2(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline(always)]
    pub fn set_pll_ena_rc_div2(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// Control data for FSM
    #[inline(always)]
    pub fn pll_fsm_ctrl_data(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline(always)]
    pub fn set_pll_fsm_ctrl_data(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// Enable FSM
    #[inline(always)]
    pub fn pll_fsm_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_pll_fsm_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Enable FSM forcing
    #[inline(always)]
    pub fn pll_fsm_force_set_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_pll_fsm_force_set_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Enable FSM recalibration
    #[inline(always)]
    pub fn pll_fsm_oor_recal_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_pll_fsm_oor_recal_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Select RB data
    #[inline(always)]
    pub fn pll_rb_data_sel(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_pll_rb_data_sel(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// SERDES1G Serializer Cfg
///
/// Configuration register for SERDES1G serializer
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SERDES1G_SER_CFG(u32);
impl SERDES1G_SER_CFG {
    /// Select reference clock source for phase alignment
    ///
    /// 0: RXCLKP 1: RefClk15MHz 2: RXCLKN 3: ext. ALICLK
    #[inline(always)]
    pub fn ser_alisel(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    #[inline(always)]
    pub fn set_ser_alisel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 4;
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// Use wider window for phase alignment
    ///
    /// 0: Use small window for low jitter (100 to 200ps) 1: Use wide window for higher jitter (150 to 300 ps)
    #[inline(always)]
    pub fn ser_big_win(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_ser_big_win(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Select source of CP/MD signals
    ///
    /// 0: Phase alignment block 1: Core
    #[inline(always)]
    pub fn ser_cpmd_sel(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_ser_cpmd_sel(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Invert and delays (one clk cycle) output D1 for de-emphasis of OB
    ///
    /// 0: Non-inverting and non-delaying 1: Inverting and delaying
    #[inline(always)]
    pub fn ser_deemph(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_ser_deemph(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Enable phase alignment
    ///
    /// 0: Disable phase alignment 1: Enable phase alignment
    #[inline(always)]
    pub fn ser_enali(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_ser_enali(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable hysteresis for phase alignment
    ///
    /// 0: Disable hysteresis 1: Enable hysteresis
    #[inline(always)]
    pub fn ser_enhys(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_ser_enhys(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Enable window for phase alignment
    ///
    /// 0: Disable window 1: Enable window
    #[inline(always)]
    pub fn ser_en_win(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_ser_en_win(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Invert output D0b for idle-mode of OB
    ///
    /// 0: Non-inverting 1. Inverting
    #[inline(always)]
    pub fn ser_idle(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_ser_idle(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Swap CP/MD signals of phase alignment circuit
    ///
    /// 0: Disable swapping 1: Enable swapping
    #[inline(always)]
    pub fn ser_swap_cpmd(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_ser_swap_cpmd(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
}
