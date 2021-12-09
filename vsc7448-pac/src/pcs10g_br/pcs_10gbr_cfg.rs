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
/// PCS configuration
///
/// Configuration register
#[derive(From, Into)]
pub struct PCS_CFG(u32);
impl PCS_CFG {
    /// Enable link control using backplane Ethernet ANEG (Auto-Negotiation)
    ///
    /// 0: Disable link control 1: Enable link control
    pub fn an_link_ctrl_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_an_link_ctrl_ena(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Disable RX local fault generation when no alignment has been reached
    ///
    /// 0: Output local fault symbol at XGMII when not aligned 1: Output IDLE symbols at XGMII when not aligned
    pub fn lf_gen_dis(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_lf_gen_dis(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Global PCS Enable/Disable configuration bit.
    ///
    /// 0 = Disable PCS 1 = Enable PCS
    pub fn pcs_ena(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    pub fn set_pcs_ena(&mut self, value: u32) {
        let value = value << 31;
        assert!(value <= 0x80000000);
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Enable PMA loopback operation. When set, Transmit PMA data is loopbacked to Receive PMA data.
    ///
    /// 0 = Normal mode 1 = Loopback Tx PMA to Rx PMA
    pub fn pma_loopback_ena(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    pub fn set_pma_loopback_ena(&mut self, value: u32) {
        let value = value << 30;
        assert!(value <= 0x40000000);
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    /// Force re-synchronization of receive logic. When asserted, the receive sync state machine is forced into the LOCK_INIT state and block_lock is lost.
    ///
    /// 0: Normal operation 1: Reset synchronization
    pub fn resync_ena(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_resync_ena(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Flip the data bus bits at the WIS/PMA interface such that bit 63 is mapped to bit 0 and bit 0 to 63. i.e. the output bus (63 down to 0) is remapped to (0 to 63) and bit 63 is the first bit.
    ///
    /// 0: No flip (LSB first) 1: Flip bus (MSB first)
    pub fn rx_data_flip(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_rx_data_flip(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Disable the descrambler. When disabled, the data is passed through without being descrambled.
    ///
    /// 0: Decrambler active 1: Decrambler disabled
    pub fn rx_scr_disable(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_rx_scr_disable(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Enable test pattern checking mode.
    ///
    /// 0: Normal operation 1: Test pattern mode
    pub fn rx_test_mode(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_rx_test_mode(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Number of sync headers required for block lock. The actual number used is this number plus 1. i.e. entering 63 implies a value of 64.
    ///
    /// Binary number
    pub fn sh_cnt_max(&self) -> u32 {
        (self.0 & 0x3f000000) >> 24
    }
    pub fn set_sh_cnt_max(&mut self, value: u32) {
        let value = value << 24;
        assert!(value <= 0x3f000000);
        self.0 &= !0x3f000000;
        self.0 |= value;
    }
    /// Flip the data bus bits at the WIS/PMA interface such that bit 63 is mapped to bit 0 and bit 0 to 63. i.e. the output bus (63 downto 0) is remapped to (0 to 63) and bit 63 is the first bit.
    ///
    /// 0 = No flip (LSB first) 1 = Flip bus (MSB first)
    pub fn tx_data_flip(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_tx_data_flip(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Disable the scrambler. When disabled, the data is passed through without being scrambled.
    ///
    /// 0: Scrambler active 1: Scrambler disabled
    pub fn tx_scr_disable(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_tx_scr_disable(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Enable test pattern generation mode
    ///
    /// 0: Normal operation 1: Test pattern mode
    pub fn tx_test_mode(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_tx_test_mode(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
}
/// PCS interrupt mask register
///
/// Masks for PCS interrupt sources and sticky bits in PCS_INTR_STAT
#[derive(From, Into)]
pub struct PCS_INTR_MASK(u32);
impl PCS_INTR_MASK {
    /// Mask for the C64B66B_ERR_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn c64b66b_err_mask(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_c64b66b_err_mask(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Mask for the LOCK_CHANGED_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn lock_changed_mask(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_lock_changed_mask(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Mask for the RX_FSET_FIFO_FULL_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn rx_fset_fifo_full_mask(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_rx_fset_fifo_full_mask(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Mask for the RX_FSET_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn rx_fset_mask(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_rx_fset_mask(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Mask for the RX_HI_BER_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn rx_hi_ber_mask(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_rx_hi_ber_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Mask for RX_OSET_FIFO_FULL_STICKY
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn rx_oset_fifo_full_mask(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_rx_oset_fifo_full_mask(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Mask for the RX_OSET_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn rx_oset_mask(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_rx_oset_mask(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Mask for the XGMII_ERR_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    pub fn xgmii_err_mask(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_xgmii_err_mask(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
}
/// PCS SignalDetect Configuration
///
/// PCS signal_detect configuration
#[derive(From, Into)]
pub struct PCS_SD_CFG(u32);
impl PCS_SD_CFG {
    /// Signal Detect Enable
    ///
    /// 0: The Signal Detect input pin is ignored. The PCS assumes an active Signal Detect at all times 1: The Signal Detect input pin is used to determine if a signal is detected
    pub fn sd_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_sd_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Signal detect polarity: The signal level on signal_detect input pin must be equal to SD_POL to indicate signal detection (SD_ENA must be set)
    ///
    /// 0: Signal Detect input pin must be '0' to indicate a signal detection 1: Signal Detect input pin must be '1' to indicate a signal detection
    pub fn sd_pol(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_sd_pol(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Signal detect selection (select input for internal signal_detect line)
    ///
    /// 0: Select signal_detect line from hardmacro 1: Select external signal_detect line
    pub fn sd_sel(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_sd_sel(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
}
/// LSB of test pattern
///
/// Least significant Rx bits of test pattern
#[derive(From, Into)]
pub struct RX_DATAPAT_LSB(u32);
impl RX_DATAPAT_LSB {
    /// Least significant 32 bits of 64-bit data pattern used in pseudo-random and user-defined test pattern mode
    pub fn rx_datapat_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_rx_datapat_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// MSB of test pattern
///
/// Most significant Rx bits of test pattern
#[derive(From, Into)]
pub struct RX_DATAPAT_MSB(u32);
impl RX_DATAPAT_MSB {
    /// Most significant 32 bits of 64-bit data pattern used in pseudo-random and user-defined test pattern mode
    pub fn rx_datapat_msb(&self) -> u32 {
        self.0
    }
    pub fn set_rx_datapat_msb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// PRBS31 initialization
///
/// Initial value to load into the PRBS31 linear-feedback shift register
#[derive(From, Into)]
pub struct RX_PRBS31_INIT(u32);
impl RX_PRBS31_INIT {
    /// PRBS31 initial value.
    pub fn rx_prbs31_init(&self) -> u32 {
        self.0 & 0x7fffffff
    }
    pub fn set_rx_prbs31_init(&mut self, value: u32) {
        assert!(value <= 0x7fffffff);
        self.0 &= !0x7fffffff;
        self.0 |= value;
    }
}
/// Test pattern mode configuration
///
/// Select test patterns when test mode is enabled
#[derive(From, Into)]
pub struct TEST_CFG(u32);
impl TEST_CFG {
    /// Disables inversion of seeds and data in the pseudo-random test pattern
    ///
    /// 0: Inversion enabled 1: Inversion disabled
    pub fn rx_dsbl_inv(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_rx_dsbl_inv(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Select the test pattern used by the test pattern checker. This register is only used if RX_TEST_MODE is active.
    ///
    /// 0: unused 1: Pseudo random 2: PRBS31 3: User defined
    pub fn rx_testpat_sel(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    pub fn set_rx_testpat_sel(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x30000);
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// Disables inversion of seeds and data in the pseudo-random test pattern
    ///
    /// 0: Inversion enabled 1: Inversion disabled
    pub fn tx_dsbl_inv(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_tx_dsbl_inv(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Period of square wave. Value in the register is (Period-4). Valid values of period are 4 to 11.
    ///
    /// Period = (TX_SQPW_4B+4)
    pub fn tx_sqpw_4b(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    pub fn set_tx_sqpw_4b(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x1c);
        self.0 &= !0x1c;
        self.0 |= value;
    }
    /// Select the test pattern used by the test pattern generator. This register is only used if TX_TEST_MODE is active.
    ///
    /// 0: Square wave 1: Pseudo random 2: PRBS31 3: User defined
    pub fn tx_testpat_sel(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_tx_testpat_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
/// LSB of test pattern
///
/// Least significant Tx bits of test pattern
#[derive(From, Into)]
pub struct TX_DATAPAT_LSB(u32);
impl TX_DATAPAT_LSB {
    /// Least significant 32 bits of 64-bit data pattern used in pseudo-random and user-defined test pattern mode
    pub fn tx_datapat_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_tx_datapat_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// MSB of test pattern
///
/// Most significant Tx bits of test pattern
#[derive(From, Into)]
pub struct TX_DATAPAT_MSB(u32);
impl TX_DATAPAT_MSB {
    /// Most significant 32 bits of 64-bit data pattern used in pseudo-random and user-defined test pattern mode
    pub fn tx_datapat_msb(&self) -> u32 {
        self.0
    }
    pub fn set_tx_datapat_msb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Scrambler initialization
///
/// Least significant bits of the scrambler. Used in pseudo-random and PRBS31 modes.
#[derive(From, Into)]
pub struct TX_SEEDA_LSB(u32);
impl TX_SEEDA_LSB {
    /// Least significant bits of scrambler used to initialize it during test mode
    pub fn tx_seeda_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_tx_seeda_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Scrambler initialization
///
/// Most significant bits of the scrambler
#[derive(From, Into)]
pub struct TX_SEEDA_MSB(u32);
impl TX_SEEDA_MSB {
    /// Most significant bits of scrambler used to initialize it during test mode
    pub fn tx_seeda_msb(&self) -> u32 {
        self.0 & 0x3ffffff
    }
    pub fn set_tx_seeda_msb(&mut self, value: u32) {
        assert!(value <= 0x3ffffff);
        self.0 &= !0x3ffffff;
        self.0 |= value;
    }
}
/// Scrambler initialization
///
/// Least significant bits of the scrambler
#[derive(From, Into)]
pub struct TX_SEEDB_LSB(u32);
impl TX_SEEDB_LSB {
    /// Least significant bits of scrambler used to initialize it during test mode
    pub fn tx_seedb_lsb(&self) -> u32 {
        self.0
    }
    pub fn set_tx_seedb_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Scrambler initialization
///
/// Most significant bits of the scrambler
#[derive(From, Into)]
pub struct TX_SEEDB_MSB(u32);
impl TX_SEEDB_MSB {
    /// Most significant bits of scrambler used to initialize it during test mode
    pub fn tx_seedb_msb(&self) -> u32 {
        self.0 & 0x3ffffff
    }
    pub fn set_tx_seedb_msb(&mut self, value: u32) {
        assert!(value <= 0x3ffffff);
        self.0 &= !0x3ffffff;
        self.0 |= value;
    }
}
