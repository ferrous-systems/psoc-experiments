#[doc = "Register `ANA_TRIM1` reader"]
pub struct R(crate::R<ANA_TRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_TRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_TRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_TRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_TRIM1` writer"]
pub struct W(crate::W<ANA_TRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_TRIM1_SPEC>;
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
impl From<crate::W<ANA_TRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_TRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_REF_BUF_TRIM` reader - SAR Reference buffer trim"]
pub type SAR_REF_BUF_TRIM_R = crate::FieldReader;
#[doc = "Field `SAR_REF_BUF_TRIM` writer - SAR Reference buffer trim"]
pub type SAR_REF_BUF_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, ANA_TRIM1_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    pub fn sar_ref_buf_trim(&self) -> SAR_REF_BUF_TRIM_R {
        SAR_REF_BUF_TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    #[must_use]
    pub fn sar_ref_buf_trim(&mut self) -> SAR_REF_BUF_TRIM_W<0> {
        SAR_REF_BUF_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog trim register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_trim1](index.html) module"]
pub struct ANA_TRIM1_SPEC;
impl crate::RegisterSpec for ANA_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_trim1::R](R) reader structure"]
impl crate::Readable for ANA_TRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_trim1::W](W) writer structure"]
impl crate::Writable for ANA_TRIM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_TRIM1 to value 0"]
impl crate::Resettable for ANA_TRIM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
