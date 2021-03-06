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
/// Loopback Configuration Register
///
/// A number of internal loopback can be enabled in each device by the configuration bits in this register.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEV_LB_CFG(u32);
impl DEV_LB_CFG {
    /// An internal loopback from the egress Taxi bus to the ingress Taxi bus can be enabled.
    ///
    /// '0': Loopback from Taxi egress to Taxi ingress is disabled '1': Loopback from Taxi egress to Taxi ingress is enabled
    #[inline(always)]
    pub fn taxi_host_lb_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_taxi_host_lb_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// An internal loopback from the igress Taxi bus to the egress Taxi bus can be enabled.
    ///
    /// '0': Loopback from Taxi ingress bus to Taxi egress bus disabled '1': Loopback from Taxi ingress bus to Taxi egress bus enabled
    #[inline(always)]
    pub fn taxi_phy_lb_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_taxi_phy_lb_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
}
/// Debug Configuration Register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEV_MISC_CFG(u32);
impl DEV_MISC_CFG {
    /// Clears RX_RESYNC_MAX_FILL_LVL that holds the max. fill level of RX RESYNC FIFO. This is a on shot bit automatically cleared by HW.
    ///
    /// '0': No action '1': clears RX_RESYNC_MAX_FILL_LVL (Bit is automatically cleared)
    #[inline(always)]
    pub fn rx_resync_max_fill_lvl_clr(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    #[inline(always)]
    pub fn set_rx_resync_max_fill_lvl_clr(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// The device can be configured to disregard the fcs_update indication from the DSM and not update the FCS of any transmitted frames.
    ///
    /// 0: The FCS of transmitted frames is updated according to the fcs_update indication from the DSM. 1: The FCS of transmitted frames is never updated. 2: The FCS of transmitted frames is always updated.
    #[inline(always)]
    pub fn tx_fcs_update_sel(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    #[inline(always)]
    pub fn set_tx_fcs_update_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 16;
        self.0 &= !0x30000;
        self.0 |= value;
    }
}
/// Port Proetection Configuration Register
///
/// When port protedction is enabled the device will snoop and transmit data destined for another device 10G identified by port_protect_id.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEV_PORT_PROTECT(u32);
impl DEV_PORT_PROTECT {
    /// Enables snooping of egress data from another port. The port from which egress data is copied and transmitted at the Ethernet port is determined by the PORT_PROTECT_ID configuration.
    ///
    /// '0': Port protection is disabled. '1': Port protection is enabled.
    #[inline(always)]
    pub fn port_protect_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_port_protect_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Indicates from which port egress data must be copied and transmitted at this Ethernet port. The port from which egress data is copied must always be a port that is closer to the ASM. I.e. DEV(X) may be configured to snoop egress data destined for DEV(X+n), where DEV(X+n) is closer to the ASM - but NOT vice versa.
    ///
    /// 0: Reserved 1: Egress data destined for DEV(1) is also transmitted by this device. 2: Egress data destined for DEV(2) is also transmitted by this device. ..
    #[inline(always)]
    pub fn port_protect_id(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    #[inline(always)]
    pub fn set_port_protect_id(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 4;
        self.0 &= !0x30;
        self.0 |= value;
    }
}
/// Clock/Reset Control Register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEV_RST_CTRL(u32);
impl DEV_RST_CTRL {
    /// Reset MAC Rx clock domains of the device.
    ///
    /// '0': The MAC Rx clock domain is NOT reset '1': The MAC Rx clock domain is reset Note: MAC_RX_RST is NOT a one-shot operation. The MAC Rx clock domain remains reset until a '0' is written to MAC_RX_RST.
    #[inline(always)]
    pub fn mac_rx_rst(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_mac_rx_rst(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Reset MAC Tx clock domain of device.
    ///
    /// '0': The MAC Tx clock domain is not reset. '1': The MAC Tx clock domain is reset. Note: The MAC Tx clock domain remains reset until 0 is written to this register field.
    #[inline(always)]
    pub fn mac_tx_rst(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_mac_tx_rst(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Backplane Ethernet: Enable parallel detection mode for autonegotiation
    ///
    /// '0':  Parallel detection mode disabled '1': Parallel detection mode enabled
    #[inline(always)]
    pub fn pardet_mode_ena(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    #[inline(always)]
    pub fn set_pardet_mode_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 23;
        self.0 &= !0x800000;
        self.0 |= value;
    }
    /// Reset PCS Rx clock domains of the device.
    ///
    /// '0': The PCS Rx clock domain is NOT reset '1': The PCS Rx clock domain is reset Note: PCS_RX_RST is NOT a one-shot operation. The PCS Rx clock domain remains reset until a '0' is written to PCS_RX_RST.
    #[inline(always)]
    pub fn pcs_rx_rst(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_pcs_rx_rst(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Reset PCS Tx clock domains of the device.
    ///
    /// '0': The PCS Tx clock domain is NOT reset '1': The PCS Tx clock domain is reset Note: PCS_TX_RST is NOT a one-shot operation. The PCS Tx clock domain remains reset until a '0' is written to PCS_TX_RST.
    #[inline(always)]
    pub fn pcs_tx_rst(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_pcs_tx_rst(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// This field is used to configure the MAC and PCS Rx/Tx clock frequencies.
    ///
    /// '100':  XAUI/RXAUI 10 Gbps or 12Gbps OXAUI '110':  Both MAC and PCS Rx/Tx clocks are disabled '111':  XFI 10 Gbps Unused values are reserved.
    #[inline(always)]
    pub fn speed_sel(&self) -> u32 {
        (self.0 & 0x700000) >> 20
    }
    #[inline(always)]
    pub fn set_speed_sel(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 20;
        self.0 &= !0x700000;
        self.0 |= value;
    }
}
/// Ingress (receive) Path Status Register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEV_RX_STATUS(u32);
impl DEV_RX_STATUS {
    /// Maximum fill level of Rx resync FIFO. Fill level can be cleared by writing to RX_RESYNC_MAX_FILL_LVL_CLR bit.
    #[inline(always)]
    pub fn rx_resync_max_fill_lvl(&self) -> u32 {
        self.0 & 0xf
    }
    #[inline(always)]
    pub fn set_rx_resync_max_fill_lvl(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// Miscellaneous Sticky Bit Register
///
/// Clear the sticky bits by writing a '1' in the relevant bitgroups.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEV_STICKY(u32);
impl DEV_STICKY {
    /// Indicates that a missing EOF has been detected. Writing a '1' clears the sticky bit.
    ///
    /// '0': No EOF error detected '1': Missing EOF indication detected in Rx path of DEV10G.
    #[inline(always)]
    pub fn rx_eof_sticky(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_rx_eof_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Indicates if an overflow has occured in the ingress resynchronization FIFO. Writing a '1' clears the sticky bit.
    ///
    /// '0': No overflow has occurred in the ingress resynchronization FIFO. '1': An overflow has occurred in the ingress resynchronization FIFO.
    #[inline(always)]
    pub fn rx_resync_fifo_oflw_sticky(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_rx_resync_fifo_oflw_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Indicates that a missing SOF has been detected in the Rx path of the DEV10G. Writing a '1' clears the sticky bit.
    ///
    /// '0': No missing SOF detected '1': Missing SOF indication detected in Rx path of DEV10G.
    #[inline(always)]
    pub fn rx_sof_sticky(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_rx_sof_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Indicates that a missing EOF has been detected. Writing a '1' clears the sticky bit.
    ///
    /// '0': No EOF error detected '1': Missing EOF indication detected in Tx path of DEV10G.
    #[inline(always)]
    pub fn tx_eof_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_tx_eof_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Indicates that a missing SOF has been detected in the Tx path of the DEV10G. Writing a '1' clears the sticky bit.
    ///
    /// '0': No missing SOF detected '1': Missing SOF indication detected in Tx path of DEV10G.
    #[inline(always)]
    pub fn tx_sof_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_tx_sof_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// Control Energy Efficient Ethernet operation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct EEE_CFG(u32);
impl EEE_CFG {
    /// Enable EEE operation on the port. A port enters the low power mode when no egress queues have data ready. The port is activated when one of the following conditions is true: - A queue has been non-empty for EEE_TIMER_AGE. - A queue has more than EEE_HIGH_FRAMES frames pending. - A queue has more than EEE_HIGH_BYTES bytes pending. - A queue is marked as a fast queue, and has data pending.
    #[inline(always)]
    pub fn eee_ena(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    #[inline(always)]
    pub fn set_eee_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// Maximum time frames in any queue must wait before the port is activated. The default value corresponds to 48 us.
    ///
    /// Time = 4**(EEE_TIMER_AGE/16) * (EEE_TIMER_AGE mod 16) microseconds
    #[inline(always)]
    pub fn eee_timer_age(&self) -> u32 {
        (self.0 & 0x3f8000) >> 15
    }
    #[inline(always)]
    pub fn set_eee_timer_age(&mut self, value: u32) {
        assert!(value <= 0x7f);
        let value = value << 15;
        self.0 &= !0x3f8000;
        self.0 |= value;
    }
    /// When all queues are empty, the port is kept active until this time has passed. Default value corresponds to 5 us.
    ///
    /// Time = 4**(EEE_TIMER_HOLDOFF/16) * (EEE_TIMER_HOLDOFF mod 16) microseconds
    #[inline(always)]
    pub fn eee_timer_holdoff(&self) -> u32 {
        (self.0 & 0xfe) >> 1
    }
    #[inline(always)]
    pub fn set_eee_timer_holdoff(&mut self, value: u32) {
        assert!(value <= 0x7f);
        let value = value << 1;
        self.0 &= !0xfe;
        self.0 |= value;
    }
    /// Time from the egress port is activated until frame transmission is restarted. Default value corresponds to 16 us.
    ///
    /// Time = 4**(EEE_TIMER_WAKEUP/16) * (EEE_TIMER_WAKEUP mod 16) microseconds
    #[inline(always)]
    pub fn eee_timer_wakeup(&self) -> u32 {
        (self.0 & 0x7f00) >> 8
    }
    #[inline(always)]
    pub fn set_eee_timer_wakeup(&mut self, value: u32) {
        assert!(value <= 0x7f);
        let value = value << 8;
        self.0 &= !0x7f00;
        self.0 |= value;
    }
    /// Status bit indicating whether port is in low-power-idle due to the LPI algorithm (EEE_CFG). If set, transmissions are held back.
    #[inline(always)]
    pub fn port_lpi(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_port_lpi(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Interrupt Source Register
///
/// Interrupt Source Register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct INTR(u32);
impl INTR {
    /// Link status is down (source: PCS1G, PCS10G or PCS2x6G depending on current DEV10G mode)
    ///
    /// 0 = no indication 1 = active indication
    #[inline(always)]
    pub fn link_dwn_intr(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_link_dwn_intr(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Link status is up (source: PCS1G, PCS10G or PCS2x6G depending on current DEV10G mode)
    ///
    /// 0 = no indication 1 = active indication
    #[inline(always)]
    pub fn link_up_intr(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_link_up_intr(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Aggregate of BaseR PCS indications, see PCS_10GBASE_R::PCS_INTR_STAT and PSC_10GBASE_R::PCS_INTR_MASK for more information. This field is set when any BaseR PCS interrupt indication is active and enabled.
    #[inline(always)]
    pub fn pcs_br_intr(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_pcs_br_intr(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// PCS10G, PCS1G: RX Low Power Idle mode has changed
    ///
    /// 0 = no indication 1 = active indication
    #[inline(always)]
    pub fn rx_lpi_intr(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_rx_lpi_intr(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// PCS10G, PCS1G: TX Low Power Idle mode has changed
    ///
    /// 0 = no indication 1 = active indication
    #[inline(always)]
    pub fn tx_lpi_intr(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_tx_lpi_intr(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// Interrupt Enable Register
///
/// Interrupt Enable Register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct INTR_ENA(u32);
impl INTR_ENA {
    /// Set to enable propagation of LINK_DWN interrupt.
    #[inline(always)]
    pub fn link_dwn_intr_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_link_dwn_intr_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set to enable propagation of LINK_UP interrupt.
    #[inline(always)]
    pub fn link_up_intr_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_link_up_intr_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set to enable propagation of PCS_BR interrupt.
    #[inline(always)]
    pub fn pcs_br_intr_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_pcs_br_intr_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set to enable propagation of RX_LPI interrupt.
    #[inline(always)]
    pub fn rx_lpi_intr_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_rx_lpi_intr_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set to enable propagation of TX_LPI interrupt.
    #[inline(always)]
    pub fn tx_lpi_intr_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_tx_lpi_intr_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// Interrupt Status Register
///
/// Interrupt Status Register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct INTR_IDENT(u32);
impl INTR_IDENT {
    /// Set if LINK_DWN interrupt is currently active (indicating interrupt towards higher level interrupt controller.)
    #[inline(always)]
    pub fn link_dwn_intr_ident(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_link_dwn_intr_ident(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set if LINK_UP interrupt is currently active (indicating interrupt towards higher level interrupt controller.)
    #[inline(always)]
    pub fn link_up_intr_ident(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_link_up_intr_ident(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set if PCS_BR interrupt is currently active (indicating interrupt towards higher level interrupt controller.)
    #[inline(always)]
    pub fn pcs_br_intr_ident(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_pcs_br_intr_ident(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set if RX_LPI interrupt is currently active (indicating interrupt towards higher level interrupt controller.)
    #[inline(always)]
    pub fn rx_lpi_intr_ident(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_rx_lpi_intr_ident(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set if TX_LPI interrupt is currently active (indicating interrupt towards higher level interrupt controller.)
    #[inline(always)]
    pub fn tx_lpi_intr_ident(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_tx_lpi_intr_ident(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
}
/// selects mode for which status counter will count the coresponding frames on RX and TX
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PFC_PAUSE_MODE_CTRL(u32);
impl PFC_PAUSE_MODE_CTRL {
    /// '0' : counters will be having number of pause frame received/transmitted '1' : counters will be having number of PFC frame received/transmitted
    #[inline(always)]
    pub fn pfc_pause_mode_select(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_pfc_pause_mode_select(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PTP Configuration per port
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_CFG(u32);
impl PTP_CFG {
    /// Configures the time domain this port is assigned to. This domain assignment must be made before the central counters in DEVCPU block is enabled.
    #[inline(always)]
    pub fn ptp_dom(&self) -> u32 {
        (self.0 & 0xc0000) >> 18
    }
    #[inline(always)]
    pub fn set_ptp_dom(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 18;
        self.0 &= !0xc0000;
        self.0 |= value;
    }
    /// Enable PTP on the port
    #[inline(always)]
    pub fn ptp_ena(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_ptp_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Time in ns to subtract from timestamper in the ingress direction to compensate for static delay through the physical encoding layers.
    #[inline(always)]
    pub fn ptp_rx_io_dly(&self) -> u32 {
        (self.0 & 0x3fe00) >> 9
    }
    #[inline(always)]
    pub fn set_ptp_rx_io_dly(&mut self, value: u32) {
        assert!(value <= 0x1ff);
        let value = value << 9;
        self.0 &= !0x3fe00;
        self.0 |= value;
    }
    /// Time in ns to add to timestamper in the egress direction to compensate for static delay through the physical encoding layers.
    #[inline(always)]
    pub fn ptp_tx_io_dly(&self) -> u32 {
        self.0 & 0x1ff
    }
    #[inline(always)]
    pub fn set_ptp_tx_io_dly(&mut self, value: u32) {
        assert!(value <= 0x1ff);
        self.0 &= !0x1ff;
        self.0 |= value;
    }
}
/// PTP Bit time accuracy configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_CFG_BTDLY(u32);
impl PTP_CFG_BTDLY {
    /// Time in 100ps to subtract from timestamper in the ingress direction to compensate for static delay through the physical encoding layers.
    #[inline(always)]
    pub fn ptp_rx_bt_dly(&self) -> u32 {
        (self.0 & 0xf0000) >> 16
    }
    #[inline(always)]
    pub fn set_ptp_rx_bt_dly(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 16;
        self.0 &= !0xf0000;
        self.0 |= value;
    }
    /// Gearbox starting value when 6466b fifo is suspened in the ingress direction.
    #[inline(always)]
    pub fn ptp_rx_gearbox_ofs(&self) -> u32 {
        (self.0 & 0xfc0) >> 6
    }
    #[inline(always)]
    pub fn set_ptp_rx_gearbox_ofs(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 6;
        self.0 &= !0xfc0;
        self.0 |= value;
    }
    /// Time in 100ps to subtract from timestamper in the egress direction to compensate for static delay through the physical encoding layers.
    #[inline(always)]
    pub fn ptp_tx_bt_dly(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    #[inline(always)]
    pub fn set_ptp_tx_bt_dly(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 12;
        self.0 &= !0xf000;
        self.0 |= value;
    }
    /// Gearbox starting value when 6466b fifo is suspended in the egress direction.
    #[inline(always)]
    pub fn ptp_tx_gearbox_ofs(&self) -> u32 {
        self.0 & 0x3f
    }
    #[inline(always)]
    pub fn set_ptp_tx_gearbox_ofs(&mut self, value: u32) {
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}
/// PTP Events per port
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_EVENTS(u32);
impl PTP_EVENTS {
    /// The correction field update went out of range. Valid range is -2^47 to 2^48-1. The frame CF will be changed to the maximum value. This range check is bypassed if ADDS48 mode is in use on the ingress or egress port.
    #[inline(always)]
    pub fn cf_too_big_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_cf_too_big_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
