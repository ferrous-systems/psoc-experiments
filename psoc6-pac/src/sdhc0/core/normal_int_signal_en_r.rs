#[doc = "Register `NORMAL_INT_SIGNAL_EN_R` reader"]
pub struct R(crate::R<NORMAL_INT_SIGNAL_EN_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NORMAL_INT_SIGNAL_EN_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NORMAL_INT_SIGNAL_EN_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NORMAL_INT_SIGNAL_EN_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NORMAL_INT_SIGNAL_EN_R` writer"]
pub struct W(crate::W<NORMAL_INT_SIGNAL_EN_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NORMAL_INT_SIGNAL_EN_R_SPEC>;
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
impl From<crate::W<NORMAL_INT_SIGNAL_EN_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NORMAL_INT_SIGNAL_EN_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_COMPLETE_SIGNAL_EN` reader - Command Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_COMPLETE_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CMD_COMPLETE_SIGNAL_EN` writer - Command Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_COMPLETE_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `XFER_COMPLETE_SIGNAL_EN` reader - Transfer Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type XFER_COMPLETE_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `XFER_COMPLETE_SIGNAL_EN` writer - Transfer Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type XFER_COMPLETE_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `BGAP_EVENT_SIGNAL_EN` reader - Block Gap Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BGAP_EVENT_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `BGAP_EVENT_SIGNAL_EN` writer - Block Gap Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BGAP_EVENT_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `DMA_INTERRUPT_SIGNAL_EN` reader - DMA Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DMA_INTERRUPT_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `DMA_INTERRUPT_SIGNAL_EN` writer - DMA Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DMA_INTERRUPT_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `BUF_WR_READY_SIGNAL_EN` reader - Buffer Write Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BUF_WR_READY_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `BUF_WR_READY_SIGNAL_EN` writer - Buffer Write Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BUF_WR_READY_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `BUF_RD_READY_SIGNAL_EN` reader - Buffer Read Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BUF_RD_READY_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `BUF_RD_READY_SIGNAL_EN` writer - Buffer Read Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BUF_RD_READY_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `CARD_INSERTION_SIGNAL_EN` reader - Card Insertion Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_INSERTION_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CARD_INSERTION_SIGNAL_EN` writer - Card Insertion Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_INSERTION_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `CARD_REMOVAL_SIGNAL_EN` reader - Card Removal Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_REMOVAL_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CARD_REMOVAL_SIGNAL_EN` writer - Card Removal Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_REMOVAL_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `CARD_INTERRUPT_SIGNAL_EN` reader - Card Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_INTERRUPT_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CARD_INTERRUPT_SIGNAL_EN` writer - Card Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CARD_INTERRUPT_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `INT_A_SIGNAL_EN` reader - N/A"]
pub type INT_A_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `INT_A_SIGNAL_EN` writer - N/A"]
pub type INT_A_SIGNAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `INT_B_SIGNAL_EN` reader - N/A"]
pub type INT_B_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `INT_B_SIGNAL_EN` writer - N/A"]
pub type INT_B_SIGNAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `INT_C_SIGNAL_EN` reader - N/A"]
pub type INT_C_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `INT_C_SIGNAL_EN` writer - N/A"]
pub type INT_C_SIGNAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `RE_TUNE_EVENT_SIGNAL_EN` reader - N/A"]
pub type RE_TUNE_EVENT_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `RE_TUNE_EVENT_SIGNAL_EN` writer - N/A"]
pub type RE_TUNE_EVENT_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `FX_EVENT_SIGNAL_EN` reader - FX Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type FX_EVENT_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `FX_EVENT_SIGNAL_EN` writer - FX Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type FX_EVENT_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `CQE_EVENT_SIGNAL_EN` reader - Command Queuing Engine Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CQE_EVENT_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CQE_EVENT_SIGNAL_EN` writer - Command Queuing Engine Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CQE_EVENT_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, NORMAL_INT_SIGNAL_EN_R_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_complete_signal_en(&self) -> CMD_COMPLETE_SIGNAL_EN_R {
        CMD_COMPLETE_SIGNAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn xfer_complete_signal_en(&self) -> XFER_COMPLETE_SIGNAL_EN_R {
        XFER_COMPLETE_SIGNAL_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn bgap_event_signal_en(&self) -> BGAP_EVENT_SIGNAL_EN_R {
        BGAP_EVENT_SIGNAL_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn dma_interrupt_signal_en(&self) -> DMA_INTERRUPT_SIGNAL_EN_R {
        DMA_INTERRUPT_SIGNAL_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready_signal_en(&self) -> BUF_WR_READY_SIGNAL_EN_R {
        BUF_WR_READY_SIGNAL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready_signal_en(&self) -> BUF_RD_READY_SIGNAL_EN_R {
        BUF_RD_READY_SIGNAL_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_insertion_signal_en(&self) -> CARD_INSERTION_SIGNAL_EN_R {
        CARD_INSERTION_SIGNAL_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_removal_signal_en(&self) -> CARD_REMOVAL_SIGNAL_EN_R {
        CARD_REMOVAL_SIGNAL_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_interrupt_signal_en(&self) -> CARD_INTERRUPT_SIGNAL_EN_R {
        CARD_INTERRUPT_SIGNAL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn int_a_signal_en(&self) -> INT_A_SIGNAL_EN_R {
        INT_A_SIGNAL_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn int_b_signal_en(&self) -> INT_B_SIGNAL_EN_R {
        INT_B_SIGNAL_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn int_c_signal_en(&self) -> INT_C_SIGNAL_EN_R {
        INT_C_SIGNAL_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn re_tune_event_signal_en(&self) -> RE_TUNE_EVENT_SIGNAL_EN_R {
        RE_TUNE_EVENT_SIGNAL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FX Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn fx_event_signal_en(&self) -> FX_EVENT_SIGNAL_EN_R {
        FX_EVENT_SIGNAL_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Command Queuing Engine Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cqe_event_signal_en(&self) -> CQE_EVENT_SIGNAL_EN_R {
        CQE_EVENT_SIGNAL_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete_signal_en(&mut self) -> CMD_COMPLETE_SIGNAL_EN_W<0> {
        CMD_COMPLETE_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete_signal_en(&mut self) -> XFER_COMPLETE_SIGNAL_EN_W<1> {
        XFER_COMPLETE_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_event_signal_en(&mut self) -> BGAP_EVENT_SIGNAL_EN_W<2> {
        BGAP_EVENT_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dma_interrupt_signal_en(&mut self) -> DMA_INTERRUPT_SIGNAL_EN_W<3> {
        DMA_INTERRUPT_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ready_signal_en(&mut self) -> BUF_WR_READY_SIGNAL_EN_W<4> {
        BUF_WR_READY_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ready_signal_en(&mut self) -> BUF_RD_READY_SIGNAL_EN_W<5> {
        BUF_RD_READY_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_insertion_signal_en(&mut self) -> CARD_INSERTION_SIGNAL_EN_W<6> {
        CARD_INSERTION_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal_signal_en(&mut self) -> CARD_REMOVAL_SIGNAL_EN_W<7> {
        CARD_REMOVAL_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_interrupt_signal_en(&mut self) -> CARD_INTERRUPT_SIGNAL_EN_W<8> {
        CARD_INTERRUPT_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_a_signal_en(&mut self) -> INT_A_SIGNAL_EN_W<9> {
        INT_A_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_b_signal_en(&mut self) -> INT_B_SIGNAL_EN_W<10> {
        INT_B_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_c_signal_en(&mut self) -> INT_C_SIGNAL_EN_W<11> {
        INT_C_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn re_tune_event_signal_en(&mut self) -> RE_TUNE_EVENT_SIGNAL_EN_W<12> {
        RE_TUNE_EVENT_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 13 - FX Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fx_event_signal_en(&mut self) -> FX_EVENT_SIGNAL_EN_W<13> {
        FX_EVENT_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 14 - Command Queuing Engine Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_event_signal_en(&mut self) -> CQE_EVENT_SIGNAL_EN_W<14> {
        CQE_EVENT_SIGNAL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Normal Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [normal_int_signal_en_r](index.html) module"]
pub struct NORMAL_INT_SIGNAL_EN_R_SPEC;
impl crate::RegisterSpec for NORMAL_INT_SIGNAL_EN_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [normal_int_signal_en_r::R](R) reader structure"]
impl crate::Readable for NORMAL_INT_SIGNAL_EN_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [normal_int_signal_en_r::W](W) writer structure"]
impl crate::Writable for NORMAL_INT_SIGNAL_EN_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NORMAL_INT_SIGNAL_EN_R to value 0"]
impl crate::Resettable for NORMAL_INT_SIGNAL_EN_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
