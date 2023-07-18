#[doc = "Register `ACTIVE` reader"]
pub struct R(crate::R<ACTIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVE` reader - Specifies active channels; i.e. enabled channels whose trigger got activated."]
pub type ACTIVE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Specifies active channels; i.e. enabled channels whose trigger got activated."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Active channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active](index.html) module"]
pub struct ACTIVE_SPEC;
impl crate::RegisterSpec for ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [active::R](R) reader structure"]
impl crate::Readable for ACTIVE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIVE to value 0"]
impl crate::Resettable for ACTIVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
