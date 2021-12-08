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

/// Register `MANUAL_CFG`
///
/// Manual extraction and injection configuration
#[derive(From, Into)]
pub struct MANUAL_CFG(u32);
impl MANUAL_CFG {    ///
    /// Set to enable manual injection by using FDMA channel number 9. When manual injection is enabled; the FDMA cannot be used for regular FDMA injection operations (on any injection channel).
    pub fn inj_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_inj_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Set to swap endianess of data injected to the MANUAL_INJ region. The manual injection status word is never swapped.
    pub fn inj_swap_ena(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_inj_swap_ena(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }    ///
    /// Set to enable manual extraction by using FDMA channel number 1. When manual extraction is enabled; the FDMA cannot be used for regular FDMA extraction operations (on any extraction channel).
    pub fn xtr_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_xtr_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }    ///
    /// Set to disable flushing/abort mechanism when manual extraction is enabled via ICPU_CFG::MANUAL_CFG.XTR_ENA. When manually extracting via 8051 SFR registers this field must be set.
    pub fn xtr_flush_dis(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_xtr_flush_dis(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }    ///
    /// Set to byte-swap endianess of data extracted from the MANUAL_XTR region. The manual extraction status word is never swapped.
    pub fn xtr_swap_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_xtr_swap_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
}

/// Register `MANUAL_INJ`
///
/// Manual injection replicated register-array
#[derive(From, Into)]
pub struct MANUAL_INJ(u32);
impl MANUAL_INJ {    ///
    /// Manual injection is done by writing to this block of registers. The manual injection status word is located at the first word-address in this block. Manual injection has to be enabled via ICPU_CFG::MANUAL_CFG.INJ_ENA.
    pub fn inj(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_inj(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `MANUAL_INTR`
///
/// Manual extraction and injection interrupt indications
#[derive(From, Into)]
pub struct MANUAL_INTR(u32);
impl MANUAL_INTR {    ///
    /// Set when there is room for more injection data-words in injection fifo.
    pub fn intr_inj_rdy(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_intr_inj_rdy(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Set when any extraction word is ready for extraction.
    pub fn intr_xtr_any_rdy(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_intr_xtr_any_rdy(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Set when there is an extraction word containing SOF ready for extraction.
    pub fn intr_xtr_sof_rdy(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_intr_xtr_sof_rdy(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `MANUAL_INTR_ENA`
///
/// Manual extraction and injection interrupt enables
#[derive(From, Into)]
pub struct MANUAL_INTR_ENA(u32);
impl MANUAL_INTR_ENA {    ///
    /// Set to enable FDMA interrupt while there is room for more injection data. This interrupt is asserted for as long as there is free space in the injection buffers.
    pub fn intr_inj_rdy_ena(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_intr_inj_rdy_ena(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }    ///
    /// Set to enable FDMA interrupt while any data is ready for manual extraction. This interrupt is asserted for as long as there is data ready in the extraction buffer.
    pub fn intr_xtr_any_rdy_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_intr_xtr_any_rdy_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    ///
    /// Set to enable FDMA interrupt when a new frame is waiting to be extracted. This event is asserted when a frame-word with sof set is waiting to be extracted. If a previous frame is only partially extracted then no interrupt will be generated until the previous frame is completely extracted.
    pub fn intr_xtr_sof_rdy_ena(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_intr_xtr_sof_rdy_ena(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}

/// Register `MANUAL_XTR`
///
/// Manual extraction replicated register-array
#[derive(From, Into)]
pub struct MANUAL_XTR(u32);
impl MANUAL_XTR {    ///
    /// Manual extraction is done by reading from this block of registers. The manual extraction status word is accessed by reading the last word-address in this block. Manual extraction has to be enabled via ICPU_CFG::MANUAL_CFG.XTR_ENA.
    pub fn xtr(&self) -> u32 {
        (self.0 & 0x0) >> 0
    }
    pub fn set_xtr(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x0);
        self.0 &= !0x0;
        self.0 |= value;
    }
}

/// Register `PCIE_INTR_STAT`
///
/// PCIe outbound MSI interrupt status
///
/// Replicated per EXT_DST interrupt.
#[derive(From, Into)]
pub struct PCIE_INTR_STAT(u32);
impl PCIE_INTR_STAT {    pub fn intr_pending_falling(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_intr_pending_falling(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }    pub fn intr_pending_rising(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_intr_pending_rising(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}