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
/// Miscellaneous debug configuration
///
/// This register holds miscellaneous configuration bit groups used for debug
#[derive(From, Into)]
pub struct DBG_CFG(u32);
impl DBG_CFG {
    /// This field can be used to configure the ASM not to silently discard frames that are aborted by a device within the first cell of a frame. Note that enabling this feature may cause overflow in the ASM when small fragments are received at the Taxi interface.

    ///

    /// '0': Frames are silently discarded by the ASM if it is aborted within the first cell of the frame. '1': No frames are silently discarded by the ASM
    pub fn abort_dis(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_abort_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// This field can be used to disable the cell bus output of the ASM - i.e. to configure the ASM to replace any data cells with IDLE cells. Data is flushed from the ASM FIFO when the output is disabled.

    ///

    /// '0': Any data cells read from the ASM FIFO are passed to the ANA block. '1': No data cells are passed to the ANA block. Only IDLE and REFRESH cells will be transmitted. Data is still read from the ASM FIFO, even though the cell bus interface has been disabled.
    pub fn cell_bus_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_cell_bus_dis(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// This field indicates if a given ASM FIFO must be reset or not. Resetting a FIFO does not affect the device status interface to the DSM.

    ///

    /// '0': The FIFO is NOT reset. '1': The FIFO is reset and it will remain reset until FIFO_RST is de-asserted.
    pub fn fifo_rst(&self) -> u32 {
        (self.0 & 0xff8) >> 3
    }
    pub fn set_fifo_rst(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0xff8);
        self.0 &= !0xff8;
        self.0 |= value;
    }
    /// If this fields is set to 1, cell cycles allocated for front ports, but with no data available, will be given to the virtual device. Otherwise the VD is only given the cycles set by the cell bus calendar.
    pub fn idle_to_vd(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_idle_to_vd(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}
/// Holds a number of sticky bits that are set if internal errors are detected.
///
/// Writing a '1' to a bit group clears that bit.
#[derive(From, Into)]
pub struct ERR_STICKY(u32);
impl ERR_STICKY {
    /// Cell words must only be granted a given Taxi bus every 3rd cell cycle or more. I.e. for Taxi A there must always be two or more cell slots given to another Taxi other than A or idle, before Taxi A is allowed to get the next grant. If the cell bus calendar causes 2 cell slots to be allocated the same Taxi bus within 3 cell cycles, the last cell slot is ignored and a sticky bit is asserted.

    ///

    /// '0': No cell slot calendar error detected. '1': One or more cell slots have been ignored by ASM. Bit is cleared by writing a '1' to this position.
    pub fn calendar_sticky(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_calendar_sticky(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// The ASM must assert a sticky bit if an internal error occurs in the 'complete cell' FIFO.

    ///

    /// '0': No internal error has been detected in the 'complete cell' FIFO. '1': An internal error has been detected in the 'complete cell' FIFO. Bit is cleared by writing a '1' to this position.
    pub fn cc_intrn_err_sticky(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_cc_intrn_err_sticky(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// The ASM must assert a sticky bit if an overflow occurs in the 'complete cell' FIFO.

    ///

    /// '0': No overflow has been detected in the 'complete cell' FIFO. '1': An overflow has been detected in the 'complete cell' FIFO. Bit is cleared by writing a '1' to this position.
    pub fn cc_oflw_sticky(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_cc_oflw_sticky(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// The ASM must assert a sticky bit if an underflow occurs in the 'complete cell' FIFO.

    ///

    /// '0': No underflow has been detected in the 'complete cell' FIFO. '1': An underflow has been detected in the 'complete cell' FIFO. Bit is cleared by writing a '1' to this position.
    pub fn cc_uflw_sticky(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_cc_uflw_sticky(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// The ASM must assert a sticky bit if an overflow occurs in the 'free cell' FIFO.

    ///

    /// '0': No overflow has been detected in the 'free cell' FIFO. '1': An overflow has been detected in the 'free cell' FIFO. Bit is cleared by writing a '1' to this position.
    pub fn fc_oflw_sticky(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_fc_oflw_sticky(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// The ASM must assert a sticky bit if an underflow occurs in the 'free cell' FIFO.

    ///

    /// '0': No underflow has been detected in the 'free cell' FIFO. '1': An underflow has been detected in the 'free cell' FIFO. Bit is cleared by writing a '1' to this position.
    pub fn fc_uflw_sticky(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_fc_uflw_sticky(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// This sticky bit is set if the ASM passes a frame to the cell bus, which is less than 64 bytes (before any padding is done, if enabled) - and has not been aborted or abortion has been disabled. The padding configuration does not affect this sticky bit.

    ///

    /// '0': No error detected. '1': One or more frames have been passed to the cell bus, where the frame size was less than 64 bytes. Bit is cleared by writing a '1' to this position.
    pub fn fragment_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_fragment_sticky(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// This sticky bit is set if the ASM receives a Taxi word where ABORT is asserted but EOF is not asserted.

    ///

    /// '0': No misaligned ABORT/EOF indications detected. '1': Misaligned ABORT/EOF indications detected. Bit is cleared by writing a '1' to this position.
    pub fn invld_abort_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_invld_abort_sticky(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// The ASM must assert a sticky bit if an overflow occurs in the main statemachine.

    ///

    /// '0': No overflow has been detected in the main statemachine. '1': An overflow has been detected in the main statemachine. Bit is cleared by writing a '1' to this position.
    pub fn main_sm_intrn_err_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_main_sm_intrn_err_sticky(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// The ASM must assert a sticky bit if an overflow occurs in the main statemachine.

    ///

    /// '0': No overflow has been detected in the main statemachine. '1': An overflow has been detected in the main statemachine. Bit is cleared by writing a '1' to this position.
    pub fn main_sm_oflw_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_main_sm_oflw_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// This sticky bit is set if the ASM receives a Taxi word where SOF is asserted and the previous valid Taxi word from that port did not hold an EOF.

    ///

    /// '0': No missing EOF detected '1': Missing EOF detected Bit is cleared by writing a '1' to this position.
    pub fn missing_eof_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_missing_eof_sticky(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// This sticky bit is set if the ASM receives a Taxi word with SOF=0 and the previous valid Taxi word from that port hold an EOF.

    ///

    /// '0': No missing EOF detected '1': Missing EOF detected Bit is cleared by writing a '1' to this position.
    pub fn missing_sof_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_missing_sof_sticky(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// This sticky bit is set if a partial Taxi word (unused_bytes <> 0) is received while EOF = 0.

    ///

    /// '0': No error detected in UNUSED_BYTES field ofTaxi word. '1': One or more Taxi words have been received where the UNUSED_BYTES field was different from 0 and EOP = 0. Bit is cleared by writing a '1' to this position.
    pub fn unused_bytes_sticky(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_unused_bytes_sticky(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
}
/// Configure custom VLAN tag for injection
#[derive(From, Into)]
pub struct INJ_VLAN_CFG(u32);
impl INJ_VLAN_CFG {
    /// The TPID used for VLAN tag matching when injection with long IFH prefix is selected in INJ_FORMAT_CFG.
    pub fn inj_tpid_cfg(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_inj_tpid_cfg(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// The VID used for VLAN tag matching when injection with long IFH prefix is selected in INJ_FORMAT_CFG.
    pub fn inj_vid_cfg(&self) -> u32 {
        (self.0 & 0xfff0000) >> 16
    }
    pub fn set_inj_vid_cfg(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xfff0000);
        self.0 &= !0xfff0000;
        self.0 |= value;
    }
}
