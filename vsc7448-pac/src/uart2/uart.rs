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
/// Halt tx
#[derive(From, Into)]
pub struct HTX(u32);
impl HTX {
    /// This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are enabled.

    ///

    /// 0: Halt tx disabled 1: Halt tx enabled
    pub fn htx(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_htx(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Interrupt enable / divisor (high)
///
/// When the LCR.DLAB is set, this register is the upper 8 bits of the 16-bit Divisor register that contains the baud rate divisor for the UART. For more information and a description of how to calculate the baud rate, see RBR_THR.
#[derive(From, Into)]
pub struct IER(u32);
impl IER {
    /// Enable modem status interrupt. This is used to enable or disable the generation of Modem Status interrupt. This is the fourth highest priority interrupt.

    ///

    /// 0: Disabled 1: Enabled
    pub fn edssi(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_edssi(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Enable receiver line status interrupt. This is used to enable or disable the generation of Receiver Line Status interrupt. This is the highest priority interrupt.

    ///

    /// 0: Disabled 1: Enabled
    pub fn elsi(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_elsi(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Enable received data available interrupt. This is used to enable or disable the generation of Received Data Available interrupt and the Character Timeout interrupt (if FIFOs are enabled). These are the second highest priority interrupts.

    ///

    /// 0: Disabled 1: Enabled
    pub fn erbfi(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_erbfi(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Enable transmit holding register empty interrupt. This is used to enable or disable the generation of Transmitter Holding Register Empty interrupt. This is the third highest priority interrupt.

    ///

    /// 0: Disabled 1: Enabled
    pub fn etbei(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_etbei(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Programmable THRE interrupt mode enable. This is used to enable or disable the generation of THRE interrupt.

    ///

    /// 0: Disabled 1: Enabled
    pub fn ptime(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_ptime(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
}
/// Interrupt identification / FIFO control register
///
/// This register has special meaning when reading, here the lowest 4 bits indicate interrupting sources. The encoding is as follows: 0110; type: Receiver line status, priority: Highest. Overrun/parity/ framing errors or break interrupt. Cleared by reading LSR. 0100; type: Received data available, priority: Second. RCVR FIFO trigger level reached. Cleared when FIFO drops below the trigger level. 1100; type: Character timeout indication, priority: Second. No characters in or out of the RCVR FIFO during the last four character times and there is at least 1 character in it during this time. Cleared by reading the receiver buffer register. 0010; type: Transmit holding register empty, priority: Third. Transmitter holding register empty (Prog. THRE Mode disabled) or XMIT FIFO at or below threshold (Prog. THRE Mode enabled). Cleared by reading the IIR register (if source of interrupt); or, writing into THR (THRE Mode disabled) or XMIT FIFO above threshold (THRE Mode enabled). 0000; type: Modem status, priority: Fourth. Clear to send. Note that if auto flow control mode is enabled, a change in CTS (that is, DCTS set) does not cause an interrupt. Cleared by reading the Modem status register. 0111; type: Busy detect indication, priortiy: Fifth. Master has tried to write to the Line Control register while the UART is busy (USR[0] is set to one). Cleared by reading the UART status register. 0001: No interrupting sources.
#[derive(From, Into)]
pub struct IIR_FCR(u32);
impl IIR_FCR {
    /// This description is valid for writes only. Reading this field has special meaning; for more information, see the general register description. FIFO Enable. This enables or disables the transmit (XMIT) and receive (RCVR) FIFOs. Whenever the value of this bit is changed, both the XMIT and RCVR controller portion of FIFOs are reset.
    pub fn fifoe(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_fifoe(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// When reading this field, the current status of the FIFO is returned; 00 for disabled or 11 for enabled. Writing this field selects the trigger level in the receive FIFO at which the Received Data Available interrupt is generated (see encoding.) In auto flow control mode, it is used to determine when to generate back-pressure using the RTS signal.

    ///

    /// 00: 1 character in the Rx FIFO 01: Rx FIFO 1/4 full 10: Rx FIFO 1/2 full 11: Rx FIFO 2 less than full
    pub fn fifose_rt(&self) -> u32 {
        (self.0 & 0xc0) >> 6
    }
    pub fn set_fifose_rt(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0xc0);
        self.0 &= !0xc0;
        self.0 |= value;
    }
    /// This description is valid for writes only. Reading this field has special meaning; for more information, see the general register description. Rx FIFO Reset. This resets the control portion of the receive FIFO and treats the FIFO as empty. Note that this bit is self-clearing. It is not necessary to clear this bit.
    pub fn rfifor(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_rfifor(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Tx empty trigger. When the THRE mode is enabled (IER.PTIME), this field selects the empty threshold level at which the THRE Interrupts are generated.

    ///

    /// 00: Tx FIFO empty 01: 2 characters in the Tx FIFO 10: Tx FIFO 1/4 full 11: Tx FIFO 1/2 full
    pub fn tet(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_tet(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }
    /// This description is valid for writes only. Reading this field has special meaning; for more information, see the general register description. Tx FIFO Reset. This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is self-clearing. It is not necessary to clear this bit.
    pub fn xfifor(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_xfifor(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// Line control
///
/// Writes can be made to this register, with the exception of the BC field, only when UART is not busy, that is, when USR.BUSY is zero. This register can always be read.
#[derive(From, Into)]
pub struct LCR(u32);
impl LCR {
    /// Break control bit.This bit is used to cause a break condition to be transmitted to the receiving device. If set to one, the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR[4], the sout line is forced low until the Break bit is cleared.
    pub fn bc(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_bc(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Divisor latch access bit. This bit is used to enable reading and writing of the Divisor registers (RBR_THR and IER) to set the baud rate of the UART. To access other registers, this bit must be cleared after initial baud rate setup.
    pub fn dlab(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_dlab(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Data length select. This is used to select the number of data bits per character that the peripheral transmits and receives. The following settings specify the number of bits that may be selected.

    ///

    /// 00: 5 bits 01: 6 bits 10: 7 bits 11: 8 bits
    pub fn dls(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_dls(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Even parity select. This bit is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked.
    pub fn eps(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_eps(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Parity enable. This bit is used to enable or disable parity generation and detection in both transmitted and received serial characters.

    ///

    /// 0: Parity disabled 1: Parity enabled
    pub fn pen(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_pen(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Number of stop bits. This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR.DLS), one and a half stop bits are transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit.

    ///

    /// 0: 1 stop bit 1: 1.5 stop bits when LCR.DLS is zero, otherwise, 2 stop bits
    pub fn stop(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_stop(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// Line status
#[derive(From, Into)]
pub struct LSR(u32);
impl LSR {
    /// Break interrupt bit. This is used to indicate the detection of a break sequence on the serial input data. It is set whenever the serial input is held in a logic 0 state for longer than the sum of start time + data bits + parity + stop bits. A break condition on serial input causes one and only one character, consisting of all-zeros, to be received by the UART. In the FIFO mode, the character associated with the break condition is carried through the FIFO and is revealed when the character is at the top of the FIFO. Reading the LSR clears the BI bit. In the non-FIFO mode, the BI indication occurs immediately and persists until the LSR is read.
    pub fn bi(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_bi(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Data ready. This is used to indicate that the receiver contains at least one character in the receiver FIFO. This bit is cleared when the RX FIFO is empty.

    ///

    /// 0: No data ready 1: Data ready
    pub fn dr(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_dr(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Framing error bit. This is used to indicate the a framing error in the receiver. A framing error occurs when the receiver does not detect a valid STOP bit in the received data. A framing error is associated with a received character. Therefore, in FIFO mode, an error is revealed when the character with the framing error is at the top of the FIFO. When a framing error occurs, the UART tries to resynchronize. It does this by assuming that the error was due to the start bit of the next character and then continues to receive the other bit, that is, data and/or parity, and then stops. Note that this field is set if a break interrupt has occurred, as indicated by Break Interrupt (LSR.BI). This field is cleared on read.

    ///

    /// 0: No framing error 1: Framing error
    pub fn fe(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_fe(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Overrun error bit. This is used to indicate the occurrence of an overrun error. This occurs if a new data character was received before the previous data was read. In non-FIFO mode, the OE bit is set when a new character arrives before the previous character was read. When this happens, the data in the RBR is overwritten. In FIFO mode, an overrun error occurs when the FIFO is full and a new character arrives at the receiver. The data in the FIFO is retained and the data in the receive shift register is lost. This field is cleared on read.

    ///

    /// 0: No overrun error 1: Overrun error
    pub fn oe(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_oe(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Parity error bit. This is used to indicate the occurrence of a parity error in the receiver if the Parity Enable bit (LCR.PEN) is set. A parity error is associated with a received character. Therefore, in FIFO mode, an error is revealed when the character with the parity error arrives at the top of the FIFO. Note that this field is set if a break interrupt has occurred, as indicated by Break Interrupt (LSR.BI). This field is cleared on read.

    ///

    /// 0: No parity error 1: Parity error
    pub fn pe(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_pe(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Receiver FIFO error bit. This bit is only valid when FIFOs are enabled. This is used to indicate whether there is at least one parity error, framing error, or break indication in the FIFO. This bit is cleared when the LSR is read, the character with the error is at the top of the receiver FIFO, and there are no subsequent errors in the FIFO.

    ///

    /// 0: No error in Rx FIFO 1: Error in Rx FIFO
    pub fn rfe(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_rfe(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Transmitter empty bit. If FIFOs are enabled, this bit is set whenever the Transmitter Shift Register and the FIFO are both empty.
    pub fn temt(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_temt(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// If FIFO (IIR_FCR.FIFOE) and THRE mode are enabled (IER.PTIME), this bit indicates that the Tx FIFO is full. Otherwise, this bit indicates that the Tx FIFO is empty.
    pub fn thre(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_thre(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
}
/// Modem control
#[derive(From, Into)]
pub struct MCR(u32);
impl MCR {
    /// Auto flow control enable. This mode requires that FIFOs are enabled and that MCR.RTS is set.

    ///

    /// 0: Auto flow control mode disabled 1: Auto flow control mode enabled
    pub fn afce(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_afce(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Loopback Bit. This is used to put the UART into a diagnostic mode for test purposes. The transmit line is held high, while serial transmit data is looped back to the receive line internally. In this mode, all the interrupts are fully functional. In addition, in loopback mode, the modem control input CTS is disconnected, and the modem control output RTS is looped back to the input internally.
    pub fn lb(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_lb(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Request to send. This is used to directly control the Request to Send (RTS) output. The RTS output is used to inform the partner that the UART is ready to exchange data. The RTS is still controlled from this field when Auto RTS Flow Control is enabled (MCR.AFCE), but the output can be forced high by the flow control mechanism. If this field is cleared, the UART permanently indicates backpressure to the partner.

    ///

    /// 0: RTS is set high 1: RTS is set low
    pub fn rts(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_rts(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
/// Modem status
#[derive(From, Into)]
pub struct MSR(u32);
impl MSR {
    /// Clear to send. This field indicates the current state of the modem control line, CTS. When the Clear to Send input (CTS) is asserted, it is an indication that the partner is ready to exchange data with the UART.

    ///

    /// 0: CTS input is deasserted (logic 0) 1: CTS input is asserted (logic 1)
    pub fn cts(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_cts(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Delta clear to send. This is used to indicate that the modem control line, CTS, has changed since the last time the MSR was read. Reading the MSR clears the DCTS bit. Note: If the DCTS bit is not set, the CTS signal is asserted, and a reset occurs (software or otherwise), then the DCTS bit is set when the reset is removed, if the CTS signal remains asserted. A read of the MSR after reset can be performed to prevent unwanted interrupts.

    ///

    /// 0: No change on CTS since the last read of the MSR 1: Change on CTS since the last read of the MSR
    pub fn dcts(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_dcts(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Receive buffer / transmit holding / divisor (low)
///
/// When the LCR.DLAB is set, this register is the lower 8 bits of the 16-bit Divisor register that contains the baud rate divisor for the UART. The output baud rate is equal to the VCore system clock frequency divided by sixteen times the value of the baud rate divisor, as follows: baud rate = (VCore clock freq) / (16 * divisor). Note that with the Divisor set to zero, the baud clock is disabled and no serial communications occur. In addition, once this register is set, wait at least 0.1us before transmitting or receiving data.
#[derive(From, Into)]
pub struct RBR_THR(u32);
impl RBR_THR {
    /// Use this register to access the Rx and Tx FIFOs. When reading: The data in this register is valid only if LSR.DR is set. If FIFOs are disabled (IIR_FCR.FIFOE), the data in this register must be read before the next data arrives, otherwise it is overwritten, resulting in an overrun error. When FIFOs are enabled (IIR_FCR.FIFOE), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO is preserved, but any incoming data is lost and an overrun error occurs. When writing: Data should only be written to this register when the LSR.THRE indicates that there is room in the FIFO. If FIFOs are disabled (IIR_FCR.FIFOE), writes to this register while LSR.THRE is zero, causes the register to be overwritten. When FIFOs are enabled (IIR_FCR.FIFOE) and LSR.THRE is set, 16 characters may be written to this register before the FIFO is full. Any attempt to write data when the FIFO is full results in the write data being lost.
    pub fn rbr_thr(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_rbr_thr(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Reserved
#[derive(From, Into)]
pub struct RESERVED1(u32);
impl RESERVED1 {    pub fn reserved1(&self) -> u32 {
        self.0
    }
    pub fn set_reserved1(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Reserved
#[derive(From, Into)]
pub struct RESERVED2(u32);
impl RESERVED2 {    pub fn reserved2(&self) -> u32 {
        self.0
    }
    pub fn set_reserved2(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Scratchpad
#[derive(From, Into)]
pub struct SCR(u32);
impl SCR {
    /// This register is for programmers to use as a temporary storage space. It has no functional purpose for the UART.
    pub fn scr(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_scr(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// UART status
#[derive(From, Into)]
pub struct USR(u32);
impl USR {
    /// UART busy.

    ///

    /// 0: UART is idle or inactive 1: UART is busy (actively transferring data)
    pub fn busy(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_busy(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
