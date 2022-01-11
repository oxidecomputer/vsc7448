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
/// Configuration of ACL policer parameters
///
/// Only frames with an VCAP IS2 action with POLICE_ENA=1 are subject to ACL policing. The policer index is then controlled by the VCAP IS2 action field POLICE_IDX.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_ACL_CTRL(u32);
impl POL_ACL_CTRL {
    /// Configures the pipeline point per ACL policer. When injecting or looping at a pipeline point after ACL_PIPELINE_PT will not cause ACL policing. When extracting at a pipeline point before ACL_PIPELINE_PT will not cause ACL policing.
    ///
    /// 0: NONE 1: ANA_VRAP 2: ANA_PORT_VOE 3: ANA_CL 4: ANA_CLM 5: ANA_IPT_PROT 6: ANA_OU_MIP 7: ANA_OU_SW 8: ANA_OU_PROT 9: ANA_OU_VOE 10: ANA_MID_PROT 11: ANA_IN_VOE 12: ANA_IN_PROT 13: ANA_IN_SW 14: ANA_IN_MIP 15: ANA_VLAN
    pub fn acl_pipeline_pt(&self) -> u32 {
        (self.0 & 0x78) >> 3
    }
    pub fn set_acl_pipeline_pt(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 3;
        self.0 &= !0x78;
        self.0 |= value;
    }
    /// Configures the frame types to be policed by the policer. Each bit in the mask enables a specific frame type. If a frame does not match any of the enabled frame types, then the frame bypasses the policer (never discarded and the bucket is not updated with the frame).
    ///
    /// '00': Police frame (if policer is not bypassed by ACL_PIPELINE_PT or DP_BYPASS_LVL) '01': Only frames to front ports are triggering policer and only front port destinations are policed. '10': Frames to CPU are triggering policer and only CPU destinations are policed. '11': Frames to front ports and/or CPU are triggering policer and being policed.
    pub fn acl_traffic_type_mask(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_acl_traffic_type_mask(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
    /// Configuration of Drop Precedence bypass level. Frames with DP below DP_BYPASS_LVL bypass the policer (frames are never policed and the bucket is not updated with the frames).
    ///
    /// 0: No frames bypass the policer 1: Frames with DP = 0 bypass the policer 2: Frames with DP = 0 or 1 bypass the policer 3: Frames with DP = 0, 1 or 2 bypass the policer
    pub fn dp_bypass_lvl(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    pub fn set_dp_bypass_lvl(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 16;
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// Enables frame rate mode for the ACL policer, where policer rates are measured in frames per second instead of bits per second.
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
    /// Value added to each frame before updating the bucket. Gap value range: -64 to +63 in two's complement format. Setting GAP_VALUE to 20 corresponds to a line-rate measurement, since on the line each frame will be preceded by 12 bytes of IFG and 8 bytes of preamble. Setting GAP_VALUE to 0 corresponds to a data-rate measurement.
    ///
    /// 0x40: -64 0x41: -63 ... 0x7F: -1 0x00: 0 0x01: 1 ... 0x3F: 63
    pub fn gap_value(&self) -> u32 {
        (self.0 & 0x7f00) >> 8
    }
    pub fn set_gap_value(&mut self, value: u32) {
        assert!(value <= 0x7f);
        let value = value << 8;
        self.0 &= !0x7f00;
        self.0 |= value;
    }
}
/// Configuration of ACL policer rates
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_ACL_RATE_CFG(u32);
impl POL_ACL_RATE_CFG {
    /// ACL policer leaky bucket rate. Regarding unit, refer to POL_UPD_INT. Related parameters: ANA_AC_POL:POL_ALL_CFG:POL_UPD_INT_CFG.POL_UPD_INT ANA_AC_POL:POL_ALL_CFG:POL_ACL_CTRL.FRAME_RATE_ENA
    ///
    /// When POL_ACL_CTRL.FRAME_RATE_ENA is disabled, policing is performed in bits per second (bps). 0: Open until burst capacity is used, then closed. 1: Rate = 1 x <unit> bps n: Rate = n x <unit> bps When POL_ACL_CTRL.FRAME_RATE_ENA is enabled, policing is performed in frames per second (fps). 0: Open until burst capacity is used, then closed. 1: Rate = <unit> fps n: Rate = n x <unit> fps
    pub fn acl_rate(&self) -> u32 {
        self.0 & 0x7ffff
    }
    pub fn set_acl_rate(&mut self, value: u32) {
        assert!(value <= 0x7ffff);
        self.0 &= !0x7ffff;
        self.0 |= value;
    }
}
/// Configuration of ACL policer thresholds
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_ACL_THRES_CFG(u32);
impl POL_ACL_THRES_CFG {
    /// Policer threshold size (a.ka. burst capacity). Unit is 8192 bytes. Related parameters: ANA_AC_POL:POL_ALL_CFG:POL_ACL_CTRL.FRAME_RATE_ENA
    ///
    /// When POL_ACL_CTRL.FRAME_RATE_ENA is disabled burst capacity is configured in steps of 8192 bytes. 0: Always closed 1: Burst capacity = 8192 bytes n: Burst capacity = n x 8192 bytes 63: Burst capacity = 516096 bytes When POL_ACL_CTRL.FRAME_RATE_ENA is enabled burst capacity is configured in steps of 8192/2504 frames. 0: Always closed 1: Burst capacity = 1 x 8192/2504 frames n: Burst capacity = n x 8192/2504 frames 63: Burst capacity = 206 frames
    pub fn acl_thres(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_acl_thres(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}
/// Miscellaneous policer parameters
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_ALL_CFG(u32);
impl POL_ALL_CFG {
    /// If set, all ACL policer buckets are forced closed (all frames are policed and no buckets are updated). The bit must be cleared for normal operation.
    ///
    /// 0: Normal operation 1: All ACL policer buckets are closed
    pub fn acl_force_close(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_acl_force_close(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// If set, all ACL policers are initialized and buckets are open. The bit must be cleared for normal operation to resume.
    ///
    /// 0: Normal operation 1: Initialization
    pub fn acl_force_init(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_acl_force_init(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// If set, all ACL policer buckets are forced open (no frames are policed and no buckets are updated). The bit must be cleared for normal operation.
    ///
    /// 0: Normal operation 1: All ACL policer buckets are forced open
    pub fn acl_force_open(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_acl_force_open(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Enables subtraction of the DP from the priority. This enables logging of differently colored frames in different log files when using the ANA testbench.
    pub fn dbg_dp_chg_prio_ena(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_dbg_dp_chg_prio_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 31;
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Determines the mapping between internal DP value and the color used by ISDX counters. One bit for each of the four internal DP values.
    ///
    /// 0: DP value will be counted as green. 1: DP value will be counted as yellow.
    pub fn dp_to_color_map(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    pub fn set_dp_to_color_map(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 16;
        self.0 &= !0xf0000;
        self.0 |= value;
    }
    /// If set, all port policer buckets are forced closed (all frames are policed and no buckets are updated). The bit must be cleared for normal operation.
    ///
    /// 0: Normal operation 1: All port policer buckets are closed
    pub fn force_close(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_force_close(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// If set, all port policers are initialized and buckets are open. The bit must be cleared for normal operation to resume.
    ///
    /// 0: Normal operation 1: Initialization
    pub fn force_init(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_force_init(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// If set, all port policer buckets are forced open (no frames are policed and no buckets are updated). The bit must be cleared for normal operation.
    ///
    /// 0: Normal operation 1: All port policer buckets are forced open
    pub fn force_open(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_force_open(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// If set, the port policers operate on logical port numbers instead of physical port numbers.
    ///
    /// 0: Policing per physical port 1: Policing per logical port
    pub fn lport_police_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_lport_police_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enables overall signaling of flow control from the port policers to DSM. Flow control is enabled for each port policer in POL_PORT_FC_CFG.FC_ENA. Related parameters: ANA_AC_POL:POL_ALL_CFG:POL_PORT_FC_CFG.FC_ENA DSM:CFG:ETH_FC_CFG.FC_ANA_ENA
    pub fn port_fc_ena(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_port_fc_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 14;
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Enables operation of port policers in parallel. In serial mode, each of the active port policers for a port are checked one after another. If a frame is discarded by a policer, the frame is not passed on to subsequent policers. In parallel mode, each of the active port policers for a port operate on all frames (policing and bucket updates) even though one or more of the other port policers is be closed.
    ///
    /// 0: Port policer operates in serial mode. 1: Port policer operates in parallel mode.
    pub fn port_pol_in_parallel_ena(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_port_pol_in_parallel_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// If set, all priority policer buckets are forced closed (all frames are policed and no buckets are updated). The bit must be cleared for normal operation.
    ///
    /// 0: Normal operation 1: All priority policer buckets are closed.
    pub fn prio_force_close(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_prio_force_close(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// If set, all priority policers are initialized and buckets are open. The bit must be cleared for normal operation to resume.
    ///
    /// 0: Normal operation 1: Initialization
    pub fn prio_force_init(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_prio_force_init(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// If set, all priority policer buckets are forced open (no frames are policed and no buckets are updated). The bit must be cleared for normal operation.
    ///
    /// 0: Normal operation 1: All priority policer buckets are forced open
    pub fn prio_force_open(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_prio_force_open(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// If set, all storm policer buckets are forced closed (all frames are policed and no buckets are updated). The bit must be cleared for normal operation.
    ///
    /// 0: Normal operation 1: All storm policer buckets are closed
    pub fn storm_force_close(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_storm_force_close(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// If set, all storm policers are initialized and buckets are open. The bit must be cleared for normal operation to resume.
    ///
    /// 0: Normal operation 1: Initialization
    pub fn storm_force_init(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_storm_force_init(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// If set, all storm policer buckets are forced open (no frames are policed and no buckets are updated). The bit must be cleared for normal operation.
    ///
    /// 0: Normal operation 1: All storm policer buckets are forced open
    pub fn storm_force_open(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_storm_force_open(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Value added to each frame before updating the bucket. Gap value range: -64 to +63 in two's complement format. Setting STORM_GAP_VALUE to 20 corresponds to a line-rate measurement, since on the line each frame will be preceded by 12 bytes of IFG and 8 bytes of preamble. Setting STORM_GAP_VALUE to 0 corresponds to a data-rate measurement.
    ///
    /// 0x40: -64 0x41: -63 ... 0x7F: -1 0x00: 0 0x01: 1 ... 0x3F: 63
    pub fn storm_gap_value(&self) -> u32 {
        (self.0 & 0x7f000000) >> 24
    }
    pub fn set_storm_gap_value(&mut self, value: u32) {
        assert!(value <= 0x7f);
        let value = value << 24;
        self.0 &= !0x7f000000;
        self.0 |= value;
    }
    /// When set, the frame color used in statistics is based only on the color determined by the SDLB policer. Related parameters: ANA_AC:STAT_CNT_CFG_ISDX
    ///
    /// 0: DP level determines color 1: SDLB determines color
    pub fn use_sdlb_color_ena(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_use_sdlb_color_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
}
/// Configuration of port policer flow control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_PORT_FC_CFG(u32);
impl POL_PORT_FC_CFG {
    /// Enables flow control mode for the port policer. If set, pause frames are transmitted when the configured policer threshold is exceeded. If cleared, frames exceeding the configured policer threshold are discarded. PORT_FC_ENA must also be set to enable pause frames. Furthermore DSM must be configured for flow control. Related parameters: ANA_AC_POL:POL_ALL_CFG:POL_ALL_CFG.PORT_FC_ENA DSM:CFG:ETH_FC_CFG.FC_ANA_ENA
    ///
    /// 0: Discard mode 1: Flow control mode
    pub fn fc_ena(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_fc_ena(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Current flow control state for the port policer.
    ///
    /// 0: Flow control is inactive 1: Flow control is active
    pub fn fc_state(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_fc_state(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
}
/// Policer diagnostic information
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_STICKY(u32);
impl POL_STICKY {
    /// Set if an ACL policer is active. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: ACL policer is active (frames added to leaky buckets)
    pub fn pol_acl_active_sticky(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_pol_acl_active_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Set if an ACL policer has been bypassed due to a frame's DP level. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: ACL policer has been bypassed
    pub fn pol_acl_bypass_sticky(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_pol_acl_bypass_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Set if frame has been dropped due to ACL policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: ACL policer drop event has occurred
    pub fn pol_acl_drop_sticky(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_pol_acl_drop_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Set if an ACL policer has been bypassed due to a pipeline point. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: policer has been bypassed
    pub fn pol_acl_pt_bypass_sticky(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_pol_acl_pt_bypass_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Set if frame has been dropped by a SDLB policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: DLB policer drop event has occurred
    pub fn pol_dlb_drop_sticky(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_pol_dlb_drop_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Set if a service policer has been bypassed due to a pipeline point. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: policer has been bypassed
    pub fn pol_dlb_pt_bypass_sticky(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_pol_dlb_pt_bypass_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Set if a port policer is active. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: Port policer is active (frames added to leaky buckets)
    pub fn pol_port_active_sticky(&self) -> u32 {
        (self.0 & 0x1e0) >> 5
    }
    pub fn set_pol_port_active_sticky(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 5;
        self.0 &= !0x1e0;
        self.0 |= value;
    }
    /// Set if a port policer has been bypassed due to a frame's DP level. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: Port policer has been bypassed
    pub fn pol_port_bypass_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pol_port_bypass_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set if a port policer has removed the CPU ports from a frame's destination set due to policing. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: A port policer has removed the CPU ports from a frame's destination set due to policing
    pub fn pol_port_drop_cpu_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_pol_port_drop_cpu_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set if a port policer has removed the front ports from a frame's destination set due to policing. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: A port policer has removed the front ports from a frame's destination set due to policing
    pub fn pol_port_drop_fwd_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_pol_port_drop_fwd_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set if a port policer's flow control state has been cleared. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: Port policer flow control state has been cleared
    pub fn pol_port_fc_clear_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_pol_port_fc_clear_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set if flow control has been active for a port policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: Port policer flow control has been activated
    pub fn pol_port_fc_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_pol_port_fc_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set if an Port policer has been bypassed due to a pipeline point. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: policer has been bypassed
    pub fn pol_port_pt_bypass_sticky(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_pol_port_pt_bypass_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Set if a priority policer is active. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: Priority policer is active (frames added to leaky buckets)
    pub fn pol_prio_active_sticky(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_pol_prio_active_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Set if a priority policer has been bypassed due to a frame's DP level. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: Priority policer has been bypassed
    pub fn pol_prio_bypass_sticky(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_pol_prio_bypass_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 14;
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Set if frame has been dropped due to priority policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: PRIO policer drop event has occurred
    pub fn pol_prio_drop_sticky(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_pol_prio_drop_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Set if a priority policer has been bypassed due to a pipeline point. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: policer has been bypassed
    pub fn pol_prio_pt_bypass_sticky(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_pol_prio_pt_bypass_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Set if a storm policer is active. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: Storm policer is active (frames added to leaky buckets)
    pub fn pol_storm_active_sticky(&self) -> u32 {
        (self.0 & 0x3fc00000) >> 22
    }
    pub fn set_pol_storm_active_sticky(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 22;
        self.0 &= !0x3fc00000;
        self.0 |= value;
    }
    /// Set if a storm policer has removed the CPU ports from a frame's destination set due to policing. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: A storm policer has removed the CPU ports from a frame's destination set due to policing
    pub fn pol_storm_drop_cpu_sticky(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_pol_storm_drop_cpu_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Set if a storm policer has removed the front ports from a frame's destination set due to policing. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: A storm policer has removed the front ports from a frame's destination set due to policing
    pub fn pol_storm_drop_fwd_sticky(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_pol_storm_drop_fwd_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
}
/// Policer diagnostic information
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_STICKY1(u32);
impl POL_STICKY1 {
    /// Set if frame has been dropped by a BDLB policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: BDLB policer drop event has occurred
    pub fn pol_bdlb_drop_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pol_bdlb_drop_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set if BUM policer has been active. One bit per BUM policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: BUM policer has been active.
    pub fn pol_bum_slb_active_sticky(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    pub fn set_pol_bum_slb_active_sticky(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 2;
        self.0 &= !0x1c;
        self.0 |= value;
    }
    /// Set if frame has been dropped by a BUM policer. One bit per BUM policer. Bit is cleared by writing a 1 to this position.
    ///
    /// 0: No event has occurred 1: BUM policer drop event has occurred
    pub fn pol_bum_slb_drop_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_pol_bum_slb_drop_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Configuration of storm policer parameters
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_STORM_CTRL(u32);
impl POL_STORM_CTRL {
    /// Controls policing of frames to the individual CPU queues for the storm policers. If a bit is set in this mask, frames to the corresponding CPU queue are allowed to be policed (frames may get discarded).
    ///
    /// '00..00': Disable policing of frames to all CPU queues 'xx..x1 ': Allow policing of frames to CPU queue #0 ... '1x..xx' : Allow policing of frames to CPU queue #n
    pub fn storm_cpu_qu_mask(&self) -> u32 {
        (self.0 & 0x3fc00) >> 10
    }
    pub fn set_storm_cpu_qu_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 10;
        self.0 &= !0x3fc00;
        self.0 |= value;
    }
    /// Enable frame rate mode for the storm policer, where policer rates are measured in frames per second instead of bits per second.
    ///
    /// 0: Rates measured in bits per second 1: Rates measured in frames per second
    pub fn storm_frame_rate_ena(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_storm_frame_rate_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Enables removing the CPU ports from a policed frame's destination set.
    ///
    /// 0: The policer does not remove the CPU ports from the destination set for a policed frame. 1: The policer removes the CPU ports from the destination set for a policed frame.
    pub fn storm_limit_cpu_traffic_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_storm_limit_cpu_traffic_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Enables removing the front ports from a policed frame's destination set.
    ///
    /// 0: The policer does not remove the front ports from the destination set for a policed frame. 0: The policer removes the front ports from the destination set for a policed frame.
    pub fn storm_limit_noncpu_traffic_ena(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_storm_limit_noncpu_traffic_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Configures the frame types to be policed by the policer. Each bit in the mask enables policing of a specific frame type. Multiple frame types can be enabled at the same time and a frame can belong to multiple frame types. If a frame belongs to one or more enabled frame types, then the frame is policed. The only exception to this is if the CPU queue bit is cleared and the frame is destined to a CPU queue in the CPU_QU_MASK. In this case the frame is NOT policed (by this policer), regardless of other settings in TRAFFIC_TYPE_MASK. If a frame does not match any of the enabled frame types, then the frame bypasses the policer (never discarded and the bucket is not updated with the frame). Frame bypassing one storm policer, may be subject to policing by one of the other storm policers. Related parameters: ANA_AC_POL:POL_ALL_CFG:POL_STORM_CTRL.STORM_CPU_QU_MASK
    ///
    /// 'xxxxxxx1' : Known multicast frames are policed. 'xxxxxx1x' : Known broadcast frames are policed. 'xxxxx1xx' : Known unicast frames are policed. 'xxxx1xxx' : Unknown multicast frames are policed. 'xxx1xxxx' : Unknown broadcast frames are policed. 'xx1xxxxx' : Unknown unicast frames are policed. 'x0xxxxxx' : Frames to a CPU queue selected by CPU_QU_MASK bypass the policer, regardless of other criterias in TRAFFIC_TYPE_MASK. 'x1xxxxxx' : Frames to a CPU queue selected by CPU_QU_MASK are policed. '1xxxxxxx' : Learn frames are policed. '00000000': Disable policer.
    pub fn storm_traffic_type_mask(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_storm_traffic_type_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Configuration of storm policer rates
///
/// These registers configure the rates of the storm policers.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_STORM_RATE_CFG(u32);
impl POL_STORM_RATE_CFG {
    /// Storm policer leaky bucket rate. Regarding unit, refer to POL_UPD_INT. Related parameters: ANA_AC_POL:POL_ALL_CFG:POL_UPD_INT_CFG.POL_UPD_INT ANA_AC_POL:POL_ALL_CFG:POL_STORM_CTRL.STORM_FRAME_RATE_ENA
    ///
    /// When POL_STORM_CTRL.STORM_FRAME_RATE_ENA is disabled, policing is performed in bits per second (bps). 0: Open until burst capacity is used, then closed. 1: Rate = 1 x <unit> bps n: Rate = n x <unit> bps When POL_STORM_CTRL.STORM_FRAME_RATE_ENA is enabled, policing is performed in frames per second (fps). 0: Open until burst capacity is used, then closed. 1: Rate = <unit> fps n: Rate = n x <unit> fps
    pub fn storm_rate(&self) -> u32 {
        self.0 & 0x7ffff
    }
    pub fn set_storm_rate(&mut self, value: u32) {
        assert!(value <= 0x7ffff);
        self.0 &= !0x7ffff;
        self.0 |= value;
    }
}
/// Configuration of storm policer thresholds
///
/// These registers configure the thresholds of the storm policers
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_STORM_THRES_CFG(u32);
impl POL_STORM_THRES_CFG {
    /// Policer threshold size (a.ka. burst capacity). Unit is 8192 bytes. Related parameters: ANA_AC_POL:POL_ALL_CFG:POL_STORM_CTRL.STORM_FRAME_RATE_ENA
    ///
    /// When POL_STORM_CTRL.STORM_FRAME_RATE_ENA is disabled burst capacity is configured in steps of 8192 bytes. 0: Always closed 1: Burst capacity = 8192 bytes n: Burst capacity = n x 8192 bytes 63: Burst capacity = 516096 bytes When POL_STORM_CTRL.STORM_FRAME_RATE_ENA is enabled burst capacity is configured in steps of 8192/2504 frames. 0: Always closed 1: Burst capacity = 1 x 8192/2504 frames n: Burst capacity = n x 8192/2504 frames 63: Burst capacity = 206 frames
    pub fn storm_thres(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_storm_thres(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}
/// Policer update interval
///
/// Configure the leaky bucket update interval for the ACL, storm- and port policers. This configuration affects the policing rate unit for these 3 policers. By setting this parameter to a clock frequency dependent value, the rate unit can be kept identical/similar across different system clock frequencies.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct POL_UPD_INT_CFG(u32);
impl POL_UPD_INT_CFG {
    /// This configuration will affect the policing rate unit for the storm, ACL and Port policers. The rate unit is calculated as follows: Rate unit = SYS_CLK / (POL_UPD_INT * 16) bps Recommended value and corresponding rate unit: 52.08Mhz: 130 => 25040bps 78.125Mhz: 195 => 25040bps 156.25Mhz: 390 => 25040bps 250MHz: 624 => 25040bps Frame rate mode is also affected be this setting. The default frame rate unit is 10fps (frames per second). This is scaled according to this formula: Frame rate = 10fps * (Rate unit / 25040bps)
    ///
    /// 0-31: Not allowed N: Update interval
    pub fn pol_upd_int(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_pol_upd_int(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
