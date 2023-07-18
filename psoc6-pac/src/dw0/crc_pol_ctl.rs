#[doc = "Register `CRC_POL_CTL` reader"]
pub struct R(crate::R<CRC_POL_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_POL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_POL_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_POL_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_POL_CTL` writer"]
pub struct W(crate::W<CRC_POL_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_POL_CTL_SPEC>;
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
impl From<crate::W<CRC_POL_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_POL_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POLYNOMIAL` reader - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
pub type POLYNOMIAL_R = crate::FieldReader<u32>;
#[doc = "Field `POLYNOMIAL` writer - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
pub type POLYNOMIAL_W<'a, const O: u8> = crate::FieldWriter<'a, CRC_POL_CTL_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
    #[inline(always)]
    pub fn polynomial(&self) -> POLYNOMIAL_R {
        POLYNOMIAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
    #[inline(always)]
    #[must_use]
    pub fn polynomial(&mut self) -> POLYNOMIAL_W<0> {
        POLYNOMIAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC polynomial control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_pol_ctl](index.html) module"]
pub struct CRC_POL_CTL_SPEC;
impl crate::RegisterSpec for CRC_POL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_pol_ctl::R](R) reader structure"]
impl crate::Readable for CRC_POL_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_pol_ctl::W](W) writer structure"]
impl crate::Writable for CRC_POL_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_POL_CTL to value 0"]
impl crate::Resettable for CRC_POL_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
