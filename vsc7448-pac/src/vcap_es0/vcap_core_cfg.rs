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
/// Configuration for move/initialization
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VCAP_MV_CFG(u32);
impl VCAP_MV_CFG {
    /// Specifies the distance during move operations. I.e. if this field is set to 4 for a move-down operation, then source address n is moved to destination address n+5.
    ///
    /// 0: Distance is one position 1: Distance is two positions n: Distance is n+1 positions
    #[inline(always)]
    pub fn mv_num_pos(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    #[inline(always)]
    pub fn set_mv_num_pos(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 16;
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    /// Specifies the number of addresses to move/initialize during	move/init operations.
    ///
    /// 0: Address VCAP_UPDATE_CTRL.UPDATE_ADDR is moved/initialized n: Addresses VCAP_UPDATE_CTRL.UPDATE_ADDR through VCAP_UPDATE_CTRL.UPDATE_ADDR+n are moved/initialized
    #[inline(always)]
    pub fn mv_size(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_mv_size(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Initiation of read/write/move/initialization operations
///
/// Operations on the VCAP cache is done via this register. The UPDATE_CMD field specifies the operation to perform when UPDATE_SHOT is set. For all of the operations it is possible to disable read/write of entries, actions, and/or counter by setting VCAP_UPDATE_CTRL.UPDATE_ENTRY_DIS, VCAP_UPDATE_CTRL.UPDATE_ACTION_DIS, and/or VCAP_UPDATE_CTRL.UPDATE_CNT_DIS respectively. Writing/moving to unimplemented addresses are ignored. Reading/moving from unimplemented addresses returns never-match for entries, and zeros from actions/counters. Active rules may only be written to empty (initialized) addresses. Software must not overwrite active rules (unless when initializing rules). To initialize a region of addresses use the init operation with CLEAR_CACHE bits set to '1'. Move operations automatically disable rules when moved; so it is OK when source and destination ranges overlap.
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct VCAP_UPDATE_CTRL(u32);
impl VCAP_UPDATE_CTRL {
    /// Set to clear the cache. This field is cleared immediately by hardware (at the same time as clearing the cache). The contents of the cache will be set to disabled/empty.
    #[inline(always)]
    pub fn clear_cache(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_clear_cache(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// For version 1 VCAPs: Set to ignore interrupting traffic during move operations, this will increase speed of the move operations but counter-events may be lost for the VCAP addresses that are moved. When this field is cleared, then interrupting traffic will cause a restart of the move operation (to ensure consistent counter values) and becasue of this, move operations on a heavily loaded device may take a long time to finish. This field is not used for version 2 VCAPs, moving of counters are safe.
    #[inline(always)]
    pub fn mv_traffic_ign(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_mv_traffic_ign(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set to disable update of actions for VCAP operations: For read-operations action-cache will remain unchanged. For write/move/init operations the VCAP-action will remain unchanged.
    #[inline(always)]
    pub fn update_action_dis(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_update_action_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    /// The address to access for VCAP operations.
    ///
    /// ES0 defaults at 0x1000-0x1034
    #[inline(always)]
    pub fn update_addr(&self) -> u32 {
        (self.0 & 0x7fff8) >> 3
    }
    #[inline(always)]
    pub fn set_update_addr(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 3;
        self.0 &= !0x7fff8;
        self.0 |= value;
    }
    /// Write and read operations access VCAP memory at address specified by UPDATE_ADDR. Move up opeation moves one or more VCAP addresses from a high address to a lower address, this is equivalent to decreasing priority of a rule. The starting address is specified by UPDATE_ADDR, the number of addresses (the range) that is moved is defined by VCAP_MV_CFG.MV_SIZE, the distance to move is defined by VCAP_MV_CFG. MV_NUM_POS. Move down opeation moves one or more VCAP addresses from a low address to a higer address, this is equivalent to increasing priority of a rule. This operation is equivalent to "Move up" except for the direction that it moves addresses, see "Move up" for more details. Init operation writes the contents of the cache to one or more VCAP addresses. The starting address is specified by UPDATE_ADDR, the number of addresses (the range) that is written is defined by VCAP_MV_CFG.MV_SIZE. Setting CLEAR_CACHE at the same time as starting the operation will clear the cache and cause the init operation to initialize the range of addresses.
    ///
    /// 000: Write from cache to VCAP 001: Read from VCAP to cache 010: Move entry and/or action up (decreasing addresses) 011: Move entry and/or action down (increasing addresses) 100: Initialize VCAP with the cache-value
    #[inline(always)]
    pub fn update_cmd(&self) -> u32 {
        (self.0 & 0x1c00000) >> 22
    }
    #[inline(always)]
    pub fn set_update_cmd(&mut self, value: u32) {
        assert!(value <= 0x7);
        let value = value << 22;
        self.0 &= !0x1c00000;
        self.0 |= value;
    }
    /// Set to disable update of counter for VCAP operations: For read-operations counter-cache will remain unchanged. For write/init operations the VCAP-counter will remain unchanged. For move operations the destination VCAP-counters will be set to zeros.
    #[inline(always)]
    pub fn update_cnt_dis(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    #[inline(always)]
    pub fn set_update_cnt_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Set to disable update of entries for VCAP operations: For read-operations entry-cache will remain unchanged. For write/move/init operations the VCAP-entry will remain unchanged.
    #[inline(always)]
    pub fn update_entry_dis(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline(always)]
    pub fn set_update_entry_dis(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    /// Set to initiate the opeation specified in UPDATE_CMD. This bit is automatically cleared by hardware when the operation is finished. Software must not change write fields in the VCAP target while this field is set (while operation is active.)
    #[inline(always)]
    pub fn update_shot(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_update_shot(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
}
