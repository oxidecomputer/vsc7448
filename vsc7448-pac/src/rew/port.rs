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
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEI_MAP_DE0(u32);
impl DEI_MAP_DE0 {
    /// Map internal priority to CFI/DEI value in tags. This table used for DP values mapped to 0 in PORT_DP_MAP.
    #[inline(always)]
    pub fn dei_de0(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_dei_de0(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Map internal priority to tag value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEI_MAP_DE1(u32);
impl DEI_MAP_DE1 {
    /// Map internal priority to CFI/DEI value in tags. This table used for DP values mapped to 1 in PORT_DP_MAP.
    #[inline(always)]
    pub fn dei_de1(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_dei_de1(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Enable DSCP updates on the port.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DSCP_MAP(u32);
impl DSCP_MAP {
    /// Enable use of a shared DSCP remap table (DSCP_REMAP). Map DSCP value resulting from DSCP_UPDATE_ENA and IFH.QOS.UPDATE_DSCP using the common mapping table (DSCP_REMAP).
    ///
    /// 0 : No remapping. 1 : Remap DSCP using DSCP remap table (DSCP_REMAP)
    #[inline(always)]
    pub fn dscp_remap_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_dscp_remap_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Update DSCP with value from Analyzer, if allowed by analyzer. (IFH.QOS.UPDATE_DSCP)
    ///
    /// 0 : No update 1 : Allow update of DSCP
    #[inline(always)]
    pub fn dscp_update_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_dscp_update_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Various Host Mode control
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct HIH_CTRL(u32);
impl HIH_CTRL {
    /// Select the source of the HIH.CKSM field.
    ///
    /// 0: Set HIH.CKSM to fixed default value (HIH_DEF_CKSM) 1: Calculate HIH.CKSM according to HIH contents.
    #[inline(always)]
    pub fn hih_auto_cksm(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_hih_auto_cksm(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enables prepending of Host Interface Header (HIH) on the port. The HiH will be placed after the SFD and will be covered by the FCS. For 10G ports it is possible to place the HiH before the SFD (in the preamble). See HIH_DEV10G_CFG.HIH_LOCATION
    ///
    /// 0: Disable HiH functionality 1: Enable HiH insertion
    #[inline(always)]
    pub fn hih_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_hih_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Select source of the HIH.CL field.
    ///
    /// 0: Always set HIH.CL to fixed default value = HIH_DEF_CL 1: Set HIH.CL to IFH.ENCAP.HIH_CL if IFH.FWD.DST_MODE = ENCAP else 0 2: Set HIH.CL to IFH.VSTAX.MISC.ISDX*4+1 if ISDX>0 else use mode 1 3: Reserved
    #[inline(always)]
    pub fn hih_frm_cl(&self) -> u32 {
        (self.0 & 0x18) >> 3
    }
    #[inline(always)]
    pub fn set_hih_frm_cl(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 3;
        self.0 &= !0x18;
        self.0 |= value;
    }
    /// Select the source of the HIH.FLAGS field.
    ///
    /// 0: Set HIH.FLAGS to fixed default value = HIH_DEF_FLAGS 1: Set HIH.FLAGS to frame IPRIO and COLOR bits. 2: Set HIH.FLAGS to frame COSID and COLOR bits. 3: Reserved
    #[inline(always)]
    pub fn hih_frm_flags(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_hih_frm_flags(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
    /// Configure which value goes into the HIH.LPID field.
    ///
    /// 0: Set LPID to fixed default value (HIH_DEF_CFG) 1: Set LPID according egress frame (Formatted by HIH_LPID_MODE)
    #[inline(always)]
    pub fn hih_frm_lpid(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_hih_frm_lpid(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Set the format of the Logical Port ID (LPID) (Value put into the HIH will be determined by HIH_FRM_LPID)
    ///
    /// 0: Egress port number 1: Ingress port number
    #[inline(always)]
    pub fn hih_lpid_mode(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_hih_lpid_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
}
/// Map internal priority to tagged priority.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCP_MAP_DE0(u32);
impl PCP_MAP_DE0 {
    /// Map internal priority to UPRIO/PCP value in tags. This table used for DP values mapped to 0 in PORT_DP_MAP.
    #[inline(always)]
    pub fn pcp_de0(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_pcp_de0(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Map internal priority to tagged priority.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCP_MAP_DE1(u32);
impl PCP_MAP_DE1 {
    /// Map internal priority to UPRIO/PCP value in tags. This table used for DP values mapped to 1 in PORT_DP_MAP.
    #[inline(always)]
    pub fn pcp_de1(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_pcp_de1(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// Configures per port custom values for TAGs
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PORT_VLAN_CFG(u32);
impl PORT_VLAN_CFG {
    /// DEI field in the TCI.
    ///
    /// n: Port DEI value
    #[inline(always)]
    pub fn port_dei(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_port_dei(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// PCP field in the TCI.
    ///
    /// n: Port PCP value
    #[inline(always)]
    pub fn port_pcp(&self) -> u32 {
        (self.0 & 0xe000) >> 13
    }
    #[inline(always)]
    pub fn set_port_pcp(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 13;
        self.0 &= !0xe000;
        self.0 |= value;
    }
    /// VID field in the TCI.
    ///
    /// n: Port VID
    #[inline(always)]
    pub fn port_vid(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_port_vid(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// 1588 configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_EDLY_CFG(u32);
impl PTP_EDLY_CFG {
    /// Signed value to add to CF when frame is transmitted on this port. Field is used if requested through the analyzer match rule. This value can be used as the egress asymmetry delay for the particular PTP flow.
    #[inline(always)]
    pub fn ptp_edly_val(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_ptp_edly_val(&mut self, value: u32) {
        self.0 = value;
    }
}
/// 1588 configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_IDLY1_CFG(u32);
impl PTP_IDLY1_CFG {
    /// Signed value to add to CF when frame is received on this port. Field is used if requested through the analyzer match rule. This value can be used as the ingress asymmetry or ingress asymmetry+path delay for the particular PTP flow.
    #[inline(always)]
    pub fn ptp_idly1_val(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_ptp_idly1_val(&mut self, value: u32) {
        self.0 = value;
    }
}
/// 1588 configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_IDLY2_CFG(u32);
impl PTP_IDLY2_CFG {
    /// Signed value to add to CF when frame is received on this port. Field is used if requested through the analyzer match rule. This value can be used as the ingress asymmetry or ingress asymmetry+path delay for the particular PTP flow.
    #[inline(always)]
    pub fn ptp_idly2_val(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_ptp_idly2_val(&mut self, value: u32) {
        self.0 = value;
    }
}
/// 1588 configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_MISC_CFG(u32);
impl PTP_MISC_CFG {
    /// Set to disable clearing of checksum field in IPv4 frames
    #[inline(always)]
    pub fn ptp_udp4_csum_dis(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_ptp_udp4_csum_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set to skip update of udp checksums for IPv6 frames
    #[inline(always)]
    pub fn ptp_udp6_csum_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_ptp_udp6_csum_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// 1588 configuration
///
/// Selects mode of port when transmitting (index 0), or receiving (index 1)
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_MODE_CFG(u32);
impl PTP_MODE_CFG {
    /// Sets the time domain this port belongs to.
    #[inline(always)]
    pub fn ptp_dom_val(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_ptp_dom_val(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// PTP operation mode for frames.
    ///
    /// 0: Front port 1: Backplane port using RSRV field 30 bit TS transfer 2: Backplane port using RSRV field 32 bit TS transfer 3: Backplane port using CF field for 44 bit TS transfer 4: Backplane port using CF field for 48 bit TS transfer 5: Monitor port. Frame updated to arrival stamper. 6: PTP Disabled port
    #[inline(always)]
    pub fn ptp_mode_val(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    #[inline(always)]
    pub fn set_ptp_mode_val(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 2;
        self.0 &= !0x1c;
        self.0 |= value;
    }
}
/// High 16 bits of PTP Mac Address
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_SMAC_HIGH(u32);
impl PTP_SMAC_HIGH {
    /// If requested by the PTP action out of the analyzer, this MAC address can be pasted into the SMAC.
    #[inline(always)]
    pub fn ptp_smac_high(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_ptp_smac_high(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Low 32 bits of PTP Mac Address
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PTP_SMAC_LOW(u32);
impl PTP_SMAC_LOW {
    /// If requested by the PTP action out of the analyzer, this MAC address can be pasted into the SMAC.
    #[inline(always)]
    pub fn ptp_smac_low(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_ptp_smac_low(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Configure PORT tagging
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TAG_CTRL(u32);
impl TAG_CTRL {
    /// Control port tagging. See TAG_CTRL.TAG_CFG_OBEY_WAS_TAGGED.
    ///
    /// 0: Port tagging disabled 1: Tag all frames, except when VID=PORT_VLAN_CFG.PORT_VID or VID=0 2: Tag all frames, except when VID=0 3: Tag all frames
    #[inline(always)]
    pub fn tag_cfg(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    #[inline(always)]
    pub fn set_tag_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 10;
        self.0 &= !0xc00;
        self.0 |= value;
    }
    /// Control how port tags are added. If this bit is set, the IFH field VSTAX.TAG.WAS_TAGGED must be '1' before a port tag is added to a frame. See TAG_CTRL.TAG_CFG
    ///
    /// 0: Normal port tagging mode 1: Frames are not port tagged, if VSTAX.TAG.WAS_TAGGED = '0' regardless of TAG_CTRL.TAG_CFG configuration
    #[inline(always)]
    pub fn tag_cfg_obey_was_tagged(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_tag_cfg_obey_was_tagged(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Select DEI fields in port tag
    ///
    /// Select DEI in port tag. 0: Classified DEI 1: PORT_DEI 2: DE and QoS mapped to DEI (DEI_MAP_DEx) 3: DE level (Color) 4: DE and COSID mapped to DEI (DEI_MAP_DEx) 5: DE and classified PCP mapped to DEI (DEI_MAP_DEx) 6-7: Reserved
    #[inline(always)]
    pub fn tag_dei_cfg(&self) -> u32 {
        self.0 & 0x7
    }
    #[inline(always)]
    pub fn set_tag_dei_cfg(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Select PCP fields in port tag
    ///
    /// Select PCP in port tag. 0: Classified PCP 1: PORT_PCP 2: DE and QoS mapped to PCP (PCP_MAP_DEx) 3: QoS class 4: DE and COSID mapped to PCP (PCP_MAP_DEx) 5: COSID 6: DE and classified PCP mapped to PCP (PCP_MAP_DEx) 7: Reserved
    #[inline(always)]
    pub fn tag_pcp_cfg(&self) -> u32 {
        (self.0 & 0x38) >> 3
    }
    #[inline(always)]
    pub fn set_tag_pcp_cfg(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 3;
        self.0 &= !0x38;
        self.0 |= value;
    }
    /// Select Tag Protocol Identifier (TPID) for port tagging
    ///
    /// 0: 0x8100 1: 0x88A8 2: Custom1. REW::TPID_CFG[0].TPID_VAL 3: Custom2. REW::TPID_CFG[1].TPID_VAL 4: Custom3. REW::TPID_CFG[2].TPID_VAL 5: Select via ifh.vstax.tag_type and ifh.encap.tag_tpid If ifh.encap.tag_tipd = STD_TPID: If ifh.vstax.tag_type = 0 then 0x8100 else 0x88A8 If ifh.encap.tag_tipd = CUSTOM<n>: Custom<n> TPID 6-7: Reserved
    #[inline(always)]
    pub fn tag_tpid_cfg(&self) -> u32 {
        (self.0 & 0x380) >> 7
    }
    #[inline(always)]
    pub fn set_tag_tpid_cfg(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 7;
        self.0 &= !0x380;
        self.0 |= value;
    }
    /// Select VID in port tag
    ///
    /// Select VID in port tag. 0: Use classified VID. 1: Use PORT_VLAN_CFG.PORT_VID
    #[inline(always)]
    pub fn tag_vid_cfg(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_tag_vid_cfg(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
}
