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
/// Word offset 4
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BAR0(u32);
impl BAR0 {
    #[inline(always)]
    pub fn bar0_mem_io(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bar0_mem_io(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar0_prefetch(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_bar0_prefetch(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar0_start(&self) -> u32 {
        (self.0 & 0xfffffff0) >> 4
    }
    #[inline(always)]
    pub fn set_bar0_start(&mut self, value: u32) {
        assert!(value <= 0xfffffff);
        let value = value << 4;
        self.0 &= !0xfffffff0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar0_type(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_bar0_type(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
}
/// Word offset 5
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BAR1(u32);
impl BAR1 {
    #[inline(always)]
    pub fn bar1_mem_io(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bar1_mem_io(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar1_prefetch(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_bar1_prefetch(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar1_start(&self) -> u32 {
        (self.0 & 0xfffffff0) >> 4
    }
    #[inline(always)]
    pub fn set_bar1_start(&mut self, value: u32) {
        assert!(value <= 0xfffffff);
        let value = value << 4;
        self.0 &= !0xfffffff0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar1_type(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_bar1_type(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
}
/// Word offset 6
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BAR2(u32);
impl BAR2 {
    #[inline(always)]
    pub fn bar2_mem_io(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bar2_mem_io(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar2_prefetch(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_bar2_prefetch(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar2_start(&self) -> u32 {
        (self.0 & 0xfffffff0) >> 4
    }
    #[inline(always)]
    pub fn set_bar2_start(&mut self, value: u32) {
        assert!(value <= 0xfffffff);
        let value = value << 4;
        self.0 &= !0xfffffff0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar2_type(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_bar2_type(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
}
/// Word offset 7
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BAR3(u32);
impl BAR3 {
    #[inline(always)]
    pub fn bar3_mem_io(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bar3_mem_io(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar3_prefetch(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_bar3_prefetch(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar3_start(&self) -> u32 {
        (self.0 & 0xfffffff0) >> 4
    }
    #[inline(always)]
    pub fn set_bar3_start(&mut self, value: u32) {
        assert!(value <= 0xfffffff);
        let value = value << 4;
        self.0 &= !0xfffffff0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar3_type(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_bar3_type(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
}
/// Word offset 8
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BAR4(u32);
impl BAR4 {
    #[inline(always)]
    pub fn bar4_mem_io(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bar4_mem_io(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar4_prefetch(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_bar4_prefetch(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar4_start(&self) -> u32 {
        (self.0 & 0xfffffff0) >> 4
    }
    #[inline(always)]
    pub fn set_bar4_start(&mut self, value: u32) {
        assert!(value <= 0xfffffff);
        let value = value << 4;
        self.0 &= !0xfffffff0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar4_type(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_bar4_type(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
}
/// Word offset 9
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BAR5(u32);
impl BAR5 {
    #[inline(always)]
    pub fn bar5_mem_io(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_bar5_mem_io(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar5_prefetch(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_bar5_prefetch(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar5_start(&self) -> u32 {
        (self.0 & 0xfffffff0) >> 4
    }
    #[inline(always)]
    pub fn set_bar5_start(&mut self, value: u32) {
        assert!(value <= 0xfffffff);
        let value = value << 4;
        self.0 &= !0xfffffff0;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn bar5_type(&self) -> u32 {
        (self.0 & 0x6) >> 1
    }
    #[inline(always)]
    pub fn set_bar5_type(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 1;
        self.0 &= !0x6;
        self.0 |= value;
    }
}
/// Word offset 3
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct BIST_HEADER_TYPE_LATENCY_CACHE_LINE_SIZE(u32);
impl BIST_HEADER_TYPE_LATENCY_CACHE_LINE_SIZE {
    #[inline(always)]
    pub fn bist(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    #[inline(always)]
    pub fn set_bist(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 24;
        self.0 &= !0xff000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn cache_line_size(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_cache_line_size(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn header_type(&self) -> u32 {
        (self.0 & 0x7f0000) >> 16
    }
    #[inline(always)]
    pub fn set_header_type(&mut self, value: u32) {
        assert!(value <= 0x7f);
        let value = value << 16;
        self.0 &= !0x7f0000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn latency_master_timer(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline(always)]
    pub fn set_latency_master_timer(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn multi_func(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    #[inline(always)]
    pub fn set_multi_func(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 23;
        self.0 &= !0x800000;
        self.0 |= value;
    }
}
/// Word offset 10
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CARDBUS_CIS_PTR(u32);
impl CARDBUS_CIS_PTR {
    #[inline(always)]
    pub fn cardbus_cis_pointer(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_cardbus_cis_pointer(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Word offset 2
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct CLASS_CODE_REVISION_ID(u32);
impl CLASS_CODE_REVISION_ID {
    #[inline(always)]
    pub fn base_class_code(&self) -> u32 {
        (self.0 & 0xff000000) >> 24
    }
    #[inline(always)]
    pub fn set_base_class_code(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 24;
        self.0 &= !0xff000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn program_interface(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline(always)]
    pub fn set_program_interface(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn revision_id(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_revision_id(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn subclass_code(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }
    #[inline(always)]
    pub fn set_subclass_code(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 16;
        self.0 &= !0xff0000;
        self.0 |= value;
    }
}
/// Word offset 0
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct DEVICE_ID_VENDOR_ID(u32);
impl DEVICE_ID_VENDOR_ID {
    #[inline(always)]
    pub fn pci_type0_device_id(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    #[inline(always)]
    pub fn set_pci_type0_device_id(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 16;
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type0_vendor_id(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_pci_type0_vendor_id(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
/// Word offset 12
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct EXP_ROM_BASE_ADDR(u32);
impl EXP_ROM_BASE_ADDR {
    #[inline(always)]
    pub fn exp_rom_base_address(&self) -> u32 {
        (self.0 & 0xfffff800) >> 11
    }
    #[inline(always)]
    pub fn set_exp_rom_base_address(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        let value = value << 11;
        self.0 &= !0xfffff800;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn rom_bar_enable(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_rom_bar_enable(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
}
/// Word offset 15
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct MAX_LATENCY_MIN_GRANT_INTERRUPT_PIN_INTERRUPT_LINE(u32);
impl MAX_LATENCY_MIN_GRANT_INTERRUPT_PIN_INTERRUPT_LINE {
    #[inline(always)]
    pub fn int_line(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_int_line(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn int_pin(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    #[inline(always)]
    pub fn set_int_pin(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 8;
        self.0 &= !0xff00;
        self.0 |= value;
    }
}
/// Word offset 13
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PCI_CAP_PTR(u32);
impl PCI_CAP_PTR {
    #[inline(always)]
    pub fn cap_pointer(&self) -> u32 {
        self.0 & 0xff
    }
    #[inline(always)]
    pub fn set_cap_pointer(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Word offset 1
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct STATUS_COMMAND(u32);
impl STATUS_COMMAND {
    #[inline(always)]
    pub fn cap_list(&self) -> u32 {
        (self.0 & 0x100000) >> 20
    }
    #[inline(always)]
    pub fn set_cap_list(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 20;
        self.0 &= !0x100000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn detected_parity_err(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    #[inline(always)]
    pub fn set_detected_parity_err(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 31;
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn dev_sel_timing(&self) -> u32 {
        (self.0 & 0x6000000) >> 25
    }
    #[inline(always)]
    pub fn set_dev_sel_timing(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 25;
        self.0 &= !0x6000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn fast_66mhz_cap(&self) -> u32 {
        (self.0 & 0x200000) >> 21
    }
    #[inline(always)]
    pub fn set_fast_66mhz_cap(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 21;
        self.0 &= !0x200000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn fast_b2b_cap(&self) -> u32 {
        (self.0 & 0x800000) >> 23
    }
    #[inline(always)]
    pub fn set_fast_b2b_cap(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 23;
        self.0 &= !0x800000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn int_status(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    #[inline(always)]
    pub fn set_int_status(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 19;
        self.0 &= !0x80000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn master_dpe(&self) -> u32 {
        (self.0 & 0x1000000) >> 24
    }
    #[inline(always)]
    pub fn set_master_dpe(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 24;
        self.0 &= !0x1000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type0_bus_master_en(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    #[inline(always)]
    pub fn set_pci_type0_bus_master_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 2;
        self.0 &= !0x4;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type0_int_en(&self) -> u32 {
        (self.0 & 0x400) >> 10
    }
    #[inline(always)]
    pub fn set_pci_type0_int_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 10;
        self.0 &= !0x400;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type0_io_en(&self) -> u32 {
        self.0 & 0x1
    }
    #[inline(always)]
    pub fn set_pci_type0_io_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type0_mem_space_en(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    #[inline(always)]
    pub fn set_pci_type0_mem_space_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 1;
        self.0 &= !0x2;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type0_parity_err_en(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    #[inline(always)]
    pub fn set_pci_type0_parity_err_en(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 6;
        self.0 &= !0x40;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type0_serren(&self) -> u32 {
        (self.0 & 0x100) >> 8
    }
    #[inline(always)]
    pub fn set_pci_type0_serren(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 8;
        self.0 &= !0x100;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type0_special_cycle_operation(&self) -> u32 {
        (self.0 & 0x8) >> 3
    }
    #[inline(always)]
    pub fn set_pci_type0_special_cycle_operation(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 3;
        self.0 &= !0x8;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type_idsel_stepping(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    #[inline(always)]
    pub fn set_pci_type_idsel_stepping(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 7;
        self.0 &= !0x80;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type_mwi_enable(&self) -> u32 {
        (self.0 & 0x10) >> 4
    }
    #[inline(always)]
    pub fn set_pci_type_mwi_enable(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 4;
        self.0 &= !0x10;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type_reserv(&self) -> u32 {
        (self.0 & 0x1f800) >> 11
    }
    #[inline(always)]
    pub fn set_pci_type_reserv(&mut self, value: u32) {
        assert!(value <= 0x3f);
        let value = value << 11;
        self.0 &= !0x1f800;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn pci_type_vga_palette_snoop(&self) -> u32 {
        (self.0 & 0x20) >> 5
    }
    #[inline(always)]
    pub fn set_pci_type_vga_palette_snoop(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 5;
        self.0 &= !0x20;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn rcvd_master_abort(&self) -> u32 {
        (self.0 & 0x20000000) >> 29
    }
    #[inline(always)]
    pub fn set_rcvd_master_abort(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 29;
        self.0 &= !0x20000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn rcvd_target_abort(&self) -> u32 {
        (self.0 & 0x10000000) >> 28
    }
    #[inline(always)]
    pub fn set_rcvd_target_abort(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 28;
        self.0 &= !0x10000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn signaled_sys_err(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    #[inline(always)]
    pub fn set_signaled_sys_err(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 30;
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn signaled_target_abort(&self) -> u32 {
        (self.0 & 0x8000000) >> 27
    }
    #[inline(always)]
    pub fn set_signaled_target_abort(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 27;
        self.0 &= !0x8000000;
        self.0 |= value;
    }
}
/// Word offset 11
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct SUBSYSTEM_ID_SUBSYSTEM_VENDOR_ID(u32);
impl SUBSYSTEM_ID_SUBSYSTEM_VENDOR_ID {
    #[inline(always)]
    pub fn subsys_dev_id(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    #[inline(always)]
    pub fn set_subsys_dev_id(&mut self, value: u32) {
        assert!(value <= 0xffff);
        let value = value << 16;
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    #[inline(always)]
    pub fn subsys_vendor_id(&self) -> u32 {
        self.0 & 0xffff
    }
    #[inline(always)]
    pub fn set_subsys_vendor_id(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
}
