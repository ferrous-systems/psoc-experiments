#[doc = "Register `FM_SRAM_ECC_CTL1` reader"]
pub struct R(crate::R<FM_SRAM_ECC_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_SRAM_ECC_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_SRAM_ECC_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_SRAM_ECC_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM_SRAM_ECC_CTL1` writer"]
pub struct W(crate::W<FM_SRAM_ECC_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_SRAM_ECC_CTL1_SPEC>;
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
impl From<crate::W<FM_SRAM_ECC_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_SRAM_ECC_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC_INJ_PARITY` reader - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type ECC_INJ_PARITY_R = crate::FieldReader;
#[doc = "Field `ECC_INJ_PARITY` writer - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type ECC_INJ_PARITY_W<'a, const O: u8> = crate::FieldWriter<'a, FM_SRAM_ECC_CTL1_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn ecc_inj_parity(&self) -> ECC_INJ_PARITY_R {
        ECC_INJ_PARITY_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_parity(&mut self) -> ECC_INJ_PARITY_W<0> {
        ECC_INJ_PARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eCT Flash SRAM ECC control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_sram_ecc_ctl1](index.html) module"]
pub struct FM_SRAM_ECC_CTL1_SPEC;
impl crate::RegisterSpec for FM_SRAM_ECC_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm_sram_ecc_ctl1::R](R) reader structure"]
impl crate::Readable for FM_SRAM_ECC_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm_sram_ecc_ctl1::W](W) writer structure"]
impl crate::Writable for FM_SRAM_ECC_CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_SRAM_ECC_CTL1 to value 0"]
impl crate::Resettable for FM_SRAM_ECC_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
