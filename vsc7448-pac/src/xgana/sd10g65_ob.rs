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

/// Register `SD10G65_MOEBDIV_CFG0`
///
/// SD10G65 MOEBDIV Configuration register 0
///
/// Configuration register 0 for SD10G65 MoebiusDivider
#[derive(From, Into)]
pub struct SD10G65_MOEBDIV_CFG0(u32);
impl SD10G65_MOEBDIV_CFG0 {    ///
    /// Bandwidth selection for cp/md of cdr loop when core NOT flags valid data detected
    pub fn moebdiv_bw_cdr_sel_a(&self) -> u32 {
        (self.0 & 0xe00) >> 9
    }
    pub fn set_moebdiv_bw_cdr_sel_a(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0xe00);
        self.0 &= !0xe00;
        self.0 |= value;
    }    ///
    /// Bandwidth selection for cp/md of cdr loop when core flags valid data detected
    pub fn moebdiv_bw_cdr_sel_b(&self) -> u32 {
        (self.0 & 0x1c0) >> 6
    }
    pub fn set_moebdiv_bw_cdr_sel_b(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x1c0);
        self.0 &= !0x1c0;
        self.0 |= value;
    }    ///
    /// Bandwidth selection for cp/md signals towards core
    pub fn moebdiv_bw_core_sel(&self) -> u32 {
        (self.0 & 0x38) >> 3
    }
    pub fn set_moebdiv_bw_core_sel(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x38);
        self.0 &= !0x38;
        self.0 |= value;
    }    ///
    /// CP/MD swapping
    pub fn moebdiv_cpmd_swap(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_moebdiv_cpmd_swap(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Divider disable
    pub fn moebdiv_dis(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_moebdiv_dis(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// MD divider enable
    pub fn moebdiv_div32_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_moebdiv_div32_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}

/// Register `SD10G65_OB_CFG0`
///
/// SD10G65 OB Configuration register 0
///
/// Configuration register 0 for SD10G65 OB.
#[derive(From, Into)]
pub struct SD10G65_OB_CFG0(u32);
impl SD10G65_OB_CFG0 {    ///
    /// Bypass data path (e.g. for JTAG), allows to drive output when EN_DIRECT=1 and EN_OB=1
    pub fn byp_d(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_byp_d(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }    ///
    /// Control of common mode voltage of clock buffer between synthesizer and OB.
    pub fn clk_buf_cmv(&self) -> u32 {
        (self.0 & 0x600000) >> 21
    }
    pub fn set_clk_buf_cmv(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x600000);
        self.0 &= !0x600000;
        self.0 |= value;
    }    ///
    /// Enable direct path
    pub fn en_direct(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_en_direct(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }    ///
    /// Enable input loop
    pub fn en_inp_loop(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_en_inp_loop(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }    ///
    /// Enable output buffer and serializer
    pub fn en_ob(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_en_ob(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }    ///
    /// Enable pad loop
    pub fn en_pad_loop(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_en_pad_loop(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }    ///
    /// Selects amplitude range controled via levn. See description of levn.
    pub fn incr_levn(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_incr_levn(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }    ///
    /// Amplitude control value. Step size is 25 mVpp, decreasing amplitude with increasing control value. Range depends on incr_levn. Coding for incr_levn=0: 31: 500mVpp, 30: 525mVpp, 29: 550mVpp, ..., 0: 1275mVpp. Coding for incr_levn=1: 31: 300mVpp, 30: 325mVpp, 29: 350mVpp, .., 0: 1075mVpp. (Note: maximum achievable amplitude depends on the supply voltage)
    pub fn levn(&self) -> u32 {
        (self.0 & 0x1f) >> 0
    }
    pub fn set_levn(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }    ///
    /// Pool of spare bits for use in late design changes.
    pub fn ob_spare_pool(&self) -> u32 {
        (self.0 & 0x180000) >> 19
    }
    pub fn set_ob_spare_pool(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x180000);
        self.0 &= !0x180000;
        self.0 |= value;
    }    ///
    /// Set digital part into pseudo reset
    pub fn rst(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rst(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }    ///
    /// Interface width
    ///
    /// 0: 8 1: 10 2: 16 3: 20 4: 32 5: 40 6-7: Reserved
    pub fn sel_ifw(&self) -> u32 {
        (self.0 & 0xe0) >> 5
    }
    pub fn set_sel_ifw(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0xe0);
        self.0 &= !0xe0;
        self.0 |= value;
    }    ///
    /// Invert input to serializer
    pub fn ser_inv(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_ser_inv(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }
}

/// Register `SD10G65_OB_CFG1`
///
/// SD10G65 OB Configuration register 1
///
/// Configuration register 1 for SD10G65 OB.
#[derive(From, Into)]
pub struct SD10G65_OB_CFG1(u32);
impl SD10G65_OB_CFG1 {    ///
    /// Enable amplitude compensation of AB bleed current
    pub fn ab_comp_en(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    pub fn set_ab_comp_en(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0x4000000);
        self.0 &= !0x4000000;
        self.0 |= value;
    }    ///
    /// Bleed current for class AB operation of driver
    ///
    /// 0: 1% 1: 0.5% 2: 2% 3: reserved
    pub fn diode_cur(&self) -> u32 {
        (self.0 & 0x3800000) >> 23
    }
    pub fn set_diode_cur(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x3800000);
        self.0 &= !0x3800000;
        self.0 |= value;
    }    ///
    /// Level shift ctrl of class AB bias generator
    ///
    /// 0: 50mV 1: 100mV 2:150mV 3: 200mV
    pub fn lev_shft(&self) -> u32 {
        (self.0 & 0x600000) >> 21
    }
    pub fn set_lev_shft(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x600000);
        self.0 &= !0x600000;
        self.0 |= value;
    }    ///
    /// Slew rate ctrl of OB (C)
    ///
    /// C=3 R=3: 25ps C=3 R=0: 35ps C=0 R=3: 55ps C=1 R=0: 70ps C=0 R=0: 120 ps
    pub fn predrv_c_ctrl(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    pub fn set_predrv_c_ctrl(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x30000);
        self.0 &= !0x30000;
        self.0 |= value;
    }    ///
    /// Slew rate ctrl of OB (R), encoding see PREDRV_C_CTRL
    pub fn predrv_r_ctrl(&self) -> u32 {
        (self.0 & 0xc0000) >> 18
    }
    pub fn set_predrv_r_ctrl(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0xc0000);
        self.0 &= !0xc0000;
        self.0 |= value;
    }    ///
    /// Additional resistor calibration trim
    pub fn r_cor(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_r_cor(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Offset resistance adjustment for CML cells (two-complement)
    ///
    /// 1000: -8 1111: -1 0000: 0 0111: 7
    pub fn r_i(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_r_i(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }    ///
    /// Ctrl of cascade volt in drv stage
    ///
    /// 0: reserved 1: 0 2: 1/12 4: 2/12 8: 3/12 16: 4/12 Intermediate values possible when setting two bits
    pub fn vcas(&self) -> u32 {
        (self.0 & 0x3e0) >> 5
    }
    pub fn set_vcas(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x3e0);
        self.0 &= !0x3e0;
        self.0 |= value;
    }    ///
    /// Tail voltage driver settings
    ///
    /// 0: reserved 1: 75mV 2: 100mV 4: 125mV 8: 150mV 16: 175mV 32: 200mV Intermediate values possible when setting two bits
    pub fn vtail(&self) -> u32 {
        (self.0 & 0xfc00) >> 10
    }
    pub fn set_vtail(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xfc00);
        self.0 &= !0xfc00;
        self.0 |= value;
    }
}

/// Register `SD10G65_OB_CFG2`
///
/// SD10G65 OB Configuration register 2
///
/// Configuration register 2 for SD10G65 OB. D_filter contains four 6-bit precalculated DA input values. Please note the differences in programming for various interface (IF) bit widths. For calculation details see documentation of OB10G.
#[derive(From, Into)]
pub struct SD10G65_OB_CFG2(u32);
impl SD10G65_OB_CFG2 {    ///
    /// Transmit filter coefficients for FIR taps. Suggested start value (no emphasis, max amplitude)
    ///
    /// 0x820820: for I/F width 8/10 bits 0x7DF820: for I/F width 16/20/32/40 bits
    pub fn d_filter(&self) -> u32 {
        (self.0 & 0xffffff) >> 0
    }
    pub fn set_d_filter(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}

/// Register `SD10G65_OB_CFG3`
///
/// SD10G65 OB Configuration register 3  access to receiver detect functionality
///
/// Configuration register 3 for SD10G65 OB.
#[derive(From, Into)]
pub struct SD10G65_OB_CFG3(u32);
impl SD10G65_OB_CFG3 {    ///
    /// Indicates a completed receiver detect measurement. Should be one few us after rec_det_start is set.
    pub fn rec_det_done(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_rec_det_done(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }    ///
    /// Enable reciver detect function. MUST be disabled for normal operation !!!
    pub fn rec_det_enable(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_rec_det_enable(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }    ///
    /// Rising edge starts receiver detect measurement. Has to be keept set until rec_det_value has been read.
    pub fn rec_det_start(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rec_det_start(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }    ///
    /// Reciver detect threshold (suggested start value: 2)
    ///
    /// 0: reserved 1: 0mV 2: 8.3mV 4: 16.7mV 8: 25mV Intermediate values possible when setting two bits
    pub fn rec_det_thrs(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    pub fn set_rec_det_thrs(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xf000);
        self.0 &= !0xf000;
        self.0 |= value;
    }    ///
    /// Holds the time between the start and the flag of the receiver detect measuremnet. Time [ns +/- 4 ns] = 8 * value - 12
    pub fn rec_det_value(&self) -> u32 {
        (self.0 & 0xfff) >> 0
    }
    pub fn set_rec_det_value(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}

/// Register `SD10G65_SBUS_TX_CFG`
///
/// SD10G65 SBUS TX CFG Service-Bus related setting
///
/// Configuration register for Service-Bus related setting. Note: this register is only used for configuration if Tx is used stand alone, otherwise SD10G65_SBUS_RX_CFG (Rx macro) is used for configuration!
#[derive(From, Into)]
pub struct SD10G65_SBUS_TX_CFG(u32);
impl SD10G65_SBUS_TX_CFG {    ///
    /// Enable analog test output
    pub fn sbus_anaout_en(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_sbus_anaout_en(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }    ///
    /// Analog test output
    ///
    /// 0: l0_ctrlspeed[0] 1: vbulk 2: nref 3: vref820m 4: vddfilt 5: vddfilt 6: ie_aout 7: ib_aout 8: ob_aout2 9: pll_frange 10: pll_srange 11: pll_vreg820m_tx 12: pll_vreg820m_rx 13: ob_aout_n 14: ob_aout_p 15: vddfilt
    pub fn sbus_anaout_sel(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    pub fn set_sbus_anaout_sel(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xf00);
        self.0 &= !0xf00;
        self.0 |= value;
    }    ///
    /// Bias enable
    ///
    /// 1: Enable 0: Disable
    pub fn sbus_bias_en(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_sbus_bias_en(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Bias speed selection
    ///
    /// 0: Below 4Gbps 1: 4Gbps to 6Gbps 2: 6Gbps to 9Gbps 3: Above 9Gbps
    pub fn sbus_bias_speed_sel(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_sbus_bias_speed_sel(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x6);
        self.0 &= !0x6;
        self.0 |= value;
    }    ///
    /// Enable BiDi loop driver for F2DF testing
    pub fn sbus_loopdrv_ena(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_sbus_loopdrv_ena(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }    ///
    /// Offset value for BIAS resistor calibration (2-complement)
    ///
    /// 1000: -8 1111: -1 0000: 0 0111: 7
    pub fn sbus_rcomp(&self) -> u32 {
        (self.0 & 0x78) >> 3
    }
    pub fn set_sbus_rcomp(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x78);
        self.0 &= !0x78;
        self.0 |= value;
    }    ///
    /// Pool of spare bits for use in late design changes.
    pub fn sbus_spare_pool(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    pub fn set_sbus_spare_pool(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xf0000);
        self.0 &= !0xf0000;
        self.0 |= value;
    }
}

/// Register `SD10G65_TX_SVN_ID`
///
/// SD10G65_TX subversion revision number
///
/// Subversion revision number for the RTL used in SD10G65_TX
#[derive(From, Into)]
pub struct SD10G65_TX_SVN_ID(u32);
impl SD10G65_TX_SVN_ID {    ///
    /// SVN revision number of RTL sources
    pub fn tx_svn_id(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_tx_svn_id(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}