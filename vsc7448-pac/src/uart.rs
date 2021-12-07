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

use crate::types::RegisterAddress;

// Register groups are stored in the tree as submodules

pub mod uart;

/// UART
pub struct UART(pub(super) u32);
impl UART {
    pub fn HTX(&self) -> RegisterAddress<uart::HTX> {
        RegisterAddress::new(self.0 + 0xa4)
    }
    pub fn IER(&self) -> RegisterAddress<uart::IER> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn IIR_FCR(&self) -> RegisterAddress<uart::IIR_FCR> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn LCR(&self) -> RegisterAddress<uart::LCR> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn LSR(&self) -> RegisterAddress<uart::LSR> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn MCR(&self) -> RegisterAddress<uart::MCR> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn MSR(&self) -> RegisterAddress<uart::MSR> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn RBR_THR(&self) -> RegisterAddress<uart::RBR_THR> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn RESERVED1(&self, index: u32) -> RegisterAddress<uart::RESERVED1> {
        assert!(index < 23);
        RegisterAddress::new(self.0 + 0x20 + index * 0x4)
    }
    pub fn RESERVED2(&self, index: u32) -> RegisterAddress<uart::RESERVED2> {
        assert!(index < 9);
        RegisterAddress::new(self.0 + 0x80 + index * 0x4)
    }
    pub fn SCR(&self) -> RegisterAddress<uart::SCR> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn USR(&self) -> RegisterAddress<uart::USR> {
        RegisterAddress::new(self.0 + 0x7c)
    }
}
