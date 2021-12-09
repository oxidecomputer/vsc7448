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

/// Register `MAC_ADV_CHK_CFG`
///
/// Advanced Check Feature Configuration Register
#[derive(From, Into)]
pub struct MAC_ADV_CHK_CFG(u32);
impl MAC_ADV_CHK_CFG {
    /// Length Drop Enable:\nConfigures the Receive Module to drop frames in reference to in-range and out-of-range errors:

    ///

    /// '0': Length Drop Disabled '1': Length Drop Enabled.
    pub fn len_drop_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_len_drop_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `MAC_ENA_CFG`
///
/// Mode Configuration Register
#[derive(From, Into)]
pub struct MAC_ENA_CFG(u32);
impl MAC_ENA_CFG {
    /// Receiver Module Enable.

    ///

    /// '0': Receiver Module Disabled '1': Receiver Module Enabled
    pub fn rx_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rx_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Transmitter Module Enable.

    ///

    /// '0': Transmitter Module Disabled '1': Transmitter Module Enabled
    pub fn tx_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_tx_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `MAC_HDX_CFG`
///
/// Half Duplex Configuration Register
#[derive(From, Into)]
pub struct MAC_HDX_CFG(u32);
impl MAC_HDX_CFG {
    /// Bypass 2-step synchronization of collision signal gmii_col in defer and backoff logic to allow for optimized collision handling in half duplex modes

    ///

    /// '0': Do not bypass gmii_col sync stage '1': Bypass gmii_col sync stage
    pub fn bypass_col_sync(&self) -> u32 {
        (self.0 & 0x4000000) >> 26
    }
    pub fn set_bypass_col_sync(&mut self, value: u32) {
        let value = value << 26;
        assert!(value <= 0x4000000);
        self.0 &= !0x4000000;
        self.0 |= value;
    }
    /// Adjustment of early/late collision boundary:\nThis bitgroup is used to adjust the MAC so that a collision on a shared transmission medium before bit 512 is handled as an early collision, whereas a collision after bit 512 is handled as a late collision, i.e. no retransmission is performed.
    pub fn late_col_pos(&self) -> u32 {
        self.0 & 0x7f
    }
    pub fn set_late_col_pos(&mut self, value: u32) {
        assert!(value <= 0x7f);
        self.0 &= !0x7f;
        self.0 |= value;
    }
    /// This bit is used to setup the MAC to retransmit a frame after an early collision even though 16 (or more) early collisions have occurred. This feature violates the IEEE 802.3 standard and should only be used when running in HDX flow control, which is not defined in the IEEE standard anyway.

    ///

    /// '0': A frame will be discarded and counted as an excessive collision if 16 collisions occur for this frame. '1': The MAC will retransmit a frame after an early collision, regardless of the number of previous early collisions. The backoff sequence will be reset after every 16 collisions.
    pub fn retry_after_exc_col_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_retry_after_exc_col_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Seed value loaded into the PRBS of the MAC.

    ///

    /// Used to prevent excessive collision events.
    pub fn seed(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    pub fn set_seed(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xff0000);
        self.0 &= !0xff0000;
        self.0 |= value;
    }
    /// Load SEED value into PRNG register. A SEED value is loaded into the PRNG register of the MAC, when SEED_LOAD is asserted. After a load, the SEED_LOAD must be deasserted.

    ///

