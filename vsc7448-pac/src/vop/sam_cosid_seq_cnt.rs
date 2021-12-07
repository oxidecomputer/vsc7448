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

/// Register `PORT_TX_COSID_MAP1`
///
/// MSB of Tx Port VOE mapping table (REW).
///
/// This register contains the upper 32 bits of the Port VOE Tx (REW) COSID mapping table. This mapping in this register is used when Port DEI = 1
#[derive(From, Into)]
pub struct PORT_TX_COSID_MAP1(u32);
impl PORT_TX_COSID_MAP1 {    ///
    /// See register description.
    pub fn port_tx_cosid_map1(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_port_tx_cosid_map1(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `SAM_LBM_TX_TRANSID`
///
/// LBM/TST/CCM Tx PDUs per priority.
///
/// When a VOE is assigned a SAM per COSID counter set, this register counts the number of Tx CCM/LBM/TST PDUs for priorities 0-6. Depending on the PDU being counted, priority 7 is counted by the following counter. CCM: * VOP:VOE_STAT:CCM_TX_SEQ_CFG.CCM_TX_SEQ LBM/LBR: * VOP:VOE_STAT:LBM_TX_TRANSID_CFG.LBM_TX_TRANSID TST: * VOP:VOE_STAT:LBM_TX_TRANSID_CFG.LBM_TX_TRANSID
#[derive(From, Into)]
pub struct SAM_LBM_TX_TRANSID(u32);
impl SAM_LBM_TX_TRANSID {    ///
    /// See register description
    pub fn sam_lbm_tx_transid(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_sam_lbm_tx_transid(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `SAM_LBR_RX_FRM_CNT`
///
/// LBR/TST/CCM Rx PDUs per priority
///
/// When a VOE is assigned a SAM per COSID counter set, this register counts the number of Rx LBR/TST/CCM PDU for priorities 0-6. Depending on the PDU being counted, priority 7 is counted by the following counter. CCM: * VOP:VOE_STAT:CCM_RX_FRM_CNT.CCM_RX_VLD_FC_CNT LBM/LBR: * VOP:VOE_STAT: LBR_RX_FRM_CNT.LBR_RX_FRM_CNT TST: * VOP:VOE_STAT: LBR_RX_FRM_CNT.LBR_RX_FRM_CNT
#[derive(From, Into)]
pub struct SAM_LBR_RX_FRM_CNT(u32);
impl SAM_LBR_RX_FRM_CNT {    ///
    /// See register description.
    pub fn sam_lbr_rx_frm_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_sam_lbr_rx_frm_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `SAM_LBR_RX_TRANSID`
///
/// LBR/TST/CCM Rx Transaction ID per priority
///
/// When a VOE is assigned a SAM per COSID counter set, this register counts the latest Sequence Number / Transaction ID received in a valid LBR/TST/CCM PDU for priorities 0-6. Depending on the PDU being counted, priority 7 is stored in the following register. CCM: * VOP:VOE_STAT:CCM_RX_SEQ_CFG.CCM_RX_SEQ LBM/LBR: * VOP:VOE_STAT:LBR_RX_TRANSID_CFG.LBR_RX_TRANSID TST: * VOP:VOE_STAT:LBR_RX_TRANSID_CFG.LBR_RX_TRANSID
#[derive(From, Into)]
pub struct SAM_LBR_RX_TRANSID(u32);
impl SAM_LBR_RX_TRANSID {    ///
    /// See register description.
    pub fn sam_lbr_rx_transid(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_sam_lbr_rx_transid(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `SAM_LBR_TX_FRM_CNT`
///
/// LBR Tx PDUs per priority
///
/// When a VOE is assigned a SAM per COSID counter set, this register counts the number of Tx LBR PDU for priorities 0-6. Depending on the PDU being counted, priority 7 is counted by the following counter. CCM: (Not used) LBR: * VOP:VOE_STAT:LBR_TX_FRM_CNT.LBR_TX_FRM_CNT TST: (Not used)
#[derive(From, Into)]
pub struct SAM_LBR_TX_FRM_CNT(u32);
impl SAM_LBR_TX_FRM_CNT {    ///
    /// See register description.
    pub fn sam_lbr_tx_frm_cnt(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_sam_lbr_tx_frm_cnt(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}
