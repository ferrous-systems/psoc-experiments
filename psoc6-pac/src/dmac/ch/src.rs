#[doc = "Register `SRC` reader"]
pub struct R(crate::R<SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Current address of source location."]
pub type ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of source location."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Channel current source address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src](index.html) module"]
pub struct SRC_SPEC;
impl crate::RegisterSpec for SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src::R](R) reader structure"]
impl crate::Readable for SRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRC to value 0"]
impl crate::Resettable for SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
