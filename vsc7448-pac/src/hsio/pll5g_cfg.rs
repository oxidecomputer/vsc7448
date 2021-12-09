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
/// PLL5G Configuration 0
///
/// Configuration register 0 for PLL5G
#[derive(From, Into)]
pub struct PLL5G_CFG0(u32);
impl PLL5G_CFG0 {
    /// setting for core clock divider 0:625MHz, 1:312.5MHz, 2:500MHz, 3:277.77MHz, 4:500MHz, 5:250MHz, 6:416.66MHz, 7:227.27MHz, 8:416.66MHz, 9:208.33MHz, 10:357.14MHz, 11:192.3MHz, 12:357.14MHz, 13:178.57MHz, 14:312.5MHz, 15:166.66,MHz, 17:156.25MHz, 33:625MHz
    pub fn core_clk_div(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_core_clk_div(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// Setting for CPU clock divider

    ///

    /// 2: 500 MHz 5: 250 MHz 6: 416.66 MHz 14: 312.5 MHz 15: 166.66 MHz Others: Reserved
    pub fn cpu_clk_div(&self) -> u32 {
        (self.0 & 0xfc0) >> 6
    }
    pub fn set_cpu_clk_div(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0xfc0);
        self.0 &= !0xfc0;
        self.0 |= value;
    }
    /// RCPLL feedback divider setting
    pub fn div4(&self) -> u32 {
        (self.0 & 0x10000000) >> 28
    }
    pub fn set_div4(&mut self, value: u32) {
        let value = value << 28;
        assert!(value <= 0x10000000);
        self.0 &= !0x10000000;
        self.0 |= value;
    }
    /// enable BIAS circuitry incl. Bandgap, voltage regulators, etc.
    pub fn ena_bias(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_ena_bias(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// RCPLL enable BIAS for clocktree buffer (active low) 0: enable BIAS, 1: disable BIAS
    pub fn ena_clktree(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    pub fn set_ena_clktree(&mut self, value: u32) {
        let value = value << 29;
        assert!(value <= 0x20000000);
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    /// enable current mode chargepump, normal mode
    pub fn ena_cp1(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_ena_cp1(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// RCPLL Global enable for serdes lane.
    pub fn ena_lane(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_ena_lane(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    /// enable fine locking, last stage in startup locking sequence
    pub fn ena_lock_fine(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    pub fn set_ena_lock_fine(&mut self, value: u32) {
        let value = value << 27;
        assert!(value <= 0x8000000);
        self.0 &= !0x8000000;
        self.0 |= value;
    }
    /// RCPLL feedback divider setting
    pub fn ena_rot(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_ena_rot(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// enable BIAS for LCPLL VCO output buffer
    pub fn ena_vco_buf(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_ena_vco_buf(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// enable fine VCO operating point regulator
    pub fn ena_vco_contrh(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_ena_vco_contrh(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// setting for filter resistor value 0: biggest resistance, 31: lowest resistance
    pub fn loop_bw_res(&self) -> u32 {
        (self.0 & 0x7c0000) >> 18
    }
    pub fn set_loop_bw_res(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x7c0000);
        self.0 &= !0x7c0000;
        self.0 |= value;
    }
    /// fine tune of bandgap voltage distribution 0: lowest voltage, 15: highest voltage
    pub fn selbgv820(&self) -> u32 {
        (self.0 & 0x7800000) >> 23
    }
    pub fn set_selbgv820(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x7800000);
        self.0 &= !0x7800000;
        self.0 |= value;
    }
    /// setting for chargepump current 0: lowest current, 3: highest current
    pub fn selcpi(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    pub fn set_selcpi(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x30000);
        self.0 &= !0x30000;
        self.0 |= value;
    }
}
/// PLL5G Configuration 1
///
/// Configuration register 1 for PLL5G
#[derive(From, Into)]
pub struct PLL5G_CFG1(u32);
impl PLL5G_CFG1 {
    /// enable for direct data mode (ATPG/JTAG) reference clock input buffer and test output buffer
    pub fn ena_direct(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_ena_direct(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// RCPLL When set to '1' the value at sx_pll_fsm_ctrl_data_I is not taken as reference value for the FSM, but is directly apllied to the PLL as frequency range setting.
    pub fn force_set_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_force_set_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// RCPLL Enable for half rate mode
    pub fn half_rate(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_half_rate(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// RCPLL Enable recalibration of PLL when out of range is detected
    pub fn out_of_range_recal_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_out_of_range_recal_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// RCPLL Power down for the RX-path
    pub fn pwd_rx(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_pwd_rx(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// RCPLL Power down for the TX-path
    pub fn pwd_tx(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_pwd_tx(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// RCPLL Enable for quarter rate mode
    pub fn quarter_rate(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_quarter_rate(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// RCPLL Control input for startup FSM
    pub fn rc_ctrl_data(&self) -> u32 {
        (self.0 & 0x3fc0) >> 6
    }
    pub fn set_rc_ctrl_data(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x3fc0);
        self.0 &= !0x3fc0;
        self.0 |= value;
    }
    /// RCPLL Enable for startup FSM
    pub fn rc_enable(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_rc_enable(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// RCPLL Selects whether (when set to '1') the frequency range setting from the FSM can be read back at sx_pll_rb_data_o or (when cleared to '0') the measured period.
    pub fn readback_data_sel(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_readback_data_sel(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// RCPLL feedback divider setting
    pub fn rot_dir(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_rot_dir(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// RCPLL feedback divider setting
    pub fn rot_speed(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rot_speed(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
}
/// PLL5G Configuration 2
///
/// Configuration register 2 for PLL5G
#[derive(From, Into)]
pub struct PLL5G_CFG2(u32);
impl PLL5G_CFG2 {
    /// static VCO amplitude control, active w/ ena_amp_ctrl_force 0: lowest current, 255: highest current
    pub fn ampc_sel(&self) -> u32 {
        (self.0 & 0xff000) >> 12
    }
    pub fn set_ampc_sel(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xff000);
        self.0 &= !0xff000;
        self.0 |= value;
    }
    /// disable automatic FSM startup frequency stepping
    pub fn disable_fsm(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_disable_fsm(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// disables the startup FSM to start ramp up the frequency from POR 0: normal,1: disable
    pub fn disable_fsm_por(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_disable_fsm_por(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// enable automatic VCO amplitude control
    pub fn ena_ampctrl(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_ena_ampctrl(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// enable static VCO amplitude control
    pub fn ena_amp_ctrl_force(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_ena_amp_ctrl_force(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// enable clock bypass for all output clocks to come from ref clock pad
    pub fn ena_clk_bypass(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_ena_clk_bypass(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }
    /// enable clock bypass for all output clocks to come from extra dividers (125MHz, 250MHz, 312.5MHz)
    pub fn ena_clk_bypass1(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_ena_clk_bypass1(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    /// enable resistor mode chargepump, test mode
    pub fn ena_cp2(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    pub fn set_ena_cp2(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x2000000);
        self.0 &= !0x2000000;
        self.0 |= value;
    }
    /// enable feedback divider output to test output buffer
    pub fn ena_fbtestout(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    pub fn set_ena_fbtestout(&mut self, value: u32) {
        let value = value << 27;
        assert!(value <= 0x8000000);
        self.0 &= !0x8000000;
        self.0 |= value;
    }
    /// enable static VCO frequency stepping
    pub fn ena_gain_test(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ena_gain_test(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// enable flip of refclk and fbclk at PFD, used for 2nd chargepump
    pub fn ena_pfd_in_flip(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    pub fn set_ena_pfd_in_flip(&mut self, value: u32) {
        let value = value << 29;
        assert!(value <= 0x20000000);
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    /// enable RCPLL clock buffer in LCPLL VCO (sx_ena_vco_buf_i must be set to 0)
    pub fn ena_rcpll(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    pub fn set_ena_rcpll(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0x4000000);
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    /// enables test modes, e.g. fbdivsel
    pub fn ena_test_mode(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_ena_test_mode(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    /// enable VCO frequency control output
    pub fn ena_vco_nref_testout(&self) -> u32 {
        (self.0 & 0x10000000) >> 28
    }
    pub fn set_ena_vco_nref_testout(&mut self, value: u32) {
        let value = value << 28;
        assert!(value <= 0x10000000);
        self.0 &= !0x10000000;
        self.0 |= value;
    }
    /// enable FSM frequency deviation detection
    pub fn en_reset_frq_det(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_en_reset_frq_det(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// enable FSM limiter detection
    pub fn en_reset_lim_det(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_en_reset_lim_det(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// enable FSM frequency deviation overrun
    pub fn en_reset_overrun(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_en_reset_overrun(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// forces the startup FSM to start ramp up the frequency by POR 0: no force,1: force
    pub fn frc_fsm_por(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_frc_fsm_por(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// setting for static VCO frequency stepping 0: lowest frequency,31: highest frequency
    pub fn gain_test(&self) -> u32 {
        (self.0 & 0x3e0) >> 5
    }
    pub fn set_gain_test(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x3e0);
        self.0 &= !0x3e0;
        self.0 |= value;
    }
    /// force VCO amplitude control output to low (no VCO current) 0:force, 1: no force
    pub fn pwd_ampctrl_n(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_pwd_ampctrl_n(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
}
/// PLL5G Configuration 3
///
/// Configuration register 3 for PLL5G
#[derive(From, Into)]
pub struct PLL5G_CFG3(u32);
impl PLL5G_CFG3 {
    /// Set to 3 to enable CLKOUT2 synchronous Ethernet reference clock output. Applies only to PLL2.
    pub fn clkout2_sel(&self) -> u32 {
        (self.0 & 0x380000) >> 19
    }
    pub fn set_clkout2_sel(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x380000);
        self.0 &= !0x380000;
        self.0 |= value;
    }
    /// enable analog test output
    pub fn ena_ana_test_out(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_ena_ana_test_out(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// enable differential test output
    pub fn ena_test_out(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_ena_test_out(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// setting for feedback divider, divide by 12..255 12..255
    pub fn fbdivsel(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_fbdivsel(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// enable feedback divider testmode
    pub fn fbdivsel_tst_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_fbdivsel_tst_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// force chargepump output to nominal VCO operating point
    pub fn force_cp(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_force_cp(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// enable force VCO frequency high/low (force_hi/lo)
    pub fn force_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_force_ena(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// force chargepump output to high, gives highest VCO frequency
    pub fn force_hi(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_force_hi(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// force chargepump output to low, gives lowest VCO frequency
    pub fn force_lo(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_force_lo(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// force vco contrh input to mid level (mid CML level)
    pub fn force_vco_contrh(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_force_vco_contrh(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// reset for feedback divider, active low 0: reset,1:no reset
    pub fn rst_fb_n(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_rst_fb_n(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// select CML or CMOS phase/frequency detector 0: CML, 1: CMOS
    pub fn sel_cml_cmos_pfd(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_sel_cml_cmos_pfd(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// enable symmetric feedback divider clock output 0: fbclk/2, 1: fbclk
    pub fn sel_fbdclk(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_sel_fbdclk(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// select analog test output input signal
    pub fn test_ana_out_sel(&self) -> u32 {
        (self.0 & 0xc00000) >> 22
    }
    pub fn set_test_ana_out_sel(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0xc00000);
        self.0 &= !0xc00000;
        self.0 |= value;
    }
}
/// PLL5G Configuration 4
///
/// Configuration register 4 for PLL5G
#[derive(From, Into)]
pub struct PLL5G_CFG4(u32);
impl PLL5G_CFG4 {
    /// settings for reference clock input buffer BIAS
    pub fn ib_bias_ctrl(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_ib_bias_ctrl(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xff0000);
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    /// settings for reference clock input buffer
    pub fn ib_ctrl(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_ib_ctrl(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// PLL5G Configuration 5
///
/// Configuration register 5 for PLL5G
#[derive(From, Into)]
pub struct PLL5G_CFG5(u32);
impl PLL5G_CFG5 {
    /// settings for test output buffer BIAS
    pub fn ob_bias_ctrl(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_ob_bias_ctrl(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xff0000);
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    /// settings for test output buffer
    pub fn ob_ctrl(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_ob_ctrl(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
