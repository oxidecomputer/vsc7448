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
/// Injection control
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct INJ_CTRL(u32);
impl INJ_CTRL {
    /// Set to abort the current frame.
    pub fn abort(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    pub fn set_abort(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0x100000);
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// Set to indicate that the next data written to DEVCPU_Qs::INJ_WR is end-of-frame. At the same time as setting this field, also set DEVCPU_QS::INJ_CTRL.VLD_BYTES to indicate the number of valid data bytes in the end-of-frame word.
    pub fn eof(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_eof(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Controls the min-spacing from EOF to SOF on injected frames, the default value emulates the delay of standard preamble/IFG setting on a front-port. Set this field to zero when injecting with IFH.
    pub fn gap_size(&self) -> u32 {
        (self.0 & 0x1e00000) >> 21
    }
    pub fn set_gap_size(&mut self, value: u32) {
        let value = value << 21;
        assert!(value <= 0x1e00000);
        self.0 &= !0x1e00000;
        self.0 |= value;
    }
    /// Set to indicate that the next data written to DEVCPU_QS::INJ_WR is start-of-frame.
    pub fn sof(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_sof(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Set to indicate how many bytes of the next data written to DEVCPU_QS::INJ_WR which are valid. This field is only used during end-of-frame words (see DEVCPU_QS::INJ_CTRL.EOF for more information). The position of the valid bytes follows the endianness encoding and swapping.
    ///
    /// 0: All bytes are valid n: 'n' byte are valid
    pub fn vld_bytes(&self) -> u32 {
        (self.0 & 0x30000) >> 16
    }
    pub fn set_vld_bytes(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x30000);
        self.0 &= !0x30000;
        self.0 |= value;
    }
}
/// Injection errors
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct INJ_ERR(u32);
impl INJ_ERR {
    /// Set if a frame has been aborted because of double-SOF injection (missing EOF).
    pub fn abort_err_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_abort_err_sticky(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set in case of overflow as a result of not obeying FIFO-ready
    pub fn wr_err_sticky(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_wr_err_sticky(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Injection group configuration
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct INJ_GRP_CFG(u32);
impl INJ_GRP_CFG {
    /// This field allows swapping the endianess of the DEVCPU_QS::INJ_WR register. Most software will want to write injection data in network order (big-endian mode), i.e. the first byte of the destiantion MAC address to be placed on byte-address 0 of DEVCPU_QS::INJ_WR. In order to do this a little endian CPU must set this field, a big endian CPU must clear this field. This field only applies to manual extraction mode (see DEVCPU_QS::INJ_GRP_CFG.MODE).
    ///
    /// 0: Same endianess as CPU 1: Swap endianness
    pub fn byte_swap(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_byte_swap(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Configures mode of the injection group. Each injection group can be assigned to one of three owners. Note: The VRAP block support only one context, if more than one injection group is assigned the lowest group-number will be used.
    ///
    /// 0: VRAP block 1: Manual injection (via DEVCPU_QS registers) 2: FDMA injection and manual injection via SBA registers
    pub fn mode(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_mode(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
}
/// Injection status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct INJ_STATUS(u32);
impl INJ_STATUS {
    /// When '1' the injector group's FIFO is ready for additional data written through the DEVCPU_QS::INJ_WR register.
    ///
    /// 0: The injector group cannot accept additional data 1: The injector group is able to accept additional data
    pub fn fifo_rdy(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    pub fn set_fifo_rdy(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xc);
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// When '1' the injector group is in the process of receiving a frame, and at least one write to INJ_WR remains before the frame is forwarded to the front ports. When '0' the injector group is waiting for an initiation of a frame injection.
    ///
    /// 0: A frame injection is not in progress 1: A frame injection is in progress
    pub fn inj_in_progress(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_inj_in_progress(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Before the CPU injects a frame, software may check if the input queue has reached high watermark. If wathermark in the IQS has been reached this bit will be set.
    ///
    /// 0: Input queue has not reached high watermark 1: Input queue has reached high watermark, and frames injected may be dropped due to buffer overflow
    pub fn wmark_reached(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    pub fn set_wmark_reached(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x30);
        self.0 &= !0x30;
        self.0 |= value;
    }
}
/// Manual injection data
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct INJ_WR(u32);
impl INJ_WR {
    /// Frame Write. Write to this register inject  the next 32 bits of the frame data currently injected into the chip. Reading from this register returns 0.
    pub fn data(&self) -> u32 {
        self.0
    }
    pub fn set_data(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Injection debug
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct VTSS_DBG(u32);
impl VTSS_DBG {
    /// For debugging purpose, frames injected are counted with a small wrapping counter.
    ///
    /// 0: No frames has been injected 1: 1 frame has been injected ... 15: 15 frames have been injected
    pub fn frm_cnt(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_frm_cnt(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// General extraction configuration and status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct XTR_CFG(u32);
impl XTR_CFG {
    /// Watermark, when filling of extraction FIFO exceeds this (or EOF is present in the xtraction buffer) the DEVCPU_QS::XTR_DATA_PRESENT register will indicate that data is available.
    pub fn dp_wm(&self) -> u32 {
        (self.0 & 0x1c) >> 2
    }
    pub fn set_dp_wm(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x1c);
        self.0 &= !0x1c;
        self.0 |= value;
    }
    /// Overflow in extraction FIFO. If this happens, SCH_WM must be decreased.
    ///
    /// 0: No buffer overruns detected 1: Buffer has overrruned at least once
    pub fn oflw_err_sticky(&self) -> u32 {
        self.0 & 0x3
    }
    pub fn set_oflw_err_sticky(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
}
