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

pub mod hw_cfgstat;
pub mod mcb_serdes1g_cfg;
pub mod mcb_serdes6g_cfg;
pub mod pll5g_bist_cfg;
pub mod pll5g_bist_status;
pub mod pll5g_cfg;
pub mod pll5g_status;
pub mod rcomp_cfg;
pub mod rcomp_status;
pub mod serdes1g_ana_cfg;
pub mod serdes1g_ana_status;
pub mod serdes1g_dig_cfg;
pub mod serdes1g_dig_status;
pub mod serdes6g_ana_cfg;
pub mod serdes6g_ana_status;
pub mod serdes6g_dig_cfg;
pub mod serdes6g_dig_status;
pub mod sync_eth_cfg;

/// General high-speed IO configuration and status
pub struct HW_CFGSTAT(pub(super) u32);
impl HW_CFGSTAT {
    pub fn HW_CFG(&self) -> RegisterAddress<hw_cfgstat::HW_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn HW_QSGMII_CFG(&self) -> RegisterAddress<hw_cfgstat::HW_QSGMII_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn HW_QSGMII_STAT(&self, index: u32) -> RegisterAddress<hw_cfgstat::HW_QSGMII_STAT> {
        assert!(index < 12);
        RegisterAddress::new(self.0 + 0x8 + index * 0x4)
    }
    pub fn MCB_SERDES6G_ADDR_CFG(&self) -> RegisterAddress<hw_cfgstat::MCB_SERDES6G_ADDR_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// MCB SERDES1G Configuration Register
pub struct MCB_SERDES1G_CFG(pub(super) u32);
impl MCB_SERDES1G_CFG {
    pub fn SERDES1G_MISC_STATUS(&self) -> RegisterAddress<mcb_serdes1g_cfg::SERDES1G_MISC_STATUS> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// MCB SERDES6G Configuration Register
pub struct MCB_SERDES6G_CFG(pub(super) u32);
impl MCB_SERDES6G_CFG {
    pub fn SERDES6G_REVID(&self) -> RegisterAddress<mcb_serdes6g_cfg::SERDES6G_REVID> {
        RegisterAddress::new(self.0 + 0x10)
    }
}

/// PLL5G BIST Configuration Register set
pub struct PLL5G_BIST_CFG(pub(super) u32);
impl PLL5G_BIST_CFG {
    pub fn PLL5G_BIST_CFG0A(&self) -> RegisterAddress<pll5g_bist_cfg::PLL5G_BIST_CFG0A> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn PLL5G_BIST_CFG0B(&self) -> RegisterAddress<pll5g_bist_cfg::PLL5G_BIST_CFG0B> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PLL5G_BIST_CFG1(&self) -> RegisterAddress<pll5g_bist_cfg::PLL5G_BIST_CFG1> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn PLL5G_STATUS1(&self) -> RegisterAddress<pll5g_bist_cfg::PLL5G_STATUS1> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// PLL5G BIST Status Register set
pub struct PLL5G_BIST_STATUS(pub(super) u32);
impl PLL5G_BIST_STATUS {
    pub fn PLL5G_BIST_CFG2(&self) -> RegisterAddress<pll5g_bist_status::PLL5G_BIST_CFG2> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn PLL5G_BIST_STAT0(&self) -> RegisterAddress<pll5g_bist_status::PLL5G_BIST_STAT0> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn PLL5G_BIST_STAT1A(&self) -> RegisterAddress<pll5g_bist_status::PLL5G_BIST_STAT1A> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// PLL5G Configuration Registers
pub struct PLL5G_CFG(pub(super) u32);
impl PLL5G_CFG {
    pub fn PLL5G_CFG0(&self) -> RegisterAddress<pll5g_cfg::PLL5G_CFG0> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn PLL5G_CFG1(&self) -> RegisterAddress<pll5g_cfg::PLL5G_CFG1> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn PLL5G_CFG2(&self) -> RegisterAddress<pll5g_cfg::PLL5G_CFG2> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn PLL5G_CFG3(&self) -> RegisterAddress<pll5g_cfg::PLL5G_CFG3> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn PLL5G_CFG4(&self) -> RegisterAddress<pll5g_cfg::PLL5G_CFG4> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn PLL5G_CFG5(&self) -> RegisterAddress<pll5g_cfg::PLL5G_CFG5> {
        RegisterAddress::new(self.0 + 0x14)
    }
}

/// PLL5G Status Registers
pub struct PLL5G_STATUS(pub(super) u32);
impl PLL5G_STATUS {
    pub fn PLL5G_CFG6(&self) -> RegisterAddress<pll5g_status::PLL5G_CFG6> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn PLL5G_STATUS0(&self) -> RegisterAddress<pll5g_status::PLL5G_STATUS0> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// RCOMP Configuration Registers
pub struct RCOMP_CFG(pub(super) u32);
impl RCOMP_CFG {
    pub fn PLL5G_BIST_STAT1B(&self) -> RegisterAddress<rcomp_cfg::PLL5G_BIST_STAT1B> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// RCOMP Status Registers
pub struct RCOMP_STATUS(pub(super) u32);
impl RCOMP_STATUS {
    pub fn RCOMP_CFG0(&self) -> RegisterAddress<rcomp_status::RCOMP_CFG0> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// SERDES1G Analog Configuration Registers
pub struct SERDES1G_ANA_CFG(pub(super) u32);
impl SERDES1G_ANA_CFG {
    pub fn SERDES1G_COMMON_CFG(&self) -> RegisterAddress<serdes1g_ana_cfg::SERDES1G_COMMON_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn SERDES1G_DES_CFG(&self) -> RegisterAddress<serdes1g_ana_cfg::SERDES1G_DES_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SERDES1G_IB_CFG(&self) -> RegisterAddress<serdes1g_ana_cfg::SERDES1G_IB_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn SERDES1G_OB_CFG(&self) -> RegisterAddress<serdes1g_ana_cfg::SERDES1G_OB_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn SERDES1G_SER_CFG(&self) -> RegisterAddress<serdes1g_ana_cfg::SERDES1G_SER_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn SYNC_ETH_SD10G_CFG(
        &self,
        index: u32,
    ) -> RegisterAddress<serdes1g_ana_cfg::SYNC_ETH_SD10G_CFG> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x14 + index * 0x4)
    }
}

/// SERDES1G Analog Status Register
pub struct SERDES1G_ANA_STATUS(pub(super) u32);
impl SERDES1G_ANA_STATUS {
    pub fn SERDES1G_PLL_CFG(&self) -> RegisterAddress<serdes1g_ana_status::SERDES1G_PLL_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
}

/// SERDES1G Digital Configuration Register
pub struct SERDES1G_DIG_CFG(pub(super) u32);
impl SERDES1G_DIG_CFG {
    pub fn SERDES1G_DFT_CFG0(&self) -> RegisterAddress<serdes1g_dig_cfg::SERDES1G_DFT_CFG0> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SERDES1G_DFT_CFG1(&self) -> RegisterAddress<serdes1g_dig_cfg::SERDES1G_DFT_CFG1> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn SERDES1G_DFT_CFG2(&self) -> RegisterAddress<serdes1g_dig_cfg::SERDES1G_DFT_CFG2> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn SERDES1G_PLL_STATUS(&self) -> RegisterAddress<serdes1g_dig_cfg::SERDES1G_PLL_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SERDES1G_RC_PLL_BIST_CFG(
        &self,
    ) -> RegisterAddress<serdes1g_dig_cfg::SERDES1G_RC_PLL_BIST_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn SERDES1G_TP_CFG(&self) -> RegisterAddress<serdes1g_dig_cfg::SERDES1G_TP_CFG> {
        RegisterAddress::new(self.0 + 0xc)
    }
}

/// SERDES1G Digital Status Register
pub struct SERDES1G_DIG_STATUS(pub(super) u32);
impl SERDES1G_DIG_STATUS {
    pub fn SERDES1G_DFT_STATUS(&self) -> RegisterAddress<serdes1g_dig_status::SERDES1G_DFT_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SERDES1G_MISC_CFG(&self) -> RegisterAddress<serdes1g_dig_status::SERDES1G_MISC_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
}

/// SERDES6G Analog ConfigStatus Registers
pub struct SERDES6G_ANA_CFG(pub(super) u32);
impl SERDES6G_ANA_CFG {
    pub fn SERDES6G_ACJTAG_CFG(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_ACJTAG_CFG> {
        RegisterAddress::new(self.0 + 0x30)
    }
    pub fn SERDES6G_COMMON_CFG(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_COMMON_CFG> {
        RegisterAddress::new(self.0 + 0x28)
    }
    pub fn SERDES6G_DES_CFG(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_DES_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SERDES6G_IB_CFG(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_IB_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn SERDES6G_IB_CFG1(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_IB_CFG1> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn SERDES6G_IB_CFG2(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_IB_CFG2> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn SERDES6G_IB_CFG3(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_IB_CFG3> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn SERDES6G_IB_CFG4(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_IB_CFG4> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn SERDES6G_IB_CFG5(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_IB_CFG5> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn SERDES6G_MISC_STATUS(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_MISC_STATUS> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn SERDES6G_OB_CFG(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_OB_CFG> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn SERDES6G_OB_CFG1(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_OB_CFG1> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn SERDES6G_PLL_CFG(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_PLL_CFG> {
        RegisterAddress::new(self.0 + 0x2c)
    }
    pub fn SERDES6G_SER_CFG(&self) -> RegisterAddress<serdes6g_ana_cfg::SERDES6G_SER_CFG> {
        RegisterAddress::new(self.0 + 0x24)
    }
}

/// SERDES6G Analog Status Registers
pub struct SERDES6G_ANA_STATUS(pub(super) u32);
impl SERDES6G_ANA_STATUS {
    pub fn SERDES6G_ACJTAG_STATUS(
        &self,
    ) -> RegisterAddress<serdes6g_ana_status::SERDES6G_ACJTAG_STATUS> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn SERDES6G_GP_CFG(&self) -> RegisterAddress<serdes6g_ana_status::SERDES6G_GP_CFG> {
        RegisterAddress::new(self.0 + 0x34)
    }
    pub fn SERDES6G_IB_STATUS0(&self) -> RegisterAddress<serdes6g_ana_status::SERDES6G_IB_STATUS0> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SERDES6G_IB_STATUS1(&self) -> RegisterAddress<serdes6g_ana_status::SERDES6G_IB_STATUS1> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn SERDES6G_PLL_STATUS(&self) -> RegisterAddress<serdes6g_ana_status::SERDES6G_PLL_STATUS> {
        RegisterAddress::new(self.0 + 0xc)
    }
}

/// SERDES6G Digital Configuration Registers
pub struct SERDES6G_DIG_CFG(pub(super) u32);
impl SERDES6G_DIG_CFG {
    pub fn MCB_SERDES1G_ADDR_CFG(
        &self,
    ) -> RegisterAddress<serdes6g_dig_cfg::MCB_SERDES1G_ADDR_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SERDES6G_DFT_CFG0(&self) -> RegisterAddress<serdes6g_dig_cfg::SERDES6G_DFT_CFG0> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn SERDES6G_DFT_CFG1(&self) -> RegisterAddress<serdes6g_dig_cfg::SERDES6G_DFT_CFG1> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn SERDES6G_DFT_CFG2(&self) -> RegisterAddress<serdes6g_dig_cfg::SERDES6G_DFT_CFG2> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn SERDES6G_DIG_CFG(&self) -> RegisterAddress<serdes6g_dig_cfg::SERDES6G_DIG_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SERDES6G_MISC_CFG(&self) -> RegisterAddress<serdes6g_dig_cfg::SERDES6G_MISC_CFG> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn SERDES6G_RC_PLL_BIST_CFG(
        &self,
    ) -> RegisterAddress<serdes6g_dig_cfg::SERDES6G_RC_PLL_BIST_CFG> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn SERDES6G_TP_CFG0(&self) -> RegisterAddress<serdes6g_dig_cfg::SERDES6G_TP_CFG0> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn SERDES6G_TP_CFG1(&self) -> RegisterAddress<serdes6g_dig_cfg::SERDES6G_TP_CFG1> {
        RegisterAddress::new(self.0 + 0x14)
    }
}

/// SERDES6G Digital Status Register
pub struct SERDES6G_DIG_STATUS(pub(super) u32);
impl SERDES6G_DIG_STATUS {
    pub fn SERDES6G_DFT_STATUS(&self) -> RegisterAddress<serdes6g_dig_status::SERDES6G_DFT_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SERDES6G_ERR_CNT(&self) -> RegisterAddress<serdes6g_dig_status::SERDES6G_ERR_CNT> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn SERDES6G_OB_ANEG_CFG(
        &self,
    ) -> RegisterAddress<serdes6g_dig_status::SERDES6G_OB_ANEG_CFG> {
        RegisterAddress::new(self.0 + 0x20)
    }
}

/// SYNC_ETH Configuration Registers
pub struct SYNC_ETH_CFG(pub(super) u32);
impl SYNC_ETH_CFG {
    pub fn RCOMP_STATUS(&self) -> RegisterAddress<sync_eth_cfg::RCOMP_STATUS> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn SYNC_ETH_CFG(&self, index: u32) -> RegisterAddress<sync_eth_cfg::SYNC_ETH_CFG> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
    pub fn SYNC_ETH_PLL2_CFG(&self) -> RegisterAddress<sync_eth_cfg::SYNC_ETH_PLL2_CFG> {
        RegisterAddress::new(self.0 + 0x10)
    }
}
