#[doc = "Register `PW_SEQ23` reader"]
pub struct R(crate::R<PW_SEQ23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PW_SEQ23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PW_SEQ23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PW_SEQ23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PW_SEQ23` writer"]
pub struct W(crate::W<PW_SEQ23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PW_SEQ23_SPEC>;
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
impl From<crate::W<PW_SEQ23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PW_SEQ23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PW_SEQ2_POST` reader - Seq2 post delay"]
pub type PW_SEQ2_POST_R = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ2_POST` writer - Seq2 post delay"]
pub type PW_SEQ2_POST_W<'a, const O: u8> = crate::FieldWriter<'a, PW_SEQ23_SPEC, 16, O, u16>;
#[doc = "Field `PW_SEQ3` reader - Seq3 delay"]
pub type PW_SEQ3_R = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ3` writer - Seq3 delay"]
pub type PW_SEQ3_W<'a, const O: u8> = crate::FieldWriter<'a, PW_SEQ23_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Seq2 post delay"]
    #[inline(always)]
    pub fn pw_seq2_post(&self) -> PW_SEQ2_POST_R {
        PW_SEQ2_POST_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Seq3 delay"]
    #[inline(always)]
    pub fn pw_seq3(&self) -> PW_SEQ3_R {
        PW_SEQ3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Seq2 post delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq2_post(&mut self) -> PW_SEQ2_POST_W<0> {
        PW_SEQ2_POST_W::new(self)
    }
    #[doc = "Bits 16:31 - Seq3 delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq3(&mut self) -> PW_SEQ3_W<16> {
        PW_SEQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HV Pulse Delay for seq2 post &amp; seq3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pw_seq23](index.html) module"]
pub struct PW_SEQ23_SPEC;
impl crate::RegisterSpec for PW_SEQ23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pw_seq23::R](R) reader structure"]
impl crate::Readable for PW_SEQ23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pw_seq23::W](W) writer structure"]
impl crate::Writable for PW_SEQ23_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PW_SEQ23 to value 0"]
impl crate::Resettable for PW_SEQ23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
