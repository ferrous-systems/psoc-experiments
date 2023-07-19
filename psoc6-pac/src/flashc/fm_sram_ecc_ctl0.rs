#[doc = "Register `FM_SRAM_ECC_CTL0` reader"]
pub struct R(crate::R<FM_SRAM_ECC_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_SRAM_ECC_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_SRAM_ECC_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_SRAM_ECC_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM_SRAM_ECC_CTL0` writer"]
pub struct W(crate::W<FM_SRAM_ECC_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_SRAM_ECC_CTL0_SPEC>;
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
impl From<crate::W<FM_SRAM_ECC_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_SRAM_ECC_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECC_INJ_DATA` reader - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type ECC_INJ_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `ECC_INJ_DATA` writer - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type ECC_INJ_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, FM_SRAM_ECC_CTL0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn ecc_inj_data(&self) -> ECC_INJ_DATA_R {
        ECC_INJ_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_data(&mut self) -> ECC_INJ_DATA_W<0> {
        ECC_INJ_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eCT Flash SRAM ECC control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_sram_ecc_ctl0](index.html) module"]
pub struct FM_SRAM_ECC_CTL0_SPEC;
impl crate::RegisterSpec for FM_SRAM_ECC_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm_sram_ecc_ctl0::R](R) reader structure"]
impl crate::Readable for FM_SRAM_ECC_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm_sram_ecc_ctl0::W](W) writer structure"]
impl crate::Writable for FM_SRAM_ECC_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_SRAM_ECC_CTL0 to value 0"]
impl crate::Resettable for FM_SRAM_ECC_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
