#[doc = "Register `SPARE` reader"]
pub struct R(crate::R<SPARE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPARE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPARE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPARE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPARE` writer"]
pub struct W(crate::W<SPARE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPARE_SPEC>;
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
impl From<crate::W<SPARE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPARE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPARE` reader - Spare MMIO"]
pub type SPARE_R = crate::FieldReader;
#[doc = "Field `SPARE` writer - Spare MMIO"]
pub type SPARE_W<'a, const O: u8> = crate::FieldWriter<'a, SPARE_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Spare MMIO"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Spare MMIO"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<0> {
        SPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spare MMIO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spare](index.html) module"]
pub struct SPARE_SPEC;
impl crate::RegisterSpec for SPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spare::R](R) reader structure"]
impl crate::Readable for SPARE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spare::W](W) writer structure"]
impl crate::Writable for SPARE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPARE to value 0"]
impl crate::Resettable for SPARE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
