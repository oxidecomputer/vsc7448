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
/// Discard frames destined for specific CPU queues
#[derive(From, Into)]
pub struct CPUQ_DISCARD(u32);
impl CPUQ_DISCARD {
    /// Disable enqueuing of traffic to specific CPU queues.
    ///
    /// xxxxxxx1: Discard frames to CPU queue 0 xxxxxx1x: Discard frames to CPU queue 1 ... 1xxxxxxx: Discard frames to CPU queue 7
    pub fn cpuq_discard(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_cpuq_discard(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Frame copying configuration
///
/// Configures how various frame copies are to be generated in the forwarding core. Frame copies are made when the CPU should get one or more copies (some CPU queues hit), when learn updates are to be generated, and when frame mirroring is requested. Per copy reason, it is configured to which physical port the frame is to be sent, and which QoS level to use in the egress queues. 0-7: Configuration for cpu queue N. 8: Configuration for learn updates. 9-11: Configuration for mirror probe 0-2.
#[derive(From, Into)]
pub struct FRAME_COPY_CFG(u32);
impl FRAME_COPY_CFG {
    /// Physical port to send the frame to. In case of replication 8 (Learn All frames), then the 2nd port used for Learn All frames is configured in FRAME_COPY_LRNA_CFG.FRMC_PORT_LRNA_VAL.
    pub fn frmc_port_val(&self) -> u32 {
        (self.0 & 0xfc0) >> 6
    }
    pub fn set_frmc_port_val(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0xfc0);
        self.0 &= !0xfc0;
        self.0 |= value;
    }
    /// Set to enable use of use FRMC_QOS_VAL. Otherwise classified priority is used.
    pub fn frmc_qos_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_frmc_qos_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Selected QoS level to use in the egress queues
    pub fn frmc_qos_val(&self) -> u32 {
        (self.0 & 0x38) >> 3
    }
    pub fn set_frmc_qos_val(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x38);
        self.0 &= !0x38;
        self.0 |= value;
    }
    /// Use super priority enqueuing. Note that even when SP is used, QoS value still controls from which pool resources are allocated, so it may be relevant to also set FRMC_QOS_ENA=1 and FRMC_QOS_VAL.
    ///
    /// 0: Use normal queues 1: Use super priority queue bypassing all shapers 2: Use super priority queue obeying/updating port shaper 3: reserved
    pub fn frmc_sp_ena(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_frmc_sp_ena(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
/// Frame copying configuration
///
/// Configures 2nd port for sending Learn All frame copies. If only one Learn All port is to be used, this port number MUST be configured to the same port as FRMC_PORT_VAL:8.
#[derive(From, Into)]
pub struct FRAME_COPY_LRNA_CFG(u32);
impl FRAME_COPY_LRNA_CFG {
    /// Physical port to send Learn All frame to.
    pub fn frmc_port_lrna_val(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_frmc_port_lrna_val(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}
/// Forwarder mischeleaneous configuration
#[derive(From, Into)]
pub struct FWD_CTRL(u32);
impl FWD_CTRL {
    /// Process only one discard per cycle, required for egress drop statistics mode. Side effect is a slower processing of multiple drops on the same frame, causing potential head-of-line blocking.
    pub fn drop_single(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_drop_single(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Await an afi busy phase between enqueings to the AFI controller
    pub fn fwd_afi_handshake(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_fwd_afi_handshake(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Counter for number of frames forwarding aborted due to forward pressure
#[derive(From, Into)]
pub struct FWD_PRESS_DROP_CNT(u32);
impl FWD_PRESS_DROP_CNT {
    /// Counts number of frames discarded due to forward pressure, since queue system reset.
    pub fn fwd_press_drop_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_fwd_press_drop_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Mirroring configuration
#[derive(From, Into)]
pub struct MIRROR_CFG(u32);
impl MIRROR_CFG {
    /// One bit per mirror probe enabling generation of mirror copies also for frames discarded due to buffer discards.
    pub fn mirror_discards(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_mirror_discards(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Various switch port mode settings
#[derive(From, Into)]
pub struct SWITCH_PORT_MODE(u32);
impl SWITCH_PORT_MODE {
    /// When enabled for a port, frames -to- that port are discarded when the controlling watermarks are hit. If disabled - the frame will stay in memory until resources are available. If INGRESS_DROP_MODE or EGRESS_DROP_MODE applies for a frame copy, it will be discared.
    pub fn egress_drop_mode(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_egress_drop_mode(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set this to clear (port) and (port,qos) reservations for the destination port. This is required to be able to move queues between ports.
    pub fn egress_rsrv_dis(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_egress_rsrv_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Control whether frames forwarded to the port may use shared resources. If egress port or queue has reserved memory left to use, frame enqueuing is always allowed.
    ///
    /// 0: Use shared memory as well 1: Do not use shared memory
    pub fn egr_no_sharing(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_egr_no_sharing(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set the switching speed per port. One forwarding guaranteed every this value of clock cycles has passed Example with 10: One forwarding per 10 cycles (at 6.4 ns every 64 ns - 10Gport minimum)
    pub fn fwd_urgency(&self) -> u32 {
        (self.0 & 0x1fe00) >> 9
    }
    pub fn set_fwd_urgency(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x1fe00);
        self.0 &= !0x1fe00;
        self.0 |= value;
    }
    /// Control whether frames received on the port may use shared resources. If ingress port or queue has reserved memory left to use, frame enqueuing is always allowed.
    ///
    /// 0: Use shared memory as well 1: Do not use shared memory
    pub fn igr_no_sharing(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_igr_no_sharing(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// When enabled for a port, frames -from- that port are discarded when the controlling watermarks are hit. If disabled - the frame will stay in memory until resources are available. If INGRESS_DROP_MODE or EGRESS_DROP_MODE applies for a frame copy, it will be discared.
    pub fn ingress_drop_mode(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_ingress_drop_mode(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Enable port for any frame transfer. Frames to or from a port with PORT_ENA cleared are discarded.
    pub fn port_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_port_ena(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Enable yellow traffic to use reserved ingress space.
    pub fn yel_rsrvd(&self) -> u32 {
        (self.0 & 0x1e0) >> 5
    }
    pub fn set_yel_rsrvd(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x1e0);
        self.0 &= !0x1e0;
        self.0 |= value;
    }
}
