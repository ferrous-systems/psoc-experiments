#[doc = "Register `RESP23_R` reader"]
pub struct R(crate::R<RESP23_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP23_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP23_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP23_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESP23` reader - Command Response These bits reflect 71-40 bits of the SD/eMMC Response"]
pub type RESP23_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 71-40 bits of the SD/eMMC Response"]
    #[inline(always)]
    pub fn resp23(&self) -> RESP23_R {
        RESP23_R::new(self.bits)
    }
}
#[doc = "Response Register 2/3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp23_r](index.html) module"]
pub struct RESP23_R_SPEC;
impl crate::RegisterSpec for RESP23_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp23_r::R](R) reader structure"]
impl crate::Readable for RESP23_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP23_R to value 0"]
impl crate::Resettable for RESP23_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
