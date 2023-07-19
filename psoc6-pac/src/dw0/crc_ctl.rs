#[doc = "Register `CRC_CTL` reader"]
pub struct R(crate::R<CRC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_CTL` writer"]
pub struct W(crate::W<CRC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_CTL_SPEC>;
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
impl From<crate::W<CRC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_REVERSE` reader - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
pub type DATA_REVERSE_R = crate::BitReader;
#[doc = "Field `DATA_REVERSE` writer - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
pub type DATA_REVERSE_W<'a, const O: u8> = crate::BitWriter<'a, CRC_CTL_SPEC, O>;
#[doc = "Field `REM_REVERSE` reader - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
pub type REM_REVERSE_R = crate::BitReader;
#[doc = "Field `REM_REVERSE` writer - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
pub type REM_REVERSE_W<'a, const O: u8> = crate::BitWriter<'a, CRC_CTL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
    #[inline(always)]
    pub fn data_reverse(&self) -> DATA_REVERSE_R {
        DATA_REVERSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
    #[inline(always)]
    pub fn rem_reverse(&self) -> REM_REVERSE_R {
        REM_REVERSE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
    #[inline(always)]
    #[must_use]
    pub fn data_reverse(&mut self) -> DATA_REVERSE_W<0> {
        DATA_REVERSE_W::new(self)
    }
    #[doc = "Bit 8 - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
    #[inline(always)]
    #[must_use]
    pub fn rem_reverse(&mut self) -> REM_REVERSE_W<8> {
        REM_REVERSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_ctl](index.html) module"]
pub struct CRC_CTL_SPEC;
impl crate::RegisterSpec for CRC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_ctl::R](R) reader structure"]
impl crate::Readable for CRC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_ctl::W](W) writer structure"]
impl crate::Writable for CRC_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_CTL to value 0"]
impl crate::Resettable for CRC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
