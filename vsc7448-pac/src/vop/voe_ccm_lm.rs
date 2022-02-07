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
/// CCM-LM Rx LM sample value.
///
/// This is the value of the Rx LM counter sampled when the latest CCM-LM frame was received. This value must be transmitted as CCM-LM.RX_FCB in the next CCM-LM frame transmitted by this VOE.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CCM_RX_FCB_CFG(u32);
impl CCM_RX_FCB_CFG {
    /// See register description.
    #[inline(always)]
    pub fn ccm_rx_fcb(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_ccm_rx_fcb(&mut self, value: u32) {
        self.0 = value;
    }
}
/// CCM-LM Tx sample value.
///
/// Value of the CCM-LM.TX_FC_F field in the lastest received valid CCM-LM frame by this VOE. This value must be transmitted as CCM-LM.TX_FCB in the next CCM-LM frame transmitted by this VOE.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CCM_TX_FCB_CFG(u32);
impl CCM_TX_FCB_CFG {
    /// See register description.
    #[inline(always)]
    pub fn ccm_tx_fcb(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_ccm_tx_fcb(&mut self, value: u32) {
        self.0 = value;
    }
}
