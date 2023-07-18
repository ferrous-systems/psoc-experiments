#[doc = "Register `DESCR_SRC` reader"]
pub struct R(crate::R<DESCR_SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Base address of source location."]
pub type ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of source location."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Channel descriptor source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_src](index.html) module"]
pub struct DESCR_SRC_SPEC;
impl crate::RegisterSpec for DESCR_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr_src::R](R) reader structure"]
impl crate::Readable for DESCR_SRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESCR_SRC to value 0"]
impl crate::Resettable for DESCR_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
