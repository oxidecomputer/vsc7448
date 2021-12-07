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

/// Register `BAUDR`
///
/// Baud Rate Select
///
/// The register derives the frequency of the serial clock that regulates the data transfer. The 16-bit field in this register defines the SI divider value. This register can only be written when master is disabled (SIMC::SIMCEN.SIMCEN = 0).
#[derive(From, Into)]
pub struct BAUDR(u32);
impl BAUDR {    ///
    /// The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (si_clk) is disabled. The frequency of the si_clk is derived from the following equation: Fsclk_out = Fsystem_clk/SCKDV where SCKDV is any even value between 2 and 65534 and Fsystem_clk is 250MHz.
    pub fn sckdv(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_sckdv(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `CTRLR0`
///
/// Control Register 0
///
/// This register controls the serial data transfer. This register can only be written when master is disabled (SIMC::SIMCEN.SIMCEN = 0).
#[derive(From, Into)]
pub struct CTRLR0(u32);
impl CTRLR0 {    ///
    /// Control Frame Size. Selects the length of the control word for the Microwire frame format.
    ///
    /// n: n+1 bit control word.
    pub fn cfs(&self) -> u32 {
        (self.0 & 0xf000) >> 12
    }
    pub fn set_cfs(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0xf000);
        self.0 &= !0xf000;
        self.0 |= value;
    }    ///
    /// Selects the data frame length. See SIMC::DR register description for how to read/write words of less than 16 bit.
    ///
    /// 0-2: Reserved. n: n+1 bit serial data transfer.
    pub fn dfs(&self) -> u32 {
        (self.0 & 0xf) >> 0
    }
    pub fn set_dfs(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }    ///
    /// Selects which serial protocol transfers the data. Note: In addition to this field, software must also configure ICPU_CFG::GENERAL_CTRL.SIMC_SSP_ENA.
    ///
    /// 0: Motorola SPI 1: Texas Instruments SSP 2-3: Reserved
    pub fn frf(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_frf(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }    ///
    /// Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock.
    ///
    /// 0: Serial clock toggles in middle of first data bit. 1: Serial clock toggles at start of first data bit.
    pub fn scph(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_scph(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }    ///
    /// Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the master is not actively transferring data on the serial bus.
    ///
    /// 0: Inactive state of serial clock is low. 1: Inactive state of serial clock is high.
    pub fn scpol(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_scpol(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }    ///
    /// Shift Register Loop. Used for testing purposes only. Set to connect the transmit shift register output to the receive shift register input.
    ///
    /// 0: Normal Mode Operation 1: Test Mode Operation
    pub fn srl(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_srl(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }    ///
    /// Selects the mode of transfer for serial communication. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor.
    ///
    /// 0: Transmit and Receive. 1: Transmit Only. 2: Receive Only. 3: Reserved.
    pub fn tmod(&self) -> u32 {
        (self.0 & 0x300) >> 8
    }
    pub fn set_tmod(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x300);
        self.0 &= !0x300;
        self.0 |= value;
    }
}

/// Register `CTRLR1`
///
/// Control Register 1
///
/// Control register 1 controls the end of serial transfers when in receive-only mode. This register can only be written when master is disabled (SIMC::SIMCEN.SIMCEN = 0).
#[derive(From, Into)]
pub struct CTRLR1(u32);
impl CTRLR1 {    ///
    /// When SIMC::CTRLR0.TMOD = 2, this register field sets the number of data frames to be continuously received by the master. The master continues to receive serial data until the number of data frames received is equal to this register value plus 1, which enables receiveing up to 64 KB of data in a continuous transfer.
    pub fn ndf(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_ndf(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `DR`
///
/// Transmit/Receive FIFO
///
/// 16-bit read/write buffer for the transmit/receive FIFOs. When the register is read, data in the receive FIFO buffer is accessed. When it is written to, data are moved into the transmit FIFO buffer; a write can occur only when SIMC::SIMCEN.SIMCEN = 1. FIFOs are reset when SIMC::SIMCEN.SIMCEN = 0. This register is replicated to allow burst access to fifo's; the replication index is not used when accessing the FIFO.
#[derive(From, Into)]
pub struct DR(u32);
impl DR {    ///
    /// Data is aligned to bit 0 (right-justified) when accessing less than 16 bit data-words. Read = Receive FIFO buffer. Write = Transmit FIFO buffer.
    pub fn dr(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_dr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `ICR`
///
/// SIMC Combined Interrupt Clear
#[derive(From, Into)]
pub struct ICR(u32);
impl ICR {    ///
    /// This field is set when any of the master's TXO, RXO, RXU, or MST interrupts are active. Reading from this register clears all interrupts, as reading from SIMC::TXOICR, SIMC::RXOICR, SIMC::RXUICR, and SIMC::MSTICR registers.
    pub fn icr(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_icr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `IMR`
///
/// Interrupt Mask
#[derive(From, Into)]
pub struct IMR(u32);
impl IMR {    ///
    /// Set to enable Multi-Master Contention Interrupt
    pub fn mstim(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_mstim(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }    ///
    /// Set to enable Receive FIFO Full Interrupt
    pub fn rxfim(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rxfim(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Set to enable Receive FIFO Overflow Interrupt
    pub fn rxoim(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_rxoim(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Set to enable Receive FIFO Underflow Interrupt
    pub fn rxuim(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_rxuim(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Set to enable Transmit FIFO Empty Interrupt
    pub fn txeim(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_txeim(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Set to enable Transmit FIFO Overflow Interrupt
    pub fn txoim(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_txoim(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}

/// Register `ISR`
///
/// Interrupt Status
///
/// If any bit is set in this register, then the SI Master Controller is indicating interrupt towards the VCore Interrupt Controller.
#[derive(From, Into)]
pub struct ISR(u32);
impl ISR {    ///
    /// Multi-Master Contention Interrupt Status, this field is masked by SIMC::IMR.MSTIM.
    pub fn mstis(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_mstis(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }    ///
    /// Receive FIFO Full Interrupt Status, this field is masked by SIMC::IMR.RXFIM. This interrupt is based on programmable fill level, see SIMC::RXFTLR for more information.
    pub fn rxfis(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rxfis(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Receive FIFO Overflow Interrupt Status, this field is masked by SIMC::IMR.RXOIM.
    pub fn rxois(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_rxois(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Receive FIFO Underflow Interrupt Status, this field is masked by SIMC::IMR.RXUIM.
    pub fn rxuis(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_rxuis(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Transmit FIFO Empty Interrupt Status, this field is masked by SIMC::IMR.TXEIM. This interrupt is based on programmable fill level, see SIMC::TXFTLR for more information.
    pub fn txeis(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_txeis(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Transmit FIFO Overflow Interrupt Status, this field is masked by SIMC::IMR.TXOIM.
    pub fn txois(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_txois(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}

/// Register `MSTICR`
///
/// Multi-Master Contention Interrupt Clear
#[derive(From, Into)]
pub struct MSTICR(u32);
impl MSTICR {    ///
    /// This field is set when Multi-Master Contention Interrupt is active, interrupt is cleared by reading this register.
    pub fn msticr(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_msticr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `MWCR`
///
/// Microwire Control Register
///
/// This register controls the direction of the data word for the half-duplex Microwire serial protocol. This register can only be written when master is disabled (SIMC::SIMCEN.SIMCEN = 0).
#[derive(From, Into)]
pub struct MWCR(u32);
impl MWCR {    ///
    /// Defines the direction of the data word when the Microwire serial protocol is used. When this bit is set to 0, the data word is received by the master from the external serial device. When this bit is set to 1, the data word is transmitted from the master to the external serial device.
    pub fn mdd(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_mdd(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Set to enable the busy/ready handshaking interface for the Microwire protocol. When enabled, the master checks for a ready status from the target slave, after the transfer of the last data/control bit, before clearing the BUSY status in the SR register.
    ///
    /// 0: handshaking interface is disabled 1: handshaking interface is enabled
    pub fn mhs(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_mhs(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Defines whether the Microwire transfer is sequential or non-sequential. When sequential mode is used, only one control word is needed to transmit or receive a block of data words. When non-sequential mode is used, there must be a control word for each data word that is transmitted or received.
    ///
    /// 0: non-sequential transfer 1: sequential transfer
    pub fn mwmod(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_mwmod(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `RISR`
///
/// Raw Interrupt Status
#[derive(From, Into)]
pub struct RISR(u32);
impl RISR {    ///
    /// Current status of Multi-Master Contention Interrupt before masking.
    pub fn mstir(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_mstir(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }    ///
    /// Current status of Receive FIFO Full Interrupt before masking
    pub fn rxfir(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rxfir(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Current status of Receive FIFO Overflow Interrupt before masking
    pub fn rxoir(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_rxoir(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Current status of Receive FIFO Underflow Interrupt before masking
    pub fn rxuir(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_rxuir(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Current status of Transmit FIFO Empty Interrupt before masking
    pub fn txeir(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_txeir(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Current status of Transmit FIFO Overflow Interrupt before masking
    pub fn txoir(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_txoir(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}

/// Register `RXFLR`
///
/// Receive FIFO Level
#[derive(From, Into)]
pub struct RXFLR(u32);
impl RXFLR {    ///
    /// Contains the number of valid data entries in the receive FIFO.
    pub fn rxtfl(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_rxtfl(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}

/// Register `RXFTLR`
///
/// Receive FIFO Threshold level
#[derive(From, Into)]
pub struct RXFTLR(u32);
impl RXFTLR {    ///
    /// When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered. The receive FIFO depth is 40, do not program value exceeding 39.
    pub fn rft(&self) -> u32 {
        (self.0 & 0x3f) >> 0
    }
    pub fn set_rft(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x3f);
        self.0 &= !0x3f;
        self.0 |= value;
    }
}

/// Register `RXOICR`
///
/// Receive FIFO Overflow Interrupt Clear
#[derive(From, Into)]
pub struct RXOICR(u32);
impl RXOICR {    ///
    /// This field is set when Receive FIFO Overflow Interrupt is active, interrupt is cleared by reading this register.
    pub fn rxoicr(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_rxoicr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `RXUICR`
///
/// Receive FIFO Underflow Interrupt Clear
#[derive(From, Into)]
pub struct RXUICR(u32);
impl RXUICR {    ///
    /// This field is set when Receive FIFO Underflow Interrupt is active, interrupt is cleared by reading this register.
    pub fn rxuicr(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_rxuicr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `SER`
///
/// Slave Enable
///
/// The register enables the individual slave select output lines from the master. 16 slave-select output pins are available on the master. This register can only be written when master is disabled and not busy.
#[derive(From, Into)]
pub struct SER(u32);
impl SER {    ///
    /// Each bit in this register corresponds to a slave select line from the master. When a bit in this register is set, the corresponding slave select line from the master is activated when a serial transfer begins. Setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, enable the bit in this register that corresponds to the slave device with which the master wants to communicate. When not operating in broadcast mode, only one bit in this field should be set.
    ///
    /// 1: Selected 0: Not Selected
    pub fn ser(&self) -> u32 {
        (self.0 & 0xffff) >> 0
    }
    pub fn set_ser(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}

/// Register `SIMCEN`
///
/// SIMC Enable
#[derive(From, Into)]
pub struct SIMCEN(u32);
impl SIMCEN {    ///
    /// Set to enable master operations. When disabled, all serial transfers are halted immediately. Transmit and receive FIFO buffers are cleared when disabled. It is impossible to program some of the master control registers when enabled. Note: The SI Master Controller must own the SI interface before it is enabled, see ICPU_CFG::GENERAL_CTRL.IF_SI_OWNER for more information.
    pub fn simcen(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_simcen(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `SR`
///
/// Status Register
#[derive(From, Into)]
pub struct SR(u32);
impl SR {    ///
    /// Set when serial transfer is in progress. Cleared when master is idle or disabled.
    pub fn busy(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_busy(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Set when receive FIFO is full.
    pub fn rff(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_rff(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Set when receive FIFO has one or more data-word.
    pub fn rfne(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_rfne(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Set when transmit FIFO is empty.
    pub fn tfe(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_tfe(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Set when transmit FIFO has room for one or more data-word.
    pub fn tfnf(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_tfnf(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}

/// Register `TXFLR`
///
/// Transmit FIFO Level
#[derive(From, Into)]
pub struct TXFLR(u32);
impl TXFLR {    ///
    /// Contains the number of valid data entries in the transmit FIFO.
    pub fn txtfl(&self) -> u32 {
        (self.0 & 0x7) >> 0
    }
    pub fn set_txtfl(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}

/// Register `TXFTLR`
///
/// Transmit FIFO Threshold Level
#[derive(From, Into)]
pub struct TXFTLR(u32);
impl TXFTLR {    ///
    /// When the number of transmit FIFO entries is less than or equal to this value, the transmit FIFO empty interrupt is triggered. The transmit FIFO depth is 8, do not program value exceeding 7.
    pub fn tft(&self) -> u32 {
        (self.0 & 0x7) >> 0
    }
    pub fn set_tft(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x7);
        self.0 &= !0x7;
        self.0 |= value;
    }
}

/// Register `TXOICR`
///
/// Transmit FIFO Overflow Interrupt Clear
#[derive(From, Into)]
pub struct TXOICR(u32);
impl TXOICR {    ///
    /// This field is set when Transmit FIFO Overflow Interrupt is active, interrupt is cleared by reading this register.
    pub fn txoicr(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_txoicr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
