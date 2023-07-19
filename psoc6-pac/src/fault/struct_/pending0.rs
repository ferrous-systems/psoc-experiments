#[doc = "Register `PENDING0` reader"]
pub struct R(crate::R<PENDING0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDING0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDING0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDING0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOURCE` reader - This field specifies the following sources: Bit 0: CM0 MPU. Bit 1: CRYPTO MPU. Bit 2: DW 0 MPU. Bit 3: DW 1 MPU. Bit 4: DMA controller MPU. ... Bit 15: DAP MPU. Bit 16: CM4 system bus MPU. Bit 17: CM4 code bus MPU (for non FLASH controller accesses). Bit 18: CM4 code bus MPU (for FLASH controller accesses)."]
pub type SOURCE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0: CM0 MPU. Bit 1: CRYPTO MPU. Bit 2: DW 0 MPU. Bit 3: DW 1 MPU. Bit 4: DMA controller MPU. ... Bit 15: DAP MPU. Bit 16: CM4 system bus MPU. Bit 17: CM4 code bus MPU (for non FLASH controller accesses). Bit 18: CM4 code bus MPU (for FLASH controller accesses)."]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits)
    }
}
#[doc = "Fault pending 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending0](index.html) module"]
pub struct PENDING0_SPEC;
impl crate::RegisterSpec for PENDING0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pending0::R](R) reader structure"]
impl crate::Readable for PENDING0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PENDING0 to value 0"]
impl crate::Resettable for PENDING0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
