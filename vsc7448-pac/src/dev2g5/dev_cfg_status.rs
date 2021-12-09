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
/// Device Debug Config
///
/// Device1G Debug Configuration Register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct DEV_DBG_CFG(u32);
impl DEV_DBG_CFG {
    /// The TX_Size and the TX_Backoff events share the same counters. Per default the counters count TX_SIZE events. BACKOFF_CNT_ENA switches the counter to count backup events instead.
    ///
    /// 0: Count TX_SIZE events 1: Count TX_BACKOFF events
    pub fn backoff_cnt_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_backoff_cnt_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Controls the FCS update function of the device.
    ///
    /// 0: Use information from DSM to control FCS updating 1: Always update FCS 2: Never update FCS 3: Reserved
    pub fn fcs_update_cfg(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_fcs_update_cfg(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Disable of advanced rate control feature.
    ///
    /// 0: Advanced rate control active 1: Advanced rate control disabled
    pub fn ifg_len_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ifg_len_dis(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Holds the ID of the last pre counter that had an overflow. The user has to check the PRE_CNT_OFLW_STICKY up front. See the IS0076 for a mapping of the counter ID to the counter type.
    ///
    /// 0: Pre counter 0 had an overflow (if sticky bit is set) 1: Pre counter 1 had an overflow ...
    pub fn pre_cnt_oflw_id(&self) -> u32 {
        (self.0 & 0x3f000000) >> 24
    }
    pub fn set_pre_cnt_oflw_id(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x3f000000);
        self.0 &= !0x3f000000;
        self.0 |= value;
    }
    /// Determines the required fill level that must be EXCEEDED before the buffer full signal to the DSM is asserted.
    ///
    /// 0: The 'buffer full' signal is asserted as soon as the Tx FIFO holds any data 1: The 'buffer full' signal is asserted if the Tx FIFO holds more than 1 data word 2: The 'buffer full' signal is asserted if the Tx FIFO holds more than 2 data word ... 31: The 'buffer full' signal is disabled
    pub fn tx_buf_high_wm(&self) -> u32 {
        (self.0 & 0x1f00) >> 8
    }
    pub fn set_tx_buf_high_wm(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x1f00);
        self.0 &= !0x1f00;
        self.0 |= value;
    }
    /// Stores the maximum reach TX FiFo fill level.
    ///
    /// 0: Max fill level was zero 1: Max fill level was one ...
    pub fn tx_max_fill_lvl(&self) -> u32 {
        (self.0 & 0x1f0000) >> 16
    }
    pub fn set_tx_max_fill_lvl(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x1f0000);
        self.0 &= !0x1f0000;
        self.0 |= value;
    }
    /// Clears TX_MAX_FILL_LVL that holds the maximal reached TX FiFo fill level. This bit is a one shot bit that is clear automatically by hardware.
    ///
    /// 0: No action 1: Clear TX_MAX_FILL_LVL (Bit is automatically cleared)
    pub fn tx_max_fill_lvl_clr_one_shot(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_tx_max_fill_lvl_clr_one_shot(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }
}
/// Configuration for the port protectio feature
///
/// This register is used to configure the port protection feature of the device.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct DEV_PORT_PROTECT(u32);
impl DEV_PORT_PROTECT {
    /// Enables snooping of egress data from another port. The port from which egress data is copied and transmitted at the Ethernet port is determined by the PORT_PROTECT_ID configuration.
    ///
    /// 0: Port protection is disabled 1: Port protection is enabled
    pub fn port_protect_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_port_protect_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Indicates from which port egress data must be copied and transmitted at this Ethernet port. The port from which egress data is copied must always be a port that is closer to the ASM. I.e. DEV_X may be configured to snoop egress data destined for DEV_X+n, where DEV_X+n is closer to the ASM - but NOT vice versa.
    ///
    /// 0: Reserved 1: Egress data destined for DEV_1 is also transmitted by this device 2: Egress data destined for DEV_2 is also transmitted by this device ...
    pub fn port_protect_id(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_port_protect_id(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }
}
/// DEV_RST_CTRL register
///
/// Clock/Reset Control Register
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct DEV_RST_CTRL(u32);
impl DEV_RST_CTRL {
    /// Reset the MAC rx clock domain in the DEV1G. The setup registers in the DEV1G is not affected by this reset.
    ///
    /// 0: No reset 1: Reset. Note: MAC_RX_RST is NOT a one-shot operation. The clock domain remains reset until a '0' is written to MAC_RX_RST.
    pub fn mac_rx_rst(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_mac_rx_rst(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Reset the MAC tx clock domain in the DEV1G. The setup registers in the DEV1G is not affected by this reset.
    ///
    /// 0: No reset 1: Reset. Note: MAC_TX_RST is NOT a one-shot operation. The clock domain remains reset until a '0' is written to MAC_TX_RST.
    pub fn mac_tx_rst(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_mac_tx_rst(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Reset the PCS rx clock domain in the DEV1G. The setup registers in the DEV1G is not affected by this reset.
    ///
    /// 0: No reset 1: Reset. Note: PCS_RX_RST is NOT a one-shot operation. The clock domain remains reset until a '0' is written to PCS_RX_RST.
    pub fn pcs_rx_rst(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_pcs_rx_rst(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Reset the PCS tx clock domain in the DEV1G. The setup registers in the DEV1G is not affected by this reset.
    ///
    /// 0: No reset 1: Reset. Note: PCS_TX_RST is NOT a one-shot operation. The clock domain remains reset until a '0' is written to PCS_TX_RST.
    pub fn pcs_tx_rst(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_pcs_tx_rst(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// This register selects the port data rate. The no clock options is intended to save power for unused ports.
    ///
    /// 0: 10 Mbps 1: 100 Mbps 2: 1000 Mbps 3: No clock.
    pub fn speed_sel(&self) -> u32 {
        (self.0 & 0x300000) >> 20
    }
    pub fn set_speed_sel(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x300000);
        self.0 &= !0x300000;
        self.0 |= value;
    }
}
/// Sticky bit Register
///
/// Clear the sticky bits by writing a '1' in the relevant bitgroups.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct DEV_STICKY(u32);
impl DEV_STICKY {
    /// Will be set if one of the statistics pre counter have an overflow.
    ///
    /// 0: No pre counter had an overflow 1: (At least) one of the pre counters had an overflow Bit is cleared by writing a 1 to this position.
    pub fn pre_cnt_oflw_sticky(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_pre_cnt_oflw_sticky(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Indicates that an overflow has occured in the Rx Taxi FIFO.
    ///
    /// 0: No overflow has occured in the Rx Taxi FIFO 1: An overflow has occured in the Rx Taxi FIFO Bit is cleared by writing a 1 to this position.
    pub fn rx_oflw_sticky(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_rx_oflw_sticky(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Signal detect failure sticky bit.
    ///
    /// 0: No signal detect failure occurred 1: Signal detect failures occurred Bit is cleared by writing a 1 to this position.
    pub fn sd_sticky(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_sd_sticky(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Indicates that a missing EOF has been detected in the Tx path of the DEV1G.
    ///
    /// 0: Missing EOF has not been detected 1: Missing EOF has been detected Bit is cleared by writing a 1 to this position.
    pub fn tx_eof_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_tx_eof_sticky(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Indicates that an overflow has occured in the Tx FIFO.
    ///
    /// 0: No buffer overflow has occured 1: A buffer overflow has occured Bit is cleared by writing a 1 to this position.
    pub fn tx_oflw_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_tx_oflw_sticky(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Indicates that a missing SOF has been detected in the Tx path of the DEV1G.
    ///
    /// 0: Missing SOF has not been detected 1: Missing SOF has been detected Bit is cleared by writing a 1 to this position.
    pub fn tx_sof_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_tx_sof_sticky(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Sticky bit that indicates a protocol error on the taxi input interface.
    ///
    /// 0: No tx taxi protocol error has occured 1: One or more tx taxi protocol errors have occured Bit is cleared by writing a 1 to this position.
    pub fn tx_taxi_prot_err_sticky(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_tx_taxi_prot_err_sticky(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Indicates that the Tx Resync FIFO has not been able to supply enough data to the MAC during frame transmission. The device automatically aborts the frame transmission.
    ///
    /// 0: No buffer underflow detected 1: Buffer underflow detected Bit is cleared by writing a 1 to this position.
    pub fn tx_uflw_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_tx_uflw_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Control Energy Efficient Ethernet operation.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct EEE_CFG(u32);
impl EEE_CFG {
    /// Enable EEE operation on the port. A port enters the low power mode when no egress queues have data ready. The port is activated when one of the following conditions is true: - A queue has been non-empty for EEE_TIMER_AGE. - A queue has more than EEE_HIGH_FRAMES frames pending. - A queue has more than EEE_HIGH_BYTES bytes pending. - A queue is marked as a fast queue, and has data pending.
    pub fn eee_ena(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_eee_ena(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Maximum time frames in any queue must wait before the port is activated. The default value corresponds to 48 us.
    ///
    /// Time = 4**(EEE_TIMER_AGE/16) * (EEE_TIMER_AGE mod 16) microseconds
    pub fn eee_timer_age(&self) -> u32 {
        (self.0 & 0x3f8000) >> 15
    }
    pub fn set_eee_timer_age(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x3f8000);
        self.0 &= !0x3f8000;
        self.0 |= value;
    }
    /// When all queues are empty, the port is kept active until this time has passed. Default value corresponds to 5 us.
    ///
    /// Time = 4**(EEE_TIMER_HOLDOFF/16) * (EEE_TIMER_HOLDOFF mod 16) microseconds
    pub fn eee_timer_holdoff(&self) -> u32 {
        (self.0 & 0xfe) >> 1
    }
    pub fn set_eee_timer_holdoff(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0xfe);
        self.0 &= !0xfe;
        self.0 |= value;
    }
    /// Time from the egress port is activated until frame transmission is restarted. Default value corresponds to 16 us.
    ///
    /// Time = 4**(EEE_TIMER_WAKEUP/16) * (EEE_TIMER_WAKEUP mod 16) microseconds
    pub fn eee_timer_wakeup(&self) -> u32 {
        (self.0 & 0x7f00) >> 8
    }
    pub fn set_eee_timer_wakeup(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x7f00);
        self.0 &= !0x7f00;
        self.0 |= value;
    }
    /// Status bit indicating whether port is in low-power-idle due to the LPI algorithm (EEE_CFG). If set, transmissions are held back.
    pub fn port_lpi(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_port_lpi(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PTP Configuration per port
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PTP_CFG(u32);
impl PTP_CFG {
    /// Configures the time domain this port is assigned to. This domain assignment must be made before the central counters in DEVCPU block is enabled.
    pub fn ptp_dom(&self) -> u32 {
        (self.0 & 0x300000) >> 20
    }
    pub fn set_ptp_dom(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x300000);
        self.0 &= !0x300000;
        self.0 |= value;
    }
    /// Disable PTP on the port
    pub fn ptp_ena(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    pub fn set_ptp_ena(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x400000);
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Bit rate for the physical interface. 0: 5Gbps(QSGMII), 1: 1.25Gbps SGMII), 2: 3.125 Gbps, 3: reserved
    ///
    /// 0: 5Gbps (QSGMII) 1: 1.125 Gbps (SGMII) 2: 3.125 Gbps (SGMII-2.5) 3: Reserved
    pub fn ptp_if_mode(&self) -> u32 {
        (self.0 & 0xc0000) >> 18
    }
    pub fn set_ptp_if_mode(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0xc0000);
        self.0 &= !0xc0000;
        self.0 |= value;
    }
    /// Time in ns to subtract from timestamper in the ingress direction to compensate for static delay through the physical encoding layers.
    pub fn ptp_rx_io_dly(&self) -> u32 {
        (self.0 & 0x3fe00) >> 9
    }
    pub fn set_ptp_rx_io_dly(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x3fe00);
        self.0 &= !0x3fe00;
        self.0 |= value;
    }
    /// Time in ns to add to timestamper in the egress direction to compensate for static delay through the physical encoding layers.
    pub fn ptp_tx_io_dly(&self) -> u32 {
        self.0 & 0x1ff
    }
    pub fn set_ptp_tx_io_dly(&mut self, value: u32) {
        assert!(value <= 0x1ff);
        self.0 &= !0x1ff;
        self.0 |= value;
    }
}
