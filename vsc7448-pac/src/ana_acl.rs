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

pub mod cnt_tbl;
pub mod port;
pub mod ptp_dom;
pub mod sticky;
pub mod vcap_s2;

/// VCAP_IS2 counter table
pub struct CNT_TBL(pub(super) u32);
impl CNT_TBL {
    #[inline(always)]
    pub fn CNT(&self) -> RegisterAddress<cnt_tbl::CNT> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// VCAP_IS2 configuration per port
pub struct PORT(pub(super) u32);
impl PORT {
    #[inline(always)]
    pub fn PTP_CFG(&self) -> RegisterAddress<port::PTP_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    #[inline(always)]
    pub fn VCAP_S2_KEY_SEL(&self, index: u8) -> RegisterAddress<port::VCAP_S2_KEY_SEL> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
}

/// PTP domain configuration
pub struct PTP_DOM(pub(super) u32);
impl PTP_DOM {
    #[inline(always)]
    pub fn PTP_CLOCK_ID_LSB(&self) -> RegisterAddress<ptp_dom::PTP_CLOCK_ID_LSB> {
        RegisterAddress::new(self.0 + 0x4)
    }
    #[inline(always)]
    pub fn PTP_CLOCK_ID_MSB(&self) -> RegisterAddress<ptp_dom::PTP_CLOCK_ID_MSB> {
        RegisterAddress::new(self.0 + 0x0)
    }
    #[inline(always)]
    pub fn PTP_MISC_CFG(&self) -> RegisterAddress<ptp_dom::PTP_MISC_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    #[inline(always)]
    pub fn PTP_SRC_PORT_CFG(&self) -> RegisterAddress<ptp_dom::PTP_SRC_PORT_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// Sticky diagnostic status
pub struct STICKY(pub(super) u32);
impl STICKY {
    #[inline(always)]
    pub fn SEC_LOOKUP_STICKY(&self, index: u8) -> RegisterAddress<sticky::SEC_LOOKUP_STICKY> {
        debug_assert!(index < 2);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
}

/// Common configurations used by VCAP_IS2
pub struct VCAP_S2(pub(super) u32);
impl VCAP_S2 {
    #[inline(always)]
    pub fn PTP_MISC_CTRL(&self) -> RegisterAddress<vcap_s2::PTP_MISC_CTRL> {
        RegisterAddress::new(self.0 + 0x164)
    }
    #[inline(always)]
    pub fn SWAP_IP_CTRL(&self) -> RegisterAddress<vcap_s2::SWAP_IP_CTRL> {
        RegisterAddress::new(self.0 + 0x168)
    }
    #[inline(always)]
    pub fn SWAP_SIP(&self, index: u8) -> RegisterAddress<vcap_s2::SWAP_SIP> {
        debug_assert!(index < 32);
        RegisterAddress::new(self.0 + 0xe4 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn VCAP_S2_CFG(&self, index: u8) -> RegisterAddress<vcap_s2::VCAP_S2_CFG> {
        debug_assert!(index < 57);
        RegisterAddress::new(self.0 + 0x0 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn VCAP_S2_MISC_CTRL(&self) -> RegisterAddress<vcap_s2::VCAP_S2_MISC_CTRL> {
        RegisterAddress::new(self.0 + 0x16c)
    }
    #[inline(always)]
    pub fn VCAP_S2_RNG_CTRL(&self, index: u8) -> RegisterAddress<vcap_s2::VCAP_S2_RNG_CTRL> {
        debug_assert!(index < 8);
        RegisterAddress::new(self.0 + 0x170 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn VCAP_S2_RNG_OFFSET_CFG(&self) -> RegisterAddress<vcap_s2::VCAP_S2_RNG_OFFSET_CFG> {
        RegisterAddress::new(self.0 + 0x1b0)
    }
    #[inline(always)]
    pub fn VCAP_S2_RNG_VALUE_CFG(
        &self,
        index: u8,
    ) -> RegisterAddress<vcap_s2::VCAP_S2_RNG_VALUE_CFG> {
        debug_assert!(index < 8);
        RegisterAddress::new(self.0 + 0x190 + u32::from(index) * 0x4)
    }
    #[inline(always)]
    pub fn VOE_LOOPBACK_CFG(&self) -> RegisterAddress<vcap_s2::VOE_LOOPBACK_CFG> {
        RegisterAddress::new(self.0 + 0x1b4)
    }
}
