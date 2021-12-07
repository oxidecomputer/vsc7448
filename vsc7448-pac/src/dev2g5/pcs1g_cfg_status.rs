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

/// Register `MAC_STICKY`
///
/// Sticky Bit Register
#[derive(From, Into)]
pub struct MAC_STICKY(u32);
impl MAC_STICKY {    ///
    /// Sticky bit indicating that an inter packet gap shrink was detected (IPG < 12 bytes).
    pub fn rx_ipg_shrink_sticky(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_rx_ipg_shrink_sticky(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }    ///
    /// Sticky bit indicating that junk was received (bytes not recognized as a frame).
    ///
    /// '0': no junk was received '1': junk was received one or more times Bit is cleared by writing a '1' to this position.
    pub fn rx_junk_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_rx_junk_sticky(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }    ///
    /// Sticky bit indicating that a preamble shrink was detected (preamble < 8 bytes).
    ///
    /// '0': no preamble shrink was detected '1': a preamble shrink was detected one or more times Bit is cleared by writing a '1' to this position.
    pub fn rx_pream_shrink_sticky(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_rx_pream_shrink_sticky(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }    ///
    /// Sticky bit indicating that the transmit host initiated abort was executed.
    pub fn tx_abort_sticky(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_tx_abort_sticky(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Sticky bit indicating that the MAC transmit FIFO has overrun.
    pub fn tx_fifo_oflw_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_tx_fifo_oflw_sticky(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Sticky bit indicating that the transmit host issued a jamming signal.
    ///
    /// '0': the transmit host issued no jamming signal '1': the transmit host issued one or morejamming signals Bit is cleared by writing a '1' to this position.
    pub fn tx_jam_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_tx_jam_sticky(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Sticky bit indicating that the transmit MAC asked the host for a frame retransmission.
    ///
    /// '0': no tx retransmission was initiated '1': one or more tx retransmissions were initiated Bit is cleared by writing a '1' to this position.
    pub fn tx_retransmit_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_tx_retransmit_sticky(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}

/// Register `PCS1G_ANEG_CFG`
///
/// PCS1G Aneg Configuration
///
/// PCS1G Auto-negotiation configuration register
#[derive(From, Into)]
pub struct PCS1G_ANEG_CFG(u32);
impl PCS1G_ANEG_CFG {    ///
    /// Advertised Ability Register: Holds the capabilities of the device as described IEEE 802.3, Clause 37.
    pub fn adv_ability(&self) -> u32 {
        (self.0 & 0xffff) >> 16
    }
    pub fn set_adv_ability(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }    ///
    /// Auto Negotiation Enable
    ///
    /// 0: Auto Negotiation Disabled 1: Auto Negotiation Enabled
    pub fn aneg_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_aneg_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Auto Negotiation Restart
    ///
    /// 0: No action 1: Restart Auto Negotiation
    pub fn aneg_restart_one_shot(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_aneg_restart_one_shot(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Software Resolve Abilities
    ///
    /// 0: If Auto Negotiation fails (no matching HD or FD capabilities) the link is disabled 1: The result of an Auto Negotiation is ignored - the link can be setup via software. This bit must be set in SGMII mode.
    pub fn sw_resolve_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_sw_resolve_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
}

/// Register `PCS1G_ANEG_NP_CFG`
///
/// PCS1G Aneg Next Page Configuration
///
/// PCS1G Auto-negotiation configuration register for next-page function
#[derive(From, Into)]
pub struct PCS1G_ANEG_NP_CFG(u32);
impl PCS1G_ANEG_NP_CFG {    ///
    /// Next page loaded
    ///
    /// 0: next page is free and can be loaded 1: next page register has been filled (to be set after np_tx has been filled)
    pub fn np_loaded_one_shot(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_np_loaded_one_shot(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Next page register: Holds the next-page information as described in IEEE 802.3, Clause 37
    pub fn np_tx(&self) -> u32 {
        (self.0 & 0xffff) >> 16
    }
    pub fn set_np_tx(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `PCS1G_ANEG_NP_STATUS`
///
/// PCS1G Aneg Next Page Status Register
///
/// PCS1G Auto-negotiation next page status register
#[derive(From, Into)]
pub struct PCS1G_ANEG_NP_STATUS(u32);
impl PCS1G_ANEG_NP_STATUS {    ///
    /// Next page ability register from link partner as described in IEEE 802.3, Clause 37
    pub fn lp_np_rx(&self) -> u32 {
        (self.0 & 0xffff) >> 16
    }
    pub fn set_lp_np_rx(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `PCS1G_ANEG_STATUS`
///
/// PCS1G ANEG Status Register
///
/// PCS1G Auto-negotiation status register
#[derive(From, Into)]
pub struct PCS1G_ANEG_STATUS(u32);
impl PCS1G_ANEG_STATUS {    ///
    /// Auto Negotiation Complete
    ///
    /// 0: No Auto Negotiation has been completed 1: Indicates that an Auto Negotiation has completed successfully
    pub fn aneg_complete(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_aneg_complete(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Advertised abilities from link partner as described in IEEE 802.3, Clause 37
    pub fn lp_adv_ability(&self) -> u32 {
        (self.0 & 0xffff) >> 16
    }
    pub fn set_lp_adv_ability(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }    ///
    /// Status indicating whether a new page has been received.
    ///
    /// 0: No new page received 1: New page received Bit is cleared by writing a 1 to this position.
    pub fn page_rx_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_page_rx_sticky(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Resolve priority
    ///
    /// 0: ANEG is in progress 1: ANEG nearly finished - priority can be resolved (via software)
    pub fn pr(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_pr(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}

/// Register `PCS1G_CDET_CFG`
///
/// PCS1G comma detection Configuration
///
/// PCS1G comma detection configuration
#[derive(From, Into)]
pub struct PCS1G_CDET_CFG(u32);
impl PCS1G_CDET_CFG {    ///
    /// Enable comma detection and code-group alignment
    ///
    /// 0: Comma detection disabled 1: Comma detection enabled
    pub fn cdet_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_cdet_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `PCS1G_CFG`
///
/// PCS1G Configuration
///
/// PCS1G main configuration register
#[derive(From, Into)]
pub struct PCS1G_CFG(u32);
impl PCS1G_CFG {    ///
    /// Enable Link control via Backplane Ethernet ANEG
    ///
    /// 0: Disable link control 1: Enable link control
    pub fn an_link_ctrl_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_an_link_ctrl_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Set type of link_status indication at CPU-System
    ///
    /// 0: Sync_status (from PCS synchronization state machine) 1: Bit 15 of PCS1G_ANEG_STATUS.lp_adv_ability (Link up/down)
    pub fn link_status_type(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_link_status_type(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// PCS enable
    ///
    /// 0: Disable PCS 1: Enable PCS
    pub fn pcs_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_pcs_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `PCS1G_DBG_CFG`
///
/// PCS1G Debug Configuration
///
/// PCS1G Debug configuration register
#[derive(From, Into)]
pub struct PCS1G_DBG_CFG(u32);
impl PCS1G_DBG_CFG {    ///
    /// Use Debug Link Timer
    ///
    /// 0: Normal 10 ms (1.6 ms in sgmii mode) timer is selected 1: Reduced 9.77 us (1.56 us in sgmii mode) timer is selected
    pub fn udlt(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_udlt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `PCS1G_DEBUG_STATUS`
///
/// PCS1G debug status
///
/// PCS1G debug status register
#[derive(From, Into)]
pub struct PCS1G_DEBUG_STATUS(u32);
impl PCS1G_DEBUG_STATUS {    ///
    /// Indicates the mode of the TBI
    ///
    /// 00: Idle mode 01: Configuration mode 10: Reserved 11: Data mode
    pub fn xmit_mode(&self) -> u32 {
        (self.0 & 0x3000) >> 12
    }
    pub fn set_xmit_mode(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x3000);
        self.0 &= !0x3000;
        self.0 |= value;
    }
}

/// Register `PCS1G_LB_CFG`
///
/// PCS1G Loopback Configuration
///
/// PCS1G Loop-Back configuration register
#[derive(From, Into)]
pub struct PCS1G_LB_CFG(u32);
impl PCS1G_LB_CFG {    ///
    /// Loops data in PCS (GMII side) from ingress direction to egress direction. Rate adaption is automatically performed in a FIFO within the PCS
    ///
    /// 0: GMII Loopback Disabled 1:GMII Loopback Enabled
    pub fn gmii_phy_lb_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_gmii_phy_lb_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Enable rate adaption capability in PCS receive direction explicitely (required when PHY data looped back within MAC)
    ///
    /// 0: Disable 1: Enable
    pub fn ra_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_ra_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Loops data in PCS (TBI side) from egress direction to ingress direction. The Rx clock is automatically set equal to the Tx clock
    ///
    /// 0: TBI Loopback Disabled 1:TBI Loopback Enabled
    pub fn tbi_host_lb_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_tbi_host_lb_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `PCS1G_LINK_DOWN_CNT`
///
/// PCS1G link down counter
///
/// PCS1G link down counter register
#[derive(From, Into)]
pub struct PCS1G_LINK_DOWN_CNT(u32);
impl PCS1G_LINK_DOWN_CNT {    ///
    /// Link Down Counter. A counter that counts the number of times a link has been down. The counter does not saturate at 255 and is only cleared when writing 0 to the register
    pub fn link_down_cnt(&self) -> u32 {
        (self.0 & 0xff) >> 0
    }
    pub fn set_link_down_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}

/// Register `PCS1G_LINK_STATUS`
///
/// PCS1G link status
///
/// PCS1G link status register
#[derive(From, Into)]
pub struct PCS1G_LINK_STATUS(u32);
impl PCS1G_LINK_STATUS {    ///
    /// Additional delay in rx-path; multiply the value of this field by the line-rate bit-period (800ps for 10/100/1000, 320ps for 2G5 mode.) This field is valid when the link is up, it remains constant for as long as the link is up, value may cange on link-down event. This field shows the number of data bits that is stored in the rx comma-alignment block, values of 0-9 is possible.
    pub fn delay_var(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    pub fn set_delay_var(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xf000);
        self.0 &= !0xf000;
        self.0 |= value;
    }    ///
    /// Indicates whether the link is up or down. A link is up when ANEG state machine is in state LINK_OK or AN_DISABLE_LINK_OK
    ///
    /// 0: Link down 1: Link up
    pub fn link_status(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_link_status(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Indicates whether or not the selected Signal Detect input line is asserted
    ///
    /// 0: No signal detected 1: Signal detected
    pub fn signal_detect(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_signal_detect(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }    ///
    /// Indicates if PCS has successfully synchronized
    ///
    /// 0: PCS is out of sync 1: PCS has synchronized
    pub fn sync_status(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_sync_status(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `PCS1G_LPI_CFG`
///
/// PCS1G Low Power Idle Configuration
///
/// Configuration register for Low Power Idle (Energy Efficient Ethernet)
#[derive(From, Into)]
pub struct PCS1G_LPI_CFG(u32);
impl PCS1G_LPI_CFG {    ///
    /// LPI-Timer test mode.
    ///
    /// 0: Normal timing constants are used 1: Shortened timing constants are used
    pub fn lpi_testmode(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_lpi_testmode(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }    ///
    /// QSGMII master/slave selection (only one master allowed per QSGMII). The master drives LPI timing on serdes
    ///
    /// 0: Slave 1: Master
    pub fn qsgmii_ms_sel(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_qsgmii_ms_sel(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }    ///
    /// Disable output of Low-Power Idle in receive direction (to core)
    ///
    /// 0: Enable 1: Disable
    pub fn rx_lpi_out_dis(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rx_lpi_out_dis(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }    ///
    /// Assert Low-Power Idle (LPI) in transmit mode
    ///
    /// 0: Disable LPI transmission 1: Enable LPI transmission
    pub fn tx_assert_lpidle(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_tx_assert_lpidle(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `PCS1G_LPI_WAKE_ERROR_CNT`
///
/// PCS1G wake error counter
///
/// PCS1G Low Power Idle wake error counter (Energy Efficient Ethernet)
#[derive(From, Into)]
pub struct PCS1G_LPI_WAKE_ERROR_CNT(u32);
impl PCS1G_LPI_WAKE_ERROR_CNT {    ///
    /// Wake Error Counter. A counter that is incremented when the link partner does not send wake-up burst in due time. The counter saturates at 65535 and is cleared when writing 0 to the register
    pub fn wake_error_cnt(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_wake_error_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `PCS1G_MODE_CFG`
///
/// PCS1G Mode Configuration
///
/// PCS1G mode configuration
#[derive(From, Into)]
pub struct PCS1G_MODE_CFG(u32);
impl PCS1G_MODE_CFG {    ///
    /// Selection of PCS operation
    ///
    /// 0: PCS is used in SERDES mode 1: PCS is used in SGMII mode
    pub fn sgmii_mode_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_sgmii_mode_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Unidirectional mode enable. Implementation of 802.3, Clause 66. When asserted, this enables MAC to transmit data independent of the state of the receive link.
    ///
    /// 0: Unidirectional mode disabled 1: Unidirectional mode enabled
    pub fn unidir_mode_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_unidir_mode_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}

/// Register `PCS1G_SD_CFG`
///
/// PCS1G Signal Detect Configuration
///
/// PCS1G signal_detect configuration
#[derive(From, Into)]
pub struct PCS1G_SD_CFG(u32);
impl PCS1G_SD_CFG {    ///
    /// Signal Detect Enable
    ///
    /// 0: The Signal Detect input pin is ignored. The PCS assumes an active Signal Detect at all times 1: The Signal Detect input pin is used to determine if a signal is detected
    pub fn sd_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_sd_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Signal detect polarity: The signal level on signal_detect input pin must be equal to SD_POL to indicate signal detection (SD_ENA must be set)
    ///
    /// 0: Signal Detect input pin must be '0' to indicate a signal detection 1: Signal Detect input pin must be '1' to indicate a signal detection
    pub fn sd_pol(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_sd_pol(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Signal detect selection (select input for internal signal_detect line)
    ///
    /// 0: Select signal_detect line from hardmacro 1: Select external signal_detect line
    pub fn sd_sel(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_sd_sel(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
}

/// Register `PCS1G_STICKY`
///
/// PCS1G sticky register
///
/// PCS1G status register for sticky bits
#[derive(From, Into)]
pub struct PCS1G_STICKY(u32);
impl PCS1G_STICKY {    ///
    /// The sticky bit is set when the link has been down - i.e. if the ANEG state machine has not been in the AN_DISABLE_LINK_OK or LINK_OK state for one or more clock cycles. This occurs if e.g. ANEG is restarted or for example if signal-detect or synchronization has been lost for more than 10 ms (1.6 ms in SGMII mode). By setting the UDLT bit, the required down time can be reduced to 9,77 us (1.56 us)
    ///
    /// 0: Link is up 1: Link has been down Bit is cleared by writing a 1 to this position.
    pub fn link_down_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_link_down_sticky(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Sticky bit indicating if PCS synchronization has been lost
    ///
    /// 0: Synchronization has not been lost at any time 1: Synchronization has been lost for one or more clock cycles Bit is cleared by writing a 1 to this position.
    pub fn out_of_sync_sticky(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_out_of_sync_sticky(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
