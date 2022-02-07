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
/// Watermark configuration
///
/// The queue system tracks four resource consumptions: Resource 0: Memory tracked per source Resource 1: Frame references tracked per source Resource 2: Memory tracked per destination Resource 3: Frame references tracked per destination Before a frame is added to the queue system, some conditions must be met: - Reserved memory for the specific (SRC, PRIO) or for the specific SRC is available OR - Reserved memory for the specific (DST,PRIO) or for the specific DST is available OR - Shared memory is available The frame reference resources are checked for availability like the memory resources. Enqueuing of a frame is allowed if both the memory resource check and the frame reference resource check succeed. The extra resources consumed when enqueuing a frame are first taken from the reserved (SRC,PRIO), next from the reserved SRC, and last from the shared memory area. The same is done for DST. Both memory consumptions and frame reference consumptions are updated. The register is layed out the following way for source memory (resource 0): Index 0-xxx: Reserved amount for (SRC,PRIO) at index 8*SRC+PRIO Index 496-503: Maximum allowed use of the shared buffer for PRIO at index PRIO+496 Index 512-568: Reserved amount for SRC at index SRC+512. Index 510: Maximum allowed use of the shared buffer for frames with DP=1. Index 511: Maximum allowed use of the shared buffer for source. The layout is similar for resources 1, 2, and 3. Resource 1 uses indices 1024-2047, resource 2 uses indices 2048-3071, and resource 3 uses indices 3072-4095. The allocation size for memory tracking is 176 bytes. All frames are prepended with a 16-byte header.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RES_CFG(u32);
impl RES_CFG {
    /// Watermark for resource. Note, the default value depends on the index. Refer to the congestion scheme documentation for details.
    ///
    /// Bit 10:   Unit; 0:1, 1:16 Bits 9-0: Value to be multiplied with unit
    #[inline(always)]
    pub fn wm_high(&self) -> u32 {
        self.0 & 0xfff
    }
    #[inline(always)]
    pub fn set_wm_high(&mut self, value: u32) {
        assert!(value <= 0xfff);
        self.0 &= !0xfff;
        self.0 |= value;
    }
}
/// Resource status
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RES_STAT(u32);
impl RES_STAT {
    /// Maximum consumption since last read for corresponding watermark in RES_CFG.
    #[inline(always)]
    pub fn maxuse(&self) -> u32 {
        self.0 & 0x1fffff
    }
    #[inline(always)]
    pub fn set_maxuse(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
/// Resource status
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct RES_STAT_CUR(u32);
impl RES_STAT_CUR {
    /// Current consumption for corresponding watermark in RES_CFG.
    #[inline(always)]
    pub fn inuse(&self) -> u32 {
        self.0 & 0x1fffff
    }
    #[inline(always)]
    pub fn set_inuse(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
