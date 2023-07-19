#[doc = "Register `CRC_DATA_CTL` reader"]
pub struct R(crate::R<CRC_DATA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_DATA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_DATA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_DATA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_DATA_CTL` writer"]
pub struct W(crate::W<CRC_DATA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_DATA_CTL_SPEC>;
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
impl From<crate::W<CRC_DATA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_DATA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_XOR` reader - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
pub type DATA_XOR_R = crate::FieldReader;
#[doc = "Field `DATA_XOR` writer - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
pub type DATA_XOR_W<'a, const O: u8> = crate::FieldWriter<'a, CRC_DATA_CTL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
    #[inline(always)]
    pub fn data_xor(&self) -> DATA_XOR_R {
        DATA_XOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
    #[inline(always)]
    #[must_use]
    pub fn data_xor(&mut self) -> DATA_XOR_W<0> {
        DATA_XOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC data control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_data_ctl](index.html) module"]
pub struct CRC_DATA_CTL_SPEC;
impl crate::RegisterSpec for CRC_DATA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_data_ctl::R](R) reader structure"]
impl crate::Readable for CRC_DATA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_data_ctl::W](W) writer structure"]
impl crate::Writable for CRC_DATA_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_DATA_CTL to value 0"]
impl crate::Resettable for CRC_DATA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
