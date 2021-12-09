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
/// APC LC softcontrol configuration register 1
///
/// Configuration register 1 for the LC-Softcontrol logic block.
#[derive(From, Into)]
pub struct APC_LC_SOFTCTRL_CFG1(u32);
impl APC_LC_SOFTCTRL_CFG1 {
    /// Target value for DFE1 during L/C-control operation
    pub fn lc_sc_dfe1_target(&self) -> u32 {
        self.0 & 0x7f
    }
    pub fn set_lc_sc_dfe1_target(&mut self, value: u32) {
        assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }
    /// Target value for DFE2 during L/C-control operation
    pub fn lc_sc_dfe2_target(&self) -> u32 {
        (self.0 & 0x3f00) >> 8
    }
    pub fn set_lc_sc_dfe2_target(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x3f00);
        self.0 &= !0x3f00;
        self.0 |= value;
    }
}
/// Vscope counter register
///
/// Vscope counter register
#[derive(From, Into)]
pub struct VSCOPE_CNT(u32);
impl VSCOPE_CNT {
    /// Counter value
    pub fn counter(&self) -> u32 {
        self.0
    }
    pub fn set_counter(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Vscope hw scan config register 1
///
/// Vscope HW scan configuration register 1
#[derive(From, Into)]
pub struct VSCOPE_HW_SCAN_CFG1(u32);
impl VSCOPE_HW_SCAN_CFG1 {
    /// Amplitude increment per scan step

    ///

    /// Increment = ampl_incr + 1
    pub fn ampl_incr(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_ampl_incr(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }
    /// Offset between AuxL amplitude (reference) and AuxH amplitude, signed (2s-complement), +- 1/4 amplitude max.
    pub fn ampl_offs_val(&self) -> u32 {
        (self.0 & 0x3e000000) >> 25
    }
    pub fn set_ampl_offs_val(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x3e000000);
        self.0 &= !0x3e000000;
        self.0 |= value;
    }
    /// Enables HW scan with N results per scan or fast-scan

    ///

    /// 0: off 1: N-point scan 2: fast-scan (sq) 3: fast-scan (diag)
    pub fn hw_scan_ena(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_hw_scan_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Maximum amplitude increment value before wrapping
    pub fn max_ampl_incr_val(&self) -> u32 {
        (self.0 & 0xfc00) >> 10
    }
    pub fn set_max_ampl_incr_val(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xfc00);
        self.0 &= !0xfc00;
        self.0 |= value;
    }
    /// Maximum phase increment value before wrapping
    pub fn max_phase_incr_val(&self) -> u32 {
        (self.0 & 0x1fe0000) >> 17
    }
    pub fn set_max_phase_incr_val(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x1fe0000);
        self.0 &= !0x1fe0000;
        self.0 |= value;
    }
    /// Number of scans per iteration in N-point-scan mode

    ///

    /// 0: 1 1: 2 2: 4 3: 8
    pub fn num_scans_per_itr(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_num_scans_per_itr(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Phase increment per scan step

    ///

    /// Increment = phase_incr + 1
    pub fn phase_incr(&self) -> u32 {
        (self.0 & 0x380) >> 7
    }
    pub fn set_phase_incr(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x380);
        self.0 &= !0x380;
        self.0 |= value;
    }
    /// Invert the jumph_ena and jumpl_ena bit in HW scan mode
    pub fn phase_jump_inv(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_phase_jump_inv(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
}
/// Vscope hw config register 2
///
/// Vscope HW scan configuration register 2
#[derive(From, Into)]
pub struct VSCOPE_HW_SCAN_CFG2(u32);
impl VSCOPE_HW_SCAN_CFG2 {
    /// Start value for VScope amplitude in N-point-scan mode and fast-scan mode (before IB amplitude symmetry compensation)
    pub fn ampl_start_val(&self) -> u32 {
        (self.0 & 0x3f00) >> 8
    }
    pub fn set_ampl_start_val(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x3f00);
        self.0 &= !0x3f00;
        self.0 |= value;
    }
    /// Disable IB amplitude symmetry compensation for AuxH and AuxL
    pub fn aux_ampl_sym_dis(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_aux_ampl_sym_dis(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Threshold for error_counter in fast-scan mode

    ///

    /// N+1
    pub fn fast_scan_thres(&self) -> u32 {
        (self.0 & 0xe0000000) >> 29
    }
    pub fn set_fast_scan_thres(&mut self, value: u32) {
        let value = value << 29;
        assert!(value <= 0xe0000000);
        self.0 &= !0xe0000000;
        self.0 |= value;
    }
    /// Left shift for threshold of error_counter in fast-scan mode

    ///

    /// threshold = (fast_scan_thres+1) shift_left fs_thres_shift
    pub fn fs_thres_shift(&self) -> u32 {
        (self.0 & 0x1f000000) >> 24
    }
    pub fn set_fs_thres_shift(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1f000000);
        self.0 &= !0x1f000000;
        self.0 |= value;
    }
    /// Value at which jumpl_ena and jumph_ena in IB must be toggled
    pub fn phase_jump_val(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_phase_jump_val(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xff0000);
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    /// Start value for VScope phase in N-point-scan mode and fast-scan mode
    pub fn phase_start_val(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_phase_start_val(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Vscope main config register
///
/// Vscope main configuration register
#[derive(From, Into)]
pub struct VSCOPE_MAIN_CFG(u32);
impl VSCOPE_MAIN_CFG {
    /// Enable Counting

    ///

    /// 0: disable counting and assign counter output; internal counters get their preload value 1: enable counting
    pub fn cnt_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_cnt_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Counter output selection

    ///

    /// 0-3: error counter 0-3 4: hit counter 5: clock counter 6: 8 LSBs of error counter 3-1 and hit counter 7: 8 LSBs of error counter 3-0
    pub fn cnt_out_sel(&self) -> u32 {
        (self.0 & 0x1c000000) >> 26
    }
    pub fn set_cnt_out_sel(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0x1c000000);
        self.0 &= !0x1c000000;
        self.0 |= value;
    }
    /// Comparator input selection

    ///

    /// [REF] 0 or 1: auxL 4 or 5: auxH 2 or 7: main [SUB] 5 or 7: auxL 0 or 2: auxH 1 or 4: main (3 or 6: reserved)
    pub fn comp_sel(&self) -> u32 {
        (self.0 & 0x3800000) >> 23
    }
    pub fn set_comp_sel(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x3800000);
        self.0 &= !0x3800000;
        self.0 |= value;
    }
    /// Counter period: preload value for clock counter

    ///

    /// After preload clock counter = 2**32 - 2**(count_per + 1)
    pub fn count_per(&self) -> u32 {
        (self.0 & 0x3e0) >> 5
    }
    pub fn set_count_per(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x3e0);
        self.0 &= !0x3e0;
        self.0 |= value;
    }
    /// Allows to freeze the GP register value to assure valid reading
    pub fn gp_reg_freeze(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_gp_reg_freeze(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Select GP reg input

    ///

    /// 0: rx (main) 1: low aux 2: high aux 3: counter
    pub fn gp_select(&self) -> u32 {
        (self.0 & 0x600000) >> 21
    }
    pub fn set_gp_select(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x600000);
        self.0 &= !0x600000;
        self.0 |= value;
    }
    /// Disables writing of ib_auxl_offset and ib_auxh_offset in IB
    pub fn ib_aux_offs_wr_dis(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_ib_aux_offs_wr_dis(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    /// Disables writing of ib_jumpl_ena and ib_jumph_ena in IB
    pub fn ib_jump_ena_wr_dis(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    pub fn set_ib_jump_ena_wr_dis(&mut self, value: u32) {
        let value = value << 29;
        assert!(value <= 0x20000000);
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    /// Interface Width

    ///

    /// 0: 8 bit 1: 10 bit 2: 16 bit 3: 20 bit 4: 32 bit 5: 40 bit others: reserved
    pub fn if_mode(&self) -> u32 {
        (self.0 & 0xe) >> 1
    }
    pub fn set_if_mode(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0xe);
        self.0 &= !0xe;
        self.0 |= value;
    }
    /// Disable interrupt output
    pub fn intr_dis(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_intr_dis(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Preload value for error counter

    ///

    /// After preload error counter = 2**32 - 2**(preload_val + 1)
    pub fn preload_val(&self) -> u32 {
        (self.0 & 0x3e000) >> 13
    }
    pub fn set_preload_val(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x3e000);
        self.0 &= !0x3e000;
        self.0 |= value;
    }
    /// Counter output assignment and internal counter reset implicitly done by reading the counter; unused in hw-scan mode where this function is permanently enabled
    pub fn quick_scan(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_quick_scan(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Scan limit, selects which counter saturation limits the other counters

    ///

    /// 0: clock counter 1: hit counter 2: error counters 3: no limit
    pub fn scan_lim(&self) -> u32 {
        (self.0 & 0xc0000) >> 18
    }
    pub fn set_scan_lim(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0xc0000);
        self.0 &= !0xc0000;
        self.0 |= value;
    }
    /// Disables writing of synth_phase_aux in synthesizer
    pub fn syn_phase_wr_dis(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_syn_phase_wr_dis(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Enable trigger
    pub fn trig_ena(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_trig_ena(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Enable Vscope
    pub fn vscope_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vscope_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Vscope pattern lock config register
///
/// Vscope pattern lock configuration register
#[derive(From, Into)]
pub struct VSCOPE_PAT_LOCK_CFG(u32);
impl VSCOPE_PAT_LOCK_CFG {
    /// Don't Care mask: Enable history mask usage.

    ///

    /// 0: enable history mask bit 1: history mask bit is "don't care"
    pub fn dc_mask(&self) -> u32 {
        (self.0 & 0xffc00) >> 10
    }
    pub fn set_dc_mask(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xffc00);
        self.0 &= !0xffc00;
        self.0 |= value;
    }
    /// History mask: Respective sequence is expected in reference input (comp_sel); if enabled (dc_mask) before hit and error counting is enabled
    pub fn hist_mask(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_hist_mask(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
    /// Preload value for hit counter

    ///

    /// After preload hit counter = 2**32 - 2**(preload_hit_cnt + 1)
    pub fn preload_hit_cnt(&self) -> u32 {
        (self.0 & 0x1f00000) >> 20
    }
    pub fn set_preload_hit_cnt(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x1f00000);
        self.0 &= !0x1f00000;
        self.0 |= value;
    }
}
/// Vscope status register
///
/// Vscope status register
#[derive(From, Into)]
pub struct VSCOPE_STAT(u32);
impl VSCOPE_STAT {
    /// Done sticky
    pub fn done_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_done_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Fast scan mode: Indicator per cursor position whether threshold was reached
    pub fn fast_scan_hit(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    pub fn set_fast_scan_hit(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0xf0);
        self.0 &= !0xf0;
        self.0 |= value;
    }
    /// 8 MSBs of general purpose register
    pub fn gp_reg_msb(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_gp_reg_msb(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
}
