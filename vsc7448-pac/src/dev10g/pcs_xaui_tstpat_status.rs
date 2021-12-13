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
/// Test Pattern Receive Sequence Counter
///
/// Random Sequence Master Counter. The counter is used only if frame based pattern are to be checked (CRPAT, CJPAT) and it is incremented by one every 8th received symbol. The counter is started when a start of frame is detected and the counter stops when the last symbol of the (internally stored) reference frame was compared. The idle phase between two frames is not checked.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_TSTPAT_RX_SEQ_CNT_STATUS(u32);
impl PCS_XAUI_TSTPAT_RX_SEQ_CNT_STATUS {
    /// Random sequence master counter
    pub fn rnd_seq_timer(&self) -> u32 {
        self.0
    }
    pub fn set_rnd_seq_timer(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Test Pattern Transmit Sequence Counter
///
/// Jitter Pattern Transmit Counter. The counter counts the number of transmitted frames (only frame based pattern, i.e. CRPAT and CJPAT).
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct PCS_XAUI_TSTPAT_TX_SEQ_CNT_STATUS(u32);
impl PCS_XAUI_TSTPAT_TX_SEQ_CNT_STATUS {
    /// Jitter pattern transmit counter
    pub fn jp_tx_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_jp_tx_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
