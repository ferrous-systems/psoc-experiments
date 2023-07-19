#[doc = "Register `DATA2[%s]` reader"]
pub struct R(crate::R<DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA2[%s]` writer"]
pub struct W(crate::W<DATA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA2_SPEC>;
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
impl From<crate::W<DATA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, DATA2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Pin Data Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data2](index.html) module"]
pub struct DATA2_SPEC;
impl crate::RegisterSpec for DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data2::R](R) reader structure"]
impl crate::Readable for DATA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data2::W](W) writer structure"]
impl crate::Writable for DATA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA2[%s]
to value 0"]
impl crate::Resettable for DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
