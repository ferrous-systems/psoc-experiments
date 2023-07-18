#[doc = "Register `INTR_MASK` reader"]
pub struct R(crate::R<INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_MASK` writer"]
pub struct W(crate::W<INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_MASK_SPEC>;
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
impl From<crate::W<INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPLETION` reader - Mask for INTR.COMPLETION interrupt."]
pub type COMPLETION_R = crate::BitReader;
#[doc = "Field `COMPLETION` writer - Mask for INTR.COMPLETION interrupt."]
pub type COMPLETION_W<'a, const O: u8> = crate::BitWriter<'a, INTR_MASK_SPEC, O>;
#[doc = "Field `SRC_BUS_ERROR` reader - Mask for INTR.SRC_BUS_ERROR interrupt."]
pub type SRC_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `SRC_BUS_ERROR` writer - Mask for INTR.SRC_BUS_ERROR interrupt."]
pub type SRC_BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, INTR_MASK_SPEC, O>;
#[doc = "Field `DST_BUS_ERROR` reader - Mask for INTR.DST_BUS_ERROR interrupt."]
pub type DST_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `DST_BUS_ERROR` writer - Mask for INTR.DST_BUS_ERROR interrupt."]
pub type DST_BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, INTR_MASK_SPEC, O>;
#[doc = "Field `SRC_MISAL` reader - Mask for INTR.SRC_MISAL interrupt."]
pub type SRC_MISAL_R = crate::BitReader;
#[doc = "Field `SRC_MISAL` writer - Mask for INTR.SRC_MISAL interrupt."]
pub type SRC_MISAL_W<'a, const O: u8> = crate::BitWriter<'a, INTR_MASK_SPEC, O>;
#[doc = "Field `DST_MISAL` reader - Mask for INTR.DST_MISAL interrupt."]
pub type DST_MISAL_R = crate::BitReader;
#[doc = "Field `DST_MISAL` writer - Mask for INTR.DST_MISAL interrupt."]
pub type DST_MISAL_W<'a, const O: u8> = crate::BitWriter<'a, INTR_MASK_SPEC, O>;
#[doc = "Field `CURR_PTR_NULL` reader - Mask for INTR.CURR_PTR_NULL interrupt."]
pub type CURR_PTR_NULL_R = crate::BitReader;
#[doc = "Field `CURR_PTR_NULL` writer - Mask for INTR.CURR_PTR_NULL interrupt."]
pub type CURR_PTR_NULL_W<'a, const O: u8> = crate::BitWriter<'a, INTR_MASK_SPEC, O>;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - Mask for INTR.ACTIVE_CH_DISABLED interrupt."]
pub type ACTIVE_CH_DISABLED_R = crate::BitReader;
#[doc = "Field `ACTIVE_CH_DISABLED` writer - Mask for INTR.ACTIVE_CH_DISABLED interrupt."]
pub type ACTIVE_CH_DISABLED_W<'a, const O: u8> = crate::BitWriter<'a, INTR_MASK_SPEC, O>;
#[doc = "Field `DESCR_BUS_ERROR` reader - Mask for INTR.DESCR_BUS_ERROR interrupt."]
pub type DESCR_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `DESCR_BUS_ERROR` writer - Mask for INTR.DESCR_BUS_ERROR interrupt."]
pub type DESCR_BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, INTR_MASK_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Mask for INTR.COMPLETION interrupt."]
    #[inline(always)]
    pub fn completion(&self) -> COMPLETION_R {
        COMPLETION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask for INTR.SRC_BUS_ERROR interrupt."]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SRC_BUS_ERROR_R {
        SRC_BUS_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask for INTR.DST_BUS_ERROR interrupt."]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DST_BUS_ERROR_R {
        DST_BUS_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask for INTR.SRC_MISAL interrupt."]
    #[inline(always)]
    pub fn src_misal(&self) -> SRC_MISAL_R {
        SRC_MISAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask for INTR.DST_MISAL interrupt."]
    #[inline(always)]
    pub fn dst_misal(&self) -> DST_MISAL_R {
        DST_MISAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask for INTR.CURR_PTR_NULL interrupt."]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CURR_PTR_NULL_R {
        CURR_PTR_NULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask for INTR.ACTIVE_CH_DISABLED interrupt."]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ACTIVE_CH_DISABLED_R {
        ACTIVE_CH_DISABLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask for INTR.DESCR_BUS_ERROR interrupt."]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DESCR_BUS_ERROR_R {
        DESCR_BUS_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for INTR.COMPLETION interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn completion(&mut self) -> COMPLETION_W<0> {
        COMPLETION_W::new(self)
    }
    #[doc = "Bit 1 - Mask for INTR.SRC_BUS_ERROR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn src_bus_error(&mut self) -> SRC_BUS_ERROR_W<1> {
        SRC_BUS_ERROR_W::new(self)
    }
    #[doc = "Bit 2 - Mask for INTR.DST_BUS_ERROR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dst_bus_error(&mut self) -> DST_BUS_ERROR_W<2> {
        DST_BUS_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Mask for INTR.SRC_MISAL interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn src_misal(&mut self) -> SRC_MISAL_W<3> {
        SRC_MISAL_W::new(self)
    }
    #[doc = "Bit 4 - Mask for INTR.DST_MISAL interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dst_misal(&mut self) -> DST_MISAL_W<4> {
        DST_MISAL_W::new(self)
    }
    #[doc = "Bit 5 - Mask for INTR.CURR_PTR_NULL interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn curr_ptr_null(&mut self) -> CURR_PTR_NULL_W<5> {
        CURR_PTR_NULL_W::new(self)
    }
    #[doc = "Bit 6 - Mask for INTR.ACTIVE_CH_DISABLED interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn active_ch_disabled(&mut self) -> ACTIVE_CH_DISABLED_W<6> {
        ACTIVE_CH_DISABLED_W::new(self)
    }
    #[doc = "Bit 7 - Mask for INTR.DESCR_BUS_ERROR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn descr_bus_error(&mut self) -> DESCR_BUS_ERROR_W<7> {
        DESCR_BUS_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