    /// '0': Do not load SEED value '1': Load SEED value.
    pub fn seed_load(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_seed_load(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
}

/// Register `MAC_IFG_CFG`
///
/// Inter Frame Gap Configuration Register
#[derive(From, Into)]
pub struct MAC_IFG_CFG(u32);
impl MAC_IFG_CFG {
    /// This configuration bit allows to relax the minimum IPG check to 11 symbols instead of 12. If the IPG is below 12 (or 11 if relaxed mode is enabled) the IPG_SHRINK statistics counter is incremented.

    ///

    /// '0' Normal mode (check for minimum IPG of 12 symbols) '1' Relaxed mode (check for minimum IPG of 11 symbols)
    pub fn restore_old_ipg_check(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_restore_old_ipg_check(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Used to adjust the duration of the first part of the inter-frame gap in the Rx direction and must be set according to the speed settings.

    ///

    /// TBA: Add correct values found by validation. 10/100 Mbps: 0xXX 1000 Mbps: 0xXX.
    pub fn rx_ifg1(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_rx_ifg1(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
    /// Used to adjust the duration of the second part of the inter-frame gap in the Rx direction and must be set according to the speed and duplex settings.

    ///

    /// TBA: Add correct values found by validation. 10/100 Mbps, HDX, FDX: 0xXX, 0xXX 1000 Mbps: 0xXX.
    pub fn rx_ifg2(&self) -> u32 {
        (self.0 & 0xf0) >> 4
    }
    pub fn set_rx_ifg2(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0xf0);
        self.0 &= !0xf0;
        self.0 |= value;
    }
    /// Used to adjust the duration of the inter-frame gap in the Tx direction and must be set according to the speed and duplex settings.

    ///

    /// TBA: Add correct values found by validation. 10/100 Mbps, HDX, FDX 0xXX, 0xXX 1000 Mbps: 0xXX.
    pub fn tx_ifg(&self) -> u32 {
        (self.0 & 0x1f00) >> 8
    }
    pub fn set_tx_ifg(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x1f00);
        self.0 &= !0x1f00;
        self.0 |= value;
    }
}

/// Register `MAC_MAXLEN_CFG`
///
/// Max Length Configuration Register
#[derive(From, Into)]
pub struct MAC_MAXLEN_CFG(u32);
impl MAC_MAXLEN_CFG {
    /// The maximum frame length accepted by the Receive Module of the MAC. If the length is exceeded, this is indicated in the Statistics Engine (RX_OVERSIZE). The maximum length is automatically adjusted to accommodate maximum sized frames containing single/double VLAN tag(s) - given that the MAC is configured to be single/double VLAN aware.
    pub fn max_len(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_max_len(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `MAC_MODE_CFG`
///
/// Mode Configuration Register
#[derive(From, Into)]
pub struct MAC_MODE_CFG(u32);
impl MAC_MODE_CFG {
    /// This bit enables synchronization of Flow Control Jamming to currently used word boundaries (10/100 Mbps mode).

    ///

    /// '0' Normal mode, Flow Control Jamming is output as soon as possible '1' Word sync mode, Flow Control Jamming is synchronized 10/100 Mbps word boundaries
    pub fn fc_word_sync_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_fc_word_sync_ena(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Enables Full Duplex:

    ///

    /// '0': Half Duplex '1': Full duplex. \nNote: Full duplex MUST be selected if GIGA_MODE is enabled.
    pub fn fdx_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_fdx_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enables 1 Gbps mode.

    ///

    /// '0': 10/100 Mbps mode '1': 1 Gbps mode. Note: FDX MUST be asserted.
    pub fn giga_mode_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_giga_mode_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}

/// Register `MAC_TAGS_CFG`
///
/// VLAN / Service tag configuration register
///
/// The MAC can be configured to accept 0, 1 and 2 tags and the TAG value can be user-defined.
#[derive(From, Into)]
pub struct MAC_TAGS_CFG(u32);
impl MAC_TAGS_CFG {
    /// Provider Bridge Enable (multiple VLAN awareness)

    ///

    /// 0: The MAC operates in a single VLAN aware mode. 1: The MAC operates in a double VLAN aware mode. 2: The MAC operates in a triple VLAN aware mode. 3: Reserved. For VLAN awareness to take effect VLAN_AWR_ENA must be set to '1'.
    pub fn pb_ena(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    pub fn set_pb_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x6);
        self.0 &= !0x6;
        self.0 |= value;
    }
    /// This field defines a 1st programmable VLAN/Service tag (custom TPID 1).\nThis field together with the TAG_ID2 and TAG_ID3 - as well as the default tags 0x8100 and 0x88A8 - are used for ALL possible tag positions (up to 3). Any order of known VLAN/Service tags (0x8100, 0x88A8, TAG_ID, TAG_ID2, TAG_ID3) is found. VLAN/Service tag awareness depends on VLAN_AWR_ENA and PB_ENA.
    pub fn tag_id(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_tag_id(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    /// VLAN Awareness: Enables the MAC to work in a VLAN aware environment.

    ///

    /// '0': VLAN awareness disabled. '1': VLAN awareness enabled.
    pub fn vlan_awr_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_vlan_awr_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// When set, single, double and triple tagged frames are allowed to be 4/8/12 bytes longer than the MAXLEN configuration.
    pub fn vlan_len_awr_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_vlan_len_awr_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
}

/// Register `MAC_TAGS_CFG2`
///
/// VLAN / Service tag configuration register2
///
/// This register contains two additional freely programmable custom VLAN tags.
#[derive(From, Into)]
pub struct MAC_TAGS_CFG2(u32);
impl MAC_TAGS_CFG2 {
    /// This field defines a 2nd programmable VLAN/Service tag (custom TPID 2).

    ///

    /// tbd
    pub fn tag_id2(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_tag_id2(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// This field defines a 3rd programmable VLAN/Service tag (custom TPID 3).

    ///

    /// tbd
    pub fn tag_id3(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_tag_id3(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
}

/// Register `PTP_EVENTS`
///
/// PTP events per port
#[derive(From, Into)]
pub struct PTP_EVENTS(u32);
impl PTP_EVENTS {
    /// The correction field update went out of range. Valid range is -2^47 to 2^48-1. The frame CF will be changed to the maximum value. This range check is bypassed if ADDS48 mode is in use on the ingress or egress port.
    pub fn cf_too_big_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_cf_too_big_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
