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

pub mod common;
pub mod ipt;
pub mod l2cp_tbl;
pub mod map_tbl;
pub mod mip_tbl;
pub mod mpls_profile;
pub mod port;
pub mod ppt;
pub mod sticky;
pub mod sticky_mask;

/// Common configurations for all ports
pub struct COMMON(pub(super) u32);
impl COMMON {
    pub fn ADV_RNG_CTRL(&self, index: u32) -> RegisterAddress<common::ADV_RNG_CTRL> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0xa4 + index * 0x4)
    }
    pub fn ADV_RNG_VALUE_CFG(&self, index: u32) -> RegisterAddress<common::ADV_RNG_VALUE_CFG> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0xc4 + index * 0x4)
    }
    pub fn AGGR_CFG(&self) -> RegisterAddress<common::AGGR_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn CLM_KEY_CFG(&self, index: u32) -> RegisterAddress<common::CLM_KEY_CFG> {
        assert!(index < 6);
        RegisterAddress::new(self.0 + 0x2bc + index * 0x4)
    }
    pub fn CLM_MISC_CTRL(&self) -> RegisterAddress<common::CLM_MISC_CTRL> {
        RegisterAddress::new(self.0 + 0xe8)
    }
    pub fn COMMON_VSTAX_CFG(&self) -> RegisterAddress<common::COMMON_VSTAX_CFG> {
        RegisterAddress::new(self.0 + 0xe4)
    }
    pub fn CPU_8021_QOS_CFG(&self, index: u32) -> RegisterAddress<common::CPU_8021_QOS_CFG> {
        assert!(index < 16);
        RegisterAddress::new(self.0 + 0x58 + index * 0x4)
    }
    pub fn CPU_8021_QU_CFG(&self, index: u32) -> RegisterAddress<common::CPU_8021_QU_CFG> {
        assert!(index < 16);
        RegisterAddress::new(self.0 + 0x18 + index * 0x4)
    }
    pub fn CPU_PROTO_QU_CFG(&self) -> RegisterAddress<common::CPU_PROTO_QU_CFG> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn DSCP_CFG(&self, index: u32) -> RegisterAddress<common::DSCP_CFG> {
        assert!(index < 64);
        RegisterAddress::new(self.0 + 0xfc + index * 0x4)
    }
    pub fn HM_CFG(&self, index: u32) -> RegisterAddress<common::HM_CFG> {
        assert!(index < 4);
        RegisterAddress::new(self.0 + 0xec + index * 0x4)
    }
    pub fn MIP_CTRL(&self) -> RegisterAddress<common::MIP_CTRL> {
        RegisterAddress::new(self.0 + 0x2e4)
    }
    pub fn MPLS_CFG(&self) -> RegisterAddress<common::MPLS_CFG> {
        RegisterAddress::new(self.0 + 0x2dc)
    }
    pub fn MPLS_LM_CFG(&self) -> RegisterAddress<common::MPLS_LM_CFG> {
        RegisterAddress::new(self.0 + 0x2d8)
    }
    pub fn MPLS_MISC_CFG(&self) -> RegisterAddress<common::MPLS_MISC_CFG> {
        RegisterAddress::new(self.0 + 0x2d4)
    }
    pub fn MPLS_RSV_LBL_CFG(&self, index: u32) -> RegisterAddress<common::MPLS_RSV_LBL_CFG> {
        assert!(index < 16);
        RegisterAddress::new(self.0 + 0x27c + index * 0x4)
    }
    pub fn OAM_CFG(&self) -> RegisterAddress<common::OAM_CFG> {
        RegisterAddress::new(self.0 + 0x2e0)
    }
    pub fn QOS_MAP_CFG(&self, index: u32) -> RegisterAddress<common::QOS_MAP_CFG> {
        assert!(index < 32);
        RegisterAddress::new(self.0 + 0x1fc + index * 0x4)
    }
    pub fn UPSID_CFG(&self) -> RegisterAddress<common::UPSID_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn VLAN_STAG_CFG(&self, index: u32) -> RegisterAddress<common::VLAN_STAG_CFG> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x8 + index * 0x4)
    }
    pub fn VRAP_CFG(&self) -> RegisterAddress<common::VRAP_CFG> {
        RegisterAddress::new(self.0 + 0x98)
    }
    pub fn VRAP_HDR_DATA(&self) -> RegisterAddress<common::VRAP_HDR_DATA> {
        RegisterAddress::new(self.0 + 0x9c)
    }
    pub fn VRAP_HDR_MASK(&self) -> RegisterAddress<common::VRAP_HDR_MASK> {
        RegisterAddress::new(self.0 + 0xa0)
    }
}

