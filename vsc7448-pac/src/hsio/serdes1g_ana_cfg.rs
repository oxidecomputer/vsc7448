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
#[derive(From, Into)]
pub struct SERDES1G_COMMON_CFG(u32);
impl SERDES1G_COMMON_CFG {
    /// Enable direct line

    ///

    /// 0: Disable 1: Enable
    pub fn ena_direct(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_ena_direct(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Enable equipment loop

    ///

    /// 0: Disable 1: Enable
    pub fn ena_eloop(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_ena_eloop(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Enable facility loop

    ///

    /// 0: Disable 1: Enable
    pub fn ena_floop(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_ena_floop(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Enable input loop

    ///

    /// 0: Disable 1: Enable
    pub fn ena_iloop(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_ena_iloop(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Enable lane

    ///

    /// 0: Disable lane 1: Enable line
    pub fn ena_lane(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_ena_lane(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Enable pad loop

    ///

    /// 0: Disable 1: Enable
    pub fn ena_ploop(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_ena_ploop(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Enable half rate

    ///

    /// 0: Disable 1: Enable
    pub fn hrate(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_hrate(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Interface mode

    ///

    /// 0: 8-bit mode 1: 10-bit mode
    pub fn if_mode(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_if_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Hidden spare bits (not connected internally yet)
    pub fn lane_ctrl(&self) -> u32 {
        (self.0 & 0xe000) >> 13
    }
    pub fn set_lane_ctrl(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0xe000);
        self.0 &= !0xe000;
        self.0 |= value;
    }
    /// Power-down RX-path

    ///

    /// 0: Normal mode 1: Power down mode
    pub fn pwd_rx(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_pwd_rx(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Power-down TX-path

    ///

    /// 0: Normal mode 1: Power down mode
    pub fn pwd_tx(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_pwd_tx(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Enable auto-squelching for sync. ethernet clock output: when set the clock output will stop toggling (keep its last value constantly) when PCS looses link synchrony.

    ///

    /// 0: Disable 1: Enable
    pub fn se_auto_squelch_ena(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_se_auto_squelch_ena(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// System reset (low active)

    ///

    /// 0: Apply reset (not self-clearing) 1: Reset released
    pub fn sys_rst(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_sys_rst(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
}
/// SERDES1G Deserializer Cfg
///
/// Configuration register for SERDES1G deserializer
#[derive(From, Into)]
pub struct SERDES1G_DES_CFG(u32);
impl SERDES1G_DES_CFG {
    /// Bandwidth selection for proportional path of CDR loop.

    ///

    /// 0: No division 1: Divide by 2 2: Divide by 4 3: Divide by 8 4: Divide by 16 5: Divide by 32 6: Divide by 64 7: Divide by 128
    pub fn des_bw_ana(&self) -> u32 {
        (self.0 & 0xe0) >> 5
    }
    pub fn set_des_bw_ana(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0xe0);
        self.0 &= !0xe0;
        self.0 |= value;
    }
    /// Selection of time constant for integrative path of CDR loop.

    ///

    /// 0: Divide by 2 1: Divide by 4 2: Divide by 8 3: Divide by 16 4: Divide by 32 5: Divide by 64 6: Divide by 128 7: Divide by 256
    pub fn des_bw_hyst(&self) -> u32 {
        (self.0 & 0xe) >> 1
    }
    pub fn set_des_bw_hyst(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0xe);
        self.0 &= !0xe;
        self.0 |= value;
    }
    /// Deserializer phase control, main CP/MD select

    ///

    /// 0: Directly from DES 1: Through hysteresis stage from DES 2: From core 3: Disabled
    pub fn des_cpmd_sel(&self) -> u32 {
        (self.0 & 0x1800) >> 11
    }
    pub fn set_des_cpmd_sel(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x1800);
        self.0 &= !0x1800;
        self.0 |= value;
    }
    /// Des phase control for 180 degrees deadlock block mode of operation

    ///

    /// 0: Depending on density of input pattern 1: Active until PCS has synchronized 2: Depending on density of input pattern until PCS has synchronized 3: Never 4: Always 5-7: Reserved
    pub fn des_mbtr_ctrl(&self) -> u32 {
        (self.0 & 0x700) >> 8
    }
    pub fn set_des_mbtr_ctrl(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x700);
        self.0 &= !0x700;
        self.0 |= value;
    }
    /// Control of phase regulator logic. Bit 3 must be set to 0.

    ///

    /// 0: Disabled 1: Enabled with 99 ppm limit 2: Enabled with 202 ppm limit 3: Enabled with 485 ppm limit 4: Enabled if corresponding PCS is in sync with 50 ppm limit 5: Enabled if corresponding PCS is in sync with 99 ppm limit 6: Enabled if corresponding PCS is in sync with 202 ppm limit 7: Enabled if corresponding PCS is in sync with 485 ppm limit
    pub fn des_phs_ctrl(&self) -> u32 {
        (self.0 & 0x1e000) >> 13
    }
    pub fn set_des_phs_ctrl(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x1e000);
        self.0 &= !0x1e000;
        self.0 |= value;
    }
    /// Swap non-hysteresis CP/MD signals.

    ///

    /// 0: No swapping 1: Swapping
    pub fn des_swap_ana(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_des_swap_ana(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Swap hysteresis CP/MD signals.

    ///

    /// 0: No swapping 1: Swapping
    pub fn des_swap_hyst(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_des_swap_hyst(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// SERDES1G Input Buffer Cfg
///
/// Configuration register for SERDES1G input buffer
#[derive(From, Into)]
pub struct SERDES1G_IB_CFG(u32);
impl SERDES1G_IB_CFG {
    /// Hysteresis level for AC-JTAG Input

    ///

    /// 0: low 7: high
    pub fn acjtag_hyst(&self) -> u32 {
        (self.0 & 0x7000000) >> 24
    }
    pub fn set_acjtag_hyst(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x7000000);
        self.0 &= !0x7000000;
        self.0 |= value;
    }
    /// Detect thresholds:

    ///

    /// 0: 159-189mVppd 1: 138-164mVppd 2: 109-124mVppd 3: 74-89mVppd
    pub fn ib_det_lev(&self) -> u32 {
        (self.0 & 0x380000) >> 19
    }
    pub fn set_ib_det_lev(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x380000);
        self.0 &= !0x380000;
        self.0 |= value;
    }
    /// Enable common mode voltage termination

    ///

    /// 0: Low termination (VDD_A x 0.7) 1: High termination (VDD_A)
    pub fn ib_ena_cmv_term(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_ib_ena_cmv_term(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Enable dc-coupling of input signal

    ///

    /// 0: Disable 1: Enable
    pub fn ib_ena_dc_coupling(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_ib_ena_dc_coupling(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Enable detect level circuit

    ///

    /// 0: Disable 1: Enable
    pub fn ib_ena_detlev(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_ib_ena_detlev(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Enable hysteresis for input signal. Hystesis can only be enabled if DC offset compensation is disabled.

    ///

    /// 0: Disable 1: Enable
    pub fn ib_ena_hyst(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_ib_ena_hyst(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Enable offset compensation of input stage. This bit must be disabled to enable hysteresis (IB_ENA_HYST).

    ///

    /// 0: Disable 1: Enable
    pub fn ib_ena_offset_comp(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_ib_ena_offset_comp(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Selects weighting between AC and DC input path:

    ///

    /// 0: Reserved 1: Reserved 2: 0dB (recommended value) 3: 1.5dB 4: 3dB 5: 6dB 6: 9dB 12.5dB
    pub fn ib_eq_gain(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    pub fn set_ib_eq_gain(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x1c0);
        self.0 &= !0x1c0;
        self.0 |= value;
    }
    /// Switches signal detect circuit into low frequency mode, must be used in FX100 mode
    pub fn ib_fx100_ena(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    pub fn set_ib_fx100_ena(&mut self, value: u32) {
        let value = value << 27;
        assert!(value <= 0x8000000);
        self.0 &= !0x8000000;
        self.0 |= value;
    }
    /// Input buffer hysteresis levels:

    ///

    /// 0: 59-79mV 1: 81-124mV
    pub fn ib_hyst_lev(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_ib_hyst_lev(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Resistor control. Value must be taken from RCOMP_STATUS.RCOMP. (default: -3)
    pub fn ib_resistor_ctrl(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_ib_resistor_ctrl(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Corner frequencies of AC path:

    ///

    /// 0: 1.3GHz 1: 1.5GHz 2: 1.6GHz 3: 1.8GHz
    pub fn ib_sel_corner_freq(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_ib_sel_corner_freq(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }
}
/// SERDES1G Output Buffer Cfg
///
/// Configuration register for SERDES1G output buffer
#[derive(From, Into)]
pub struct SERDES1G_OB_CFG(u32);
impl SERDES1G_OB_CFG {
    /// Amplitude control in steps of 50mVppd.

    ///

    /// 0: 0.4Vppd 15: 1.1Vppd
    pub fn ob_amp_ctrl(&self) -> u32 {
        (self.0 & 0x1e000) >> 13
    }
    pub fn set_ob_amp_ctrl(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x1e000);
        self.0 &= !0x1e000;
        self.0 |= value;
    }
    /// CMM bias control
    pub fn ob_cmm_bias_ctrl(&self) -> u32 {
        (self.0 & 0x1c00) >> 10
    }
    pub fn set_ob_cmm_bias_ctrl(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x1c00);
        self.0 &= !0x1c00;
        self.0 |= value;
    }
    /// Disable VCM control
    pub fn ob_dis_vcm_ctrl(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_ob_dis_vcm_ctrl(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Enable measure vreg
    pub fn ob_en_meas_vreg(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_ob_en_meas_vreg(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Resistor control. Value must be taken from RCOMP_STATUS.RCOMP. (default: +1)
    pub fn ob_resistor_ctrl(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_ob_resistor_ctrl(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Slope / slew rate control:

    ///

    /// 0: 45ps 1: 85ps 2: 105ps 3: 115ps
    pub fn ob_slp(&self) -> u32 {
        (self.0 & 0x60000) >> 17
    }
    pub fn set_ob_slp(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x60000);
        self.0 &= !0x60000;
        self.0 |= value;
    }
    /// Common mode voltage control:

    ///

    /// 0: Reserved 1: 440mV 2: 480mV 3: 460mV 4: 530mV 5: 500mV 6: 570mV 7: 550mV
    pub fn ob_vcm_ctrl(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    pub fn set_ob_vcm_ctrl(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0xf0);
        self.0 &= !0xf0;
        self.0 |= value;
    }
}
/// SERDES1G Serializer Cfg
///
/// Configuration register for SERDES1G serializer
#[derive(From, Into)]
pub struct SERDES1G_SER_CFG(u32);
impl SERDES1G_SER_CFG {
    /// Select reference clock source for phase alignment

    ///

    /// 0: RXCLKP 1: RefClk15MHz 2: RXCLKN 3: ext. ALICLK
    pub fn ser_alisel(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_ser_alisel(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// Use wider window for phase alignment

    ///

    /// 0: Use small window for low jitter (100 to 200ps) 1: Use wide window for higher jitter (150 to 300 ps)
    pub fn ser_big_win(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_ser_big_win(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Select source of CP/MD signals

    ///

    /// 0: Phase alignment block 1: Core
    pub fn ser_cpmd_sel(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_ser_cpmd_sel(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Invert and delays (one clk cycle) output D1 for de-emphasis of OB

    ///

    /// 0: Non-inverting and non-delaying 1: Inverting and delaying
    pub fn ser_deemph(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_ser_deemph(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Enable phase alignment

    ///

    /// 0: Disable phase alignment 1: Enable phase alignment
    pub fn ser_enali(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ser_enali(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable hysteresis for phase alignment

    ///

    /// 0: Disable hysteresis 1: Enable hysteresis
    pub fn ser_enhys(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_ser_enhys(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Enable window for phase alignment

    ///

    /// 0: Disable window 1: Enable window
    pub fn ser_en_win(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ser_en_win(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Invert output D0b for idle-mode of OB

    ///

    /// 0: Non-inverting 1. Inverting
    pub fn ser_idle(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_ser_idle(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Swap CP/MD signals of phase alignment circuit

    ///

    /// 0: Disable swapping 1: Enable swapping
    pub fn ser_swap_cpmd(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_ser_swap_cpmd(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
}
#[derive(From, Into)]
pub struct SYNC_ETH_SD10G_CFG(u32);
impl SYNC_ETH_SD10G_CFG {
    /// Set to enable auto-squelching for sync. ethernet clock output: when set the clock output will stop toggling (keep its last value constantly) when PCS looses link synchrony.
    pub fn sd10g_auto_squelch_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_sd10g_auto_squelch_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Divider setting for the SD10G recovered clock output. These settings are applied prior to sending recovered clock to the optional PAD-divder (see HSIO::SYNC_ETH_CFG.SEL_RECO_CLK_DIV.)

    ///

    /// 0: No clock dividing 1: Divide clock by 2 2: Divide clock by (66/32) 3: reserved
    pub fn sd10g_reco_clk_div(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_sd10g_reco_clk_div(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x6);
        self.0 &= !0x6;
        self.0 |= value;
    }
}
