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
/// SD10G65 OB Configuration register 0
///
/// Configuration register 0 for SD10G65 OB.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_OB_CFG0(u32);
impl SD10G65_OB_CFG0 {
    /// Bypass data path (e.g. for JTAG), allows to drive output when EN_DIRECT=1 and EN_OB=1
    #[inline(always)]
    pub fn byp_d(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_byp_d(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Control of common mode voltage of clock buffer between synthesizer and OB.
    #[inline(always)]
    pub fn clk_buf_cmv(&self) -> u32 {
        (self.0 & 0x600000) >> 21
    }
    #[inline(always)]
    pub fn set_clk_buf_cmv(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 21;
        self.0 &= !0x600000;
        self.0 |= value;
    }
    /// Enable direct path
    #[inline(always)]
    pub fn en_direct(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    #[inline(always)]
    pub fn set_en_direct(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 14;
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Enable input loop
    #[inline(always)]
    pub fn en_inp_loop(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    #[inline(always)]
    pub fn set_en_inp_loop(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Enable output buffer and serializer
    #[inline(always)]
    pub fn en_ob(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    #[inline(always)]
    pub fn set_en_ob(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Enable pad loop
    #[inline(always)]
    pub fn en_pad_loop(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_en_pad_loop(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Selects amplitude range controled via levn. See description of levn.
    #[inline(always)]
    pub fn incr_levn(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_incr_levn(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Amplitude control value. Step size is 25 mVpp, decreasing amplitude with increasing control value. Range depends on incr_levn. Coding for incr_levn=0: 31: 500mVpp, 30: 525mVpp, 29: 550mVpp, ..., 0: 1275mVpp. Coding for incr_levn=1: 31: 300mVpp, 30: 325mVpp, 29: 350mVpp, .., 0: 1075mVpp. (Note: maximum achievable amplitude depends on the supply voltage)
    #[inline(always)]
    pub fn levn(&self) -> u32 {
        self.0 & 0x1f
    }
    #[inline(always)]
    pub fn set_levn(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
    /// Pool of spare bits for use in late design changes.
    #[inline(always)]
    pub fn ob_spare_pool(&self) -> u32 {
        (self.0 & 0x180000) >> 19
    }
    #[inline(always)]
    pub fn set_ob_spare_pool(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 19;
        self.0 &= !0x180000;
        self.0 |= value;
    }
    /// Set digital part into pseudo reset
    #[inline(always)]
    pub fn rst(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline(always)]
    pub fn set_rst(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Interface width
    ///
    /// 0: 8 1: 10 2: 16 3: 20 4: 32 5: 40 6-7: Reserved
    #[inline(always)]
    pub fn sel_ifw(&self) -> u32 {
        (self.0 & 0xe0) >> 5
    }
    #[inline(always)]
    pub fn set_sel_ifw(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 5;
        self.0 &= !0xe0;
        self.0 |= value;
    }
    /// Invert input to serializer
    #[inline(always)]
    pub fn ser_inv(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    #[inline(always)]
    pub fn set_ser_inv(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 23;
        self.0 &= !0x800000;
        self.0 |= value;
    }
}
/// SD10G65 OB Configuration register 1
///
/// Configuration register 1 for SD10G65 OB.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_OB_CFG1(u32);
impl SD10G65_OB_CFG1 {
    /// Enable amplitude compensation of AB bleed current
    #[inline(always)]
    pub fn ab_comp_en(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    #[inline(always)]
    pub fn set_ab_comp_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 26;
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    /// Bleed current for class AB operation of driver
    ///
    /// 0: 1% 1: 0.5% 2: 2% 3: reserved
    #[inline(always)]
    pub fn diode_cur(&self) -> u32 {
        (self.0 & 0x3800000) >> 23
    }
    #[inline(always)]
    pub fn set_diode_cur(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 23;
        self.0 &= !0x3800000;
        self.0 |= value;
    }
    /// Level shift ctrl of class AB bias generator
    ///
    /// 0: 50mV 1: 100mV 2:150mV 3: 200mV
    #[inline(always)]
    pub fn lev_shft(&self) -> u32 {
        (self.0 & 0x600000) >> 21
    }
    #[inline(always)]
    pub fn set_lev_shft(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 21;
        self.0 &= !0x600000;
        self.0 |= value;
    }
    /// Slew rate ctrl of OB (C)
    ///
    /// C=3 R=3: 25ps C=3 R=0: 35ps C=0 R=3: 55ps C=1 R=0: 70ps C=0 R=0: 120 ps
    #[inline(always)]
    pub fn predrv_c_ctrl(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    #[inline(always)]
    pub fn set_predrv_c_ctrl(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 16;
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// Slew rate ctrl of OB (R), encoding see PREDRV_C_CTRL
    #[inline(always)]
    pub fn predrv_r_ctrl(&self) -> u32 {
        (self.0 & 0xc0000) >> 18
    }
    #[inline(always)]
    pub fn set_predrv_r_ctrl(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 18;
        self.0 &= !0xc0000;
        self.0 |= value;
    }
    /// Additional resistor calibration trim
    #[inline(always)]
    pub fn r_cor(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_r_cor(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Offset resistance adjustment for CML cells (two-complement)
    ///
    /// 1000: -8 1111: -1 0000: 0 0111: 7
    #[inline(always)]
    pub fn r_i(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_r_i(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Ctrl of cascade volt in drv stage
    ///
    /// 0: reserved 1: 0 2: 1/12 4: 2/12 8: 3/12 16: 4/12 Intermediate values possible when setting two bits
    #[inline(always)]
    pub fn vcas(&self) -> u32 {
        (self.0 & 0x3e0) >> 5
    }
    #[inline(always)]
    pub fn set_vcas(&mut self, value: u32) {
        assert!(value <= 0x1f);
        let value = value << 5;
        self.0 &= !0x3e0;
        self.0 |= value;
    }
    /// Tail voltage driver settings
    ///
    /// 0: reserved 1: 75mV 2: 100mV 4: 125mV 8: 150mV 16: 175mV 32: 200mV Intermediate values possible when setting two bits
    #[inline(always)]
    pub fn vtail(&self) -> u32 {
        (self.0 & 0xfc00) >> 10
    }
    #[inline(always)]
    pub fn set_vtail(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 10;
        self.0 &= !0xfc00;
        self.0 |= value;
    }
}
/// SD10G65 OB Configuration register 2
///
/// Configuration register 2 for SD10G65 OB. D_filter contains four 6-bit precalculated DA input values. Please note the differences in programming for various interface (IF) bit widths. For calculation details see documentation of OB10G.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_OB_CFG2(u32);
impl SD10G65_OB_CFG2 {
    /// Transmit filter coefficients for FIR taps. Suggested start value (no emphasis, max amplitude)
    ///
    /// 0x820820: for I/F width 8/10 bits 0x7DF820: for I/F width 16/20/32/40 bits
    #[inline(always)]
    pub fn d_filter(&self) -> u32 {
        self.0 & 0xffffff
    }
    #[inline(always)]
    pub fn set_d_filter(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// SD10G65 OB Configuration register 3  access to receiver detect functionality
///
/// Configuration register 3 for SD10G65 OB.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_OB_CFG3(u32);
impl SD10G65_OB_CFG3 {
    /// Indicates a completed receiver detect measurement. Should be one few us after rec_det_start is set.
    #[inline(always)]
    pub fn rec_det_done(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_rec_det_done(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Enable reciver detect function. MUST be disabled for normal operation !!!
    #[inline(always)]
    pub fn rec_det_enable(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_rec_det_enable(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Rising edge starts receiver detect measurement. Has to be keept set until rec_det_value has been read.
    #[inline(always)]
    pub fn rec_det_start(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline(always)]
    pub fn set_rec_det_start(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Reciver detect threshold (suggested start value: 2)
    ///
    /// 0: reserved 1: 0mV 2: 8.3mV 4: 16.7mV 8: 25mV Intermediate values possible when setting two bits
    #[inline(always)]
    pub fn rec_det_thrs(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    #[inline(always)]
    pub fn set_rec_det_thrs(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 12;
        self.0 &= !0xf000;
        self.0 |= value;
    }
    /// Holds the time between the start and the flag of the receiver detect measuremnet. Time [ns +/- 4 ns] = 8 * value - 12
    #[inline(always)]
    pub fn rec_det_value(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_rec_det_value(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// SD10G65 SBUS TX CFG Service-Bus related setting
///
/// Configuration register for Service-Bus related setting. Note: this register is only used for configuration if Tx is used stand alone, otherwise SD10G65_SBUS_RX_CFG (Rx macro) is used for configuration!
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_SBUS_TX_CFG(u32);
impl SD10G65_SBUS_TX_CFG {
    /// Enable analog test output
    #[inline(always)]
    pub fn sbus_anaout_en(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_sbus_anaout_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Analog test output
    ///
    /// 0: l0_ctrlspeed[0] 1: vbulk 2: nref 3: vref820m 4: vddfilt 5: vddfilt 6: ie_aout 7: ib_aout 8: ob_aout2 9: pll_frange 10: pll_srange 11: pll_vreg820m_tx 12: pll_vreg820m_rx 13: ob_aout_n 14: ob_aout_p 15: vddfilt
    #[inline(always)]
    pub fn sbus_anaout_sel(&self) -> u32 {
        (self.0 & 0xf00) >> 8
    }
    #[inline(always)]
    pub fn set_sbus_anaout_sel(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 8;
        self.0 &= !0xf00;
        self.0 |= value;
    }
    /// Bias enable
    ///
    /// 1: Enable 0: Disable
    #[inline(always)]
    pub fn sbus_bias_en(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_sbus_bias_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Bias speed selection
    ///
    /// 0: Below 4Gbps 1: 4Gbps to 6Gbps 2: 6Gbps to 9Gbps 3: Above 9Gbps
    #[inline(always)]
    pub fn sbus_bias_speed_sel(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_sbus_bias_speed_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
    /// Enable BiDi loop driver for F2DF testing
    #[inline(always)]
    pub fn sbus_loopdrv_ena(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_sbus_loopdrv_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Offset value for BIAS resistor calibration (2-complement)
    ///
    /// 1000: -8 1111: -1 0000: 0 0111: 7
    #[inline(always)]
    pub fn sbus_rcomp(&self) -> u32 {
        (self.0 & 0x78) >> 3
    }
    #[inline(always)]
    pub fn set_sbus_rcomp(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 3;
        self.0 &= !0x78;
        self.0 |= value;
    }
    /// Pool of spare bits for use in late design changes.
    #[inline(always)]
    pub fn sbus_spare_pool(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    #[inline(always)]
    pub fn set_sbus_spare_pool(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 16;
        self.0 &= !0xf0000;
        self.0 |= value;
    }
}
/// SD10G65_TX Revision ID
///
/// Revision numbers of the analog sub IPs used in the SD10G65_TX
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_REV_ID(u32);
impl SD10G65_TX_REV_ID {
    /// Feature set number of output buffer (ob10g_N)
    #[inline(always)]
    pub fn ob_rev_id(&self) -> u32 {
        (self.0 & 0x3f00000) >> 20
    }
    #[inline(always)]
    pub fn set_ob_rev_id(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 20;
        self.0 &= !0x3f00000;
        self.0 |= value;
    }
    /// Feature set number of RC-PLL (pll10g_N)
    #[inline(always)]
    pub fn rcpll_rev_id(&self) -> u32 {
        (self.0 & 0x3f00) >> 8
    }
    #[inline(always)]
    pub fn set_rcpll_rev_id(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 8;
        self.0 &= !0x3f00;
        self.0 |= value;
    }
    /// Feature set number of synthesizer (syn_N)
    #[inline(always)]
    pub fn synth_rev_id(&self) -> u32 {
        (self.0 & 0xfc000) >> 14
    }
    #[inline(always)]
    pub fn set_synth_rev_id(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 14;
        self.0 &= !0xfc000;
        self.0 |= value;
    }
    /// Feature set number of Toplevel (sd10g65_N)
    #[inline(always)]
    pub fn top_rev_id(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_top_rev_id(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// SD10G65_TX subversion revision number
///
/// Subversion revision number for the RTL used in SD10G65_TX
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SD10G65_TX_SVN_ID(u32);
impl SD10G65_TX_SVN_ID {
    /// SVN revision number of RTL sources
    #[inline(always)]
    pub fn tx_svn_id(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_tx_svn_id(&mut self, value: u32) {
        self.0 = value;
    }
}