/// Ingress Protection Table
pub struct IPT(pub(super) u32);
impl IPT {
    pub fn IPT(&self) -> RegisterAddress<ipt::IPT> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn ISDX_CFG(&self) -> RegisterAddress<ipt::ISDX_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn OAM_MEP_CFG(&self) -> RegisterAddress<ipt::OAM_MEP_CFG> {
        RegisterAddress::new(self.0 + 0x8)
    }
    pub fn VSI_CFG(&self) -> RegisterAddress<ipt::VSI_CFG> {
        RegisterAddress::new(self.0 + 0x4)
    }
}

/// L2CP table
pub struct L2CP_TBL(pub(super) u32);
impl L2CP_TBL {
    pub fn L2CP_ENTRY_CFG(&self) -> RegisterAddress<l2cp_tbl::L2CP_ENTRY_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// QoS mapping table
pub struct MAP_TBL(pub(super) u32);
impl MAP_TBL {
    pub fn MAP_ENTRY(&self, index: u32) -> RegisterAddress<map_tbl::MAP_ENTRY> {
        assert!(index < 8);
        RegisterAddress::new(self.0 + 0x4 + index * 0x4)
    }
    pub fn SET_CTRL(&self) -> RegisterAddress<map_tbl::SET_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// MIP table
pub struct MIP_TBL(pub(super) u32);
impl MIP_TBL {
    pub fn CCM_HMO_CTRL(&self) -> RegisterAddress<mip_tbl::CCM_HMO_CTRL> {
        RegisterAddress::new(self.0 + 0x4)
    }
    pub fn LBM_MAC_HIGH(&self) -> RegisterAddress<mip_tbl::LBM_MAC_HIGH> {
        RegisterAddress::new(self.0 + 0xc)
    }
    pub fn LBM_MAC_LOW(&self) -> RegisterAddress<mip_tbl::LBM_MAC_LOW> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn MIP_CFG(&self) -> RegisterAddress<mip_tbl::MIP_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn MIP_CL_VID_CTRL(&self) -> RegisterAddress<mip_tbl::MIP_CL_VID_CTRL> {
        RegisterAddress::new(self.0 + 0x8)
    }
}

/// Configuriong of profiles used for MPLS traffic exception handling
pub struct MPLS_PROFILE(pub(super) u32);
impl MPLS_PROFILE {
    pub fn PROFILE_CFG(&self) -> RegisterAddress<mpls_profile::PROFILE_CFG> {
        RegisterAddress::new(self.0 + 0x0)
    }
}

/// Classification and filter configurations per port
pub struct PORT(pub(super) u32);
impl PORT {
    pub fn ADV_CL_CFG(&self, index: u32) -> RegisterAddress<port::ADV_CL_CFG> {
        assert!(index < 6);
        RegisterAddress::new(self.0 + 0xb4 + index * 0x4)
    }
    pub fn CAPTURE_BPDU_CFG(&self) -> RegisterAddress<port::CAPTURE_BPDU_CFG> {
        RegisterAddress::new(self.0 + 0xb0)
    }
    pub fn CAPTURE_CFG(&self) -> RegisterAddress<port::CAPTURE_CFG> {
        RegisterAddress::new(self.0 + 0xa4)
    }
    pub fn CAPTURE_GXRP_CFG(&self) -> RegisterAddress<port::CAPTURE_GXRP_CFG> {
        RegisterAddress::new(self.0 + 0xac)
    }
    pub fn CAPTURE_Y1731_AG_CFG(&self) -> RegisterAddress<port::CAPTURE_Y1731_AG_CFG> {
        RegisterAddress::new(self.0 + 0xa8)
    }
    pub fn FILTER_CTRL(&self) -> RegisterAddress<port::FILTER_CTRL> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn PCP_DEI_MAP_CFG(&self, index: u32) -> RegisterAddress<port::PCP_DEI_MAP_CFG> {
        assert!(index < 16);
        RegisterAddress::new(self.0 + 0x60 + index * 0x4)
    }
    pub fn PCP_DEI_TRANS_CFG(&self, index: u32) -> RegisterAddress<port::PCP_DEI_TRANS_CFG> {
        assert!(index < 16);
        RegisterAddress::new(self.0 + 0x1c + index * 0x4)
    }
    pub fn PORT_ID_CFG(&self) -> RegisterAddress<port::PORT_ID_CFG> {
        RegisterAddress::new(self.0 + 0x5c)
    }
    pub fn QOS_CFG(&self) -> RegisterAddress<port::QOS_CFG> {
        RegisterAddress::new(self.0 + 0xa0)
    }
    pub fn STACKING_CTRL(&self) -> RegisterAddress<port::STACKING_CTRL> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn VLAN_CTRL(&self) -> RegisterAddress<port::VLAN_CTRL> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn VLAN_FILTER_CTRL(&self, index: u32) -> RegisterAddress<port::VLAN_FILTER_CTRL> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x4 + index * 0x4)
    }
    pub fn VLAN_TPID_CTRL(&self) -> RegisterAddress<port::VLAN_TPID_CTRL> {
        RegisterAddress::new(self.0 + 0x14)
    }
}

