#[doc = "Register `RESP67_R` reader"]
pub struct R(crate::R<RESP67_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP67_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP67_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP67_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESP67` reader - Command Response These bits reflect bits 135-104 of SD/EMMC Response Field. Note: For Auto CMD, this register also reflects the 32-bit response (bits 39-8 of the Response Field)."]
pub type RESP67_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect bits 135-104 of SD/EMMC Response Field. Note: For Auto CMD, this register also reflects the 32-bit response (bits 39-8 of the Response Field)."]
    #[inline(always)]
    pub fn resp67(&self) -> RESP67_R {
        RESP67_R::new(self.bits)
    }
}
#[doc = "Response Register 6/7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp67_r](index.html) module"]
pub struct RESP67_R_SPEC;
impl crate::RegisterSpec for RESP67_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp67_r::R](R) reader structure"]
impl crate::Readable for RESP67_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP67_R to value 0"]
impl crate::Resettable for RESP67_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
