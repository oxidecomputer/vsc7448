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

/// Register `DEBUG_CTRL`
///
/// Internal control for debugging only
///
/// Core events.
#[derive(From, Into)]
pub struct DEBUG_CTRL(u32);
impl DEBUG_CTRL {    ///
    /// Force port to be frame pending. To be used when a port for some unknown reason gets stuck. The port configured in FLUSH_PORT will be marked pending.
    pub fn port_kick(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_port_kick(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `DWRR_ENTRY`
///
/// Configuration and status of a dwrr entry
#[derive(From, Into)]
pub struct DWRR_ENTRY(u32);
impl DWRR_ENTRY {    ///
    /// Current balance of the input
    pub fn dwrr_balance(&self) -> u32 {
        (self.0 & 0xfffff) >> 0
    }
    pub fn set_dwrr_balance(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }    ///
    /// When a specific input to an element is used, the cost is used when updating the balance.
    pub fn dwrr_cost(&self) -> u32 {
        (self.0 & 0x1f00000) >> 20
    }
    pub fn set_dwrr_cost(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x1f00000);
        self.0 &= !0x1f00000;
        self.0 |= value;
    }
}

/// Register `EQ_STAT`
///
/// Egress queue status
#[derive(From, Into)]
pub struct EQ_STAT(u32);
impl EQ_STAT {    ///
    /// Number of free frame references.
    pub fn fp_free_cnt(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_fp_free_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `EVENTS_CORE`
///
/// Internal events for debugging only
///
/// Core events.
#[derive(From, Into)]
pub struct EVENTS_CORE(u32);
impl EVENTS_CORE {    ///
    /// If an frame is added to an invalid queue in the scheduling hierarchy, this sticky bit will be set, and the violating request is see the EVENT_ENQ_ERR register.
    pub fn ev_enq_err(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_ev_enq_err(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Look in the RTL!
    pub fn ev_frd(&self) -> u32 {
        (self.0 & 0x1e) >> 1
    }
    pub fn set_ev_frd(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x1e);
        self.0 &= !0x1e;
        self.0 |= value;
    }    ///
    /// Look in the RTL!
    pub fn ev_hsch(&self) -> u32 {
        (self.0 & 0xfe0) >> 5
    }
    pub fn set_ev_hsch(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0xfe0);
        self.0 &= !0xfe0;
        self.0 |= value;
    }
}

/// Register `EVENT_ENQ_ERR`
///
/// Information about enqueueing error
#[derive(From, Into)]
pub struct EVENT_ENQ_ERR(u32);
impl EVENT_ENQ_ERR {    ///
    /// Contains last enqueuing error egress port number
    pub fn enq_err_port(&self) -> u32 {
        (self.0 & 0x1f8000) >> 15
    }
    pub fn set_enq_err_port(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x1f8000);
        self.0 &= !0x1f8000;
        self.0 |= value;
    }    ///
    /// Scheduling element being violated.
    pub fn enq_err_qno(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_enq_err_qno(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
}

/// Register `FLUSH_CTRL`
///
/// Enable flushing of selected framesy
#[derive(From, Into)]
pub struct FLUSH_CTRL(u32);
impl FLUSH_CTRL {    ///
    /// Frame transmitted on the configured port will be flushed if set.
    pub fn flush_dst(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_flush_dst(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }    ///
    /// Set to enable flushing of all frames matching the flush criterias in this register
    pub fn flush_ena(&self) -> u32 {
        (self.0 & 0x2000000) >> 25
    }
    pub fn set_flush_ena(&mut self, value: u32) {
        let value = value << 25;
        assert!(value <= 0x2000000);
        self.0 &= !0x2000000;
        self.0 |= value;
    }    ///
    /// Flushing will only affect frames from this queue or SE.
    pub fn flush_hier(&self) -> u32 {
        (self.0 & 0x7fff) >> 0
    }
    pub fn set_flush_hier(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }    ///
    /// Flushing will only affect frames on this port.
    pub fn flush_port(&self) -> u32 {
        (self.0 & 0x7e0000) >> 17
    }
    pub fn set_flush_port(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x7e0000);
        self.0 &= !0x7e0000;
        self.0 |= value;
    }    ///
    /// Frames transmitted from the configured queue specified in FLUSH_HIER will be flushed.
    pub fn flush_queue(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_flush_queue(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }    ///
    /// Frames transmitted from the configured SE index specified in FLUSH_HIER will be flushed.
    pub fn flush_se(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_flush_se(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }    ///
    /// Frame received on the configured port will be flushed if set.
    pub fn flush_src(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    pub fn set_flush_src(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x1000000);
        self.0 &= !0x1000000;
        self.0 |= value;
    }
}

/// Register `HSCH_CFG_CFG`
///
/// Configuration selection register
#[derive(From, Into)]
pub struct HSCH_CFG_CFG(u32);
impl HSCH_CFG_CFG {    ///
    /// The DWRR balances and queue shapers will be accessed for the scheduling element indexed by this field.
    pub fn cfg_se_idx(&self) -> u32 {
        (self.0 & 0x3ffc000) >> 14
    }
    pub fn set_cfg_se_idx(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x3ffc000);
        self.0 &= !0x3ffc000;
        self.0 |= value;
    }    ///
    /// Skip a hierarchy update every time this number of updates has been done. If zero, the feature is disabled. Setting to 4095 will disable hierachy updates.
    pub fn csr_grant(&self) -> u32 {
        (self.0 & 0xfff) >> 0
    }
    pub fn set_csr_grant(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }    ///
    /// The HSCH layer set in this field will be accessed by the configuration interfaces.
    pub fn hsch_layer(&self) -> u32 {
        (self.0 & 0x3000) >> 12
    }
    pub fn set_hsch_layer(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x3000);
        self.0 &= !0x3000;
        self.0 |= value;
    }
}

/// Register `HSCH_LARGE_ENA`
///
/// Enable large scheduling elements
#[derive(From, Into)]
pub struct HSCH_LARGE_ENA(u32);
impl HSCH_LARGE_ENA {    ///
    /// Bit n in replication k enables extended width on scheduling element (32k+n)*2. Scheduling element (32k+n)*2+1 must not be used if enabled. Fx. if scheduling element 180 should handle 16 inputs, HSCH_LARGE_ENA[2] bit 26 should be set to 1, and element 181 must not be used.
    pub fn hsch_large_ena(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_hsch_large_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `HSCH_MISC_CFG`
///
/// Common config for HSCH and policer module
#[derive(From, Into)]
pub struct HSCH_MISC_CFG(u32);
impl HSCH_MISC_CFG {    ///
    /// Values to add each frame when frame length adjustment is in use.
    pub fn frm_adj(&self) -> u32 {
        (self.0 & 0x1f) >> 0
    }
    pub fn set_frm_adj(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }    ///
    /// Set this to one to disabled all leaking from leaky bucket shapers
    pub fn leak_dis(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_leak_dis(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
}

/// Register `HSCH_UPDATE_STAT`
///
/// Return information about scheduler busyness
#[derive(From, Into)]
pub struct HSCH_UPDATE_STAT(u32);
impl HSCH_UPDATE_STAT {    ///
    /// Return the maximum period of constant update need. Clear by writing one to the lsb of the register.
    pub fn hsch_update_cnt(&self) -> u32 {
        (self.0 & 0x1fff) >> 0
    }
    pub fn set_hsch_update_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1fff);
        self.0 &= !0x1fff;
        self.0 |= value;
    }
}

/// Register `OUTB_CPU_SHARE_ENA`
///
/// Cellbus configuration
#[derive(From, Into)]
pub struct OUTB_CPU_SHARE_ENA(u32);
impl OUTB_CPU_SHARE_ENA {    ///
    /// When enabled, unused bandwidth sharing will be granted to the an internal CPUport, only when the calendar designated port is another internal CPU port. The OUTB_SHARE_ENA must be configured for the CPU ports when this is enabled
    pub fn outb_cpu_share_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_outb_cpu_share_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `OUTB_SHARE_ENA`
///
/// Cellbus configuration
#[derive(From, Into)]
pub struct OUTB_SHARE_ENA(u32);
impl OUTB_SHARE_ENA {    ///
    /// Sets the minimum distance between grants to an internal port. Extra grants are disabled when configured value is 0, otherwise the port seeks extra bandwidth, and the minimim distance in clock cycles is given by this value. The four replications are for internal CPU 0, internal CPU 1, VD0 and VD1. Setting a value of 14 grants extra bandwidth every 14 cycles, which for minimum sized frames corresponds to 84 bytes per 14 x 6,4 ns, or 7.5Gbps. Setting a value of 10 grants every 10 cycles, corresponding to 84 bytes per 64 ns, or 10.5Gbps. Minimum value for VD0 is 14, and 8 for the other internal ports.
    pub fn outb_share_ena(&self) -> u32 {
        (self.0 & 0xff) >> 0
    }
    pub fn set_outb_share_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}

/// Register `PFC_CFG`
///
/// Per port PFC configuration
///
/// These configurations exists per front port.
#[derive(From, Into)]
pub struct PFC_CFG(u32);
impl PFC_CFG {    ///
    /// Set the layer at which PFC status should be applied for this port. Only layers 1 and 2 supports PFC blocking of the hierarchy.
    ///
    /// 0: Dont block any branches through PFC status 1: Use pfc status for the port on layer 1 2: Use pfc status for the port on layer 2 3: Reserved
    pub fn pfc_layer(&self) -> u32 {
        (self.0 & 0xc0) >> 6
    }
    pub fn set_pfc_layer(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0xc0);
        self.0 &= !0xc0;
        self.0 |= value;
    }    ///
    /// Set to the scheduling element number which should be affected by pfc status for this port.
    pub fn pfc_se(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_pfc_se(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}

/// Register `PORT_MODE`
///
/// Per device port configuration
///
/// These configurations exists per front port and for each of the two CPU ports (11+12).
#[derive(From, Into)]
pub struct PORT_MODE(u32);
impl PORT_MODE {    ///
    /// Disable aging of all frames transmitted to the port. Frame aging related parameters: QSYS:SYSTEM:FRM_AGING.MAX_AGE HSCH:HSCH_MISC:PORT_MODE.AGE_DIS DSM:CFG:BUF_CFG.AGING_ENA
    pub fn age_dis(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_age_dis(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Select the priority mode for CPU copies transmitted on the port.
    ///
    /// 0: IFH CPU mask shows all CPU queues the frame applies to. Priority of frame set to the CPU queue the frame copy is generated for 1: IFH CPU mask shows the CPU queue number the frame copy is generated for. Priority of frame set to the priority selected for the particular frame copy (see QFWD::FRAME_COPY_CFG)
    pub fn cpu_prio_mode(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_cpu_prio_mode(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Disable dequeuing from the egress queues. Frames are not discarded, but may become aged when dequeuing is reenabled.
    pub fn dequeue_dis(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_dequeue_dis(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Increase DP by one in case scheduled frame passed due to excess rate shaper.
    pub fn eir_remark_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_eir_remark_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Enable truncation of stack learnall frames.
    pub fn trunc_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_trunc_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}

/// Register `SYS_CLK_PER`
///
/// System clock period configuration
#[derive(From, Into)]
pub struct SYS_CLK_PER(u32);
impl SYS_CLK_PER {    ///
    /// Must be set to the system clock period with unit 100 picoseconds.
    pub fn sys_clk_per_100ps(&self) -> u32 {
        (self.0 & 0xff) >> 0
    }
    pub fn set_sys_clk_per_100ps(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