/// Protection Programming Table
pub struct PPT(pub(super) u32);
impl PPT {
    pub fn PP_CFG(&self, index: u32) -> RegisterAddress<ppt::PP_CFG> {
        assert!(index < 16);
        RegisterAddress::new(self.0 + 0x0 + index * 0x4)
    }
}

/// Sticky diagnostic status
pub struct STICKY(pub(super) u32);
impl STICKY {
    pub fn ADV_CL_MPLS_STICKY(&self) -> RegisterAddress<sticky::ADV_CL_MPLS_STICKY> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn ADV_CL_STICKY(&self) -> RegisterAddress<sticky::ADV_CL_STICKY> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn CAT_STICKY(&self) -> RegisterAddress<sticky::CAT_STICKY> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn CLASS_STICKY(&self) -> RegisterAddress<sticky::CLASS_STICKY> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn FILTER_STICKY(&self) -> RegisterAddress<sticky::FILTER_STICKY> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn IP_HDR_CHK_STICKY(&self) -> RegisterAddress<sticky::IP_HDR_CHK_STICKY> {
        RegisterAddress::new(self.0 + 0x24)
    }
    pub fn MIP_STICKY(&self) -> RegisterAddress<sticky::MIP_STICKY> {
        RegisterAddress::new(self.0 + 0x20)
    }
    pub fn VLAN_FILTER_STICKY(&self, index: u32) -> RegisterAddress<sticky::VLAN_FILTER_STICKY> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x4 + index * 0x4)
    }
}

/// Sticky diagnostic global port counter event configuration
pub struct STICKY_MASK(pub(super) u32);
impl STICKY_MASK {
    pub fn CAT_STICKY_MASK(&self) -> RegisterAddress<sticky_mask::CAT_STICKY_MASK> {
        RegisterAddress::new(self.0 + 0x14)
    }
    pub fn CLASS_STICKY_MASK(&self) -> RegisterAddress<sticky_mask::CLASS_STICKY_MASK> {
        RegisterAddress::new(self.0 + 0x10)
    }
    pub fn FILTER_STICKY_MASK(&self) -> RegisterAddress<sticky_mask::FILTER_STICKY_MASK> {
        RegisterAddress::new(self.0 + 0x0)
    }
    pub fn IP_HDR_CHK_STICKY_MASK(&self) -> RegisterAddress<sticky_mask::IP_HDR_CHK_STICKY_MASK> {
        RegisterAddress::new(self.0 + 0x1c)
    }
    pub fn MIP_STICKY_MASK(&self) -> RegisterAddress<sticky_mask::MIP_STICKY_MASK> {
        RegisterAddress::new(self.0 + 0x18)
    }
    pub fn VLAN_FILTER_STICKY_MASK(
        &self,
        index: u32,
    ) -> RegisterAddress<sticky_mask::VLAN_FILTER_STICKY_MASK> {
        assert!(index < 3);
        RegisterAddress::new(self.0 + 0x4 + index * 0x4)
    }
}
