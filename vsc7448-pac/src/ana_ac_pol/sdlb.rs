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
/// Configuration of DLB policer
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DLB_CFG(u32);
impl DLB_CFG {
    /// Configuration of the drop precedence change for green frames being remarked to yellow. For such frames, the DP level will be incremented with CIR_INC_DP_VAL.
    ///
    /// 0: DP is not increased. n: DP is increased with n for PIR traffic.
    #[inline(always)]
    pub fn cir_inc_dp_val(&self) -> u32 {
        (self.0 & 0x3000) >> 12
    }
    #[inline(always)]
    pub fn set_cir_inc_dp_val(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 12;
        self.0 &= !0x3000;
        self.0 |= value;
    }
    /// Configuration of dual leaky bucket color awareness. Frames with DP below or equal to COLOR_AWARE_LVL is treated as green. Frames with DP above COLOR_AWARE_LVL are treated as yellow.
    ///
    /// 0: DLB policer is color aware. Incoming frames with DP=0 are green and frames with DP>0 are yellow. 1: DLB policer is color aware. Incoming frames with DP<=1 are green and frames with DP>1 are yellow. 2: DLB policer is color aware. Incoming frames with DP<=2 are green and frames with DP>2 are yellow. 3: DLB policer is color unaware. All incoming frames are green.
    #[inline(always)]
    pub fn color_aware_lvl(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    #[inline(always)]
    pub fn set_color_aware_lvl(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 14;
        self.0 &= !0xc000;
        self.0 |= value;
    }
    /// Set Coupling Flag (MEF CF). Depending on the setting of COUPLING_MODE, LB_CFG[0] and LB_CFG[1] must be configured as follows: COUPLING_MODE=0: LB_CFG[0].RATE_VAL  must be configured to MEF CIR LB_CFG[0].THRES_VAL must be configured to MEF CBS LB_CFG[1].RATE_VAL  must be configured to MEF EIR LB_CFG[1].THRES_VAL must be configured to MEF EBS COUPLING_MODE=1: LB_CFG[0].RATE_VAL  must be configured to MEF CIR LB_CFG[0].THRES_VAL must be configured to MEF CBS LB_CFG[1].RATE_VAL  must be configured to MEF EIR + MEF CIR LB_CFG[1].THRES_VAL must be configured to MEF EBS + MEF CBS
    ///
    /// 0: CF=0 1: CF=1
    #[inline(always)]
    pub fn coupling_mode(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_coupling_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Configures if stripped encapsulation data (normalized data) is policed by the policer.
    ///
    /// 0: Encapsulation data is counted as frame data. 1: Encapsulation data in not counted as frame data.
    #[inline(always)]
    pub fn encap_data_dis(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_encap_data_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Value added to each frame before updating the bucket. Gap value range: -64 to +63 in two's complement format. Setting GAP_VALUE to 20 corresponds to a line-rate measurement, since on the line each frame will be preceded by 12 bytes of IFG and 8 bytes of preamble. Setting GAP_VALUE to 0 corresponds to a data-rate measurement.
    ///
    /// 0x40: -64 0x41: -63 ... 0x7F: -1 0x00: 0 0x01: 1 ... 0x3F: 63
    #[inline(always)]
    pub fn gap_val(&self) -> u32 {
        (self.0 & 0x1fc) >> 2
    }
    #[inline(always)]
    pub fn set_gap_val(&mut self, value: u32) {
        assert!(value <= 0x7f);
        let value = value << 2;
        self.0 &= !0x1fc;
        self.0 |= value;
    }
    /// TIMESCALE_VAL and BASE_TICK_CNT controls the the rate interval as well as the rate granularity available for LB rate configuration The rate granularity is calculated as follows: 8 / (BASE_TICK_CNT*1e-10 * 2^(3 * TIMESCALE_VAL) The rate granularity also becomes the smallest configurable rate. The largest configurable rate is granularity * (2**<width of RATE_VAL>-2)
    ///
    /// Assuming BASE_TICK_CNT= 9765, RATE_VAL width = 11 bits: 0: Granularity: 8,192,524bps. Range: 8193kbps - 16.7Gbps 1: Granularity: 1,024,066bps. Range 1024kbps - 2Gbps 2: Granularity: 128,008bps. Range: 128kbps - 262Mbps 3: Granularity: 16,001bps. Range: 16kbps - 32Mbps
    #[inline(always)]
    pub fn timescale_val(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_timescale_val(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// The time - in BASE_TICK_CNTs - at which last leak was performed.
    #[inline(always)]
    pub fn timestamp_val(&self) -> u32 {
        (self.0 & 0xffe0000) >> 17
    }
    #[inline(always)]
    pub fn set_timestamp_val(&mut self, value: u32) {
        assert!(value <= 0x7ff);
        let value = value << 17;
        self.0 &= !0xffe0000;
        self.0 |= value;
    }
    /// Configures action to be applied to policed frames.
    ///
    /// 0: No action. 1: Remove front ports from frame's destination port set. 2: Remove CPU ports from frame's destination port set. 3: Remove both front ports and CPU ports from frame's destination port set.
    #[inline(always)]
    pub fn traffic_type_mask(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    #[inline(always)]
    pub fn set_traffic_type_mask(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 10;
        self.0 &= !0xc00;
        self.0 |= value;
    }
}
/// Configuration of leaky bucket value
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct LB_BUCKET_VAL(u32);
impl LB_BUCKET_VAL {
    /// Number of bytes in leaky bucket.
    #[inline(always)]
    pub fn bucket_val(&self) -> u32 {
        (self.0 & 0xffffe00) >> 9
    }
    #[inline(always)]
    pub fn set_bucket_val(&mut self, value: u32) {
        assert!(value <= 0x7ffff);
        let value = value << 9;
        self.0 &= !0xffffe00;
        self.0 |= value;
    }
    /// Number of subbytes in leaky bucket.
    #[inline(always)]
    pub fn rem_val(&self) -> u32 {
        self.0 & 0x1ff
    }
    #[inline(always)]
    pub fn set_rem_val(&mut self, value: u32) {
        assert!(value <= 0x1ff);
        self.0 &= !0x1ff;
        self.0 |= value;
    }
}
/// Threshold and rate configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct LB_CFG(u32);
impl LB_CFG {
    /// Specify rate in steps of configured granularity. The rate granularity is configured in TIMESCALE_VAL.
    ///
    /// 0: Disable leak. For THRES_VAL = 0 bucket is always closed. For THRES_VAL > 0, the configured burst size is available. 1: 1 * granularity 2: 2 * granularity ... max_value-1: (max_value-1)*granularity max_value: Disable leaky bucket (always open)
    #[inline(always)]
    pub fn rate_val(&self) -> u32 {
        self.0 & 0x7ff
    }
    #[inline(always)]
    pub fn set_rate_val(&mut self, value: u32) {
        assert!(value <= 0x7ff);
        self.0 &= !0x7ff;
        self.0 |= value;
    }
    /// Policer threshold size (a.ka. burst capacity). Unit is 2048 bytes
    ///
    /// 0: Threshold = 0 bytes (no burst allowed) 1: Threshold = 2048 bytes n: Threshold = n x 2048 bytes
    #[inline(always)]
    pub fn thres_val(&self) -> u32 {
        (self.0 & 0x7f0000) >> 16
    }
    #[inline(always)]
    pub fn set_thres_val(&mut self, value: u32) {
        assert!(value <= 0x7f);
        let value = value << 16;
        self.0 &= !0x7f0000;
        self.0 |= value;
    }
}
/// Configuration of various LB policer handles
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MISC_CFG(u32);
impl MISC_CFG {
    /// Configuration of Drop Precedence bypass level. Frames with DP below DP_BYPASS_LVL bypass the policer (frames are never policed and the bucket is not updated with the frames).
    ///
    /// 0: No frames bypass the policer 1: Frames with DP = 0 bypass the policer 2: Frames with DP = 0 or 1 bypass the policer 3: Frames with DP = 0, 1 or 2 bypass the policer
    #[inline(always)]
    pub fn dp_bypass_lvl(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    #[inline(always)]
    pub fn set_dp_bypass_lvl(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 2;
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Enables frame rate mode for the policer, where policer rates are measured in frames per second instead of bits per second.
    ///
    /// 0: Rates measured in bits per second 1: Rates measured in frames per second
    #[inline(always)]
    pub fn frame_rate_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_frame_rate_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// If set, BDLB policing is disabled.
    #[inline(always)]
    pub fn hier_dlb_dis(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_hier_dlb_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
