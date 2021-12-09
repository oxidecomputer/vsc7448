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

/// Register `CCM_LM_RX_B_REG`
///
/// CCM-LM sample
///
/// Contains the sampled value of CCM_LM.rx_fc_b from the last valid CCM_LM frame.
#[derive(From, Into)]
pub struct CCM_LM_RX_B_REG(u32);
impl CCM_LM_RX_B_REG {
    /// Contains the sampled value of CCM_LM.rx_fc_b from the last valid CCM_LM frame.
    pub fn ccm_lm_rx_b(&self) -> u32 {
        self.0
    }
    pub fn set_ccm_lm_rx_b(&mut self, value: u32) {
        self.0 = value;
    }
}

/// Register `DM_PTP_DOMAIN_CFG`
///
/// Port PTP domain select
///
/// Jaguar2-R supports three PTP time domains. The Delay Measurements (Y.1731) uses the PTP timing for updating the DM PDUs. For this to work, each port on the switch must be configured for which PTP timedomain to use. This is done by configuring this register. The settings must be the same in the REW and the ANA_AC instatiation of the OAM_PDU_MOD block.
#[derive(From, Into)]
pub struct DM_PTP_DOMAIN_CFG(u32);
impl DM_PTP_DOMAIN_CFG {    pub fn ptp_domain(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_ptp_domain(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}

/// Register `RD_LAST_PORT_BYTE_CNT_LSB`
///
/// PORT_BYTE_CNT_LSB sample register.
///
/// Whenever either of the bit fields the following RAM are read: * ANA_AC_OAM_MOD:VOE_SRV_LM_CNT (ANA) * REW:VOE_SRV_LM_CNT (REW) this register will sample the value of the following bit field: * ANA_AC_OAM_MOD:VOE_PORT_LM_CNT:PORT_BYTE_CNT_LSB.PORT_BYTE_CNT_LSB (ANA) * REW:VOE_PORT_LM_CNT:PORT_BYTE_CNT_LSB.PORT_BYTE_CNT_LSB (REW)
#[derive(From, Into)]
pub struct RD_LAST_PORT_BYTE_CNT_LSB(u32);
impl RD_LAST_PORT_BYTE_CNT_LSB {
    /// See register description.
    pub fn rd_last_port_byte_cnt_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_rd_last_port_byte_cnt_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}

/// Register `RD_LAST_PORT_BYTE_CNT_MSB`
///
/// PORT_BYTE_CNT_MSB sample register.
///
/// Whenever either of the bit fields the following RAM are read: * ANA_AC_OAM_MOD:VOE_SRV_LM_CNT (ANA) * REW:VOE_SRV_LM_CNT (REW) this register will sample the value of the following bit field: * ANA_AC_OAM_MOD:VOE_PORT_LM_CNT:PORT_BYTE_CNT_MSB.PORT_BYTE_CNT_MSB (ANA) * REW:VOE_PORT_LM_CNT:PORT_BYTE_CNT_MSB.PORT_BYTE_CNT_MSB (REW)
#[derive(From, Into)]
pub struct RD_LAST_PORT_BYTE_CNT_MSB(u32);
impl RD_LAST_PORT_BYTE_CNT_MSB {
    /// See register description.
    pub fn rd_last_port_byte_cnt_msb(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_rd_last_port_byte_cnt_msb(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}

/// Register `RD_LAST_PORT_FRM_CNT_LSB`
///
/// PORT_NON_LM_CNT_LSB sample register.
///
/// Whenever either of the bit fields the following RAM are read: * ANA_AC_OAM_MOD:VOE_SRV_LM_CNT (ANA) * REW:VOE_SRV_LM_CNT (REW) this register will sample the value of the following bit field: * ANA_AC_OAM_MOD:VOE_PORT_LM_CNT:PORT_FRM_CNT_LSB.PORT_FRM_CNT_LSB (ANA) * REW:VOE_PORT_LM_CNT:PORT_FRM_CNT_LSB.PORT_FRM_CNT_LSB (REW)
#[derive(From, Into)]
pub struct RD_LAST_PORT_FRM_CNT_LSB(u32);
impl RD_LAST_PORT_FRM_CNT_LSB {
    /// See register description.
    pub fn rd_last_port_frm_cnt_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_rd_last_port_frm_cnt_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}

/// Register `RD_LAST_PORT_LM_CNT_LSB`
///
/// PORT_LM_CNT_LSB sample register.
///
/// Whenever either of the bit fields the following RAM are read: * ANA_AC_OAM_MOD:VOE_SRV_LM_CNT (ANA) * REW:VOE_SRV_LM_CNT (REW) this register will sample the value of the following bit field: * ANA_AC_OAM_MOD:VOE_PORT_LM_CNT:PORT_LM_CNT_LSB.PORT_LM_CNT_LSB (ANA) * REW:VOE_PORT_LM_CNT:PORT_LM_CNT_LSB.PORT_LM_CNT_LSB (REW)
#[derive(From, Into)]
pub struct RD_LAST_PORT_LM_CNT_LSB(u32);
impl RD_LAST_PORT_LM_CNT_LSB {
    /// See register description.
    pub fn rd_last_port_lm_cnt_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_rd_last_port_lm_cnt_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
