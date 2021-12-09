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

/// Register `CW_VAL`
///
/// MPLS label stack Control Word
#[derive(From, Into)]
pub struct CW_VAL(u32);
impl CW_VAL {
    /// Control Word value. The control word is always placed after the last MPLS label.

    ///

    /// x: Control Word
    pub fn cw_val(&self) -> u32 {
        self.0
    }
    pub fn set_cw_val(&mut self, value: u32) {
        self.0 = value;
    }
}

/// Register `LABEL_VAL`
///
/// MPLS label stack field values
///
/// This register contains the MPLS label fields. Label 0 is always placed closest to the link layer MAC addresses: [LL] [LL][LBL0][CW] [LL][LBL0][LBL1][CW] [LL][LBL0][LBL1][LBL2][CW] The register MPLS_LABEL_CFG.LABEL_CNT controls the number of labels.
#[derive(From, Into)]
pub struct LABEL_VAL(u32);
impl LABEL_VAL {
    /// Label field value in MPLS  label N

    ///

    /// x: Label field value
    pub fn label_val(&self) -> u32 {
        (self.0 & 0xfffff000) >> 12
    }
    pub fn set_label_val(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xfffff000);
        self.0 &= !0xfffff000;
        self.0 |= value;
    }
    /// SBIT value in MPLS label N

    ///

    /// x: SBIT field value
    pub fn sbit_val(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_sbit_val(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// TC value in MPLS label N

    ///

    /// x: TC field value
    pub fn tc_val(&self) -> u32 {
        (self.0 & 0xe00) >> 9
    }
    pub fn set_tc_val(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0xe00);
        self.0 &= !0xe00;
        self.0 |= value;
    }
    /// TTL value in MPLS label N

    ///

    /// x: TTL field value
    pub fn ttl_val(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_ttl_val(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}

/// Register `LL_DMAC_LSB`
///
/// Configure DMAC of MPLS link layer
#[derive(From, Into)]
pub struct LL_DMAC_LSB(u32);
impl LL_DMAC_LSB {
    /// DMAC in Link layer. 32 LSB

    ///

    /// n: DMAC LSB
    pub fn dmac_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_dmac_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}

/// Register `LL_DMAC_MSB`
///
/// Configure DMAC of MPLS link layer
#[derive(From, Into)]
pub struct LL_DMAC_MSB(u32);
impl LL_DMAC_MSB {
    /// DMAC in Link layer. 16 MSB

    ///

    /// n: DMAC MSB
    pub fn dmac_msb(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_dmac_msb(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `LL_ETYPE`
///
/// Configure ETYPE of MPLS link layer
#[derive(From, Into)]
pub struct LL_ETYPE(u32);
impl LL_ETYPE {
    /// Ethertype in Link layer

    ///

    /// n: Ethertype
    pub fn etype(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_etype(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `LL_SMAC_LSB`
///
/// Configure SMAC of MPLS link layer
#[derive(From, Into)]
pub struct LL_SMAC_LSB(u32);
impl LL_SMAC_LSB {
    /// SMAC in Link layer. 32 LSB

    ///

    /// n: SMAC LSB
    pub fn smac_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_smac_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}

/// Register `LL_SMAC_MSB`
///
/// Configure SMAC of MPLS link layer
#[derive(From, Into)]
pub struct LL_SMAC_MSB(u32);
impl LL_SMAC_MSB {
    /// SMAC in Link layer. 16 MSB

    ///

    /// n: SMAC MSB
    pub fn smac_msb(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_smac_msb(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `LL_TAG_CFG`
///
/// Configure VLAN tags in MPLS link layer
#[derive(From, Into)]
pub struct LL_TAG_CFG(u32);
impl LL_TAG_CFG {
    /// Enable IFH encapsulation mode for this entry. The frame link layer format is changed to: [LL_DMAC][LL_SMAC][0x8880][0x0009] Optionally one VLAN tag can be added if LL_TAG_CFG.TAG_CFG = 1 [LL_DMAC][LL_SMAC][LL_TAG:0][0x8880][0x0009] None of the other encapsulation fields are used in this mode

    ///

    /// 0: Normal encapsulation mode 1: IFH encapsulation mode
    pub fn ifh_encap_mode(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_ifh_encap_mode(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Control VLAN tags in MPLS link layer

    ///

    /// 0: No tags in link layer 1: One tag after SMAC 2: Two tags after SMAC 3: Reserved
    pub fn tag_cfg(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_tag_cfg(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}

/// Register `LL_TAG_VAL`
///
/// MPLS label stack Control Word
///
/// This register contains the VLAN tags
#[derive(From, Into)]
pub struct LL_TAG_VAL(u32);
impl LL_TAG_VAL {
    /// DEI value in link layer tags

    ///

    /// x: DEI value
    pub fn tag_dei_val(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_tag_dei_val(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// PCP value in link layer tags

    ///

    /// x: PCP value
    pub fn tag_pcp_val(&self) -> u32 {
        (self.0 & 0xe) >> 1
    }
    pub fn set_tag_pcp_val(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0xe);
        self.0 &= !0xe;
        self.0 |= value;
    }
    /// TPID of link layer VLAN tags

    ///

    /// x: TPID value
    pub fn tag_tpid(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_tag_tpid(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    /// VID value in link layer tags

    ///

    /// x: VID value
    pub fn tag_vid_val(&self) -> u32 {
        (self.0 & 0xfff0) >> 4
    }
    pub fn set_tag_vid_val(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0xfff0);
        self.0 &= !0xfff0;
        self.0 |= value;
    }
}

/// Register `MPLS_LABEL_CFG`
///
/// Configure MPLS label
#[derive(From, Into)]
pub struct MPLS_LABEL_CFG(u32);
impl MPLS_LABEL_CFG {
    /// Configure Control Word (CW) in label stack

    ///

    /// 0: No Control Word 1: Add Control Word
    pub fn cw_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_cw_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Select innermost label. Source can be ES0 or encapsulation RAM

    ///

    /// 0: Use label from encapsulation table 1: Use label from ES0
    pub fn inner_lbl_sel(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_inner_lbl_sel(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Configure the number of MPLS labels to use for this encapsulation entry

    ///

    /// 0: No Labels in encapsulation 1: One MPLS label 2: Two MPLS labels 3: Three MPLS labels
    pub fn label_cnt(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_label_cnt(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
}

/// Register `MPLS_REMARK_CFG`
///
/// Configure MPLS label remarking
///
/// The rewriter can remark certain parameters in the pushed MPLS data. The following illustrates provides an overview of the available remark options.
#[derive(From, Into)]
pub struct MPLS_REMARK_CFG(u32);
impl MPLS_REMARK_CFG {
    /// Configure the Label-field used in label N.

    ///

    /// 0 : LABEL_VAL[N].LABEL_VAL 1-3: Reserved 4 : Mapped using mapping table 0 5 : Mapped using mapping table 1 6 : Mapped using mapping table 2 7 : Mapped using mapping table 3
    pub fn lbl_sel(&self) -> u32 {
        (self.0 & 0x380) >> 7
    }
    pub fn set_lbl_sel(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x380);
        self.0 &= !0x380;
        self.0 |= value;
    }
    /// Configure the S-bit used in label N.

    ///

    /// 0: Classified SBIT (ISBIT) 1: Fixed: LABEL_VAL[N].SBIT_VAL 2: Mixed: Use Classified SBIT if IFH.DST.ENCAP.PDU_TYPE = OAM_MPLS_TP else LABEL_VAL[N].SBIT_VAL 3: Reserved
    pub fn sbit_sel(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_sbit_sel(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Configure the TC-field used in label N.

    ///

    /// 0 : Classified TC 1 : LABEL_VAL[N].TC_VAL 2-3: Reserved 4: Mapped using mapping table 0, otherwise use LABEL_VAL[N].TC_VAL 5: Mapped using mapping table 1, otherwise use mapping table 0 6: Mapped using mapping table 2, otherwise use LABEL_VAL[N].TC_VAL 7: Mapped using mapping table 3, otherwise use mapping table 2
    pub fn tc_sel(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_tc_sel(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }
    /// Configure the TTL-field used in label N.

    ///

    /// 0: Classified TTL (ITTL) 1: Fixed: LABEL_VAL[N].TTL_VAL 2: Mixed: Use Classified TTL if IFH.DST.ENCAP.PDU_TYPE = OAM_MPLS_TP else LABEL_VAL[N].TTL_VAL 3: Reserved
    pub fn ttl_sel(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_ttl_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}

/// Register `PTP_RSRV_NOT_ZERO_1`
///
/// PTP reserved field check
#[derive(From, Into)]
pub struct PTP_RSRV_NOT_ZERO_1(u32);
impl PTP_RSRV_NOT_ZERO_1 {
    /// This register covers ports 32-56. See PTP_RSRV_NOT_ZERO for description.
    pub fn ptp_rsrv_not_zero_1(&self) -> u32 {
        self.0 & 0x1ffffff
    }
    pub fn set_ptp_rsrv_not_zero_1(&mut self, value: u32) {
        assert!(value <= 0x1ffffff);
        self.0 &= !0x1ffffff;
        self.0 |= value;
    }
}

/// Register `RSV_LABEL_CFG`
///
/// Configure MPLS label
#[derive(From, Into)]
pub struct RSV_LABEL_CFG(u32);
impl RSV_LABEL_CFG {
    /// Enable reserved MPLS label insertion for MPLS-OAM frames. When this bit is set, an additional MPLS label is inserted if CW insertion is disabled and IFH.DST.ENCAP.PDU_TYPE=OAM_MPLS_TP or IFH.DST.ENCAP.PDU_TYPE=Y1731 and IFH.DST.ENCAP.TYPE_AFTER_POP=CW Note: The reserved label can only be inserted if a CW is not inserted for the frame.

    ///

    /// 0: Disabled 1: Add reserved label if allowed
    pub fn rsv_lbl_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_rsv_lbl_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Select position of the reserved label. It can be added before or after the last MPLS label

    ///

    /// 0: Add before last label 1: Add after last label like the CW
    pub fn rsv_lbl_pos(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_rsv_lbl_pos(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Configure TC-field used in the reserved label

    ///

    /// 0 : Classified TC 1 : RSV_LABEL_VAL.RSV_TC_VAL 2-3: Reserved 4: Mapped using mapping table 0, otherwise use RSV_LABEL_VAL.RSV_TC_VAL 5: Mapped using mapping table 1, otherwise use mapping table 0 6: Mapped using mapping table 2, otherwise use RSV_LABEL_VAL.RSV_TC_VAL 7: Mapped using mapping table 3, otherwise use mapping table 2
    pub fn rsv_tc_sel(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    pub fn set_rsv_tc_sel(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x1c);
        self.0 &= !0x1c;
        self.0 |= value;
    }
}

/// Register `RSV_LABEL_VAL`
///
/// Reserved label field values
///
/// This register contains the reserved MPLS label fields. The reserved label can only be added if the control word (CW) is not enabled.
#[derive(From, Into)]
pub struct RSV_LABEL_VAL(u32);
impl RSV_LABEL_VAL {
    /// Label field value in the reserved MPLS label

    ///

    /// x: Label field value
    pub fn rsv_lbl_val(&self) -> u32 {
        (self.0 & 0xfffff000) >> 12
    }
    pub fn set_rsv_lbl_val(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xfffff000);
        self.0 &= !0xfffff000;
        self.0 |= value;
    }
    /// SBIT value in reserved the MPLS label

    ///

    /// x: SBIT field value
    pub fn rsv_sbit_val(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_rsv_sbit_val(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// TC value in reserved the MPLS label

    ///

    /// x: TC field value
    pub fn rsv_tc_val(&self) -> u32 {
        (self.0 & 0xe00) >> 9
    }
    pub fn set_rsv_tc_val(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0xe00);
        self.0 &= !0xe00;
        self.0 |= value;
    }
    /// TTL value in reserved the MPLS label

    ///

    /// x: TTL field value
    pub fn rsv_ttl_val(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_rsv_ttl_val(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
