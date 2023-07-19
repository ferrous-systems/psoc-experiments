#[doc = "Register `ECC_CTL` reader"]
pub struct R(crate::R<ECC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECC_CTL` writer"]
pub struct W(crate::W<ECC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECC_CTL_SPEC>;
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
impl From<crate::W<ECC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WORD_ADDR` reader - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
pub type WORD_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `WORD_ADDR` writer - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
pub type WORD_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, ECC_CTL_SPEC, 24, O, u32>;
#[doc = "Field `PARITY` reader - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
pub type PARITY_R = crate::FieldReader;
#[doc = "Field `PARITY` writer - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
pub type PARITY_W<'a, const O: u8> = crate::FieldWriter<'a, ECC_CTL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:23 - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
    #[inline(always)]
    pub fn word_addr(&self) -> WORD_ADDR_R {
        WORD_ADDR_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
    #[inline(always)]
    #[must_use]
    pub fn word_addr(&mut self) -> WORD_ADDR_W<0> {
        WORD_ADDR_W::new(self)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<24> {
        PARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_ctl](index.html) module"]
pub struct ECC_CTL_SPEC;
impl crate::RegisterSpec for ECC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_ctl::R](R) reader structure"]
impl crate::Readable for ECC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecc_ctl::W](W) writer structure"]
impl crate::Writable for ECC_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECC_CTL to value 0"]
impl crate::Resettable for ECC_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
