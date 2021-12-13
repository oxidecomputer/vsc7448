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
/// Configuration of leaky bucket value
///
/// Configures each Single LB
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct LB_BUCKET_VAL(u32);
impl LB_BUCKET_VAL {
    /// Number of bytes in leaky bucket.
    pub fn bucket_val(&self) -> u32 {
        (self.0 & 0xffffe00) >> 9
    }
    pub fn set_bucket_val(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0xffffe00);
        self.0 &= !0xffffe00;
        self.0 |= value;
    }
    /// Number of subbytes in leaky bucket.
    pub fn rem_val(&self) -> u32 {
        self.0 & 0x1ff
    }
    pub fn set_rem_val(&mut self, value: u32) {
        assert!(value <= 0x1ff);
        self.0 &= !0x1ff;
        self.0 |= value;
    }
}
/// Threshold and rate configuration
///
/// Configures each Single LB
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct LB_CFG(u32);
impl LB_CFG {
    /// Specify rate in steps of configured granularity. The rate granularity is configured in TIMESCALE_VAL.
    ///
    /// 0: Disable leak. For THRES_VAL = 0 bucket is always closed. For THRES_VAL > 0, the configured burst size is available. 1: 1 * granularity 2: 2 * granularity ... max_value-1: (max_value-1)*granularity max_value: Disable leaky bucket (always open)
    pub fn rate_val(&self) -> u32 {
        self.0 & 0x7ff
    }
    pub fn set_rate_val(&mut self, value: u32) {
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
/// Configuration of various LB policer handles
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MISC_CFG(u32);
impl MISC_CFG {
    /// Enables frame rate mode for the policer, where policer rates are measured in frames per second instead of bits per second.
    ///
    /// 0: Rates measured in bits per second 1: Rates measured in frames per second
    pub fn frame_rate_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_frame_rate_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Configuration of BUM SLB policer
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct SLB_CFG(u32);
impl SLB_CFG {
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
        self.0 & 0x3
    }
    pub fn set_timescale_val(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// The time - in BASE_TICK_CNTs - at which last leak was performed.
    pub fn timestamp_val(&self) -> u32 {
        (self.0 & 0x1ffc00) >> 10
    }
    pub fn set_timestamp_val(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x1ffc00);
        self.0 &= !0x1ffc00;
        self.0 |= value;
    }
}
