#[doc = "Register `DST` reader"]
pub struct R(crate::R<DST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Current address of destination location."]
pub type ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of destination location."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "Channel current destination address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst](index.html) module"]
pub struct DST_SPEC;
impl crate::RegisterSpec for DST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst::R](R) reader structure"]
impl crate::Readable for DST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DST to value 0"]
impl crate::Resettable for DST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
