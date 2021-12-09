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
/// Constants for this FDMA implementation.
#[derive(From, Into)]
pub struct FDMA_CONST(u32);
impl FDMA_CONST {
    /// The number of injection channels.
    pub fn ch_inj_cnt(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_ch_inj_cnt(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// The number of extraction channels.
    pub fn ch_xtr_cnt(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_ch_xtr_cnt(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// BAR1 offset mask into SBA address space
#[derive(From, Into)]
pub struct PCIEMST_BAR1_MASK(u32);
impl PCIEMST_BAR1_MASK {
    /// See ICPU_CFG::PCIEMST_BAR1_OFFSET.BAR1_OFFSET for more information.
    pub fn bar1_mask(&self) -> u32 {
        (self.0 & 0xffffff00) >> 8
    }
    pub fn set_bar1_mask(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xffffff00);
        self.0 &= !0xffffff00;
        self.0 |= value;
    }
}
/// BAR1 offset into SBA address space
#[derive(From, Into)]
pub struct PCIEMST_BAR1_OFFSET(u32);
impl PCIEMST_BAR1_OFFSET {
    /// PCIe BAR1 is a 16MByte region that, by default maps to FLASH memory space. It can be reconfigured to point at an other region in SBA by using this field. The upper 24 bit of the address can be overwritten by setting this field. The corresponding mask-field determines which bits to actually overwrite (see ICPU_CFG::PCIEMST_BAR1_MASK.BAR1_MASK), the default-value of the mask field "enables" the upper 8 bit of this field.
    pub fn bar1_offset(&self) -> u32 {
        (self.0 & 0xffffff00) >> 8
    }
    pub fn set_bar1_offset(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xffffff00);
        self.0 &= !0xffffff00;
        self.0 |= value;
    }
}
/// BAR2 offset mask into SBA address space
#[derive(From, Into)]
pub struct PCIEMST_BAR2_MASK(u32);
impl PCIEMST_BAR2_MASK {
    /// See ICPU_CFG::PCIEMST_BAR2_OFFSET.BAR2_OFFSET for more information.
    pub fn bar2_mask(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_bar2_mask(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xff000000);
        self.0 &= !0xff000000;
        self.0 |= value;
    }
}
/// BAR2 offset into SBA address space
#[derive(From, Into)]
pub struct PCIEMST_BAR2_OFFSET(u32);
impl PCIEMST_BAR2_OFFSET {
    /// PCIe BAR2 is a 128MByte region that, by default maps to DDR interface. It can be reconfigured to point at an other region in SBA by using this field. The upper 8 bit of the address can be overwritten by setting this field. The corresponding mask-field determines which bits to actually overwrite (see ICPU_CFG::PCIEMST_BAR2_MASK.BAR2_MASK), the default-value of the mask field "enables" the upper 5 bit of this field.
    pub fn bar2_offset(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    pub fn set_bar2_offset(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xff000000);
        self.0 &= !0xff000000;
        self.0 |= value;
    }
}
/// Additional inbound reply information
#[derive(From, Into)]
pub struct PCIEMST_REPLY_INFO(u32);
impl PCIEMST_REPLY_INFO {
    /// This field allows additional PCIe-transaction settings in addition to those provided via SBA. The settings in this register is applied to all PCIe inbound accesses.
    ///
    /// [0] SBA Response with bad EOT. Cause drop of inbound response. [1] EP-field. [2] TD-field. [4:3] Reserved, must be "01".
    pub fn mst_reply_info(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_mst_reply_info(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}
/// PCIe beacon parameters
#[derive(From, Into)]
pub struct PCIEPCS_BEACON(u32);
impl PCIEPCS_BEACON {
    /// Set to disable the BEACON_MAX_DLY feature, when set beacon will be generated until reply from upsteam port is received.
    pub fn beacon_max_dis(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_beacon_max_dis(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Set this field to the maximum number of PCIe (transmit) clock cycles for which beacon may be asserted. The standard says beacon must not be asserted for longer than 32 us.
    pub fn beacon_max_dly(&self) -> u32 {
        self.0 & 0x7fff
    }
    pub fn set_beacon_max_dly(&mut self, value: u32) {
        assert!(value <= 0x7fff);
        self.0 &= !0x7fff;
        self.0 |= value;
    }
    /// 10bit value to transmit during PCIe-beacon signaling, if 0x3E becaon will be 250Mhz alternating signal. Special, if all-zeros PCS will generate a 125MHz alternating signal.
    pub fn beacon_val(&self) -> u32 {
        (self.0 & 0x3ff0000) >> 16
    }
    pub fn set_beacon_val(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x3ff0000);
        self.0 &= !0x3ff0000;
        self.0 |= value;
    }
}
/// PCIe PCS configuration
#[derive(From, Into)]
pub struct PCIEPCS_CFG(u32);
impl PCIEPCS_CFG {
    /// Set this field to disable outband PCIe beacon signalling when attempting to wake from D3. When beacon is disabled the WAKE# signal (available as alternate GPIO function) must be used instead.
    pub fn beacon_dis(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_beacon_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set this field to force beacon signal to be transmitted on serdes output.
    pub fn beacon_force(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_beacon_force(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Set this field to disable drive of constant TBI data (zeros) during IDLE indication.
    pub fn idle_data_dis(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_idle_data_dis(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Set this field to disable support for outband idle signalling.
    pub fn idle_dis(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_idle_dis(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Set this field allows skewing TBI DATA vs. IDLE signals. When configured to -1 then IDLE_DATA_DIS will also need to be set, else first symbol transmitted will just be 0s'.
    ///
    /// 0: No delay of IDLE. 1: Delay IDLE by 1 symbol. 2: Delay IDLE by 2 symbols. 3: Delay IDLE by -1 symbol (is delaying data by 1 symbol).
    pub fn idle_dly(&self) -> u32 {
        (self.0 & 0x60) >> 5
    }
    pub fn set_idle_dly(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x60);
        self.0 &= !0x60;
        self.0 |= value;
    }
    /// Set to enable RX inversion from serdes to PCS block. This is for test only, the PCS handles inversion internally when ever this is requested by the PCIe MAC.
    pub fn rxinv_ena(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_rxinv_ena(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Set this field to (always) signal receive lowpower towards the serdes.
    pub fn rxlp_force(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rxlp_force(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set this field to force the value of the receive reset from serdes to PCS to the value of ICPU_CFG::PCIEPCS_CFG.RXRST_VAL.
    pub fn rxrst_force(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_rxrst_force(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// See ICPU_CFG::PCIEPCS_CFG.RXRST_FORCE for more information.
    ///
    /// 0: Reset is asserted 1: Reset is not asserted
    pub fn rxrst_val(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_rxrst_val(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
    /// Set this field to force the value of the signal-detect from serdes to PCS to the value of ICPU_CFG::PCIEPCS_CFG.RXSDET_XOR.
    pub fn rxsdet_force(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_rxsdet_force(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Set this field to invert the value of the signal-detect from serdes to PCS.
    pub fn rxsdet_xor(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_rxsdet_xor(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Set this field to swap TBI from serdes to PCS block (crossover bit 0 to 9, 1 to 8, etc.)
    pub fn rxswap_ena(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_rxswap_ena(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Set to disable reset of comma-detection logic when loss of rx signal is "detected".
    pub fn sdetcom_dis(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_sdetcom_dis(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Set to enable contineous reset of comma-detection logic during loss of rx signal. SDETCOM_DIS must be cleared for this to work.
    pub fn sdetcom_perm(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_sdetcom_perm(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Set this field to (always) signal transmit lowpower towards the serdes.
    pub fn txlp_force(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_txlp_force(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set this field to force the value of the transmit reset from serdes to PCS to the value of ICPU_CFG::PCIEPCS_CFG.TXRST_VAL.
    pub fn txrst_force(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_txrst_force(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// See ICPU_CFG::PCIEPCS_CFG.TXRST_FORCE for more information.
    ///
    /// 0: Reset is asserted 1: Reset is not asserted
    pub fn txrst_val(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_txrst_val(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Configure receiver detection result (when requested by PCIe MAC on pipe-if)
    ///
    /// 0: Receiver detected 1: Receiver not detected
    pub fn txrx_detect(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    pub fn set_txrx_detect(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x200000);
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// Set this field to swap TBI from PCS to serdes block (crossover bit 0 to 9, 1 to 8, etc.)
    pub fn txswap_ena(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_txswap_ena(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Set to permanently drive PCIe WAKE output, by default the WAKE output is only driven when active and thusly allowing pull-resistor network.
    ///
    /// 0: Only drive output when active. 1: Always drive output.
    pub fn wake_oe(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_wake_oe(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Polarity of the PCIe WAKE output, WAKE is typically an active low output - but if an amplifier is needed for driving a large WAKE net then polarity may need to be changed.
    ///
    /// 0: Active low 1: Active high
    pub fn wake_pol(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_wake_pol(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// FDMA access into PCIe address space
#[derive(From, Into)]
pub struct PCIESLV_FDMA(u32);
impl PCIESLV_FDMA {
    /// This field allows configuration of outbound PCIe-transaction ATTR field. This is applied to all FDMA initiated outbound PCIe accesses.
    pub fn fdma_attr(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_fdma_attr(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// The FDMA has access to one 1GByte region (0xC0000000 though 0xFFFFFFFF) that maps accesses to PCIe interface. The value of this field is used for address-bits [31:30] towards the PCIe endpoint.
    ///
    /// Set this field to 1.
    pub fn fdma_offset(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_fdma_offset(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// This field allows configuration of outbound PCIe-transaction TC field. This is applied to all FDMA initiated outbound PCIe accesses.
    pub fn fdma_tc(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_fdma_tc(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }
}
/// SBA access into PCIe address space
#[derive(From, Into)]
pub struct PCIESLV_SBA(u32);
impl PCIESLV_SBA {
    /// This field allows configuration of outbound PCIe-transaction ATTR field. This is applied to all SBA (non-FDMA) initiated outbound PCIe accesses.
    pub fn sba_attr(&self) -> u32 {
        (self.0 & 0xc00) >> 10
    }
    pub fn set_sba_attr(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0xc00);
        self.0 &= !0xc00;
        self.0 |= value;
    }
    /// This field allows configuration of outbound PCIe-transaction Byte-Enable field. This is applied to all SBA (non-FDMA) initiated outbound PCIe accesses. This field is not used for TYPE=MRr/MWr/MRdLk accesses. Byte-enables are needed in order to support Zero-byte and non-contiguous byte IO and CFG transfers and Zero-byte Messages.
    pub fn sba_be(&self) -> u32 {
        (self.0 & 0xf000000) >> 24
    }
    pub fn set_sba_be(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0xf000000);
        self.0 &= !0xf000000;
        self.0 |= value;
    }
    /// This field allows configuration of outbound PCIe-transaction EP field. This is applied to all SBA (non-FDMA) initiated outbound PCIe accesses.
    pub fn sba_ep(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_sba_ep(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// This field allows configuration of outbound PCIe-transaction MSG field. This is applied to all SBA (non-FDMA) initiated outbound PCIe accesses.
    pub fn sba_msg_code(&self) -> u32 {
        (self.0 & 0x7f8000) >> 15
    }
    pub fn set_sba_msg_code(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x7f8000);
        self.0 &= !0x7f8000;
        self.0 |= value;
    }
    /// SBA masters (non-FDMA) has access to one 1GByte region (0xC0000000 though 0xFFFFFFFF) that maps accesses to PCIe interface. The value of this field is used for address-bits [31:30] towards the PCIe endpoint.
    ///
    /// Set this field to 0.
    pub fn sba_offset(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_sba_offset(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Reserved, must be 0.
    pub fn sba_reserved0(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_sba_reserved0(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Reserved, must be 0.
    pub fn sba_reserved1(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    pub fn set_sba_reserved1(&mut self, value: u32) {
        let value = value << 23;
        assert!(value <= 0x800000);
        self.0 &= !0x800000;
        self.0 |= value;
    }
    /// This field allows configuration of outbound PCIe-transaction TC field. This is applied to all SBA (non-FDMA) initiated outbound PCIe accesses.
    pub fn sba_tc(&self) -> u32 {
        (self.0 & 0x7000) >> 12
    }
    pub fn set_sba_tc(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x7000);
        self.0 &= !0x7000;
        self.0 |= value;
    }
    /// This field allows configuration of outbound PCIe-transaction TD field. This is applied to all SBA (non-FDMA) initiated outbound PCIe accesses.
    pub fn sba_td(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_sba_td(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// This field allows configuration of outbound PCIe-transaction TYPE field. This is applied to all SBA (non-FDMA) initiated outbound PCIe accesses.
    ///
    /// Encoding as defined by PCIe standard.
    pub fn sba_type(&self) -> u32 {
        (self.0 & 0x7c) >> 2
    }
    pub fn set_sba_type(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x7c);
        self.0 &= !0x7c;
        self.0 |= value;
    }
}
/// Auxiliary power configuration
#[derive(From, Into)]
pub struct PCIE_AUX_CFG(u32);
impl PCIE_AUX_CFG {
    /// Set to force "detection" of PCIe auxiliary power.
    pub fn aux_power_val(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_aux_power_val(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PCIe endpoint configuration
#[derive(From, Into)]
pub struct PCIE_CFG(u32);
impl PCIE_CFG {
    /// Set to disable clock gating requests from PCIe core.
    pub fn cg_dis(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_cg_dis(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Set to defer incomming configuration requests with a Configuration Request Retry Status.
    pub fn conf_req_retry_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_conf_req_retry_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set this field to disable initliazation of the PCIe link. By default the PCIe core will start up and try to acchieve link when the SERDES is started, by setting this field before starting the SERDES it is possible to make changes to the PCIe configruration/registers prior to linking with the root complex.
    pub fn ltssm_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ltssm_dis(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set to disable clock gating in PCIe core memories.
    pub fn mem_cg_dis(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_mem_cg_dis(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set to add the PCIe core memories to the RAM integrity ring.
    pub fn mem_ring_core_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_mem_ring_core_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set this field to enable write of PCIe BAR masks via PCIE register target. Only registers PCIE::BAR0, PCIE::BAR1, and PCIE::BAR2 may be written while this field is set. The minimum size for Memory and IO BARs are 64K (mask 0xFFFF). Note: The low 4 bits of all BARs can be written via the PCIe target when this field is not set.
    pub fn pcie_bar_wr_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_pcie_bar_wr_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// PCIe endpoint control
#[derive(From, Into)]
pub struct PCIE_CTRL(u32);
impl PCIE_CTRL {
    /// Set this field to send out PME from the controller, this is only possible if PME is enabled in the PCIe EP core. This field is cleared automatically after controller has received the request. In the D3 state the controller will transmit beacon and/or assert #WAKE (see ICPU_CFG::PCIEPCS_CFG.BEACON_DIS for more information). In other states the link will be transitioned to L0 (when applicable) and PME event will then be transmitted upsteam.
    pub fn powerup(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_powerup(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PCIe endpoint debug status
#[derive(From, Into)]
pub struct PCIE_DBG_STAT(u32);
impl PCIE_DBG_STAT {
    /// Set when the PCIe Data Link Layer is up and ready to receive/transmit packages. This value is read directly from other clock domain, keep reading until same value was read twice in a row - then read was sucessful.
    pub fn data_link_layer_up(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_data_link_layer_up(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set when the Power Management is exiting L2 state. This value is read directly from other clock domain, keep reading until same value was read twice in a row - then read was sucessful.
    pub fn pm_l2_exit(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_pm_l2_exit(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// This field is set when the PCIe receive path detects 8b10b decoder errors.
    pub fn tbi_rx_ce(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_tbi_rx_ce(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// This field just returns the TBI DATA value returned from the Serdes macro. This field is read directly from another clock domain, during normal operation the value presented in this field will be garbage.
    pub fn tbi_rx_data(&self) -> u32 {
        (self.0 & 0xffc) >> 2
    }
    pub fn set_tbi_rx_data(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xffc);
        self.0 &= !0xffc;
        self.0 |= value;
    }
    /// This field is a sticky loss of signal indication that is set whenever ICPU_CFG::PCIE_DBG_STAT.TBI_RX_SDET is low.
    pub fn tbi_rx_los(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_tbi_rx_los(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// This field returns the TBI SDET value returned from the Serdes macro.
    pub fn tbi_rx_sdet(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_tbi_rx_sdet(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
}
/// PCIe events
#[derive(From, Into)]
pub struct PCIE_INTR(u32);
impl PCIE_INTR {
    /// This event is set whenever an external CPU reads from BAR1 region.
    pub fn intr_bar1_rd(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_intr_bar1_rd(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// This event is set whenever an external CPU writes to BAR1 region.
    pub fn intr_bar1_wr(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_intr_bar1_wr(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// This event is set whenever the ICPU_CFG::PCIE_STAT.LINK_STATE field is changed.
    pub fn intr_link_state(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_intr_link_state(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// This event is set whenever the ICPU_CFG::PCIE_STAT.LTSSM_STATE field is changed.
    pub fn intr_ltssm_state(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_intr_ltssm_state(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// This event is set whenever the ICPU_CFG::PCIE_STAT.PM_STATE field is changed.
    pub fn intr_pm_state(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_intr_pm_state(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PCIe outbound MSI interrupt configuration
///
/// Replicated per EXT_DST interrupt.
#[derive(From, Into)]
pub struct PCIE_INTR_CFG(u32);
impl PCIE_INTR_CFG {
    /// Configure MSI interrupt vector for falling edge of corresponding EXT_DST interrupt.
    pub fn falling_vector_val(&self) -> u32 {
        (self.0 & 0xf80) >> 7
    }
    pub fn set_falling_vector_val(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0xf80);
        self.0 &= !0xf80;
        self.0 |= value;
    }
    /// Reserved, must be 0.
    pub fn function_number(&self) -> u32 {
        (self.0 & 0x38000) >> 15
    }
    pub fn set_function_number(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x38000);
        self.0 &= !0x38000;
        self.0 |= value;
    }
    /// Set to enable MSI interrupt on falling edge of corresponding EXT_DST interrupt.
    pub fn intr_falling_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_intr_falling_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set to enable MSI interrupt on rising edge of corresponding EXT_DST interrupt.
    pub fn intr_rising_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_intr_rising_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Configure MSI interrupt vector for rising edge of corresponding EXT_DST interrupt.
    pub fn rising_vector_val(&self) -> u32 {
        (self.0 & 0x7c) >> 2
    }
    pub fn set_rising_vector_val(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x7c);
        self.0 &= !0x7c;
        self.0 |= value;
    }
    /// Configure MSI interrupt traffic class for corresponding EXT_DST interrupt.
    pub fn traffic_class(&self) -> u32 {
        (self.0 & 0x7000) >> 12
    }
    pub fn set_traffic_class(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x7000);
        self.0 &= !0x7000;
        self.0 |= value;
    }
}
/// PCIe outbound interrupt configuration
#[derive(From, Into)]
pub struct PCIE_INTR_COMMON_CFG(u32);
impl PCIE_INTR_COMMON_CFG {
    /// Select the external interrupt from the VCore interrupt controller that must be used to generate PCIe legacy interrupt.
    ///
    /// 0: Use EXT_DST0 1: Use EXT_DST1
    pub fn legacy_mode_intr_sel(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_legacy_mode_intr_sel(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set to enable PCIe interrupts. The PCIe endpoint's MSI Capability Register Set must have been configured before enabling interrupts.
    pub fn pcie_intr_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pcie_intr_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set to disable wake-up on interrupt. By default the PCIe endpoint will attempt to wake up from powerdown when a change in interrupt state is detected.
    pub fn wakeup_on_intr_dis(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_wakeup_on_intr_dis(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// PCIe interrupt enable
#[derive(From, Into)]
pub struct PCIE_INTR_ENA(u32);
impl PCIE_INTR_ENA {
    /// Set to enable propagation of the BAR1_RD interrupt.
    pub fn intr_bar1_rd_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_intr_bar1_rd_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set to enable propagation of the BAR1_WR interrupt.
    pub fn intr_bar1_wr_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_intr_bar1_wr_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set to enable propagation of the LINK_STATE interrupt.
    pub fn intr_link_state_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_intr_link_state_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set to enable propagation of the LTSSM_STATE interrupt.
    pub fn intr_ltssm_state_ena(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_intr_ltssm_state_ena(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set to enable propagation of the PM_STATE interrupt.
    pub fn intr_pm_state_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_intr_pm_state_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Currently active PCIe interrupts
#[derive(From, Into)]
pub struct PCIE_INTR_IDENT(u32);
impl PCIE_INTR_IDENT {
    /// Set if the BAR1_RD interrupt is currently active.
    pub fn intr_bar1_rd_ident(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_intr_bar1_rd_ident(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set if the BAR1_WR interrupt is currently active.
    pub fn intr_bar1_wr_ident(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_intr_bar1_wr_ident(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set if the LINK_STATE interrupt is currently active.
    pub fn intr_link_state_ident(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_intr_link_state_ident(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set if the LTSSM_STATE interrupt is currently active.
    pub fn intr_ltssm_state_ident(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_intr_ltssm_state_ident(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Set if the PM_STATE interrupt is currently active.
    pub fn intr_pm_state_ident(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_intr_pm_state_ident(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// PCIe endpoint status
#[derive(From, Into)]
pub struct PCIE_STAT(u32);
impl PCIE_STAT {
    /// Current Link State of the PCIe core.
    ///
    /// 0: L0 (or not working, check LTSSM) 1: L0s 2: L1 3: L2 4: L3
    pub fn link_state(&self) -> u32 {
        (self.0 & 0x38) >> 3
    }
    pub fn set_link_state(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x38);
        self.0 &= !0x38;
        self.0 |= value;
    }
    /// The current LTSSM state of the PCIe core.
    ///
    /// 0x00: DETECT_QUIET 0x01: DETECT_ACT 0x02: POLL_ACTIVE 0x03: POLL_COMPLIANCE 0x04: POLL_CONFIG 0x05: PRE_DETECT_QUIET 0x06: DETECT_WAIT 0x07: CFG_LINKWD_START 0x08: CFG_LINKWD_ACEPT 0x09: CFG_LANENUM_WAIT 0x0A: CFG_LANENUM_ACEPT 0x0B: CFG_COMPLETE 0x0C: CFG_IDLE 0x0D: RCVRY_LOCK 0x0E: RCVRY_SPEED 0x0F: RCVRY_RCVRCFG 0x10: RCVRY_IDLE 0x20: RCVRY_EQ0 0x21: RCVRY_EQ1 0x22: RCVRY_EQ2 0x23: RCVRY_EQ3 0x11: L0 0x12: L0S 0x13: L123_SEND_EIDLE 0x14: L1_IDLE 0x15: L2_IDLE 0x16: L2_WAKE 0x17: DISABLED_ENTRY 0x18: DISABLED_IDLE 0x19: DISABLED 0x1A: LPBK_ENTRY 0x1B: LPBK_ACTIVE 0x1C: LPBK_EXIT 0x1D: LPBK_EXIT_TIMEOUT 0x1E: HOT_RESET_ENTRY 0x1F: HOT_RESET
    pub fn ltssm_state(&self) -> u32 {
        (self.0 & 0xfc0) >> 6
    }
    pub fn set_ltssm_state(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0xfc0);
        self.0 &= !0xfc0;
        self.0 |= value;
    }
    /// The current power managment state of the PCIe core.
    ///
    /// 0: D0 1: D1 2: D2 3: D3 4: D0-Uninitialized
    pub fn pm_state(&self) -> u32 {
        self.0 & 0x7
    }
    pub fn set_pm_state(&mut self, value: u32) {
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
    /// Set when the PCIe outbound request interface returns a non-ok SBA reply, ie. ERROR.
    pub fn slv_error_reply(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_slv_error_reply(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
}
