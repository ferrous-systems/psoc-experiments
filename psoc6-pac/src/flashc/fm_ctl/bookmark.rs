#[doc = "Register `BOOKMARK` reader"]
pub struct R(crate::R<BOOKMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOKMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOKMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOKMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOKMARK` writer"]
pub struct W(crate::W<BOOKMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOKMARK_SPEC>;
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
impl From<crate::W<BOOKMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOKMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOKMARK` reader - Used by FW. Keeps the Current HV cycle sequence"]
pub type BOOKMARK_R = crate::FieldReader<u32>;
#[doc = "Field `BOOKMARK` writer - Used by FW. Keeps the Current HV cycle sequence"]
pub type BOOKMARK_W<'a, const O: u8> = crate::FieldWriter<'a, BOOKMARK_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    pub fn bookmark(&self) -> BOOKMARK_R {
        BOOKMARK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    #[must_use]
    pub fn bookmark(&mut self) -> BOOKMARK_W<0> {
        BOOKMARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bookmark register - keeps the current FW HV seq\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bookmark](index.html) module"]
pub struct BOOKMARK_SPEC;
impl crate::RegisterSpec for BOOKMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bookmark::R](R) reader structure"]
impl crate::Readable for BOOKMARK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bookmark::W](W) writer structure"]
impl crate::Writable for BOOKMARK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOKMARK to value 0"]
impl crate::Resettable for BOOKMARK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
