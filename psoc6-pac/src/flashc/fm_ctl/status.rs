#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_ENABLED` reader - This is the timer_en bit set by writing a '1' in the TIMER_CTL bit 31. It is reset by HW when the timer expires 0: timer not running 1: Timer is enabled and not expired yet"]
pub type TIMER_ENABLED_R = crate::BitReader;
#[doc = "Field `HV_REGS_ISOLATED` reader - Indicates the isolation status at HV trim and redundancy registers inputs 0: Not isolated, writing permitted 1: isolated writing disabled"]
pub type HV_REGS_ISOLATED_R = crate::BitReader;
#[doc = "Field `ILLEGAL_HVOP` reader - Indicates a bulk, sector erase, program has been requested when axa=1 0: no error 1: illegal HV operation error"]
pub type ILLEGAL_HVOP_R = crate::BitReader;
#[doc = "Field `TURBO_N` reader - After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. 0: turbo mode 1: normal mode"]
pub type TURBO_N_R = crate::BitReader;
#[doc = "Field `WR_EN_MON` reader - FM_CTL.WR_EN bit after being synchronized in clk_r domain"]
pub type WR_EN_MON_R = crate::BitReader;
#[doc = "Field `IF_SEL_MON` reader - FM_CTL.IF_SEL bit after being synchronized in clk_r domain"]
pub type IF_SEL_MON_R = crate::BitReader;
#[doc = "Field `TIMER_STATUS` reader - The actual timer state sync-ed in clk_c domain: 0: timer is not running: 1: timer is running;"]
pub type TIMER_STATUS_R = crate::BitReader;
#[doc = "Field `R_GRANT_DELAY_STATUS` reader - 0: R_GRANT_DELAY timer is not running 1: R_GRANT_DELAY timer is running"]
pub type R_GRANT_DELAY_STATUS_R = crate::BitReader;
#[doc = "Field `FM_BUSY` reader - 0': FM not busy 1: FM BUSY : R_GRANT is 0 as result of a busy request from FM ready, or from HV operations."]
pub type FM_BUSY_R = crate::BitReader;
#[doc = "Field `FM_READY` reader - 0: FM not ready 1: FM ready"]
pub type FM_READY_R = crate::BitReader;
#[doc = "Field `POS_PUMP_VLO` reader - POS pump VLO"]
pub type POS_PUMP_VLO_R = crate::BitReader;
#[doc = "Field `NEG_PUMP_VHI` reader - NEG pump VHI"]
pub type NEG_PUMP_VHI_R = crate::BitReader;
#[doc = "Field `RWW` reader - FM Type (Read While Write or Not Read While Write): 0: Non RWW FM Type 1: RWW FM Type"]
pub type RWW_R = crate::BitReader;
#[doc = "Field `MAX_DOUT_WIDTH` reader - Internal memory core max data out size (number of data out bits per column): 0: x128 bits 1: x256 bits"]
pub type MAX_DOUT_WIDTH_R = crate::BitReader;
#[doc = "Field `SECTOR0_SR` reader - 0: Sector 0 does not contain special rows. The special rows are located in separate special sectors. 1: Sector 0 contains special rows"]
pub type SECTOR0_SR_R = crate::BitReader;
#[doc = "Field `RESET_MM` reader - Test_only, internal node: mpcon reset_mm"]
pub type RESET_MM_R = crate::BitReader;
#[doc = "Field `ROW_ODD` reader - Test_only, internal node: mpcon row_odd"]
pub type ROW_ODD_R = crate::BitReader;
#[doc = "Field `ROW_EVEN` reader - Test_only, internal node: mpcon row_even"]
pub type ROW_EVEN_R = crate::BitReader;
#[doc = "Field `HVOP_SUB_SECTOR_N` reader - Test_only, internal node: mpcon bk_subb"]
pub type HVOP_SUB_SECTOR_N_R = crate::BitReader;
#[doc = "Field `HVOP_SECTOR` reader - Test_only, internal node: mpcon bk_sec"]
pub type HVOP_SECTOR_R = crate::BitReader;
#[doc = "Field `HVOP_BULK_ALL` reader - Test_only, internal node: mpcon bk_all"]
pub type HVOP_BULK_ALL_R = crate::BitReader;
#[doc = "Field `CBUS_RA_MATCH` reader - Test_only, internal node: mpcon ra match"]
pub type CBUS_RA_MATCH_R = crate::BitReader;
#[doc = "Field `CBUS_RED_ROW_EN` reader - Test_only, internal node: mpcon red_row_en"]
pub type CBUS_RED_ROW_EN_R = crate::BitReader;
#[doc = "Field `RQ_ERROR` reader - Test_only, internal node: rq_error sync-de in clk_c domain"]
pub type RQ_ERROR_R = crate::BitReader;
#[doc = "Field `PUMP_PDAC` reader - Test_only, internal node: regif pdac outputs to pos pump"]
pub type PUMP_PDAC_R = crate::FieldReader;
#[doc = "Field `PUMP_NDAC` reader - Test_only, internal node: regif ndac outputs to pos pump"]
pub type PUMP_NDAC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This is the timer_en bit set by writing a '1' in the TIMER_CTL bit 31. It is reset by HW when the timer expires 0: timer not running 1: Timer is enabled and not expired yet"]
    #[inline(always)]
    pub fn timer_enabled(&self) -> TIMER_ENABLED_R {
        TIMER_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the isolation status at HV trim and redundancy registers inputs 0: Not isolated, writing permitted 1: isolated writing disabled"]
    #[inline(always)]
    pub fn hv_regs_isolated(&self) -> HV_REGS_ISOLATED_R {
        HV_REGS_ISOLATED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates a bulk, sector erase, program has been requested when axa=1 0: no error 1: illegal HV operation error"]
    #[inline(always)]
    pub fn illegal_hvop(&self) -> ILLEGAL_HVOP_R {
        ILLEGAL_HVOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - After FM power up indicates the analog blocks currents are boosted to faster reach their functional state.. Used in the testchip boot only as an 'FM READY' flag. 0: turbo mode 1: normal mode"]
    #[inline(always)]
    pub fn turbo_n(&self) -> TURBO_N_R {
        TURBO_N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FM_CTL.WR_EN bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn wr_en_mon(&self) -> WR_EN_MON_R {
        WR_EN_MON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FM_CTL.IF_SEL bit after being synchronized in clk_r domain"]
    #[inline(always)]
    pub fn if_sel_mon(&self) -> IF_SEL_MON_R {
        IF_SEL_MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The actual timer state sync-ed in clk_c domain: 0: timer is not running: 1: timer is running;"]
    #[inline(always)]
    pub fn timer_status(&self) -> TIMER_STATUS_R {
        TIMER_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: R_GRANT_DELAY timer is not running 1: R_GRANT_DELAY timer is running"]
    #[inline(always)]
    pub fn r_grant_delay_status(&self) -> R_GRANT_DELAY_STATUS_R {
        R_GRANT_DELAY_STATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0': FM not busy 1: FM BUSY : R_GRANT is 0 as result of a busy request from FM ready, or from HV operations."]
    #[inline(always)]
    pub fn fm_busy(&self) -> FM_BUSY_R {
        FM_BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: FM not ready 1: FM ready"]
    #[inline(always)]
    pub fn fm_ready(&self) -> FM_READY_R {
        FM_READY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - POS pump VLO"]
    #[inline(always)]
    pub fn pos_pump_vlo(&self) -> POS_PUMP_VLO_R {
        POS_PUMP_VLO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NEG pump VHI"]
    #[inline(always)]
    pub fn neg_pump_vhi(&self) -> NEG_PUMP_VHI_R {
        NEG_PUMP_VHI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FM Type (Read While Write or Not Read While Write): 0: Non RWW FM Type 1: RWW FM Type"]
    #[inline(always)]
    pub fn rww(&self) -> RWW_R {
        RWW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Internal memory core max data out size (number of data out bits per column): 0: x128 bits 1: x256 bits"]
    #[inline(always)]
    pub fn max_dout_width(&self) -> MAX_DOUT_WIDTH_R {
        MAX_DOUT_WIDTH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 0: Sector 0 does not contain special rows. The special rows are located in separate special sectors. 1: Sector 0 contains special rows"]
    #[inline(always)]
    pub fn sector0_sr(&self) -> SECTOR0_SR_R {
        SECTOR0_SR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Test_only, internal node: mpcon reset_mm"]
    #[inline(always)]
    pub fn reset_mm(&self) -> RESET_MM_R {
        RESET_MM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Test_only, internal node: mpcon row_odd"]
    #[inline(always)]
    pub fn row_odd(&self) -> ROW_ODD_R {
        ROW_ODD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Test_only, internal node: mpcon row_even"]
    #[inline(always)]
    pub fn row_even(&self) -> ROW_EVEN_R {
        ROW_EVEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Test_only, internal node: mpcon bk_subb"]
    #[inline(always)]
    pub fn hvop_sub_sector_n(&self) -> HVOP_SUB_SECTOR_N_R {
        HVOP_SUB_SECTOR_N_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Test_only, internal node: mpcon bk_sec"]
    #[inline(always)]
    pub fn hvop_sector(&self) -> HVOP_SECTOR_R {
        HVOP_SECTOR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Test_only, internal node: mpcon bk_all"]
    #[inline(always)]
    pub fn hvop_bulk_all(&self) -> HVOP_BULK_ALL_R {
        HVOP_BULK_ALL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Test_only, internal node: mpcon ra match"]
    #[inline(always)]
    pub fn cbus_ra_match(&self) -> CBUS_RA_MATCH_R {
        CBUS_RA_MATCH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Test_only, internal node: mpcon red_row_en"]
    #[inline(always)]
    pub fn cbus_red_row_en(&self) -> CBUS_RED_ROW_EN_R {
        CBUS_RED_ROW_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Test_only, internal node: rq_error sync-de in clk_c domain"]
    #[inline(always)]
    pub fn rq_error(&self) -> RQ_ERROR_R {
        RQ_ERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Test_only, internal node: regif pdac outputs to pos pump"]
    #[inline(always)]
    pub fn pump_pdac(&self) -> PUMP_PDAC_R {
        PUMP_PDAC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Test_only, internal node: regif ndac outputs to pos pump"]
    #[inline(always)]
    pub fn pump_ndac(&self) -> PUMP_NDAC_R {
        PUMP_NDAC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x1800"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x1800;
}
