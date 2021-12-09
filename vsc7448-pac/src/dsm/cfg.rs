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
/// Configuration per device buffer.
///
/// Miscellaneous configurations per device buffer.
#[derive(From, Into)]
pub struct BUF_CFG(u32);
impl BUF_CFG {
    /// Enable aging of frames stuck in the DSM buffer system for long periods. Aging is done by comparing the age era stamped into the frame with the current age era. This check is only performed at SOF. If the current age era is more than one higher than the era stamped into the frame, then the frame is discarded.

    ///

    /// '0': Aging disabled. '1': Aging enabled.
    pub fn aging_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_aging_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Disables the CSC statistics counters in the DSM for the port. Set this when the port utilizes a DEV10G device as this handles the statistics locally in the device.
    pub fn csc_stat_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_csc_stat_dis(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Clear a single buffer in the DSM.
///
/// This register controls clearing of buffers in the DSM.
#[derive(From, Into)]
pub struct CLR_BUF(u32);
impl CLR_BUF {
    /// A write to this register clears the indicated buffer. No other buffers will be influenced by clearing this buffer. Encoding: 1: Clears buffer for port 0 2: Clears buffer for port 1 2^N: Clears buffer for port N If N > 31 Use respective next register.

    ///

    /// '0': No action '1': Buffer is cleared (Bit is automatically cleared)
    pub fn clr_buf(&self) -> u32 {
        self.0
    }
    pub fn set_clr_buf(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Data register for core memory access.
#[derive(From, Into)]
pub struct CM_DATA(u32);
impl CM_DATA {
    /// Data register for core memory access. Wider memories are big endian mapped into the 32 BIT inspection space.
    pub fn cm_data(&self) -> u32 {
        self.0
    }
    pub fn set_cm_data(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Token count tx stop watermark
///
/// Token count tx stop watermark
#[derive(From, Into)]
pub struct DEV_TX_STOP_WM_CFG(u32);
impl DEV_TX_STOP_WM_CFG {
    /// Set this to '1' when a 10G capable port runs at speeds below 10G.
    pub fn dev10g_shadow_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_dev10g_shadow_ena(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// When the token counter value for the device exceeds this value the DSM will stop transmission to the device. When set to 0 a hardware calculated default value is used. When a port is configured for HDX this WM must be set to 1.
    pub fn dev_tx_stop_wm(&self) -> u32 {
        self.0 & 0x3f
    }
    pub fn set_dev_tx_stop_wm(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
    /// Set this to 0 to disable the fast startup of frames in the token system.
    pub fn fast_startup_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_fast_startup_ena(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
}
/// FC configuration for Ethernet ports.
///
/// FC configuration for Ethernet ports.
#[derive(From, Into)]
pub struct ETH_FC_CFG(u32);
impl ETH_FC_CFG {
    /// Obey FC status received from ANA
    pub fn fc_ana_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_fc_ana_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Obey FC status received from QSYS
    pub fn fc_qs_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_fc_qs_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PFC configuration for Ethernet ports.
///
/// PFC configuration for Ethernet ports.
#[derive(From, Into)]
pub struct ETH_PFC_CFG(u32);
impl ETH_PFC_CFG {
    /// Enable PFC operation for the port.
    pub fn pfc_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pfc_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Minimum time between two PFC PDUs when PFC state changes after transmission of PFC PDU.
    pub fn pfc_min_update_time(&self) -> u32 {
        (self.0 & 0x1fffc) >> 2
    }
    pub fn set_pfc_min_update_time(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x1fffc);
        self.0 &= !0x1fffc;
        self.0 |= value;
    }
    /// Upon sending  PFC PDU with flow control deasserted for all priorities, enforce a PFC_MIN_UPDATE_TIME delay before allowing transmission of next PFC PDU.
    pub fn pfc_xoff_min_update_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_pfc_xoff_min_update_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Configuration register for IPG shrink mode
///
/// Configuration register for IPG shrink mode
#[derive(From, Into)]
pub struct IPG_SHRINK_CFG(u32);
impl IPG_SHRINK_CFG {
    /// Enable for preamble shrink in IPG shrink mode of DEV10G, DEV24G. Preamble shrink is only allowed when IPG shrink mode is enabled.
    pub fn ipg_pream_shrink_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ipg_pream_shrink_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enable for IPG shrink mode of DEV10G, DEV24G. In shrink mode frame gap compensation is depending on frame size.
    pub fn ipg_shrink_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ipg_shrink_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// MAC Address Base Configuration Register - bits 47-24.
///
/// MAC base address. Used when generating Pause Control Frames with the specified MAC address.
#[derive(From, Into)]
pub struct MAC_ADDR_BASE_HIGH_CFG(u32);
impl MAC_ADDR_BASE_HIGH_CFG {
    /// Bits 47-24 of MAC base address. Used when generating Pause Control Frames with the specified MAC address.
    pub fn mac_addr_high(&self) -> u32 {
        self.0 & 0xffffff
    }
    pub fn set_mac_addr_high(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// MAC Address Base Configuration Register - bits 23-0.
///
/// MAC base address. Used when generating Pause Control Frames with the specified MAC address.
#[derive(From, Into)]
pub struct MAC_ADDR_BASE_LOW_CFG(u32);
impl MAC_ADDR_BASE_LOW_CFG {
    /// Bits 23-0 of MAC base address. Used when generating Pause Control Frames with the specified MAC address.
    pub fn mac_addr_low(&self) -> u32 {
        self.0 & 0xffffff
    }
    pub fn set_mac_addr_low(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// MAC Configuration Register.
///
/// Contains configuration for flowcontrol and operation in FDX or HDX for Ethernet ports connected to the DSM.
#[derive(From, Into)]
pub struct MAC_CFG(u32);
impl MAC_CFG {
    /// Enables HDX backpressure instead of FDX FC when FC is generated. Note: 10G and 24G ports can not run HDX, so for those ports this field must always be set to '0'.

    ///

    /// '0': FDX FC is used. '1': HDX backpreassure is used.
    pub fn hdx_backpressure(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_hdx_backpressure(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Defines whether or not each pause frame will be sent twice.

    ///

    /// '0': Send one pause frame two times per pause period. '1': Send two pause frames back to back two times per pause period.
    pub fn send_pause_frm_twice(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_send_pause_frm_twice(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Tx Pause Value: The pause value inserted in each Pause Control Frame in the Tx direction. It is also used to schedule the transmission of Pause Control Frames when Flow Control is enabled and active. If flow control is enabled, the pause value must be set to a value different from 0, based on the link characteristics.

    ///

    /// 0x0000: 0 quanta (512-bit times) 0x0001: 1 quanta ... 0xFFFF: 64K-1 quanta.
    pub fn tx_pause_val(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_tx_pause_val(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    /// TX pause zero on de-assert. Determines whether or not a pause control frame with pause value zero is transmitted when flow control is de-asserted.

    ///

    /// '0': No pause frame with value zero is transmitted. '1': Pause frame with value zero is transmitted when flow control becomes inactive.
    pub fn tx_pause_xon_xoff(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_tx_pause_xon_xoff(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// txAdditionalFrameOverhead configuration
///
/// Configuration of the txAdditionalFrameOverhead in RateLimit
#[derive(From, Into)]
pub struct RATE_CTRL(u32);
impl RATE_CTRL {
    /// If Rate Limit mode Frame Rate Overhead is enabled this bitgroup is used for configuration of txAdditionalFrameOverhead.
    pub fn frm_gap_comp(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_frm_gap_comp(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xff000000);
        self.0 &= !0xff000000;
        self.0 |= value;
    }
}
/// Pause frame handling in RX direction
///
/// Pause frame handling in RX direction
#[derive(From, Into)]
pub struct RX_PAUSE_CFG(u32);
impl RX_PAUSE_CFG {
    /// Configures for each Ethernet port whether flow control is obeyed locally in the DSM or if the flow_control information is sent to the SCH. If not all priorities should obey flow control for this port, then the DSM should not stop the data stream in a flow control case, but let the SCH stop the traffic to avoid head of line blocking. If all priorities should obey flow control for this port, then the DSM should stop the data stream locally to be able to pass 802.3 conformance testing. If the data stream is stopped by the SCH frames in the OQS can not be aged, thus the Allocation Bitmaps in the QS must not recover lost elements.

    ///

    /// '0': This Ethernet port obeys flow control locally in the DSM. '1': This Ethernet port sends flow control information to the SCH and does not stop traffic in the DSM.
    pub fn fc_obey_local(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_fc_obey_local(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Rx Pause Enable: Enables Flow Control in Rx direction:

    ///

    /// '0': Flow Control Disabled '1': Flow Control Enabled.
    pub fn rx_pause_en(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_rx_pause_en(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// SCH stop fill level
///
/// SCH stop fill level
#[derive(From, Into)]
pub struct SCH_STOP_WM_CFG(u32);
impl SCH_STOP_WM_CFG {
    /// DSM buffer fill level at which SCH is stopped to send to this device.

    ///

    /// The SCH is stopped when the fill level is above the configured value. When set to 0 the hardware will use a default watermark set to the mid of the buffer.
    pub fn sch_stop_wm(&self) -> u32 {
        self.0 & 0x1ff
    }
    pub fn set_sch_stop_wm(&mut self, value: u32) {
        assert!(value <= 0x1ff);
        self.0 &= !0x1ff;
        self.0 |= value;
    }
}
/// Transmit start fill level
///
/// Transmit start fill level
#[derive(From, Into)]
pub struct TX_START_WM_CFG(u32);
impl TX_START_WM_CFG {
    /// The number of 4-bytes words in DSM buffer required before the DSM will start to transmit on the taxi bus. The fill level must above a certain threshold to avoid underflow in the devices. Single cell frames will be transfered immediately.

    ///

    /// The DSM will not begin transmission until the fill level is above this watermark or one complete frame is present in the buffer.
    pub fn tx_start_wm(&self) -> u32 {
        self.0 & 0x1ff
    }
    pub fn set_tx_start_wm(&mut self, value: u32) {
        assert!(value <= 0x1ff);
        self.0 &= !0x1ff;
        self.0 |= value;
    }
}
