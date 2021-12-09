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
/// Map internal priority to tag value.
#[derive(From, Into)]
pub struct DEI_MAP_DE0(u32);
impl DEI_MAP_DE0 {
    /// Map internal priority to CFI/DEI value in tags. This table used for DP values mapped to 0 in PORT_DP_MAP.
    pub fn dei_de0(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_dei_de0(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Map internal priority to tag value.
#[derive(From, Into)]
pub struct DEI_MAP_DE1(u32);
impl DEI_MAP_DE1 {
    /// Map internal priority to CFI/DEI value in tags. This table used for DP values mapped to 1 in PORT_DP_MAP.
    pub fn dei_de1(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_dei_de1(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Enable DSCP updates on the port.
#[derive(From, Into)]
pub struct DSCP_MAP(u32);
impl DSCP_MAP {
    /// Enable use of a shared DSCP remap table (DSCP_REMAP). Map DSCP value resulting from DSCP_UPDATE_ENA and IFH.QOS.UPDATE_DSCP using the common mapping table (DSCP_REMAP).

    ///

    /// 0 : No remapping. 1 : Remap DSCP using DSCP remap table (DSCP_REMAP)
    pub fn dscp_remap_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_dscp_remap_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Update DSCP with value from Analyzer, if allowed by analyzer. (IFH.QOS.UPDATE_DSCP)

    ///

    /// 0 : No update 1 : Allow update of DSCP
    pub fn dscp_update_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_dscp_update_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Configuration of mapping tables 2 and 3. MPLS label
///
/// Lookup 2 and 3 values
#[derive(From, Into)]
pub struct MAP_LBL_B(u32);
impl MAP_LBL_B {
    /// Mapped MPLS label value

    ///

    /// n: Label value
    pub fn label_val(&self) -> u32 {
        self.0 & 0xfffff
    }
    pub fn set_label_val(&mut self, value: u32) {
        assert!(value <= 0xfffff);
        self.0 &= !0xfffff;
        self.0 |= value;
    }
}
/// Map internal priority to tagged priority.
#[derive(From, Into)]
pub struct PCP_MAP_DE0(u32);
impl PCP_MAP_DE0 {
    /// Map internal priority to UPRIO/PCP value in tags. This table used for DP values mapped to 0 in PORT_DP_MAP.
    pub fn pcp_de0(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_pcp_de0(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Map internal priority to tagged priority.
#[derive(From, Into)]
pub struct PCP_MAP_DE1(u32);
impl PCP_MAP_DE1 {
    /// Map internal priority to UPRIO/PCP value in tags. This table used for DP values mapped to 1 in PORT_DP_MAP.
    pub fn pcp_de1(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_pcp_de1(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Configures per port custom values for TAGs
#[derive(From, Into)]
pub struct PORT_VLAN_CFG(u32);
impl PORT_VLAN_CFG {
    /// DEI field in the TCI.

    ///

    /// n: Port DEI value
    pub fn port_dei(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_port_dei(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// PCP field in the TCI.

    ///

    /// n: Port PCP value
    pub fn port_pcp(&self) -> u32 {
        (self.0 & 0xe000) >> 13
    }
    pub fn set_port_pcp(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0xe000);
        self.0 &= !0xe000;
        self.0 |= value;
    }
    /// VID field in the TCI.

    ///

    /// n: Port VID
    pub fn port_vid(&self) -> u32 {
        self.0 & 0xfff
    }
    pub fn set_port_vid(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// 1588 configuration
#[derive(From, Into)]
pub struct PTP_EDLY_CFG(u32);
impl PTP_EDLY_CFG {
    /// Signed value to add to CF when frame is transmitted on this port. Field is used if requested through the analyzer match rule. This value can be used as the egress asymmetry delay for the particular PTP flow.
    pub fn ptp_edly_val(&self) -> u32 {
        self.0
    }
    pub fn set_ptp_edly_val(&mut self, value: u32) {
        self.0 = value;
    }
}
/// 1588 configuration
#[derive(From, Into)]
pub struct PTP_IDLY1_CFG(u32);
impl PTP_IDLY1_CFG {
    /// Signed value to add to CF when frame is received on this port. Field is used if requested through the analyzer match rule. This value can be used as the ingress asymmetry or ingress asymmetry+path delay for the particular PTP flow.
    pub fn ptp_idly1_val(&self) -> u32 {
        self.0
    }
    pub fn set_ptp_idly1_val(&mut self, value: u32) {
        self.0 = value;
    }
}
/// 1588 configuration
#[derive(From, Into)]
pub struct PTP_IDLY2_CFG(u32);
impl PTP_IDLY2_CFG {
    /// Signed value to add to CF when frame is received on this port. Field is used if requested through the analyzer match rule. This value can be used as the ingress asymmetry or ingress asymmetry+path delay for the particular PTP flow.
    pub fn ptp_idly2_val(&self) -> u32 {
        self.0
    }
    pub fn set_ptp_idly2_val(&mut self, value: u32) {
        self.0 = value;
    }
}
/// 1588 configuration
#[derive(From, Into)]
pub struct PTP_MISC_CFG(u32);
impl PTP_MISC_CFG {
    /// Set to disable clearing of checksum field in IPv4 frames
    pub fn ptp_udp4_csum_dis(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ptp_udp4_csum_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set to skip update of udp checksums for IPv6 frames
    pub fn ptp_udp6_csum_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ptp_udp6_csum_dis(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// 1588 configuration
///
/// Selects mode of port when transmitting (index 0), or receiving (index 1)
#[derive(From, Into)]
pub struct PTP_MODE_CFG(u32);
impl PTP_MODE_CFG {
    /// Sets the time domain this port belongs to.
    pub fn ptp_dom_val(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_ptp_dom_val(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// PTP operation mode for frames.

    ///

    /// 0: Front port 1: Backplane port using RSRV field 30 bit TS transfer 2: Backplane port using RSRV field 32 bit TS transfer 3: Backplane port using CF field for 44 bit TS transfer 4: Backplane port using CF field for 48 bit TS transfer 5: Monitor port. Frame updated to arrival stamper. 6: PTP Disabled port
    pub fn ptp_mode_val(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    pub fn set_ptp_mode_val(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x1c);
        self.0 &= !0x1c;
        self.0 |= value;
    }
}
/// High 16 bits of PTP Mac Address
#[derive(From, Into)]
pub struct PTP_SMAC_HIGH(u32);
impl PTP_SMAC_HIGH {
    /// If requested by the PTP action out of the analyzer, this MAC address can be pasted into the SMAC.
    pub fn ptp_smac_high(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_ptp_smac_high(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Low 32 bits of PTP Mac Address
#[derive(From, Into)]
pub struct PTP_SMAC_LOW(u32);
impl PTP_SMAC_LOW {
    /// If requested by the PTP action out of the analyzer, this MAC address can be pasted into the SMAC.
    pub fn ptp_smac_low(&self) -> u32 {
        self.0
    }
    pub fn set_ptp_smac_low(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Configure PORT tagging
#[derive(From, Into)]
pub struct TAG_CTRL(u32);
impl TAG_CTRL {
    /// Control port tagging. See TAG_CTRL.TAG_CFG_OBEY_WAS_TAGGED.

    ///

    /// 0: Port tagging disabled 1: Tag all frames, except when VID=PORT_VLAN_CFG.PORT_VID or VID=0 2: Tag all frames, except when VID=0 3: Tag all frames
    pub fn tag_cfg(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    pub fn set_tag_cfg(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xc00);
        self.0 &= !0xc00;
        self.0 |= value;
    }
    /// Control how port tags are added. If this bit is set, the IFH field VSTAX.TAG.WAS_TAGGED must be '1' before a port tag is added to a frame. See TAG_CTRL.TAG_CFG

    ///

    /// 0: Normal port tagging mode 1: Frames are not port tagged, if VSTAX.TAG.WAS_TAGGED = '0' regardless of TAG_CTRL.TAG_CFG configuration
    pub fn tag_cfg_obey_was_tagged(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_tag_cfg_obey_was_tagged(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Select DEI fields in port tag

    ///

    /// Select DEI in port tag. 0: Classified DEI 1: PORT_DEI 2: DE and QoS mapped to DEI (DEI_MAP_DEx) 3: DE level (Color) 4: DE and COSID mapped to DEI (DEI_MAP_DEx) 5: DE and classified PCP mapped to DEI (DEI_MAP_DEx) 6-7: Reserved
    pub fn tag_dei_cfg(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_tag_dei_cfg(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Select PCP fields in port tag

    ///

    /// Select PCP in port tag. 0: Classified PCP 1: PORT_PCP 2: DE and QoS mapped to PCP (PCP_MAP_DEx) 3: QoS class 4: DE and COSID mapped to PCP (PCP_MAP_DEx) 5: COSID 6: DE and classified PCP mapped to PCP (PCP_MAP_DEx) 7: Reserved
    pub fn tag_pcp_cfg(&self) -> u32 {
        (self.0 & 0x38) >> 3
    }
    pub fn set_tag_pcp_cfg(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x38);
        self.0 &= !0x38;
        self.0 |= value;
    }
    /// Select Tag Protocol Identifier (TPID) for port tagging

    ///

    /// 0: 0x8100 1: 0x88A8 2: Custom1. REW::TPID_CFG[0].TPID_VAL 3: Custom2. REW::TPID_CFG[1].TPID_VAL 4: Custom3. REW::TPID_CFG[2].TPID_VAL 5: Select via ifh.vstax.tag_type and ifh.encap.tag_tpid If ifh.encap.tag_tipd = STD_TPID: If ifh.vstax.tag_type = 0 then 0x8100 else 0x88A8 If ifh.encap.tag_tipd = CUSTOM<n>: Custom<n> TPID 6-7: Reserved
    pub fn tag_tpid_cfg(&self) -> u32 {
        (self.0 & 0x380) >> 7
    }
    pub fn set_tag_tpid_cfg(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x380);
        self.0 &= !0x380;
        self.0 |= value;
    }
    /// Select VID in port tag

    ///

    /// Select VID in port tag. 0: Use classified VID. 1: Use PORT_VLAN_CFG.PORT_VID
    pub fn tag_vid_cfg(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_tag_vid_cfg(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
}
