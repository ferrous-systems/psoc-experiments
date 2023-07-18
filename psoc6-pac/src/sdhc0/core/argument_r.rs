#[doc = "Register `ARGUMENT_R` reader"]
pub struct R(crate::R<ARGUMENT_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARGUMENT_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARGUMENT_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARGUMENT_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARGUMENT_R` writer"]
pub struct W(crate::W<ARGUMENT_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARGUMENT_R_SPEC>;
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
impl From<crate::W<ARGUMENT_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARGUMENT_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARGUMENT` reader - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
pub type ARGUMENT_R = crate::FieldReader<u32>;
#[doc = "Field `ARGUMENT` writer - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
pub type ARGUMENT_W<'a, const O: u8> = crate::FieldWriter<'a, ARGUMENT_R_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    pub fn argument(&self) -> ARGUMENT_R {
        ARGUMENT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    #[must_use]
    pub fn argument(&mut self) -> ARGUMENT_W<0> {
        ARGUMENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argument_r](index.html) module"]
pub struct ARGUMENT_R_SPEC;
impl crate::RegisterSpec for ARGUMENT_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [argument_r::R](R) reader structure"]
impl crate::Readable for ARGUMENT_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [argument_r::W](W) writer structure"]
impl crate::Writable for ARGUMENT_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARGUMENT_R to value 0"]
impl crate::Resettable for ARGUMENT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
