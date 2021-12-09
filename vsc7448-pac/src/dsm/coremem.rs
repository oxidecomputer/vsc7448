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
/// Address selection
#[derive(From, Into)]
pub struct CM_ADDR(u32);
impl CM_ADDR {
    /// Address selection within selected core memory (CMID register). Address is automatically advanced at every data access.
    pub fn cm_addr(&self) -> u32 {
        self.0 & 0x3fffff
    }
    pub fn set_cm_addr(&mut self, value: u32) {
        assert!(value <= 0x3fffff);
        self.0 &= !0x3fffff;
        self.0 |= value;
    }
    /// Please refer to cmid.xls in the AS1000, misc_docs folder.
    pub fn cm_id(&self) -> u32 {
        (self.0 & 0x3fc00000) >> 22
    }
    pub fn set_cm_id(&mut self, value: u32) {
        let value = value << 22;
        assert!(value <= 0x3fc00000);
        self.0 &= !0x3fc00000;
        self.0 |= value;
    }
}
/// Core reset control
///
/// Controls reset and initialization of the switching core. Proper startup sequence is: - Enable memories - Initialize memories - Enable core
#[derive(From, Into)]
pub struct RAM_INIT(u32);
impl RAM_INIT {
    /// Core memory controllers are enabled when this field is set.
    pub fn ram_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_ram_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Initialize core memories. Field is automatically cleared when operation is complete ( approx. 40 us).
    pub fn ram_init(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_ram_init(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
