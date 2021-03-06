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
/// MPLS-TP BFD configuration.
///
/// This register contains misc. bit fields used to figure the BFD session monitored by the VOE.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BFD_CONFIG(u32);
impl BFD_CONFIG {
    /// The configuration of this bit field indicates if the incoming BFD CC PDUs are expected to have the AUTH bit set. The value of the AUTH bit in the incoming BFD PDUs is verified as part of the Rx verification  AUTH_MISMATCH test. If this test fails, the following sticky bit is asserted: * VOP_MPLS:VOE_STAT_MPLS:BFD_RX_STICKY.AUTH_MISMATCH_ERR_STICKY
    #[inline(always)]
    pub fn bfd_cc_auth_ena(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    #[inline(always)]
    pub fn set_bfd_cc_auth_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 9;
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// The VOE can accept two different G-ACH Channel Types as being valid BFD CC PDUs, depending of the setting of this register. If the VOE receives a MPLS-TP with the G-ACH Channel Type which is not the one configured for BFD CC, the PDU will be classified as either Generic or Unknown G-ACH Channel Type. I.e. if BFD_CC_RFC6428 = 1, MPLS-TP PDUs with G-ACH Channel Type = 0x0007 will be classified as either Generic or UNKNOWN G-ACH Channel Type.
    ///
    /// The pending on configuration of this bit field, the VOE will expect the following G-ACH Channel Type for BFD CC PDUs: 0: 0x0007 (RFC_5885) 1: 0x0022 (RFC_6428)
    #[inline(always)]
    pub fn bfd_cc_rfc6428(&self) -> u32 {
        (self.0 & 0x400000) >> 22
    }
    #[inline(always)]
    pub fn set_bfd_cc_rfc6428(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 22;
        self.0 &= !0x400000;
        self.0 |= value;
    }
    /// This bit field configures whether the BFD session is running in: 0: Independent BFD Mode 1: Coordinated BFD Mode
    #[inline(always)]
    pub fn bfd_coordinated_mode_ena(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    #[inline(always)]
    pub fn set_bfd_coordinated_mode_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 16;
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// The configuration of this bit field indicates if the incoming BFD CV PDUs are expected to have the AUTH bit set. The value of the AUTH bit in the incoming BFD PDUs is verified as part of the Rx verification  AUTH_MISMATCH test. If this test fails, the following sticky bit is asserted: * VOP_MPLS:VOE_STAT_MPLS:BFD_RX_STICKY.AUTH_MISMATCH_ERR_STICKY
    #[inline(always)]
    pub fn bfd_cv_auth_ena(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_bfd_cv_auth_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// As part of the Rx verification of BFD PDUs, the BFD lengh field is compared to the value configured in this bitfield as part of the MAX_LEN test. If the MAX_LEN test fails, the frame is discarded at the following sticky bit is asserted. * VOP_MPLS:VOE_STAT_MPLS:BFD_RX_STICKY.MAX_LEN_ERR_STICKY
    #[inline(always)]
    pub fn bfd_max_len(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_bfd_max_len(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    /// If enabled the VOE will update the following registers based on the information received in valid BFD CC PDUs: Coordinated / NEIS: * VOP_MPLS:VOE_STAT_MPLS:BFD_SRC_INFO.BFD_REMOTE_STATE_SRC * VOP_MPLS:VOE_STAT_MPLS:BFD_SRC_INFO.BFD_REMOTE_DIAG_SRC * VOP_MPLS:VOE_STAT_MPLS:BFD_SRC_INFO.BFD_REMOTE_DM_SRC FEIS session: * VOP_MPLS:VOE_STAT_MPLS:BFD_SINK_INFO.BFD_REMOTE_STATE_SINK * VOP_MPLS:VOE_STAT_MPLS:BFD_SINK_INFO.BFD_REMOTE_DIAG_SINK * VOP_MPLS:VOE_STAT_MPLS:BFD_SINK_INFO.BFD_REMOTE_DM_SINK The session to which a BFD PDU belongs is determined as part of the BFD Rx verification, based on comparing the BFD PDU Your Discriminator to the values configured in the following bit fields: * VOP_MPLS:VOE_CONF_MPLS:BFD_CONFIG.BFD_COORDINATED_MODE_ENA Coordinated / NEIS session: * VOP_MPLS:VOE_CONF_MPLS:BFD_LOCAL_DISCR_SRC.BFD_LOCAL_DISCR_SRC FEIS session: * VOP_MPLS:VOE_CONF_MPLS:BFD_LOCAL_DISCR_SINK.BFD_LOCAL_DISCR_SINK
    #[inline(always)]
    pub fn bfd_rx_sample_ena(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_bfd_rx_sample_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// If this field is asserted all incoming BFD CC PDUs will be verified in HW. Deasserting this field will disable all Rx validation of BFD CC PDUs.
    #[inline(always)]
    pub fn bfd_rx_verify_cc_ena(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    #[inline(always)]
    pub fn set_bfd_rx_verify_cc_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// If this field is asserted all incoming BFD CV PDUs will be verified in HW. Deasserting this field will disable all Rx validation of BFD CV PDUs.
    #[inline(always)]
    pub fn bfd_rx_verify_cv_ena(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    #[inline(always)]
    pub fn set_bfd_rx_verify_cv_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 14;
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// When disabled, the Your/My Discriminator fields of incoming BFD CC/CV PDUs are not verified of BFD CC/CV PDUs. The BFD PDU is assumed to  belong to the Coordinated / NEIS session. When an Rx PDU does not match any of the configured discriminators, the following sticky bit is asserted: * VOP_MPLS:VOE_STAT_MPLS:BFD_RX_STICKY.YOUR_DISCR_ERR_STICKY This test is disabled if: * BFD_RX_VERIFY_CC_ENA = 0 * BFD_RX_VERIFY_CV_ENA = 0 depending on the PDU type.
    #[inline(always)]
    pub fn bfd_rx_verify_discr_ena(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_bfd_rx_verify_discr_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// When enabled, the Rx verification will verify: Detect Mult is ZERO Multipoint asserted Auth Bit matches configured Demand bit is asserted F and P bits are not set in the same frame. If any of the above test fails, the corresponding sticky bit is asserted (VOP_MPLS:VOE_STAT_MPLS:BFD_RX_STICKY.*): * DM_ZERO_ERR_STICKY * M_BIT_SET_ERR_STICKY * AUTH_MISMATCH_ERR_STICKY * D_BIT_SET_ERR_STICKY * P_AND_F_BIT_SET_ERR_STICKY The AUTH-bit testing is further dependent on the following bit fields: * BFD_CC_AUTH_ENA * BFD_CV_AUTH_ENA The FLAGS test is disabled if: * BFD_RX_VERIFY_CC_ENA = 0 * BFD_RX_VERIFY_CV_ENA = 0 depending on the PDU type.
    #[inline(always)]
    pub fn bfd_rx_verify_flags_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_bfd_rx_verify_flags_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// If not enabled, the MIN_LEN test will not be done as part of the BFD Rx verification of BFD CC/CV PDUs. This testing will depend on the setting of the following bit fields: * BFD_CC_AUTH_ENA * BFD_CV_AUTH_ENA When an Rx PDU fails the MIN_LEN test, the following sticky bit is asserted: * VOP_MPLS:VOE_STAT_MPLS:BFD_RX_STICKY.MIN_LEN_ERR_STICKY The MIN_LEN test is disabled if: * BFD_RX_VERIFY_CC_ENA = 0 * BFD_RX_VERIFY_CV_ENA = 0 depending on the PDU type.
    #[inline(always)]
    pub fn bfd_rx_verify_min_len_ena(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_bfd_rx_verify_min_len_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// If not enabled, the VERSION test will not be done as part of the BFD Rx verification of BFD CC/CV PDUs. This testing will depend on the setting of the following bit fields: * BFD_CC_AUTH_ENA * BFD_CV_AUTH_ENA When an Rx PDU fails the VERSION test, the following sticky bit is asserted: * VOP_MPLS:VOE_STAT_MPLS:BFD_RX_STICKY.VERSION_ERR_STICKY The VERSION test is disabled if: * BFD_RX_VERIFY_CC_ENA = 0 * BFD_RX_VERIFY_CV_ENA = 0 depending on the PDU type.
    #[inline(always)]
    pub fn bfd_rx_verify_version_ena(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    #[inline(always)]
    pub fn set_bfd_rx_verify_version_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// The configuration of this bit field specifies the LOC timeout counter to be used for LOC scan for this VOE. Every time the LOC timeout counter indicated by this register causes a scan, the VOE LOC counter is incremented. The LOC counter is cleared upon reception of a valid BFD CC/CV PDU. The LOC counter is located in the following bit field: * VOP_MPLS:VOE_STAT_MPLS:BFD_STAT.BFD_MISS_CNT
    ///
    /// 0: Indicates that the LOC counter is not incremented. 1: Indicates that the LOC counter is incremented by LOC timeout counter 0 2: Indicates that the LOC counter is incremented by LOC timeout counter 1 3: Indicates that the LOC counter is incremented by LOC timeout counter 2 4: Indicates that the LOC counter is incremented by LOC timeout counter 3 5: Indicates that the LOC counter is incremented by LOC timeout counter 4 6: Indicates that the LOC counter is incremented by LOC timeout counter 5 7: Indicates that the LOC counter is incremented by LOC timeout counter 6
    #[inline(always)]
    pub fn bfd_scan_period(&self) -> u32 {
        (self.0 & 0x380000) >> 19
    }
    #[inline(always)]
    pub fn set_bfd_scan_period(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 19;
        self.0 &= !0x380000;
        self.0 |= value;
    }
    /// If enabled, the VOE will update the following fields in the outgoing PDUs when transmitting BFD CC / CV PDUs * BFD.STATE * BFD.DIAG * BFD.DM
    #[inline(always)]
    pub fn bfd_tx_update_ena(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    #[inline(always)]
    pub fn set_bfd_tx_update_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 17;
        self.0 &= !0x20000;
        self.0 |= value;
    }
}
/// BFD Local Discriminator (BFD_SINK)
///
/// The Discriminator of the Local BFD Sink. Only used if the BFD session is configured for Independent Mode: * VOP_MPLS:VOE_CONF_MPLS:BFD_CONFIG.BFD_COORDINATED_MODE_ENA = 0
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BFD_LOCAL_DISCR_SINK(u32);
impl BFD_LOCAL_DISCR_SINK {
    /// See register description.
    #[inline(always)]
    pub fn bfd_local_discr_sink(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_bfd_local_discr_sink(&mut self, value: u32) {
        self.0 = value;
    }
}
/// BFD Local Discriminator (BFD_SRC)
///
/// The Discriminator of the Local BFD Source.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BFD_LOCAL_DISCR_SRC(u32);
impl BFD_LOCAL_DISCR_SRC {
    /// See the register description.
    #[inline(always)]
    pub fn bfd_local_discr_src(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_bfd_local_discr_src(&mut self, value: u32) {
        self.0 = value;
    }
}
/// BFD Remote Discriminator (BFD_SINK)
///
/// The Discriminator of the remote BFD entity communicating with the Local BFD Sink. Only used if the BFD session is configured for Independent Mode: * VOP_MPLS:VOE_CONF_MPLS:BFD_CONFIG.BFD_COORDINATED_MODE_ENA = 0
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BFD_REMOTE_DISCR_SINK(u32);
impl BFD_REMOTE_DISCR_SINK {
    /// See register description.
    #[inline(always)]
    pub fn bfd_remote_discr_sink(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_bfd_remote_discr_sink(&mut self, value: u32) {
        self.0 = value;
    }
}
/// BFD Remote Discriminator (BFD_SRC)
///
/// The Discriminator of the remote BFD entity communicating with the Local BFD Source
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BFD_REMOTE_DISCR_SRC(u32);
impl BFD_REMOTE_DISCR_SRC {
    /// See register description.
    #[inline(always)]
    pub fn bfd_remote_discr_src(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_bfd_remote_discr_src(&mut self, value: u32) {
        self.0 = value;
    }
}
/// CPU extraction for the supported MPLS-TP OAM G-ACH Channel Types.
///
/// Configures CPU copy for the supported MPLS-TP PDU G-ACH Channel Types. Configuring a PDU type for CPU extraction, will result in all valid OAM PDUs of this type to extracted to the CPU. The PDU will be extsracted to the extraction queue configured for the G-ACH Channel Type in the following registers: * VOP::CPU_EXTR_MPLS.* Invalid OAM PDUs are not extracted based on the configuration in this register group. OAM PDUs are considered invalid if they fail either of the following checks: * Protocol Specific verification (E.g. BFD Rx / Tx verification)
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CPU_COPY_CTRL_MPLS(u32);
impl CPU_COPY_CTRL_MPLS {
    /// If asserted all valid BFD CC PDUs received by the VOE are extracted to the CPU. Extraction queue is determined by: * VOP::CPU_EXTR_MPLS.BFD_CC_CPU_QU
    ///
    /// '0': No extraction to CPU '1': Extract valid BFD CC PDUs to CPU
    #[inline(always)]
    pub fn bfd_cc_cpu_copy_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_bfd_cc_cpu_copy_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// If asserted all valid BFD CV PDUs received by the VOE are extracted to the CPU. Extraction queue is determined by: * VOP::CPU_EXTR_MPLS.BFD_CV_CPU_QU
    ///
    /// '0': No extraction to CPU '1': Extract valid BFD CV frames to CPU
    #[inline(always)]
    pub fn bfd_cv_cpu_copy_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bfd_cv_cpu_copy_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// This bit field contains 8 bits each of which represent one of the Generic G-ACH Channel Types. If the bit representing a specific Generic G-ACH Channel Type is asserted, all valid PDUs received by the VOE of that type are extracted to the CPU queue configured in the following bit fields: * VOP::MPLS_GENERIC_CODEPOINT.GENERIC_CODEPOINT_CPU_QU
    ///
    /// x0x: No CPU copy x1x: Copy to CPU
    #[inline(always)]
    pub fn generic_copy_mask(&self) -> u32 {
        (self.0 & 0x7f8) >> 3
    }
    #[inline(always)]
    pub fn set_generic_copy_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 3;
        self.0 &= !0x7f8;
        self.0 |= value;
    }
    /// Configures whether MPLS-TP OAM PDUs with UNKNOWN G-ACH Channel Type should be extracted to the CPU. Extracted frames are extracted to the default CPU queue, configured in: * VOP::CPU_EXTR_CFG.DEF_COPY_QU
    ///
    /// '0': No CPU copy '1': Copy to CPU
    #[inline(always)]
    pub fn unk_cpt_cpu_copy_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_unk_cpt_cpu_copy_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// Configuration of which OAM PDUs should be counted by LM counters.
///
/// Default behavior is that all MPLS-TP OAM PDUs processed by a VOE will not be counted as part of the LM count. Using this register (OAM_CNT_DATA_MPLS) it is possible to configure the OAM PDUs separately to be counted as part of the LM count. Frames are counted separately in the Rx and Tx direction. The data counters are located: Service VOE: --------------------- Egress: REW:VOE_SRV_LM_CNT.SRV_LM_CNT_LSB.SRV_LM_CNT_LSB Ingress: ANA_AC_OAM_MOD:VOE_SRV_LM_CNT.SRV_LM_CNT_LSB.SRV_LM_CNT_LSB Port VOE: ------------------- Egress: REW:VOE_PORT_LM_CNT:PORT_LM_CNT_LSB.PORT_LM_CNT_LSB Ingress: ANA_AC_OAM_MOD:VOE_PORT_LM_CNT:PORT_LM_CNT_LSB.PORT_LM_CNT_LSB
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct OAM_CNT_DATA_MPLS(u32);
impl OAM_CNT_DATA_MPLS {
    /// Enable / disable count of valid BFD CC OAM PDUs as part of the LM counters.
    ///
    /// '0': Do not count as data '1': Count as data
    #[inline(always)]
    pub fn bfd_cc_cnt_data_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_bfd_cc_cnt_data_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enable / disable count of valid BFD CV OAM PDUs as part of the LM counters.
    ///
    /// '0': Do not count as data '1': Count as data
    #[inline(always)]
    pub fn bfd_cv_cnt_data_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bfd_cv_cnt_data_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable / disable that valid MPLS-TP OAM PDUs with Generic G-ACH Channel Type are counted by the VOE LM counters. This bit field contains a separate bit for each of the possible 8 Generic G-ACH Channel Types.
    ///
    /// x0x: Do NOT count generic Channel Type in LM counters. x1x: DO count generic Channel Type in LM counters.
    #[inline(always)]
    pub fn generic_cpt_cnt_data_mask(&self) -> u32 {
        (self.0 & 0x7f8) >> 3
    }
    #[inline(always)]
    pub fn set_generic_cpt_cnt_data_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 3;
        self.0 &= !0x7f8;
        self.0 |= value;
    }
    /// If a PDU is received with an G-ACH Channel Type which does not match any Specific G-ACH Channel Type or a Generic G-ACH Channel Type, it will be processed as an UNKNOWN G-ACH Channel Type. This bit field configures if OAM frames with UNKNOWN G-ACH Channel Type are counted as data in the LM counters.
    ///
    /// '0': Do not count as data '1': Count as data
    #[inline(always)]
    pub fn unk_cpt_cnt_data_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_unk_cpt_cnt_data_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// Configuration which MPLS-TP Channel Types are counted in selected PDU counter.
///
/// The OAM frames processed by the VOE can be counted separately in Rx and Tx direction. In each direction there are two counters: 1) Selected OAM counter: -------------------------------------------------------- This counter counts all the PDU types selected for counting using the OAM_CNT_SEL_MPLS register: * VOP_MPLS:VOE_STAT_MPLS:RX_CNT_SEL_OAM_MPLS * VOP_MPLS:VOE_STAT_MPLS:TX_CNT_SEL_OAM_MPLS 2) NON Selected OAM counter: -------------------------------------------------------- * VOP_MPLS:VOE_STAT_MPLS:RX_CNT_NON_SEL_OAM_MPLS * VOP_MPLS:VOE_STAT_MPLS:TX_CNT_NON_SEL_OAM_MPLS Any valid OAM PDU is counted in exactly one of the above registers. I.e. as default all OAM PDUs are not selected, and they are all counted in the default OAM counters: RX / TX_CNT_NON_SEL_OAM_MPLS. Using this register (OAM_CNT_SEL_MPLS), PDUs can be moved to the selected counters: RX / TX_CNT_SEL_OAM_MPLS. The selection of OAM PDUs for the selected counter is done commonly for the Tx and Rx direction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct OAM_CNT_SEL_MPLS(u32);
impl OAM_CNT_SEL_MPLS {
    /// This register configures whether valid BFD CC PDUs are counted Selected OAM or NON Selected OAM.
    ///
    /// '0': Count as other OAM '1': Count as selected OAM
    #[inline(always)]
    pub fn bfd_cc_cnt_sel_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_bfd_cc_cnt_sel_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// This register configures whether valid BFD CV PDUs are counted Selected OAM or NON Selected OAM.
    ///
    /// '0': Count as other OAM '1': Count as selected OAM
    #[inline(always)]
    pub fn bfd_cv_cnt_sel_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bfd_cv_cnt_sel_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable / disable that valid OAM PDUs with Generic G-ACH Channel Type are counted as selected OAM. This bit field contains a separate bit for each of the possible 8 Generic opcodes.
    ///
    /// x0x: Count as other OAM x1x: Count as selected OAM
    #[inline(always)]
    pub fn generic_cpt_cnt_sel_mask(&self) -> u32 {
        (self.0 & 0x7f8) >> 3
    }
    #[inline(always)]
    pub fn set_generic_cpt_cnt_sel_mask(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 3;
        self.0 &= !0x7f8;
        self.0 |= value;
    }
    /// MPLS-TP OAM PDUs not recognized as either one of the PDUs with special configuration or as a Generic G-ACH Channel Type, will be classified as an UNKNOWN PDU. This register configures whether UNKNOWN PDUs should be counted as selected OAM.
    ///
    /// '0': Count as other OAM '1': Count as selected OAM
    #[inline(always)]
    pub fn unk_cpt_cnt_sel_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_unk_cpt_cnt_sel_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// OAM HW processing control
///
/// Configures per MPLS-TP G-ACH Channel Type if it is processed by VOE hardware. If an MPLS-TP OAM type is not enabled in this register, the OAM PDU will not be modified by the VOE. This implies that the OAM PDU is not updated. However, note the following: * The Rx sticky bits will be set for a PDU, even when the HW processing is not enabled. * OAM PDU can be extracted to the CPU, even when HW processing is not enabled. * LM counters will be updated. * SEL / non SEL counters are updated.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct OAM_HW_CTRL_MPLS(u32);
impl OAM_HW_CTRL_MPLS {
    /// Enable HW processing of valid BFD CC PDUs received by the VOE in both the Tx and the Rx direction. If this is disabled, no verification of the YourDiscriminator is done of the incoming BFD CC PDUs. All Rx PDUs will be processed as belonging to the Coordinated Mode. I.e. a BFD CC PDUs will never be processed as belonging to the FEIS Session.
    #[inline(always)]
    pub fn bfd_cc_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_bfd_cc_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enable HW processing of valid BFD CV PDUs received by the VOE in both the Tx and the Rx direction. If this is disabled, no verification of the YourDiscriminator is done of the incoming BFD CV PDUs. All Rx PDUs will be processed as belonging to the Coordinated Mode. I.e. a BFD CV PDUs will never be processed as belonging to the FEIS Session.
    #[inline(always)]
    pub fn bfd_cv_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bfd_cv_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Path MEP configuration
///
/// This register is used to assign a Path VOE to the current service VOE. Assigning a Path VOE to a Service VOE implies that all frames received by this VOE, will also be counted by the Path VOE. The VOE index of the Path VOE is configured by the following bit field: * PATH_VOEID The path VOE must be enabled by asserting the following field: * PATH_VOE_ENA
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PATH_VOE_MPLS(u32);
impl PATH_VOE_MPLS {
    /// Assigns a Path VOE to the VOE. Must be enabled by: PATH_VOE_ENA = 1
    #[inline(always)]
    pub fn path_voeid(&self) -> u32 {
        self.0 & 0x3ff
    }
    #[inline(always)]
    pub fn set_path_voeid(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
    /// Configures if a service VOE is part of a Path VOE.
    #[inline(always)]
    pub fn path_voe_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_path_voe_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
}
/// Misc. VOE control configuration
///
/// This register includes configuration of misc. VOE control properties.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VOE_CTRL_MPLS(u32);
impl VOE_CTRL_MPLS {
    /// Configures VOE for Down-MEP or Up-MEP functionality. Note: Port VOE may NOT be configured for Up-MEP functionality, they only support Down-MEP implementation.
    #[inline(always)]
    pub fn upmep_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_upmep_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Enables VOE functionality. When the VOE is not enabled, it will not do any OAM processing or update statistics. The VOE can be configured while not enabled.
    #[inline(always)]
    pub fn voe_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_voe_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// If another VOE is pointing to this VOE as a Path VOE using the following configuration: * VOP_MPLS:VOE_CONF_MPLS:PATH_VOE_MPLS.PATH_VOEID * VOP_MPLS:VOE_CONF_MPLS:PATH_VOE_MPLS.PATH_VOE_ENA this register MUST be set to '1'. If not this register must be set to '0'.
    #[inline(always)]
    pub fn voe_is_path(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_voe_is_path(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
