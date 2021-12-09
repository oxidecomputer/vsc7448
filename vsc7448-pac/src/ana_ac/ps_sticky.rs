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
/// Miscellaneous PGID configuration
#[derive(From, Into)]
pub struct PGID_MISC_CFG(u32);
impl PGID_MISC_CFG {
    /// Copy frame to CPU. Related parameters: ANA_AC:PGID:PGID_MISC_CFG.PGID_CPU_QU
    pub fn pgid_cpu_copy_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pgid_cpu_copy_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// CPU queue for frames copied to CPU due to PGID_CPU_COPY_ENA. Related parameters: ANA_AC:PGID:PGID_MISC_CFG.PGID_CPU_COPY_ENA

    ///

    /// 0: CPU queue 0 1: CPU queue 1 ... n: CPU queue n.
    pub fn pgid_cpu_qu(&self) -> u32 {
        (self.0 & 0x70) >> 4
    }
    pub fn set_pgid_cpu_qu(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x70);
        self.0 &= !0x70;
        self.0 |= value;
    }
    /// Encoding of PGID_CFG.PORT_MASK. Related parameters: ANA_AC:PGID[0-1076]:PGID_CFG.PORT_MASK

    ///

    /// 0: Port mask encoding 1: Stack forwarding encoding
    pub fn stack_type_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_stack_type_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
}
