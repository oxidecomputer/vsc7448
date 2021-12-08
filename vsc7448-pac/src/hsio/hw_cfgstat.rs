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

/// Register `HW_CFG`
///
/// Various configration
#[derive(From, Into)]
pub struct HW_CFG(u32);
impl HW_CFG {    ///
    /// Set to enable drive of analog test-output pin.
    pub fn anaout_ena(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_anaout_ena(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }    ///
    /// Configure mode of DEV10G and hookup of SerDes10G blocks. The device has dedicated DEV10G for 10 Gbit/sec speed, and DEV2G5 for 2.5 Gbit/sec and lower speeds. The DEV10G and corresponding DEV2G5 cannot be active at the same time.
    ///
    /// 0: DEV10G_0 XFI-mode via SerDes10G_0. 1: DEV10G_0 XAUI-mode via SerDes6G_16, SerDes6G_17, SerDes6G_18, and SerDes6G_19. 2: DEV10G_0 RXAUI-mode via SerDes6G_16 and SerDes6G_18. 3: DEV2G5_25 via SerDes10G_0.
    pub fn dev10g_0_mode(&self) -> u32 {
        (self.0 & 0x3000) >> 12
    }
    pub fn set_dev10g_0_mode(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x3000);
        self.0 &= !0x3000;
        self.0 |= value;
    }    ///
    /// See DEVCPU_GCB::DEV10G_0_MODE for description.
    ///
    /// 0: DEV10G_1 XFI-mode via SerDes10G_1. 1: DEV10G_1 XAUI-mode via SerDes6G_20, SerDes6G_21, SerDes6G_22, and SerDes6G_23. 2: DEV10G_1 RXAUI-mode via SerDes6G_20 and SerDes6G_22. 3: DEV2G5_26 via SerDes10G_1.
    pub fn dev10g_1_mode(&self) -> u32 {
        (self.0 & 0xc000) >> 14
    }
    pub fn set_dev10g_1_mode(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0xc000);
        self.0 &= !0xc000;
        self.0 |= value;
    }    ///
    /// See DEVCPU_GCB::DEV10G_0_MODE for description.
    ///
    /// 0: DEV10G_2 XFI-mode via SerDes10G_2. 1: DEV10G_2 XAUI-mode via SerDes6G_8, SerDes6G_9, SerDes6G_10, and SerDes6G_11. 2: DEV10G_2 RXAUI-mode via SerDes6G_8 and SerDes6G_10. 3: DEV2G5_27 via SerDes10G_2.
    pub fn dev10g_2_mode(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    pub fn set_dev10g_2_mode(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x30000);
        self.0 &= !0x30000;
        self.0 |= value;
    }    ///
    /// See DEVCPU_GCB::DEV10G_0_MODE for description.
    ///
    /// 0: DEV10G_3 XFI-mode via SerDes10G_3. 1: DEV10G_3 XAUI-mode via SerDes6G_12, SerDes6G_13, SerDes6G_14, and SerDes6G_15. 2: DEV10G_3 RXAUI-mode via SerDes6G_12 and SerDes6G_14. 3: DEV2G5_28 via SerDes10G_3.
    pub fn dev10g_3_mode(&self) -> u32 {
        (self.0 & 0xc0000) >> 18
    }
    pub fn set_dev10g_3_mode(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0xc0000);
        self.0 &= !0xc0000;
        self.0 |= value;
    }    ///
    /// Set bit 0 to enable QSGMII mode for devices DEV1G_0, DEV1G_1, DEV1G_2, and DEV1G_3 via SerDes6G_4. Set bit 1 to enable QSGMII mode for devices DEV1G_4, DEV1G_5, DEV1G_6, and DEV1G_7 via SerDes6G_5. Set bit 2 to enable QSGMII mode for devices DEV2G5_0, DEV2G5_1, DEV2G5_2, and DEV2G5_3 via SerDes6G_6. Set bit 3 to enable QSGMII mode for devices DEV2G5_4, DEV2G5_5, DEV2G5_6, and DEV2G5_7 via SerDes6G_7. Set bit 4 to enable QSGMII mode for devices DEV2G5_8, DEV2G5_9, DEV2G5_10, and DEV2G5_11 via SerDes6G_8. Set bit 5 to enable QSGMII mode for devices DEV2G5_12, DEV2G5_13, DEV2G5_14, and DEV2G5_15 via SerDes6G_9. Set bit 6 to enable QSGMII mode for devices DEV2G5_16, DEV2G5_17, DEV2G5_18, and DEV2G5_19 via SerDes6G_10. Set bit 7 to enable QSGMII mode for devices DEV2G5_20, DEV2G5_21, DEV2G5_22, and DEV2G5_23 via SerDes6G_11. Set bit 8 to enable QSGMII mode for devices DEV1G_8, DEV1G_9, DEV1G_10, and DEV1G_11 via SerDes6G_12. Set bit 9 to enable QSGMII mode for devices DEV1G_12, DEV1G_13, DEV1G_14, and DEV1G_15 via SerDes6G_13. Set bit 10 to enable QSGMII mode for devices DEV1G_16, DEV1G_17, DEV1G_18, and DEV1G_19 via SerDes6G_14. Set bit 11 to enable QSGMII mode for devices DEV1G_20, DEV1G_21, DEV1G_22, and DEV1G_23 via SerDes6G_15.
    pub fn qsgmii_ena(&self) -> u32 {
        (self.0 & 0xfff) >> 0
    }
    pub fn set_qsgmii_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}

