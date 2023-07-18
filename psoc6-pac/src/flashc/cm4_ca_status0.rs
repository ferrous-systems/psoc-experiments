#[doc = "Register `CM4_CA_STATUS0` reader"]
pub struct R(crate::R<CM4_CA_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_CA_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_CA_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_CA_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALID32` reader - See CM0_CA_STATUS0."]
pub type VALID32_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - See CM0_CA_STATUS0."]
    #[inline(always)]
    pub fn valid32(&self) -> VALID32_R {
        VALID32_R::new(self.bits)
    }
}
#[doc = "CM4 cache status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_ca_status0](index.html) module"]
pub struct CM4_CA_STATUS0_SPEC;
impl crate::RegisterSpec for CM4_CA_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_ca_status0::R](R) reader structure"]
impl crate::Readable for CM4_CA_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM4_CA_STATUS0 to value 0"]
impl crate::Resettable for CM4_CA_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
