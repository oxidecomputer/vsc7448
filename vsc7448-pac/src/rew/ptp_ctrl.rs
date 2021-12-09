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
/// COSID value selection
///
/// The multi COSID auto update function is active if the following IFH fields are set by the AFI in the injected frames: IFH.FWD.TS_MODE = INJ_LBK IFH.TS.INJ_LBK.COS_NXT_SEL: Selects one of three multi COSID injections per ISDX (0: disable) IFH.TS.INJ_LBK.COS_MASK[7:0] : Specifies the COSIDs to be used. IFH.TS.INJ_LBK.CHG_COSID_ENA: Controls updating of IFH.VSTAX.MISC.COSID IFH.TS.INJ_LBK.CHG_OUTER_PCP_ENA: Controls updating of outermost PCP value IFH.TS.INJ_LBK.CHG_IFH_TC_ENA: Controls updating of IFH.DST.ENCAP.MPLS_TC IFH.TS.INJ_LBK.CHG_IFH_PCP_ENA: Controls updating of IFH.VSTAX.TAG.UPRIO The multi COSID function operates differently for injected Up-MEP or Down-MEP frames. Up-MEP frames are injected by the AFI on the VD1 port and looped back to the ANA. Injected Up-MEP frames are modified when they pass through the REW the first time on the VD1 port. The IFH of the looped frames will be modified if enabled by the CHG-fields. The PCP of the outer most VLAN tag in the ETH link layer is changed if this is enabled. The IFH.TS.INJ_LBK.COS_NXT_SEL field is set to 0 to in the frame. This disables further COSID updates when the frame reaches the REW again after the loop back. Down-MEP frames are injected by the AFI on a physical port. If enabled by the INJ_LBK.CHG bits the the REW will use the new COSID value for the selected fields. The INJ_LBK.CHG_OUTER_PCP_ENA field has no functionality in Down-MEP mode. The outer PCP value will be controlled by the normal tagging configuration.
#[derive(From, Into)]
pub struct COS_CTRL(u32);
impl COS_CTRL {
    /// The auto updated COSID value is determined according to the following algorithm: mask = IFH.TS.INJ_LBK.COS_MASK[7:0] isdx = IFH.VSTAX.MISC.ISDX cos_nxt_sel = IFH.TS.INJ_LBK.COS_NXT_SEL if (cos_nxt_sel > 0 and isdx > 0 and mask > 0) { cos_nxt = REW:ISDX_TBL:COS_CTRL[IFH.VSTAX.MISC.ISDX].COS_NXT[cos_nxt_sel-1] # Use cos_nxt to find next bit in cos_mask for idx in 0:7 { if (mask[(idx+cos_nxt) mod 8] = '1') { cosid_new = idx break } } # Update next pointer REW:ISDX_TBL:COS_CTRL[IFH.VSTAX.MISC.ISDX].COS_NXT[cos_nxt_sel-1] = ((cosid_new+1) mod 8) }

    ///

    /// 0-7: Next COS value to use
    pub fn cos_nxt(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_cos_nxt(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}
/// 1588 configuration
///
/// Selects ingress PTP mode of the CPU and virtual device ports. Replication n configures port 53+n.
#[derive(From, Into)]
pub struct PTP_CPUVD_MODE_CFG(u32);
impl PTP_CPUVD_MODE_CFG {
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
/// PTP reserved field check
#[derive(From, Into)]
pub struct PTP_RSRV_NOT_ZERO(u32);
impl PTP_RSRV_NOT_ZERO {
    /// Register contains one bit per port being set when the port has received a frame with non-zero reserved bytes field This register covers ports 0-31
    pub fn ptp_rsrv_not_zero(&self) -> u32 {
        self.0
    }
    pub fn set_ptp_rsrv_not_zero(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Configuration register for PTP stamping
#[derive(From, Into)]
pub struct PTP_TWOSTEP_CTRL(u32);
impl PTP_TWOSTEP_CTRL {
    /// Write one to advance the stamp queue to the next available.
    pub fn ptp_nxt(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_ptp_nxt(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// The stamp is overflown, and some stamps are lost.
    pub fn ptp_ovfl(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ptp_ovfl(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// If the fifo is overflown, additional stamps will overwrite older.
    pub fn ptp_ovwr_ena(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_ptp_ovwr_ena(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// The stamp queue is non empty
    pub fn ptp_vld(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_ptp_vld(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Field contains the port number the stamp was made on
    pub fn stamp_port(&self) -> u32 {
        (self.0 & 0x1fe) >> 1
    }
    pub fn set_stamp_port(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x1fe);
        self.0 &= !0x1fe;
        self.0 |= value;
    }
    /// Current stamp is an egress stamp
    pub fn stamp_tx(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_stamp_tx(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
}
/// Ingress time stamp
#[derive(From, Into)]
pub struct PTP_TWOSTEP_STAMP(u32);
impl PTP_TWOSTEP_STAMP {
    /// Contains the 32 bit timestamp.
    pub fn stamp_nsec(&self) -> u32 {
        self.0
    }
    pub fn set_stamp_nsec(&mut self, value: u32) {
        self.0 = value;
    }
}
