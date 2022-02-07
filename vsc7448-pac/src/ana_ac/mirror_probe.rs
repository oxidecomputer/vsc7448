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
/// Mirror probe configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PROBE_CFG(u32);
impl PROBE_CFG {
    /// Mirror traffic from CPU,  which is set to bypass the analyzer. Traffic, which is set to bypass the analyzer, is identified by the following criteria: IFH.FWD.DST_MODE == INJECT || IFH.MISC.PIPELINE_PT >= ANA_DONE
    #[inline(always)]
    pub fn mirror_cpu_inject_ena(&self) -> u32 {
        (self.0 & 0x80000000) >> 31
    }
    #[inline(always)]
    pub fn set_mirror_cpu_inject_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 31;
        self.0 &= !0x80000000;
        self.0 |= value;
    }
    /// The set of CPU extraction ports from where traffic is Tx mirrored by this mirror probe. Only works when Tx mirroring is  enabled. Related parameters: ANA_AC:MIRROR_PROBE:PROBE_CFG.PROBE_DIRECTION
    ///
    /// 0: CPU extraction port is not Tx mirrored 1: CPU extraction port is Tx mirrored
    #[inline(always)]
    pub fn probe_cpu_set(&self) -> u32 {
        (self.0 & 0x3fc0000) >> 18
    }
    #[inline(always)]
    pub fn set_probe_cpu_set(&mut self, value: u32) {
        assert!(value <= 0xff);
        let value = value << 18;
        self.0 &= !0x3fc0000;
        self.0 |= value;
    }
    /// Direction of traffic that is mirrored.
    ///
    /// "00" : Mirroring is disabled. "01" : TX - Only traffic to destination ports in the probe port set "10" : RX - Only traffic from source ports in the probe port set "11" : RX+TX - Traffic to and from ports in the probe port set (always mirrored as RX)
    #[inline(always)]
    pub fn probe_direction(&self) -> u32 {
        self.0 & 0x3
    }
    #[inline(always)]
    pub fn set_probe_direction(&mut self, value: u32) {
        assert!(value <= 0x3);
        self.0 &= !0x3;
        self.0 |= value;
    }
    /// Mirror probe's MAC address filtering mode, based on the mirror bit in the MAC table.
    ///
    /// "00" : No MAC filtering "01" : Mirror only traffic with mirror bit set for known DMAC "10" : Mirror only traffic with mirror bit set for known SMAC "11" : Mirror only traffic with mirror bit set for known DMAC and/or known SMAC
    #[inline(always)]
    pub fn probe_mac_mode(&self) -> u32 {
        (self.0 & 0xc) >> 2
    }
    #[inline(always)]
    pub fn set_probe_mac_mode(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 2;
        self.0 &= !0xc;
        self.0 |= value;
    }
    /// Controls whether physical or masqueraded port number will be used for lookup in PROBE_PORT_MASK. CPU injected frames with masqueraded port number are identfied by having IFH.MIWC.PIPELINE_ACT == INJ_MASQ. Frames, which have been looped back from Up-MEP can only be mirrored if masqueraded port number is used for lookup in PROBE_PORT_MASK. Related parameters: ANA_AC:MIRROR_PROBE:PROBE_PORT_CFG.PROBE_PORT_MASK.
    ///
    /// 0: If frame is masqueraded, then use masqueraded port number for lookup in PROBE_PORT_MASK. Otherwise use physical port number for lookup in PROBE_PORT_MASK. 1: Use physical port.number for lookup in PROBE_PORT_MASK.
    #[inline(always)]
    pub fn probe_phys_rx_port(&self) -> u32 {
        (self.0 & 0x40000000) >> 30
    }
    #[inline(always)]
    pub fn set_probe_phys_rx_port(&mut self, value: u32) {
        assert!(value <= 0x1);
        let value = value << 30;
        self.0 &= !0x40000000;
        self.0 |= value;
    }
    /// Mirror traffic received from CPU by this mirror probe. PROBE_RX_CPU_AND_VD works as an extension of the PROBE_PORT_MASK to cover CPU ports and VD0/VD1.
    ///
    /// "0000": No RX mirror "xxx1": Rx from CPU port 53 is mirrored "xx1x": Rx from CPU port 54 is mirrored "x1xx": Rx from VD0 is mirrored "1xxx": Rx from VD1 is mirrored Only works with RX mirror enabled. See ANA_AC:MIRROR_PROBE:PROBE_CFG.PROBE_DIRECTION.
    #[inline(always)]
    pub fn probe_rx_cpu_and_vd(&self) -> u32 {
        (self.0 & 0x3c000000) >> 26
    }
    #[inline(always)]
    pub fn set_probe_rx_cpu_and_vd(&mut self, value: u32) {
        assert!(value <= 0xf);
        let value = value << 26;
        self.0 &= !0x3c000000;
        self.0 |= value;
    }
    /// The mirror probe's VLAN ID when PROBE_VLAN_MODE is 10 or 11. Probing per VLAN can also be enabled in ANA_L3:VLAN, but such configuration will affect all mirror probes. Related parameters: ANA_AC.MIRROR_PROBE.PROBE_CFG.PROBE_VLAN_MODE ANA_L3:VLAN:VLAN_CFG.VLAN_MIRROR_ENA
    #[inline(always)]
    pub fn probe_vid(&self) -> u32 {
        (self.0 & 0x3ffc0) >> 6
    }
    #[inline(always)]
    pub fn set_probe_vid(&mut self, value: u32) {
        assert!(value <= 0xfff);
        let value = value << 6;
        self.0 &= !0x3ffc0;
        self.0 |= value;
    }
    /// Mirror probe's VLAN filtering mode. Related parameters: ANA_L3:VLAN:VLAN_CFG.VLAN_MIRROR_ENA
    ///
    /// 00 : No VLAN filtering 01 : Mirror only traffic in VLANs with ANA_L3:VLAN:VLAN_CFG.VLAN_MIRROR_ENA set 10 : Mirror only traffic with CL-IVID = PROBE_VID. 11 : Reserved.
    #[inline(always)]
    pub fn probe_vlan_mode(&self) -> u32 {
        (self.0 & 0x30) >> 4
    }
    #[inline(always)]
    pub fn set_probe_vlan_mode(&mut self, value: u32) {
        assert!(value <= 0x3);
        let value = value << 4;
        self.0 &= !0x30;
        self.0 |= value;
    }
}
/// Mirror probe port mask configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PROBE_PORT_CFG(u32);
impl PROBE_PORT_CFG {
    /// Specifies the set of ingress port(s) subject to Rx mirroring and the set of egress port(s) subject to Tx mirroring. PROBE_PHYS_RX_PORT controls whether physical or masqueraded port number is used for lookup in PROBE_PORT_MASK. PROBE_RX_CPU_AND_VD works as an extension of the PROBE_PORT_MASK to cover CPU ports and VD0/VD1. Ports enabled in PROBE_PORT_MASK are either RX mirrored (ingress mirrored) and/or TX mirrored (egress mirrored) as determined by PROBE_DIRECTION. Mirror destination port(s) are determined by QFWD:SYSTEM:FRAME_COPY_CFG For exact TX mirror, the configuration of this register must be consistent with the configuration of REW:COMMON:MIRROR_PROBE_CFG.MIRROR_TX_PORT. See also description for register group ANA_AC:MIRROR_PROBE. Related parameters: ANA_AC:MIRROR_PROBE:PROBE_CFG.PROBE_PHYS_RX_PORT ANA_AC:MIRROR_PROBE:PROBE_CFG.PROBE_RX_CPU_AND_VD ANA_AC:MIRROR_PROBE:PROBE_CFG.PROBE_DIRECTION REW:COMMON:MIRROR_PROBE_CFG.MIRROR_TX_PORT QFWD:SYSTEM:FRAME_COPY_CFG
    ///
    /// 0: Port is not mirrored 1: Port is mirrored
    #[inline(always)]
    pub fn probe_port_mask(&self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn set_probe_port_mask(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Mirror probe port mask configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into)]
pub struct PROBE_PORT_CFG1(u32);
impl PROBE_PORT_CFG1 {
    /// Refer to PROBE_PORT_CFG.PROBE_PORT_MASK description.
    #[inline(always)]
    pub fn probe_port_mask1(&self) -> u32 {
        self.0 & 0x1fffff
    }
    #[inline(always)]
    pub fn set_probe_port_mask1(&mut self, value: u32) {
        assert!(value <= 0x1fffff);
        self.0 &= !0x1fffff;
        self.0 |= value;
    }
}
