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
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCS_CFG(u32);
impl PCS_CFG {
    /// Enable link control using backplane Ethernet ANEG (Auto-Negotiation)
    ///
    /// 0: Disable link control 1: Enable link control
    #[inline(always)]
    pub fn an_link_ctrl_ena(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_an_link_ctrl_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Disable RX local fault generation when no alignment has been reached
    ///
    /// 0: Output local fault symbol at XGMII when not aligned 1: Output IDLE symbols at XGMII when not aligned
    #[inline(always)]
    pub fn lf_gen_dis(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    #[inline(always)]
    pub fn set_lf_gen_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 14;
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Global PCS Enable/Disable configuration bit.
    ///
    /// 0 = Disable PCS 1 = Enable PCS
    #[inline(always)]
    pub fn pcs_ena(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    #[inline(always)]
    pub fn set_pcs_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 31;
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// Enable PMA loopback operation. When set, Transmit PMA data is loopbacked to Receive PMA data.
    ///
    /// 0 = Normal mode 1 = Loopback Tx PMA to Rx PMA
    #[inline(always)]
    pub fn pma_loopback_ena(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    #[inline(always)]
    pub fn set_pma_loopback_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 30;
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    /// Force re-synchronization of receive logic. When asserted, the receive sync state machine is forced into the LOCK_INIT state and block_lock is lost.
    ///
    /// 0: Normal operation 1: Reset synchronization
    #[inline(always)]
    pub fn resync_ena(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    #[inline(always)]
    pub fn set_resync_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 15;
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Flip the data bus bits at the WIS/PMA interface such that bit 63 is mapped to bit 0 and bit 0 to 63. i.e. the output bus (63 down to 0) is remapped to (0 to 63) and bit 63 is the first bit.
    ///
    /// 0: No flip (LSB first) 1: Flip bus (MSB first)
    #[inline(always)]
    pub fn rx_data_flip(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_rx_data_flip(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Disable the descrambler. When disabled, the data is passed through without being descrambled.
    ///
    /// 0: Decrambler active 1: Decrambler disabled
    #[inline(always)]
    pub fn rx_scr_disable(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_rx_scr_disable(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Enable test pattern checking mode.
    ///
    /// 0: Normal operation 1: Test pattern mode
    #[inline(always)]
    pub fn rx_test_mode(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    #[inline(always)]
    pub fn set_rx_test_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 13;
        self.0 &= !0x2000;
        self.0 |= value;
    }
    /// Number of sync headers required for block lock. The actual number used is this number plus 1. i.e. entering 63 implies a value of 64.
    ///
    /// Binary number
    #[inline(always)]
    pub fn sh_cnt_max(&self) -> u32 {
        (self.0 & 0x3f000000) >> 24
    }
    #[inline(always)]
    pub fn set_sh_cnt_max(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 24;
        self.0 &= !0x3f000000;
        self.0 |= value;
    }
    /// Flip the data bus bits at the WIS/PMA interface such that bit 63 is mapped to bit 0 and bit 0 to 63. i.e. the output bus (63 downto 0) is remapped to (0 to 63) and bit 63 is the first bit.
    ///
    /// 0 = No flip (LSB first) 1 = Flip bus (MSB first)
    #[inline(always)]
    pub fn tx_data_flip(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_tx_data_flip(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Disable the scrambler. When disabled, the data is passed through without being scrambled.
    ///
    /// 0: Scrambler active 1: Scrambler disabled
    #[inline(always)]
    pub fn tx_scr_disable(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_tx_scr_disable(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Enable test pattern generation mode
    ///
    /// 0: Normal operation 1: Test pattern mode
    #[inline(always)]
    pub fn tx_test_mode(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_tx_test_mode(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
}
/// PCS interrupt mask register
///
/// Masks for PCS interrupt sources and sticky bits in PCS_INTR_STAT
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCS_INTR_MASK(u32);
impl PCS_INTR_MASK {
    /// Mask for the C64B66B_ERR_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    #[inline(always)]
    pub fn c64b66b_err_mask(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_c64b66b_err_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Mask for the LOCK_CHANGED_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    #[inline(always)]
    pub fn lock_changed_mask(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_lock_changed_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Mask for the RX_FSET_FIFO_FULL_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    #[inline(always)]
    pub fn rx_fset_fifo_full_mask(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    #[inline(always)]
    pub fn set_rx_fset_fifo_full_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 12;
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Mask for the RX_FSET_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    #[inline(always)]
    pub fn rx_fset_mask(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    #[inline(always)]
    pub fn set_rx_fset_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 11;
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Mask for the RX_HI_BER_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    #[inline(always)]
    pub fn rx_hi_ber_mask(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_rx_hi_ber_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Mask for RX_OSET_FIFO_FULL_STICKY
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    #[inline(always)]
    pub fn rx_oset_fifo_full_mask(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_rx_oset_fifo_full_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Mask for the RX_OSET_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    #[inline(always)]
    pub fn rx_oset_mask(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_rx_oset_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Mask for the XGMII_ERR_STICKY bit
    ///
    /// 0: Interrupt disabled 1: Interrupt enabled
    #[inline(always)]
    pub fn xgmii_err_mask(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_xgmii_err_mask(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
}
/// PCS SignalDetect Configuration
///
/// PCS signal_detect configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCS_SD_CFG(u32);
impl PCS_SD_CFG {
    /// Signal Detect Enable
    ///
    /// 0: The Signal Detect input pin is ignored. The PCS assumes an active Signal Detect at all times 1: The Signal Detect input pin is used to determine if a signal is detected
    #[inline(always)]
    pub fn sd_ena(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_sd_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Signal detect polarity: The signal level on signal_detect input pin must be equal to SD_POL to indicate signal detection (SD_ENA must be set)
    ///
    /// 0: Signal Detect input pin must be '0' to indicate a signal detection 1: Signal Detect input pin must be '1' to indicate a signal detection
    #[inline(always)]
    pub fn sd_pol(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_sd_pol(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Signal detect selection (select input for internal signal_detect line)
    ///
    /// 0: Select signal_detect line from hardmacro 1: Select external signal_detect line
    #[inline(always)]
    pub fn sd_sel(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_sd_sel(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
}
/// LSB of test pattern
///
/// Least significant Rx bits of test pattern
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RX_DATAPAT_LSB(u32);
impl RX_DATAPAT_LSB {
    /// Least significant 32 bits of 64-bit data pattern used in pseudo-random and user-defined test pattern mode
    #[inline(always)]
    pub fn rx_datapat_lsb(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_rx_datapat_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// MSB of test pattern
///
/// Most significant Rx bits of test pattern
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RX_DATAPAT_MSB(u32);
impl RX_DATAPAT_MSB {
    /// Most significant 32 bits of 64-bit data pattern used in pseudo-random and user-defined test pattern mode
    #[inline(always)]
    pub fn rx_datapat_msb(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_rx_datapat_msb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// PRBS31 initialization
///
/// Initial value to load into the PRBS31 linear-feedback shift register
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RX_PRBS31_INIT(u32);
impl RX_PRBS31_INIT {
    /// PRBS31 initial value.
    #[inline(always)]
    pub fn rx_prbs31_init(&self) -> u32 {
        self.0 & 0x7fffffff
    }
    #[inline(always)]
    pub fn set_rx_prbs31_init(&mut self, value: u32) {
        assert!(value <= 0x7fffffff);
        self.0 &= !0x7fffffff;
        self.0 |= value;
    }
}
/// Test pattern mode configuration
///
/// Select test patterns when test mode is enabled
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TEST_CFG(u32);
impl TEST_CFG {
    /// Disables inversion of seeds and data in the pseudo-random test pattern
    ///
    /// 0: Inversion enabled 1: Inversion disabled
    #[inline(always)]
    pub fn rx_dsbl_inv(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    #[inline(always)]
    pub fn set_rx_dsbl_inv(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 18;
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Select the test pattern used by the test pattern checker. This register is only used if RX_TEST_MODE is active.
    ///
    /// 0: unused 1: Pseudo random 2: PRBS31 3: User defined
    #[inline(always)]
    pub fn rx_testpat_sel(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    #[inline(always)]
    pub fn set_rx_testpat_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 16;
        self.0 &= !0x30000;
        self.0 |= value;
    }
    /// Disables inversion of seeds and data in the pseudo-random test pattern
    ///
    /// 0: Inversion enabled 1: Inversion disabled
    #[inline(always)]
    pub fn tx_dsbl_inv(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_tx_dsbl_inv(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Period of square wave. Value in the register is (Period-4). Valid values of period are 4 to 11.
    ///
    /// Period = (TX_SQPW_4B+4)
    #[inline(always)]
    pub fn tx_sqpw_4b(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    #[inline(always)]
    pub fn set_tx_sqpw_4b(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 2;
        self.0 &= !0x1c;
        self.0 |= value;
    }
    /// Select the test pattern used by the test pattern generator. This register is only used if TX_TEST_MODE is active.
    ///
    /// 0: Square wave 1: Pseudo random 2: PRBS31 3: User defined
    #[inline(always)]
    pub fn tx_testpat_sel(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_tx_testpat_sel(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
/// Counter for 125 microsecond period
///
/// This register sets the number of WIS/PMA divide-by-2 clocks in one 125 microsecond interval. The counter increments and wraps. It should be set to (125 * freq_Mhz/2) where freq_Mhz is the WIS/PMA frequency in Megahertz. 0 disables the counter.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TIMER_125(u32);
impl TIMER_125 {
    /// Sets the maximum count for the 125 microsecond counter. Counts input clocks.
    ///
    /// 16-bit binary number
    #[inline(always)]
    pub fn timer_125(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_timer_125(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// LSB of test pattern
///
/// Least significant Tx bits of test pattern
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TX_DATAPAT_LSB(u32);
impl TX_DATAPAT_LSB {
    /// Least significant 32 bits of 64-bit data pattern used in pseudo-random and user-defined test pattern mode
    #[inline(always)]
    pub fn tx_datapat_lsb(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_tx_datapat_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// MSB of test pattern
///
/// Most significant Tx bits of test pattern
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TX_DATAPAT_MSB(u32);
impl TX_DATAPAT_MSB {
    /// Most significant 32 bits of 64-bit data pattern used in pseudo-random and user-defined test pattern mode
    #[inline(always)]
    pub fn tx_datapat_msb(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_tx_datapat_msb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Scrambler initialization
///
/// Least significant bits of the scrambler. Used in pseudo-random and PRBS31 modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TX_SEEDA_LSB(u32);
impl TX_SEEDA_LSB {
    /// Least significant bits of scrambler used to initialize it during test mode
    #[inline(always)]
    pub fn tx_seeda_lsb(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_tx_seeda_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Scrambler initialization
///
/// Most significant bits of the scrambler
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TX_SEEDA_MSB(u32);
impl TX_SEEDA_MSB {
    /// Most significant bits of scrambler used to initialize it during test mode
    #[inline(always)]
    pub fn tx_seeda_msb(&self) -> u32 {
        self.0 & 0x3ffffff
    }
    #[inline(always)]
    pub fn set_tx_seeda_msb(&mut self, value: u32) {
        assert!(value <= 0x3ffffff);
        self.0 &= !0x3ffffff;
        self.0 |= value;
    }
}
/// Scrambler initialization
///
/// Least significant bits of the scrambler
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TX_SEEDB_LSB(u32);
impl TX_SEEDB_LSB {
    /// Least significant bits of scrambler used to initialize it during test mode
    #[inline(always)]
    pub fn tx_seedb_lsb(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_tx_seedb_lsb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Scrambler initialization
///
/// Most significant bits of the scrambler
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct TX_SEEDB_MSB(u32);
impl TX_SEEDB_MSB {
    /// Most significant bits of scrambler used to initialize it during test mode
    #[inline(always)]
    pub fn tx_seedb_msb(&self) -> u32 {
        self.0 & 0x3ffffff
    }
    #[inline(always)]
    pub fn set_tx_seedb_msb(&mut self, value: u32) {
        assert!(value <= 0x3ffffff);
        self.0 &= !0x3ffffff;
        self.0 |= value;
    }
}
