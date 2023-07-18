#[doc = "Register `DESCR_NEXT` reader"]
pub struct R(crate::R<DESCR_NEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_NEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_NEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_NEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PTR` reader - Address of next descriptor in descriptor list. When this field is '0', this is the last descriptor in the descriptor list."]
pub type PTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - Address of next descriptor in descriptor list. When this field is '0', this is the last descriptor in the descriptor list."]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[doc = "Channel descriptor next pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_next](index.html) module"]
pub struct DESCR_NEXT_SPEC;
impl crate::RegisterSpec for DESCR_NEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr_next::R](R) reader structure"]
impl crate::Readable for DESCR_NEXT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESCR_NEXT to value 0"]
impl crate::Resettable for DESCR_NEXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
