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
/// Holds DEVCPU specific Flow Control configuration signals
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct CPU_FC_CFG(u32);
impl CPU_FC_CFG {
    /// This field determines if the ASM must assert the flow control signal to the CPU device when the ASM FIFO fill level exceeds the watermark given in	   CPU_FC_WM.
    ///
    /// '0': Flow control is disabled. '1': Flow control is enabled.
    pub fn cpu_fc_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_cpu_fc_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// This field determines the ASM FIFO fill level required for the ASM to activate FC. The fill level is given by the number of complete cells in the FIFO that are ready to be passed to the ANA block.
    ///
    /// 0: Flow control is activated if the ASM FIFO of the DEVCPU holds 1 or more complete cells. 1: Flow control is activated if the ASM FIFO of the DEVCPU holds 2 or more complete cells. --- X: Flow control is activated if the ASM FIFO of the DEVCPU holds X+1 or more complete cells.
    pub fn cpu_fc_wm(&self) -> u32 {
        (self.0 & 0xe) >> 1
    }
    pub fn set_cpu_fc_wm(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0xe);
        self.0 &= !0xe;
        self.0 |= value;
    }
}
/// Configure custom VLAN tag for injection
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct INJ_VLAN_CFG(u32);
impl INJ_VLAN_CFG {
    /// The TPID used for VLAN tag matching when injection with long IFH prefix is selected in INJ_FORMAT_CFG.
    pub fn inj_tpid_cfg(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_inj_tpid_cfg(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// The VID used for VLAN tag matching when injection with long IFH prefix is selected in INJ_FORMAT_CFG.
    pub fn inj_vid_cfg(&self) -> u32 {
        (self.0 & 0xfff0000) >> 16
    }
    pub fn set_inj_vid_cfg(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xfff0000);
        self.0 &= !0xfff0000;
        self.0 |= value;
    }
}
/// MAC Address Configuration Register (MSB)
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MAC_ADDR_HIGH_CFG(u32);
impl MAC_ADDR_HIGH_CFG {
    /// Upper 24 bits of MAC address. The MAC address is used when filtering incoming Pause Control Frames - i.e. when the ASM detemines whether or not a pause value must be passed to the DSM.
    ///
    /// The resulting MAC address of a device is determined as: MAC_ADDR_HIGH  & MAC_ADDR_LOW.
    pub fn mac_addr_high(&self) -> u32 {
        self.0 & 0xffffff
    }
    pub fn set_mac_addr_high(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// MAC Address Configuration Register (LSB)
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct MAC_ADDR_LOW_CFG(u32);
impl MAC_ADDR_LOW_CFG {
    /// Lower 24 bits of MAC address. The MAC address is used when filtering incoming Pause Control Frames - i.e. when the ASM detemines whether or not a pause value must be passed to the DSM.
    ///
    /// The resulting MAC address of a device is determined as: MAC_ADDR_HIGH  & MAC_ADDR_LOW.
    pub fn mac_addr_low(&self) -> u32 {
        self.0 & 0xffffff
    }
    pub fn set_mac_addr_low(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// Holds configuration related to Pause frame detection.
///
/// This register control whether pause and control frames should be forwarded or terminated at ingress.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PAUSE_CFG(u32);
impl PAUSE_CFG {
    /// This field indicates whether or not the ASM must discard Control frames with type 0x8808 not being Pause frames, to the IQS by asserting the abort signal. One configuration bit is defined for each port.
    ///
    /// '0': The ASM must not discard Control frames. '1': The ASM must discard Control frames to the IQS by asserting the abort signal.
    pub fn abort_ctrl_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_abort_ctrl_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// This field indicates whether or not the ASM must discard a valid Pause frame to the IQS by asserting the abort signal. One configuration bit is defined for each port.
    ///
    /// '0': The ASM must not discard valid Pause frames. '1': The ASM must discard valid Pause frames to the IQS by asserting the abort signal, but the Pause value must still be used to stall the egress data flow.
    pub fn abort_pause_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_abort_pause_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Port configuration
///
/// This register holds port configuration bit groups
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PORT_CFG(u32);
impl PORT_CFG {
    /// Disables the CSC statistics counters in the ASM for the port. Set this when the port utilizes a DEV10G device as this handles the statistics locally in the device.
    pub fn csc_stat_dis(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_csc_stat_dis(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// This field determines if the ASM must abort mark frames that become older than 16-24 ms before and EOF is received.
    ///
    /// '0': Aging enabled. '1': Aging disabled.
    pub fn frm_aging_dis(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_frm_aging_dis(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// If this field is set the ASM remove the first 4 bytes of the payload and insert it into the HIH field of the IFH.
    pub fn hih_after_preamble_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_hih_after_preamble_ena(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// If this field is set the ASM will ignore any abort indication received on the TAXI interface for the port.
    pub fn ign_taxi_abort_ena(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_ign_taxi_abort_ena(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Configure discard behaviour depending on matching result of the selected injection format. This setting is only valid for injection formats with short or long prefix.
    ///
    /// 0: Discard none 1: Discard frames with wrong injection format 2: Discard frames with correct injection format
    pub fn inj_discard_cfg(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    pub fn set_inj_discard_cfg(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x18);
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// Set the mode for the formatting of incoming frames. Four different modes can be selected: - Normal mode (No IFH) - IFH without prefix - IFH with short prefix - IFH with long prefix If one of the IFH modes are selected incoming frames are expected to contain the selected prefix followed by an IFH as the first part of the frame. Frames are forwarded based on the contents in the IFH instead of normal forwarding. Three different prefix modes are supported: - No prefix. - IFH short prefix:. any DMAC, any SMAC, EtherType=0x8880, payload=0x0007 - IFH long prefix: any DMAC, any SMAC, VLAN Tag, EtherType=0x8880, payload=0x0007. In the IFH modes, if the incoming frame's format does not comply with the prefix, then IFH_PREFIX_ERR_STICKY is set.
    ///
    /// 0: Normal mode (No IFH) 1: IFH without prefix 2: IFH with short prefix 3: IFH with long prefix
    pub fn inj_format_cfg(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_inj_format_cfg(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x6);
        self.0 &= !0x6;
        self.0 |= value;
    }
    /// If this field is set the ASM does not expect the incoming frame data to have a preamble prepended.
    pub fn no_preamble_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_no_preamble_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// This field determines if the ASM mustzero-pad Ethernet frames that are less than 64 bytes.
    ///
    /// '0': Padding is disabled. Frames that are less than 64 bytes and have not been abort marked are passed to the ANA block 'as is'. Frames that are less than 64 bytes and have been abort marked are normally discarded silently by the ASM. '1': Padding is enabled. If the resulting frame size will be less than 64 bytes, the frame is zero-padded, so that the resulting frame size is 64 bytes.
    pub fn pad_ena(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_pad_ena(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// If this field is set the ASM will not store the first 8 bytes from the packet. This must be enabled when injecting with IFH without prefix from an extenal CPU (INJ_FORMAT_CFG=1).
    pub fn skip_preamble_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_skip_preamble_ena(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// This bit defines if the ASM must be Vstax2 aware or not. If Vstax2 awareness is enabled and a frame holds a Vstax2 header following the SMAC address, this Vstax2 header is removed from the frame and placed in the IFH and the vstax_avail and update_fcs fields in the IFH will be set, so that the frame FCS is recalculated in the egress direction. If Vstax2 awareness is disabled or a frame does not hold a Vstax2 header, no bytes will be removed from the frame and the vstax_hdr, vstax_avail and fcs_update fields in the IFH will be cleared. When Vstax2 awareness is enabled INJ_FORMAT_CFG must be set to 0
    ///
    /// 0: Vstax2 awareness is disabled. 1: Vstax2 awareness is enabled.
    pub fn vstax2_awr_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vstax2_awr_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Statistics counter configuration
///
/// Register that contains the bitgroups to configure/control the statistics counters.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct STAT_CFG(u32);
impl STAT_CFG {
    /// Setting of this bit initiates the clearing of all statistics counter.
    ///
    /// '0': No action '1': Stat cnt clr (Bit is automatically cleared)
    pub fn stat_cnt_clr_shot(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_stat_cnt_clr_shot(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
