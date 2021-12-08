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

/// Register `DLB_CFG`
///
/// Configuration of DLB policer
#[derive(From, Into)]
pub struct DLB_CFG(u32);
impl DLB_CFG {
    /// Configuration of the drop precedence change for green frames being remarked to yellow. For such frames, the DP level will be incremented with CIR_INC_DP_VAL.

    ///

    /// 0: DP is not increased. n: DP is increased with n for PIR traffic.
    pub fn cir_inc_dp_val(&self) -> u32 {
        (self.0 & 0x3000) >> 12
    }
    pub fn set_cir_inc_dp_val(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x3000);
        self.0 &= !0x3000;
        self.0 |= value;
    }
    /// Configuration of dual leaky bucket color awareness. Frames with DP below or equal to COLOR_AWARE_LVL is treated as green. Frames with DP above COLOR_AWARE_LVL are treated as yellow.

    ///

    /// 0: DLB policer is color aware. Incoming frames with DP=0 are green and frames with DP>0 are yellow. 1: DLB policer is color aware. Incoming frames with DP<=1 are green and frames with DP>1 are yellow. 2: DLB policer is color aware. Incoming frames with DP<=2 are green and frames with DP>2 are yellow. 3: DLB policer is color unaware. All incoming frames are green.
    pub fn color_aware_lvl(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    pub fn set_color_aware_lvl(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0xc000);
        self.0 &= !0xc000;
        self.0 |= value;
    }
    /// Set Coupling Flag (MEF CF). Depending on the setting of COUPLING_MODE, LB_CFG[0] and LB_CFG[1] must be configured as follows: COUPLING_MODE=0: LB_CFG[0].RATE_VAL  must be configured to MEF CIR LB_CFG[0].THRES_VAL must be configured to MEF CBS LB_CFG[1].RATE_VAL  must be configured to MEF EIR LB_CFG[1].THRES_VAL must be configured to MEF EBS COUPLING_MODE=1: LB_CFG[0].RATE_VAL  must be configured to MEF CIR LB_CFG[0].THRES_VAL must be configured to MEF CBS LB_CFG[1].RATE_VAL  must be configured to MEF EIR + MEF CIR LB_CFG[1].THRES_VAL must be configured to MEF EBS + MEF CBS

    ///

    /// 0: CF=0 1: CF=1
    pub fn coupling_mode(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_coupling_mode(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Configures if stripped encapsulation data (normalized data) is policed by the policer.

    ///

    /// 0: Encapsulation data is counted as frame data. 1: Encapsulation data in not counted as frame data.
    pub fn encap_data_dis(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_encap_data_dis(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Value added to each frame before updating the bucket. Gap value range: -64 to +63 in two's complement format. Setting GAP_VALUE to 20 corresponds to a line-rate measurement, since on the line each frame will be preceded by 12 bytes of IFG and 8 bytes of preamble. Setting GAP_VALUE to 0 corresponds to a data-rate measurement.

    ///

    /// 0x40: -64 0x41: -63 ... 0x7F: -1 0x00: 0 0x01: 1 ... 0x3F: 63
    pub fn gap_val(&self) -> u32 {
        (self.0 & 0x1fc) >> 2
    }
    pub fn set_gap_val(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x1fc);
        self.0 &= !0x1fc;
        self.0 |= value;
    }
    /// TIMESCALE_VAL and BASE_TICK_CNT controls the the rate interval as well as the rate granularity available for LB rate configuration The rate granularity is calculated as follows: 8 / (BASE_TICK_CNT*1e-10 * 2^(3 * TIMESCALE_VAL) The rate granularity also becomes the smallest configurable rate. The largest configurable rate is granularity * (2**<width of RATE_VAL>-2)

    ///

    /// Assuming BASE_TICK_CNT= 9765, RATE_VAL width = 11 bits: 0: Granularity: 8,192,524bps. Range: 8193kbps - 16.7Gbps 1: Granularity: 1,024,066bps. Range 1024kbps - 2Gbps 2: Granularity: 128,008bps. Range: 128kbps - 262Mbps 3: Granularity: 16,001bps. Range: 16kbps - 32Mbps
    pub fn timescale_val(&self) -> u32 {
        (self.0 & 0x3) >> 0
    }
    pub fn set_timescale_val(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// The time - in BASE_TICK_CNTs - at which last leak was performed.
    pub fn timestamp_val(&self) -> u32 {
        (self.0 & 0xffe0000) >> 17
    }
    pub fn set_timestamp_val(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0xffe0000);
        self.0 &= !0xffe0000;
        self.0 |= value;
    }
    /// Configures action to be applied to policed frames.

    ///

    /// 0: No action. 1: Remove front ports from frame's destination port set. 2: Remove CPU ports from frame's destination port set. 3: Remove both front ports and CPU ports from frame's destination port set.
    pub fn traffic_type_mask(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    pub fn set_traffic_type_mask(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xc00);
        self.0 &= !0xc00;
        self.0 |= value;
    }
}

/// Register `DLB_STICKY`
///
/// LB policer diagnostic
#[derive(From, Into)]
pub struct DLB_STICKY(u32);
impl DLB_STICKY {
    /// Set when the frame rate is exceeding the Committed Information Rate. Bit is cleared by writing a 1 to this position.

    ///

    /// 0: No event has occured 1: CIR exceeded
    pub fn cir_exceeded_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_cir_exceeded_sticky(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set when the frame rate is below both the Committed Information Rate and the Peak Information Rate. Bit is cleared by writing a 1 to this position.

    ///

    /// 0: No event has occured 1: Traffic received without triggering CIR and PIR policing
    pub fn cir_pir_open_sticky(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_cir_pir_open_sticky(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set when a LB scan completes. Bit is cleared by writing a 1.

    ///

    /// 0: No event has occured 1: Leak scan completes
    pub fn leak_scan_completed_sticky(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_leak_scan_completed_sticky(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Set when a LB scan starts. Bit is cleared by writing a 1 to this position.

    ///

    /// 0: No event has occured 1: Leak scan started
    pub fn leak_scan_started_sticky(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_leak_scan_started_sticky(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    /// Set when a LB scan could not start because a scan is already ongoing. If this occur, BASE_TICK_CNT is set too low and must be increased. Bit is cleared by writing a 1 to this position.

    ///

    /// 0: No event has occured 1: Leak scan could not start at time
    pub fn leak_start_delayed_sticky(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    pub fn set_leak_start_delayed_sticky(&mut self, value: u32) {
        let value = value << 29;
        assert!(value <= 0x20000000);
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    /// Set when the frame rate is exceeding the Peak Information Rate. Bit is cleared by writing a 1 to this position.

    ///

    /// 0: No event has occured 1: PIR exceeded
    pub fn pir_exceeded_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_pir_exceeded_sticky(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}

/// Register `LB_CFG`
///
/// Threshold and rate configuration
#[derive(From, Into)]
pub struct LB_CFG(u32);
impl LB_CFG {
    /// Specify rate in steps of configured granularity. The rate granularity is configured in TIMESCALE_VAL.

    ///

    /// 0: Disable leak. For THRES_VAL = 0 bucket is always closed. For THRES_VAL > 0, the configured burst size is available. 1: 1 * granularity 2: 2 * granularity ... max_value-1: (max_value-1)*granularity max_value: Disable leaky bucket (always open)
    pub fn rate_val(&self) -> u32 {
        (self.0 & 0x7ff) >> 0
    }
    pub fn set_rate_val(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7ff);
        self.0 &= !0x7ff;
        self.0 |= value;
    }
    /// Policer threshold size (a.ka. burst capacity). Unit is 2048 bytes

    ///

    /// 0: Threshold = 0 bytes (no burst allowed) 1: Threshold = 2048 bytes n: Threshold = n x 2048 bytes
    pub fn thres_val(&self) -> u32 {
        (self.0 & 0x7f0000) >> 16
    }
    pub fn set_thres_val(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x7f0000);
        self.0 &= !0x7f0000;
        self.0 |= value;
    }
}

/// Register `MISC_CFG`
///
/// Configuration of various LB policer handles
#[derive(From, Into)]
pub struct MISC_CFG(u32);
impl MISC_CFG {
    /// Configuration of Drop Precedence bypass level. Frames with DP below DP_BYPASS_LVL bypass the policer (frames are never policed and the bucket is not updated with the frames).

    ///

    /// 0: No frames bypass the policer 1: Frames with DP = 0 bypass the policer 2: Frames with DP = 0 or 1 bypass the policer 3: Frames with DP = 0, 1 or 2 bypass the policer
    pub fn dp_bypass_lvl(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_dp_bypass_lvl(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Enables frame rate mode for the policer, where policer rates are measured in frames per second instead of bits per second.

    ///

    /// 0: Rates measured in bits per second 1: Rates measured in frames per second
    pub fn frame_rate_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_frame_rate_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// If set, BDLB policing is disabled.
    pub fn hier_dlb_dis(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_hier_dlb_dis(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
