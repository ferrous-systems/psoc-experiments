#[doc = "Register `PERIOD` reader"]
pub struct R(crate::R<PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIOD` writer"]
pub struct W(crate::W<PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIOD_SPEC>;
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
impl From<crate::W<PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
pub type PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `PERIOD` writer - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, PERIOD_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<0> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period](index.html) module"]
pub struct PERIOD_SPEC;
impl crate::RegisterSpec for PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [period::R](R) reader structure"]
impl crate::Readable for PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [period::W](W) writer structure"]
impl crate::Writable for PERIOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIOD to value 0xffff_ffff"]
impl crate::Resettable for PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
