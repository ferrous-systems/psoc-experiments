#[doc = "Register `NORMAL_INT_STAT_EN_R` reader"]
pub struct R(crate::R<NORMAL_INT_STAT_EN_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NORMAL_INT_STAT_EN_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NORMAL_INT_STAT_EN_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NORMAL_INT_STAT_EN_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NORMAL_INT_STAT_EN_R` writer"]
pub struct W(crate::W<NORMAL_INT_STAT_EN_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NORMAL_INT_STAT_EN_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NORMAL_INT_STAT_EN_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NORMAL_INT_STAT_EN_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_COMPLETE_STAT_EN` reader - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_COMPLETE_STAT_EN_R = crate::BitReader;
#[doc = "Field `CMD_COMPLETE_STAT_EN` writer - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_COMPLETE_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `XFER_COMPLETE_STAT_EN` reader - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type XFER_COMPLETE_STAT_EN_R = crate::BitReader;
#[doc = "Field `XFER_COMPLETE_STAT_EN` writer - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type XFER_COMPLETE_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `BGAP_EVENT_STAT_EN` reader - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BGAP_EVENT_STAT_EN_R = crate::BitReader;
#[doc = "Field `BGAP_EVENT_STAT_EN` writer - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BGAP_EVENT_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `DMA_INTERRUPT_STAT_EN` reader - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DMA_INTERRUPT_STAT_EN_R = crate::BitReader;
#[doc = "Field `DMA_INTERRUPT_STAT_EN` writer - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DMA_INTERRUPT_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `BUF_WR_READY_STAT_EN` reader - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BUF_WR_READY_STAT_EN_R = crate::BitReader;
#[doc = "Field `BUF_WR_READY_STAT_EN` writer - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BUF_WR_READY_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `BUF_RD_READY_STAT_EN` reader - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BUF_RD_READY_STAT_EN_R = crate::BitReader;
#[doc = "Field `BUF_RD_READY_STAT_EN` writer - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BUF_RD_READY_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `CARD_INSERTION_STAT_EN` reader - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_INSERTION_STAT_EN_R = crate::BitReader;
#[doc = "Field `CARD_INSERTION_STAT_EN` writer - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_INSERTION_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `CARD_REMOVAL_STAT_EN` reader - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_REMOVAL_STAT_EN_R = crate::BitReader;
#[doc = "Field `CARD_REMOVAL_STAT_EN` writer - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_REMOVAL_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `CARD_INTERRUPT_STAT_EN` reader - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_INTERRUPT_STAT_EN_R = crate::BitReader;
#[doc = "Field `CARD_INTERRUPT_STAT_EN` writer - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_INTERRUPT_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `INT_A_STAT_EN` reader - N/A"]
pub type INT_A_STAT_EN_R = crate::BitReader;
#[doc = "Field `INT_A_STAT_EN` writer - N/A"]
pub type INT_A_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `INT_B_STAT_EN` reader - N/A"]
pub type INT_B_STAT_EN_R = crate::BitReader;
#[doc = "Field `INT_B_STAT_EN` writer - N/A"]
pub type INT_B_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `INT_C_STAT_EN` reader - N/A"]
pub type INT_C_STAT_EN_R = crate::BitReader;
#[doc = "Field `INT_C_STAT_EN` writer - N/A"]
pub type INT_C_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `RE_TUNE_EVENT_STAT_EN` reader - N/A"]
pub type RE_TUNE_EVENT_STAT_EN_R = crate::BitReader;
#[doc = "Field `RE_TUNE_EVENT_STAT_EN` writer - N/A"]
pub type RE_TUNE_EVENT_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `FX_EVENT_STAT_EN` reader - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type FX_EVENT_STAT_EN_R = crate::BitReader;
#[doc = "Field `FX_EVENT_STAT_EN` writer - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type FX_EVENT_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
#[doc = "Field `CQE_EVENT_STAT_EN` reader - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CQE_EVENT_STAT_EN_R = crate::BitReader;
#[doc = "Field `CQE_EVENT_STAT_EN` writer - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CQE_EVENT_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, NORMAL_INT_STAT_EN_R_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_complete_stat_en(&self) -> CMD_COMPLETE_STAT_EN_R {
        CMD_COMPLETE_STAT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn xfer_complete_stat_en(&self) -> XFER_COMPLETE_STAT_EN_R {
        XFER_COMPLETE_STAT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn bgap_event_stat_en(&self) -> BGAP_EVENT_STAT_EN_R {
        BGAP_EVENT_STAT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn dma_interrupt_stat_en(&self) -> DMA_INTERRUPT_STAT_EN_R {
        DMA_INTERRUPT_STAT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready_stat_en(&self) -> BUF_WR_READY_STAT_EN_R {
        BUF_WR_READY_STAT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready_stat_en(&self) -> BUF_RD_READY_STAT_EN_R {
        BUF_RD_READY_STAT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_insertion_stat_en(&self) -> CARD_INSERTION_STAT_EN_R {
        CARD_INSERTION_STAT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_removal_stat_en(&self) -> CARD_REMOVAL_STAT_EN_R {
        CARD_REMOVAL_STAT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_interrupt_stat_en(&self) -> CARD_INTERRUPT_STAT_EN_R {
        CARD_INTERRUPT_STAT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn int_a_stat_en(&self) -> INT_A_STAT_EN_R {
        INT_A_STAT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn int_b_stat_en(&self) -> INT_B_STAT_EN_R {
        INT_B_STAT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn int_c_stat_en(&self) -> INT_C_STAT_EN_R {
        INT_C_STAT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn re_tune_event_stat_en(&self) -> RE_TUNE_EVENT_STAT_EN_R {
        RE_TUNE_EVENT_STAT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn fx_event_stat_en(&self) -> FX_EVENT_STAT_EN_R {
        FX_EVENT_STAT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cqe_event_stat_en(&self) -> CQE_EVENT_STAT_EN_R {
        CQE_EVENT_STAT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete_stat_en(&mut self) -> CMD_COMPLETE_STAT_EN_W<0> {
        CMD_COMPLETE_STAT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete_stat_en(&mut self) -> XFER_COMPLETE_STAT_EN_W<1> {
        XFER_COMPLETE_STAT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_event_stat_en(&mut self) -> BGAP_EVENT_STAT_EN_W<2> {
        BGAP_EVENT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dma_interrupt_stat_en(&mut self) -> DMA_INTERRUPT_STAT_EN_W<3> {
        DMA_INTERRUPT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ready_stat_en(&mut self) -> BUF_WR_READY_STAT_EN_W<4> {
        BUF_WR_READY_STAT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ready_stat_en(&mut self) -> BUF_RD_READY_STAT_EN_W<5> {
        BUF_RD_READY_STAT_EN_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_insertion_stat_en(&mut self) -> CARD_INSERTION_STAT_EN_W<6> {
        CARD_INSERTION_STAT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal_stat_en(&mut self) -> CARD_REMOVAL_STAT_EN_W<7> {
        CARD_REMOVAL_STAT_EN_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_interrupt_stat_en(&mut self) -> CARD_INTERRUPT_STAT_EN_W<8> {
        CARD_INTERRUPT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_a_stat_en(&mut self) -> INT_A_STAT_EN_W<9> {
        INT_A_STAT_EN_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_b_stat_en(&mut self) -> INT_B_STAT_EN_W<10> {
        INT_B_STAT_EN_W::new(self)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_c_stat_en(&mut self) -> INT_C_STAT_EN_W<11> {
        INT_C_STAT_EN_W::new(self)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn re_tune_event_stat_en(&mut self) -> RE_TUNE_EVENT_STAT_EN_W<12> {
        RE_TUNE_EVENT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 13 - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fx_event_stat_en(&mut self) -> FX_EVENT_STAT_EN_W<13> {
        FX_EVENT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 14 - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_event_stat_en(&mut self) -> CQE_EVENT_STAT_EN_W<14> {
        CQE_EVENT_STAT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [normal_int_stat_en_r](index.html) module"]
pub struct NORMAL_INT_STAT_EN_R_SPEC;
impl crate::RegisterSpec for NORMAL_INT_STAT_EN_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [normal_int_stat_en_r::R](R) reader structure"]
impl crate::Readable for NORMAL_INT_STAT_EN_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [normal_int_stat_en_r::W](W) writer structure"]
impl crate::Writable for NORMAL_INT_STAT_EN_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NORMAL_INT_STAT_EN_R to value 0"]
impl crate::Resettable for NORMAL_INT_STAT_EN_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
