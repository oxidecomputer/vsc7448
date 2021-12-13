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
/// Activate specific channels
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_ACTIVATE(u32);
impl FDMA_CH_ACTIVATE {
    /// Enables specific FDMA channels, there is one bit per channel. Setting a bit in this field will clear a corresponding pending ICPU_CFG::FDMA_CH_DISABLE.CH_DISABLE request. Bits in this field are cleared immediately when set.
    pub fn ch_activate(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_ch_activate(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// Channel specific configurations
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_CFG(u32);
impl FDMA_CH_CFG {
    /// This field is only applicable to injection channels. Use this field to map the injection channel to an injection group.
    pub fn ch_inj_grp(&self) -> u32 {
        (self.0 & 0x40) >> 6
    }
    pub fn set_ch_inj_grp(&mut self, value: u32) {
        let value = value << 6;
        assert!(value <= 0x40);
        self.0 &= !0x40;
        self.0 |= value;
    }
    /// The FDMA implements a strict priority scheme between all channels - both injection and extraction. Observe: The FDMA does not directly control the order in which ports are serviced in the Queuing System. In order to adjust for this (and to avoid head of line blocking) all extraction channels are automatically assigned the highest priority of the extraction channels with available data. If multiple channels are configured with equal priorities then the following strict scheme is in place: Higher channel number takes priority over lower channel number. This implies that injection takes priority over extraction.
    pub fn ch_prio(&self) -> u32 {
        (self.0 & 0x3c) >> 2
    }
    pub fn set_ch_prio(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x3c);
        self.0 &= !0x3c;
        self.0 |= value;
    }
    /// Set this field to automatically disable the channel after completing any DCB with EOF indication. The channel will disable after saving DCB status. An LLP-event will be generated at the same time as the channel is disabled. Be careful when using this feature, extraction channels may head-of-line block other extraction channels if not immediately re-activated.
    pub fn doneeof_stop_ena(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_doneeof_stop_ena(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set this field to automatically disable the channel after completing any DCB. The channel will disable after saving DCB status. An LLP-event will be generated at the same time as the channel is disabled. Be careful when using this feature, extraction channels may head-of-line block other extraction channels if not immediately re-activated.
    pub fn done_stop_ena(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_done_stop_ena(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// When this field is set the FDMA will save the STAT-word to the DATAP-address instead of the DCB's STAT-word position. The DCB's DATAP field will be incremented by 4 (bytes) when the DCB is loaded (the FDMA will continue as if the DATAP field was DATAP+4). This feature is meant to be used for channels that extract or inject from PCIe mapped memory. The ICPU_CFG::FDMA_DCB_LLP_PREV field is repurposed as DATAP pointer and can no longer be used when this field is set.
    pub fn stat_in_data_ena(&self) -> u32 {
        (self.0 & 0x80) >> 7
    }
    pub fn set_stat_in_data_ena(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0x80);
        self.0 &= !0x80;
        self.0 |= value;
    }
}
/// Channel counters
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_CNT(u32);
impl FDMA_CH_CNT {
    /// This field is incremented every time the channel loads a DCB. This counter can be modified safely while the corresponding channel is safe (see ICPU_CFG::FDMA_CH_SAFE.CH_SAFE for more information).
    pub fn ch_cnt_dcb(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_ch_cnt_dcb(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// This field is incremented every time the channel saves status for a DCB that has EOF. This counter can only be modified safely when the corresponding channel is disabled (see ICPU_CFG::FDMA_CH_STAT.CH_STAT for more information).
    pub fn ch_cnt_frm(&self) -> u32 {
        (self.0 & 0xffff0000) >> 16
    }
    pub fn set_ch_cnt_frm(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0xffff0000);
        self.0 &= !0xffff0000;
        self.0 |= value;
    }
    /// This field is incremented every time the channel loads a DCB that has the SIG field set. The FDMA can generate interrupt whenever this counter is incremented (see ICPU_CFG::FDMA_INTR_SIG.INTR_SIG for more information). This counter can be modified safely while the corresponding channel is safe (see ICPU_CFG::FDMA_CH_SAFE.CH_SAFE for more information).
    pub fn ch_cnt_sig(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_ch_cnt_sig(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Disable specific channels
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_DISABLE(u32);
impl FDMA_CH_DISABLE {
    /// Schedules specific FDMA channels to be disabled, there is one bit per channel. The channel will finish the current DCB and then disable (after writing the DCB status word). Bits in this field is cleared either when the channel disables or by writing ICPU_CFG::FDMA_CH_ACTIVATE.CH_ACTIVATE).
    pub fn ch_disable(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_ch_disable(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// Ungraceful disable of specific channels
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_FORCEDIS(u32);
impl FDMA_CH_FORCEDIS {
    /// Immediately disable specific FDMA channels, there is one bit per channel. Unlike ICPU_CFG::FDMA_CH_DISABLE using CH_FORCEDIS will not take the state of the channel into account, if the channel is actively extracting or injecting from/to QS there is no guarantee that it will be functional after disabling the channel.
    pub fn ch_forcedis(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_ch_forcedis(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// Injection channel token counter
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_INJ_TOKEN_CNT(u32);
impl FDMA_CH_INJ_TOKEN_CNT {
    /// Every time a channel activates with a DCB that has the TOKEN field set this counter is decremented by one. Channels that loads a DCB with the TOKEN field set cannot activate unless this counter is different from zero. This counter can be writen by software, or incremented automatically by using the token tick counter (see ICPU_CFG::FDMA_CH_INJ_TOKEN_TICK_CNT for more information). This counter can be modified safely when automatic incrementing is not enabled and the corresponding injection channel is in safe mode (see ICPU_CFG::FDMA_CH_INJ_TOKEN_CNT and ICPU_CFG::FDMA_CH_SAFE.CH_SAFE for more information).
    pub fn ch_inj_token_cnt(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_ch_inj_token_cnt(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Injection channel token tick counter
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_INJ_TOKEN_TICK_CNT(u32);
impl FDMA_CH_INJ_TOKEN_TICK_CNT {
    /// Down-counter, when enabled by ICPU_CFG::FDMA_CH_INJ_TOKEN_TICK_RLD this field is decremented by one every 200ns. When zero is reached one token will be added to ICPU_CFG::FDMA_CH_INJ_TOKEN_CNT and this counted will load the value from ICPU_CFG::FDMA_CH_INJ_TOKEN_TICK_RLD (subtract one and continue decrementing from that value).
    pub fn ch_inj_token_tick_cnt(&self) -> u32 {
        self.0
    }
    pub fn set_ch_inj_token_tick_cnt(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Injection channel token tick counter reload value
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_INJ_TOKEN_TICK_RLD(u32);
impl FDMA_CH_INJ_TOKEN_TICK_RLD {
    /// Automatic incrementing of the token counter is enabled by setting this field different from 0. This field holds the reload value for the ICPU_CFG::FDMA_CH_INJ_TOKEN_TICK_CNT. Note: When changing the value of this field the same value should also be written to the ICPU_CH_INJ_TOKEN_TICK_CNT field, this is needed for speeding up token counter increments when changing from a high reload value to a low reload value.
    ///
    /// 0: Token tick counter is disabled n: Add one token every n * 200ns clock cycles
    pub fn ch_inj_token_tick_rld(&self) -> u32 {
        self.0
    }
    pub fn set_ch_inj_token_tick_rld(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Current channel safe-status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_SAFE(u32);
impl FDMA_CH_SAFE {
    /// When set it is safe for software to read/modify/write ICPU_CFG::FDMA_DCB_LLP.LLP, ICPU_CFG::FDMA_CH_CNT.CH_CNT_SIG, ICPU_CFG::FDMA_CH_CNT.CH_CNT_DCB, and ICPU_CFG::FDMA_CH_INJ_TOKEN_CNT.CH_INJ_TOKEN_CNT. There is one bit per channel. This field is set when a channel is a) disabled or b) active and scheduled for disabling.
    pub fn ch_safe(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_ch_safe(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// Current channel status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CH_STAT(u32);
impl FDMA_CH_STAT {
    /// Shows status for all FDMA channels, there is one bit per channel.
    ///
    /// 0:Disabled 1:Updating, or Active
    pub fn ch_stat(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_ch_stat(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// Constants for this FDMA implementation.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_CONST(u32);
impl FDMA_CONST {
    /// The number of injection channels.
    pub fn ch_inj_cnt(&self) -> u32 {
        (self.0 & 0xff00) >> 8
    }
    pub fn set_ch_inj_cnt(&mut self, value: u32) {
        let value = value << 8;
        assert!(value <= 0xff00);
        self.0 &= !0xff00;
        self.0 |= value;
    }
    /// The number of extraction channels.
    pub fn ch_xtr_cnt(&self) -> u32 {
        self.0 & 0xff
    }
    pub fn set_ch_xtr_cnt(&mut self, value: u32) {
        assert!(value <= 0xff);
        self.0 &= !0xff;
        self.0 |= value;
    }
}
/// Length of data block
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_DCB_DATAL(u32);
impl FDMA_DCB_DATAL {
    /// For debug, current data-length.
    pub fn datal(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_datal(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// For debug, current token-indication.
    pub fn token(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_token(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
}
/// Pointer to data block
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_DCB_DATAP(u32);
impl FDMA_DCB_DATAP {
    /// For debug, current data-pointer.
    pub fn datap(&self) -> u32 {
        self.0
    }
    pub fn set_datap(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Pointer to next DCB
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_DCB_LLP(u32);
impl FDMA_DCB_LLP {
    /// This field is used by the FDMA for tracking lists of DCBs. This field is updated automatically when the FDMA load DCBs from memory. This field can only be modified when the channel is in safe mode, see ICPU_CFG::FDMA_CH_SAFE.CH_SAFE for more information.
    pub fn llp(&self) -> u32 {
        self.0
    }
    pub fn set_llp(&mut self, value: u32) {
        self.0 = value;
    }
}
/// Pointer to current DCB
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_DCB_LLP_PREV(u32);
impl FDMA_DCB_LLP_PREV {
    /// This field holds the pointer to current DCB (the previous ICPU_CFG::FDMA_DCB_LLP.LLP).
    pub fn llp_prev(&self) -> u32 {
        (self.0 & 0xfffffffc) >> 2
    }
    pub fn set_llp_prev(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0xfffffffc);
        self.0 &= !0xfffffffc;
        self.0 |= value;
    }
}
/// Status word
///
/// This register is updated by the FDMA during extraction or injection. Software cannot rely on the value of this register.
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_DCB_STAT(u32);
impl FDMA_DCB_STAT {
    /// Abort indication.
    pub fn abort(&self) -> u32 {
        (self.0 & 0x40000) >> 18
    }
    pub fn set_abort(&mut self, value: u32) {
        let value = value << 18;
        assert!(value <= 0x40000);
        self.0 &= !0x40000;
        self.0 |= value;
    }
    /// Block size in bytes, excluding offset (as specified in ICPU_CFG::FDMA_DCB_STAT.BLOCKO). For frames that span multiple DCBs, this field only shows the amount of data in the current DCB.
    pub fn blockl(&self) -> u32 {
        self.0 & 0xffff
    }
    pub fn set_blockl(&mut self, value: u32) {
        assert!(value <= 0xffff);
        self.0 &= !0xffff;
        self.0 |= value;
    }
    /// Block offset in bytes, the value of this field is loaded from the DCB.
    pub fn blocko(&self) -> u32 {
        (self.0 & 0xfff00000) >> 20
    }
    pub fn set_blocko(&mut self, value: u32) {
        let value = value << 20;
        assert!(value <= 0xfff00000);
        self.0 &= !0xfff00000;
        self.0 |= value;
    }
    /// Set when the current DCB contains end-of-frame.
    pub fn eof(&self) -> u32 {
        (self.0 & 0x20000) >> 17
    }
    pub fn set_eof(&mut self, value: u32) {
        let value = value << 17;
        assert!(value <= 0x20000);
        self.0 &= !0x20000;
        self.0 |= value;
    }
    /// Pruned/Done indication.
    pub fn pd(&self) -> u32 {
        (self.0 & 0x80000) >> 19
    }
    pub fn set_pd(&mut self, value: u32) {
        let value = value << 19;
        assert!(value <= 0x80000);
        self.0 &= !0x80000;
        self.0 |= value;
    }
    /// Set when the current DCB contains start-of-frame.
    pub fn sof(&self) -> u32 {
        (self.0 & 0x10000) >> 16
    }
    pub fn set_sof(&mut self, value: u32) {
        let value = value << 16;
        assert!(value <= 0x10000);
        self.0 &= !0x10000;
        self.0 |= value;
    }
}
/// Error event
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_EVT_ERR(u32);
impl FDMA_EVT_ERR {
    /// Shows if an Error-event has occurred, there is one bit per channel. See ICPU_CFG::FDMA_EVT_ERR_CODE.EVT_ERR_CODE for description of errors for which the FDMA implements run-time checks.
    pub fn evt_err(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_evt_err(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// Additional error information
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_EVT_ERR_CODE(u32);
impl FDMA_EVT_ERR_CODE {
    /// This field shows information about Error-events that has been recorded by the FDMA, this can be used for software development and debugging. If multiple errors happen in succession, only the newest of the err-codes is shown.
    ///
    /// 0:Default (no error has occurred) 1:CH_ACTIVATE set for channel w. DCB_LLP==NULL 2:Got DCB w. DATAP==NULL 3:Got extraction DCB w. DATAL==0 4:Got extraction DCB w. DATAL<=BLOCKO 5:Got injection DCB w. BLOCKL==0 6:Got injection DCB w. SOF for already active channel 7:Activate attempted for channel w. error indication. 8:Activate attempted for channel enabled for manual mode. 9:Manual mode enabled for channel in active FDMA mode.
    pub fn evt_err_code(&self) -> u32 {
        self.0 & 0xf
    }
    pub fn set_evt_err_code(&mut self, value: u32) {
        assert!(value <= 0xf);
        self.0 &= !0xf;
        self.0 |= value;
    }
}
/// General FDMA configurations
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_GCFG(u32);
impl FDMA_GCFG {
    /// Injection resync FIFO fill-level watermark, when exceeded backpressure will be asserted towards SBA. The maximum fill-level for the FIFO is reported via ICPU_CFG::FDMA_GSTAT.INJ_RF_HIGH.
    ///
    /// n: backpressure when n+1 or more words in buffer.
    pub fn inj_rf_wm(&self) -> u32 {
        (self.0 & 0xf80) >> 7
    }
    pub fn set_inj_rf_wm(&mut self, value: u32) {
        let value = value << 7;
        assert!(value <= 0xf80);
        self.0 &= !0xf80;
        self.0 |= value;
    }
    /// Set this field to make the FDMA ignore the value of the DCB's PD field when injecting frames. By default the FDMA will treat the PD field in the same way as ABORT.
    pub fn pd_ignore(&self) -> u32 {
        self.0 & 0x1
    }
    pub fn set_pd_ignore(&mut self, value: u32) {
        assert!(value <= 0x1);
        self.0 &= !0x1;
        self.0 |= value;
    }
    /// Set this field to disable extending of data available indications in the extraction direction of the FDMA.
    pub fn xtr_avail_ext_dis(&self) -> u32 {
        (self.0 & 0x4) >> 2
    }
    pub fn set_xtr_avail_ext_dis(&mut self, value: u32) {
        let value = value << 2;
        assert!(value <= 0x4);
        self.0 &= !0x4;
        self.0 |= value;
    }
    /// Set this field to disable extraction group backpressure based on priorities of groups that has data available. By default the FDMA will assert backpressure on low priority extraction channels when a higher priority channel has data available.
    pub fn xtr_prio_bp_dis(&self) -> u32 {
        (self.0 & 0x2) >> 1
    }
    pub fn set_xtr_prio_bp_dis(&mut self, value: u32) {
        let value = value << 1;
        assert!(value <= 0x2);
        self.0 &= !0x2;
        self.0 |= value;
    }
    /// Extraction resync fifo fill-level watermark, when exceeded backpressure will be asserted towards towards QS. The maximum fill-level for the fifo is reported via ICPU_CFG::FDMA_GSTAT.XTR_RF_HIGH. This field must not be modified at the same time as frames are extracted through the FDMA.
    ///
    /// n: backpressure when n+1 or more words in buffer.
    pub fn xtr_rf_wm(&self) -> u32 {
        (self.0 & 0x78) >> 3
    }
    pub fn set_xtr_rf_wm(&mut self, value: u32) {
        let value = value << 3;
        assert!(value <= 0x78);
        self.0 &= !0x78;
        self.0 |= value;
    }
}
/// General FDMA status
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_GSTAT(u32);
impl FDMA_GSTAT {
    /// This field shows the highest fill level that the injection resync FIFO has experienced since reset of the injection logic. The depth of the FIFO is 32 words, reaching a fill-level of 33 (or more) means that overflow has occurred.
    pub fn inj_rf_high(&self) -> u32 {
        (self.0 & 0x7e0) >> 5
    }
    pub fn set_inj_rf_high(&mut self, value: u32) {
        let value = value << 5;
        assert!(value <= 0x7e0);
        self.0 &= !0x7e0;
        self.0 |= value;
    }
    /// This field shows the highest fill level that the extraction resync FIFO has experienced since reset of the extraction logic. The depth of the FIFO is 16 words, reaching a fill-level of 17 (or more) means that overflow has occurred. This field is read directly from other clock-domain; software must keep reading this field until same value has been returned by two consecutive read accesses.
    pub fn xtr_rf_high(&self) -> u32 {
        self.0 & 0x1f
    }
    pub fn set_xtr_rf_high(&mut self, value: u32) {
        assert!(value <= 0x1f);
        self.0 &= !0x1f;
        self.0 |= value;
    }
}
/// FDMA idle Counter
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_IDLECNT(u32);
impl FDMA_IDLECNT {
    /// The counter is reset whenever a channel is enabled and when FDMA moves frame data to or from the queuing system. When the FDMA is idle this counter is incremented once every 200ns. The counter saturates at maximum value (approx 3.3 seconds of idle time).
    pub fn idlecnt(&self) -> u32 {
        self.0 & 0xffffff
    }
    pub fn set_idlecnt(&mut self, value: u32) {
        assert!(value <= 0xffffff);
        self.0 &= !0xffffff;
        self.0 |= value;
    }
}
/// Channel interrupt enable
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_INTR_ENA(u32);
impl FDMA_INTR_ENA {
    /// Enables propagation of enabled channel LLP-event, FRM-event and SIG-event as interrupt, there is one bit per channel. ERR-events are always propagated when interrupt is enabled for a channel.
    pub fn intr_ena(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_intr_ena(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// FRM-event
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_INTR_FRM(u32);
impl FDMA_INTR_FRM {
    /// Shows if a FRM-event has occurred, there is one bit per channel. See the data sheet for information on when this event can occur.
    pub fn intr_frm(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_intr_frm(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// FRM-event interrupt enable
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_INTR_FRM_ENA(u32);
impl FDMA_INTR_FRM_ENA {
    /// Enables FRM-event to be propagated as interrupt, there is one bit per channel. See ICPU_CFG::FDMA_INTR_FRM.INTR_FRM for additional information.
    pub fn intr_frm_ena(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_intr_frm_ena(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// Currently interrupting channels
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_INTR_IDENT(u32);
impl FDMA_INTR_IDENT {
    /// Shows currently interrupting channels, there is one bit per channel.
    pub fn intr_ident(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_intr_ident(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// LLP-event
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_INTR_LLP(u32);
impl FDMA_INTR_LLP {
    /// Shows if an LLP-event has occurred, there is one bit per channel. See the data sheet for information on when this event can occur.
    pub fn intr_llp(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_intr_llp(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// LLP-event interrupt enable
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_INTR_LLP_ENA(u32);
impl FDMA_INTR_LLP_ENA {
    /// Enables LLP-event to be propagated as interrupt, there is one bit per channel. See ICPU_CFG::FDMA_INTR_LLP.INTR_LLP for additional information.
    pub fn intr_llp_ena(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_intr_llp_ena(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// SIG-event
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_INTR_SIG(u32);
impl FDMA_INTR_SIG {
    /// Shows if a SIG-event has occurred, there is one bit per channel. See the data sheet for information on when this event can occur.
    pub fn intr_sig(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_intr_sig(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
/// SIG-event interrupt enable
#[derive(Copy, Clone, Eq, PartialEq, From, Into)]
pub struct FDMA_INTR_SIG_ENA(u32);
impl FDMA_INTR_SIG_ENA {
    /// Enables SIG-event to be propagated as interrupt, there is one bit per channel. See ICPU_CFG::FDMA_INTR_SIG.INTR_SIG for additional information.
    pub fn intr_sig_ena(&self) -> u32 {
        self.0 & 0x3ff
    }
    pub fn set_intr_sig_ena(&mut self, value: u32) {
        assert!(value <= 0x3ff);
        self.0 &= !0x3ff;
        self.0 |= value;
    }
}
