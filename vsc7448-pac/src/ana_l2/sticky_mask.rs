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

/// Register `STICKY`
///
/// Sticky diagnostic status
#[derive(From, Into)]
pub struct STICKY(u32);
impl STICKY {
    /// Set if an autonomous learning operation has failed due to specified lrn rate is exceeded and LEARN max cnt was enabled. Write '1' to clear this field.

    ///

    /// 0: No cnt exceeded. 1: An autonomous learning operation has failed due to cnt exceeded. Write '1' to clear this field.
    pub fn auto_lrn_rate_exceed_sticky(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_auto_lrn_rate_exceed_sticky(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Set if a frame has been dropped due to ANA_L2::LRN_SECUR_CFG.DROP_UNKNOWN_SRC_ENA. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No frames have been dropped 1: A frame has been dropped. Bit is cleared by writing a 1 to this position.
    pub fn drop_unknown_src_sticky(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_drop_unknown_src_sticky(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set if a valid (EFID, DMAC) entry was found to be used for forwarding. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No (EFID, DMAC) entries have been found 1: An (EFID, DMAC) entry has been found. Bit is cleared by writing a 1 to this position.
    pub fn fwd_entry_found_sticky(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_fwd_entry_found_sticky(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Set if flood has been removed due to indication from VLAN table. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: An integrity error has occured Bit is cleared by writing a 1 to this position.
    pub fn fwd_flood_kill_sticky(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_fwd_flood_kill_sticky(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Set if forwarding is based on flood. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0:  No flood event 1: Flood event Bit is cleared by writing a 1 to this position.
    pub fn fwd_flood_sticky(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_fwd_flood_sticky(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Indication of a frame received on a GLAG which was previously learned with a different GLAG. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: A GLAG to GLAG port move has been detected Bit is cleared by writing a 1 to this position.
    pub fn global_to_global_portmove_sticky(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_global_to_global_portmove_sticky(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Indication of a frame received at a front local port which was previously learned with a GLAG . To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: A GLAG to local port move has been detected Bit is cleared by writing a 1 to this position.
    pub fn global_to_local_portmove_sticky(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_global_to_local_portmove_sticky(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Set if a valid (IFID, SMAC) entry was found. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No known source entries (IFID, SMAC) has been found 1: At least one known entry (IFID, SMAC) has been found. Write 1 to clear this field.
    pub fn learn_known_sticky(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_learn_known_sticky(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Set if no valid (IFID, SMAC) entry was found. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No unknown sources (IFID, SMAC) has been found 1: At least one unknown source IFID, SMAC) has been found. Write 1 to clear this field.
    pub fn learn_unknown_sticky(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_learn_unknown_sticky(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Indication of a frame received with GLAG which was previously learned at a front local port. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: A Local to GLAG port move has been detected Bit is cleared by writing a 1 to this position.
    pub fn local_to_global_portmove_sticky(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_local_to_global_portmove_sticky(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Indication of a frame received on a local front port, which was previously learned at a different local front port. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: A Local to local port move has been detected Bit is cleared by writing a 1 to this position.
    pub fn local_to_local_portmove_sticky(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_local_to_local_portmove_sticky(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Indication of a frame received with an UPSID != own UPSID (ANA_L2::VSTAX_CTRL.OWN_UPSID) which was previously learned on own front local port. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: A Local to remote port move has been detected Bit is cleared by writing a 1 to this position.
    pub fn local_to_remote_portmove_sticky(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_local_to_remote_portmove_sticky(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Indication of a frame received on a local front port, which was previously learned on a remote UPSID front port. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: A remote port to local port move has been detected Bit is cleared by writing a 1 to this position.
    pub fn remote_to_local_portmove_sticky(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_remote_to_local_portmove_sticky(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Indication of a frame received on a remote front port, which was previously learned on another remote UPSID front port. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: A remote port to remote port move has been detected Bit is cleared by writing a 1 to this position.
    pub fn remote_to_remote_portmove_sticky(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_remote_to_remote_portmove_sticky(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Set if forwarding is performed without SRC contribution either because of a known Destination entry with VLAN ignore set (LRN.MAC_ACCESS_CFG_2.MAC_ENTRY_VLAN_IGNORE FLAG) set or for an unknown destination with flood VLAN ignore set (ANA_L2::FWD_CFG.FLOOD_IGNORE_VLAN_ENA) and filter_mode_sel set to SRC ignore. To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: VLAN ignore as occured Bit is cleared by writing a 1 to this position.
    pub fn src_ignore_sticky(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_src_ignore_sticky(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Set if forwarding is performed without VLAN contribution either because of a known Destination entry with VLAN ignore set (LRN.MAC_ACCESS_CFG_2.MAC_ENTRY_VLAN_IGNORE FLAG) set or for an unknown destination with flood VLAN ignore set (ANA_L2::FWD_CFG.FLOOD_IGNORE_VLAN_ENA). To enable the event as one of four counter events to the PORT STAT block set the corresponding *_STICKY_MASK

    ///

    /// 0: No event has occured 1: VLAN ignore as occured Bit is cleared by writing a 1 to this position.
    pub fn vlan_ignore_sticky(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_vlan_ignore_sticky(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
}

/// Register `STICKY_MASK`
///
/// Sticky diagnostic counter mask
#[derive(From, Into)]
pub struct STICKY_MASK(u32);
impl STICKY_MASK {
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn auto_lrn_rate_exceed_sticky_mask(&self) -> u32 {
        (self.0 & 0x8000) >> 15
    }
    pub fn set_auto_lrn_rate_exceed_sticky_mask(&mut self, value: u32) {
        let value = value << 15;
        assert!(value <= 0x8000);
        self.0 &= !0x8000;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn drop_unknown_src_sticky_mask(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_drop_unknown_src_sticky_mask(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn fwd_entry_found_sticky_mask(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    pub fn set_fwd_entry_found_sticky_mask(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x8);
        self.0 &= !0x8;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn fwd_flood_kill_sticky_mask(&self) -> u32 {
        (self.0 & 0x4000) >> 14
    }
    pub fn set_fwd_flood_kill_sticky_mask(&mut self, value: u32) {
        let value = value << 14;
        assert!(value <= 0x4000);
        self.0 &= !0x4000;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn fwd_flood_sticky_mask(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    pub fn set_fwd_flood_sticky_mask(&mut self, value: u32) {
        let value = value << 4;
        assert!(value <= 0x10);
        self.0 &= !0x10;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn global_to_global_portmove_sticky_mask(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    pub fn set_global_to_global_portmove_sticky_mask(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0x100);
        self.0 &= !0x100;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn global_to_local_portmove_sticky_mask(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_global_to_local_portmove_sticky_mask(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn learn_known_sticky_mask(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_learn_known_sticky_mask(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn learn_unknown_sticky_mask(&self) -> u32 {
        (self.0 & 0x1) >> 0
    }
    pub fn set_learn_unknown_sticky_mask(&mut self, value: u32) {
        let value = value << 0;
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn local_to_global_portmove_sticky_mask(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_local_to_global_portmove_sticky_mask(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn local_to_local_portmove_sticky_mask(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    pub fn set_local_to_local_portmove_sticky_mask(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x20);
        self.0 &= !0x20;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn local_to_remote_portmove_sticky_mask(&self) -> u32 {
        (self.0 & 0x800) >> 11
    }
    pub fn set_local_to_remote_portmove_sticky_mask(&mut self, value: u32) {
        let value = value << 11;
        assert!(value <= 0x800);
        self.0 &= !0x800;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn remote_to_local_portmove_sticky_mask(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    pub fn set_remote_to_local_portmove_sticky_mask(&mut self, value: u32) {
        let value = value << 10;
        assert!(value <= 0x400);
        self.0 &= !0x400;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn remote_to_remote_portmove_sticky_mask(&self) -> u32 {
        (self.0 & 0x200) >> 9
    }
    pub fn set_remote_to_remote_portmove_sticky_mask(&mut self, value: u32) {
        let value = value << 9;
        assert!(value <= 0x200);
        self.0 &= !0x200;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn src_ignore_sticky_mask(&self) -> u32 {
        (self.0 & 0x1000) >> 12
    }
    pub fn set_src_ignore_sticky_mask(&mut self, value: u32) {
        let value = value << 12;
        assert!(value <= 0x1000);
        self.0 &= !0x1000;
        self.0 |= value;
    }
    /// Mask count of sticky event.

    ///

    /// 0: Disable event count 1: Enable event count
    pub fn vlan_ignore_sticky_mask(&self) -> u32 {
        (self.0 & 0x2000) >> 13
    }
    pub fn set_vlan_ignore_sticky_mask(&mut self, value: u32) {
        let value = value << 13;
        assert!(value <= 0x2000);
        self.0 &= !0x2000;
        self.0 |= value;
    }
}
