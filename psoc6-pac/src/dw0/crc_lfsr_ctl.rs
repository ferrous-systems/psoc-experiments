#[doc = "Register `CRC_LFSR_CTL` reader"]
pub struct R(crate::R<CRC_LFSR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_LFSR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_LFSR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_LFSR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_LFSR_CTL` writer"]
pub struct W(crate::W<CRC_LFSR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_LFSR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CRC_LFSR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_LFSR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFSR32` reader - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
pub type LFSR32_R = crate::FieldReader<u32>;
#[doc = "Field `LFSR32` writer - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
pub type LFSR32_W<'a, const O: u8> = crate::FieldWriter<'a, CRC_LFSR_CTL_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
    #[inline(always)]
    pub fn lfsr32(&self) -> LFSR32_R {
        LFSR32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr32(&mut self) -> LFSR32_W<0> {
        LFSR32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC LFSR control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_lfsr_ctl](index.html) module"]
pub struct CRC_LFSR_CTL_SPEC;
impl crate::RegisterSpec for CRC_LFSR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_lfsr_ctl::R](R) reader structure"]
impl crate::Readable for CRC_LFSR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_lfsr_ctl::W](W) writer structure"]
impl crate::Writable for CRC_LFSR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_LFSR_CTL to value 0"]
impl crate::Resettable for CRC_LFSR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