/// Register `HW_QSGMII_CFG`
///
/// Additional configuration for QSGMII channels
#[derive(From, Into)]
pub struct HW_QSGMII_CFG(u32);
impl HW_QSGMII_CFG {    ///
    /// Set to enable 8b10b receive error propagation; 8b10b error code-groups are replaced by K70.7 error symbols. This setting applies to all QSGMII channels.
    pub fn e_det_ena(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_e_det_ena(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }    ///
    /// Set to flip QSGMII lanes: Lane 0 is interchanged with 3, and 1 is interchanged with 2 for both receie and transmit directions. Each bit in this field correspond to a QSGMII channel, bit 0 configures QSGMII#0, bit 1 configures QSGMII#1, etc.
    pub fn flip_lanes(&self) -> u32 {
        (self.0 & 0xfff) >> 0
    }
    pub fn set_flip_lanes(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }    ///
    /// Set to disable hysteresis of receive synchronization state machine. This setting applies to all QSGMII channels.
    pub fn shyst_dis(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_shyst_dis(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }    ///
    /// Set to transmit I1 during idle sequencing only. This setting applies to all QSGMII channels.
    pub fn use_i1_ena(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_use_i1_ena(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
}

/// Register `HW_QSGMII_STAT`
///
/// Additional status from QSGMII channels
#[derive(From, Into)]
pub struct HW_QSGMII_STAT(u32);
impl HW_QSGMII_STAT {    ///
    /// Variable delay in QSGMII ingress path provided in steps of 200ps, values 0 though 39 is possible allowing a span from 0ns to 7.8ns. Each replication of this register correspond to a QSGMII channel, replication 0 is from QSGMII#0, replication 1 is from QSGMII#1, etc. The value for a QSGMII channel is valid when it has synchronized to an incomming QSGMII signal, and will remain constant while the channel stays in sync.
    pub fn delay_var_x200ps(&self) -> u32 {
        (self.0 & 0x7e) >> 1
    }
    pub fn set_delay_var_x200ps(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x7e);
        self.0 &= !0x7e;
        self.0 |= value;
    }    ///
    /// Set when QSGMII channel has successfully synchronized on K28.1 code-group, this field is only valid when DEVCPU_GCB::HW_QSGMII_CFG.SHYST_DIS is 0.
    pub fn sync(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_sync(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `MCB_SERDES6G_ADDR_CFG`
///
/// MCB SERDES6G Address Cfg
///
/// Configuration of SERDES6G MCB Slaves to be accessed
#[derive(From, Into)]
pub struct MCB_SERDES6G_ADDR_CFG(u32);
impl MCB_SERDES6G_ADDR_CFG {    ///
    /// Activation vector for SERDES6G-Slaves, one-hot coded, each bit is related to one macro, e.g. bit 0 enables/disables access to macro no. 0.
    ///
    /// 0: Disable macro access via MCB 1: Enable macro access via MCB
    pub fn serdes6g_addr(&self) -> u32 {
        (self.0 & 0x1ffffff) >> 0
    }
    pub fn set_serdes6g_addr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1ffffff);
        self.0 &= !0x1ffffff;
        self.0 |= value;
    }    ///
    /// Initiate a read access to marked SERDES6G Slaves
    ///
    /// 0: No read operation pending (read op finished after bit has been set) 1: Initiate a read access (kept 1 until read operation has finished)
    pub fn serdes6g_rd_one_shot(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_serdes6g_rd_one_shot(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }    ///
    /// Initiate a write access to marked SERDES6G Slaves
    ///
    /// 0: No write operation pending 1: Initiate write to slaves (kept 1 until write operation has finished)
    pub fn serdes6g_wr_one_shot(&self) -> u32 {
        (self.0 & 0x7fffffff) >> 31
    }
    pub fn set_serdes6g_wr_one_shot(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x7fffffff);
        self.0 &= !0x7fffffff;
        self.0 |= value;
    }
}