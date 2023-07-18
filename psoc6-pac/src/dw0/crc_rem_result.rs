#[doc = "Register `CRC_REM_RESULT` reader"]
pub struct R(crate::R<CRC_REM_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_REM_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_REM_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_REM_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REM` reader - Remainder value. The alignment of the remainder depends on CRC_REM_CTL0.REM_REVERSE: '0': the more significant bits (bit 31 and down) contain the remainder. '1': the less significant bits (bit 0 and up) contain the remainder. Note: This field is combinatorially derived from CRC_LFSR_CTL.LFSR32, CRC_CTL.REM_REVERSE and CRC_REM_CTL.REM_XOR."]
pub type REM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Remainder value. The alignment of the remainder depends on CRC_REM_CTL0.REM_REVERSE: '0': the more significant bits (bit 31 and down) contain the remainder. '1': the less significant bits (bit 0 and up) contain the remainder. Note: This field is combinatorially derived from CRC_LFSR_CTL.LFSR32, CRC_CTL.REM_REVERSE and CRC_REM_CTL.REM_XOR."]
    #[inline(always)]
    pub fn rem(&self) -> REM_R {
        REM_R::new(self.bits)
    }
}
#[doc = "CRC remainder result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_rem_result](index.html) module"]
pub struct CRC_REM_RESULT_SPEC;
impl crate::RegisterSpec for CRC_REM_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_rem_result::R](R) reader structure"]
impl crate::Readable for CRC_REM_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRC_REM_RESULT to value 0"]
impl crate::Resettable for CRC_REM_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
