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
/// Header size configuration register for txRateLimitPayloadRate mode
///
/// Header size configuration register for txRateLimitPayloadRate mode
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TX_RATE_LIMIT_HDR_CFG(u32);
impl TX_RATE_LIMIT_HDR_CFG {
    /// Defines how much of the frame is seen as header and not counted as payload in txRateLimitPayloadRate and txRateLimitFrameRate mode when TX_RATE_LIMIT_MODE::PAYLOAD_CFG is set to 1.
    ///
    /// 0: 32 bytes are counted as header 1-31: 1-31 bytes are counted as header
    pub fn tx_rate_limit_hdr_size(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_tx_rate_limit_hdr_size(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}
/// Sticky bit register for rate limit modes
///
/// Sticky bit register for rate limit modes
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct TX_RATE_LIMIT_STICKY(u32);
impl TX_RATE_LIMIT_STICKY {
    /// Sticky bit set when one of the three tx rate limitation modes has increased the IPG. I.e. when a tx rate limit mode was enabled an DSM has requested the device to use a IPG different from 12 byte.
    ///
    /// '0': Tx Rate Limitation has not occurred. '1': Tx Rate Limitation has occurred. Bit is cleared by writing a '1' to this position.
    pub fn tx_rate_limit_sticky(&self) -> u32 {
        self.0
    }
    pub fn set_tx_rate_limit_sticky(&mut self, value: u32) {
        self.0 = value;
    }
}
