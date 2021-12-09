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
/// Port policer configuration
#[derive(From, Into)]
pub struct POL_PORT_CFG(u32);
impl POL_PORT_CFG {
    /// Controls policing of frames to the individual CPU queues for the port policer. If a bit is set in this mask, frames to the corresponding CPU queue are allowed to be policed (frames may get discarded and the bucket is updated with the frame).
    ///
    /// '00..00': Disable policing of frames to all CPU queues 'xx..x1 ': Allow policing of frames to CPU queue #0 ... '1x..xx' : Allow policing of frames to CPU queue #n
    pub fn cpu_qu_mask(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_cpu_qu_mask(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xff0000);
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    /// Configuration of DP bypass level. Frames with DP below DP_BYPASS_LVL bypass the port policers (frames are never policed and the bucket is not updated with the frames).
    ///
    /// 0: No frames bypass the policer 1: Frames with DP = 0 bypass the policer 2: Frames with DP = 0 or 1 bypass the policer 3: Frames with DP = 0, 1 or 2 bypass the policer
    pub fn dp_bypass_lvl(&self) -> u32 {
        (self.0 & 0x3000) >> 12
    }
    pub fn set_dp_bypass_lvl(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x3000);
        self.0 &= !0x3000;
        self.0 |= value;
    }
    /// Enables frame rate mode for the port policers, where policer rates are measured in frames per second instead of bits per second.
    ///
    /// 0: Rates measured in bits per second 1: Rates measured in frames per second
    pub fn frame_rate_ena(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_frame_rate_ena(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Enables removing the CPU ports from a policed frame's destination set.
    ///
    /// 0: The policer does not remove the CPU ports from the destination set for a policed frame. 1: The policer removes the CPU ports from the destination set for a policed frame.
    pub fn limit_cpu_traffic_ena(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_limit_cpu_traffic_ena(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Enables removing the front ports from a policed frame's destination set.
    ///
    /// 0: The policer does not remove the front ports from the destination set for a policed frame. 0: The policer removes the front ports from the destination set for a policed frame.
    pub fn limit_noncpu_traffic_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_limit_noncpu_traffic_ena(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Configures if service frames (ISDX <> 0) bypasses the port policers.
    ///
    /// 0: All frames are applicable for port policing 1: Only non service frames are applicable for port policing
    pub fn service_bypass_ena(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_service_bypass_ena(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Configures the frame types to be policed by the policer. Each bit in the mask enables policing of a specific frame type. Multiple frame types can be enabled at the same time and a frame can belong to multiple frame types. If a frame belongs to one or more enabled frame types, then the frame is policed. The only exception to this is if the CPU queue bit is cleared and the frame is destined to a CPU queue in the CPU_QU_MASK. In this case the frame is NOT policed (by this policer), regardless of other settings in TRAFFIC_TYPE_MASK. If a frame does not match any of the enabled frame types, then the frame bypasses the policer (never discarded and the bucket is not updated with the frame). Frame bypassing one port policer, may be subject to policing by one of the other port policers. Related parameters: ANA_AC_POL:POL_PORT_CTRL[0-56]:POL_PORT_CFG.CPU_QU_MASK
    ///
    /// 'xxxxxxx1' : Known multicast frames are policed. 'xxxxxx1x' : Known broadcast frames are policed. 'xxxxx1xx' : Known unicast frames are policed. 'xxxx1xxx' : Unknown multicast frames are policed. 'xxx1xxxx' : Unknown broadcast frames are policed. 'xx1xxxxx' : Unknown unicast frames are policed. 'x0xxxxxx' : Frames to a CPU queue selected by CPU_QU_MASK bypass the policer, regardless of other criterias in TRAFFIC_TYPE_MASK. 'x1xxxxxx' : Frames to a CPU queue selected by CPU_QU_MASK are policed. '1xxxxxxx' : Learn frames are policed. '00000000': Disable policer.
    pub fn traffic_type_mask(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_traffic_type_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
