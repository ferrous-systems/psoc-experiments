#[doc = "Register `CRYPTO_INPUT0` reader"]
pub struct R(crate::R<CRYPTO_INPUT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPTO_INPUT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPTO_INPUT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPTO_INPUT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPTO_INPUT0` writer"]
pub struct W(crate::W<CRYPTO_INPUT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPTO_INPUT0_SPEC>;
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
impl From<crate::W<CRYPTO_INPUT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPTO_INPUT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT` reader - Four Bytes of the plaintext PT\\[31:0\\]
= CRYPTO_INPUT0.INPUT\\[31:0\\]."]
pub type INPUT_R = crate::FieldReader<u32>;
#[doc = "Field `INPUT` writer - Four Bytes of the plaintext PT\\[31:0\\]
= CRYPTO_INPUT0.INPUT\\[31:0\\]."]
pub type INPUT_W<'a, const O: u8> = crate::FieldWriter<'a, CRYPTO_INPUT0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[31:0\\]
= CRYPTO_INPUT0.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[31:0\\]
= CRYPTO_INPUT0.INPUT\\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn input(&mut self) -> INPUT_W<0> {
        INPUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cryptography input 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypto_input0](index.html) module"]
pub struct CRYPTO_INPUT0_SPEC;
impl crate::RegisterSpec for CRYPTO_INPUT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypto_input0::R](R) reader structure"]
impl crate::Readable for CRYPTO_INPUT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypto_input0::W](W) writer structure"]
impl crate::Writable for CRYPTO_INPUT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_INPUT0 to value 0"]
impl crate::Resettable for CRYPTO_INPUT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
