#[doc = "Register `RAM2_CTL0` reader"]
pub struct R(crate::R<RAM2_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM2_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM2_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM2_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM2_CTL0` writer"]
pub struct W(crate::W<RAM2_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM2_CTL0_SPEC>;
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
impl From<crate::W<RAM2_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM2_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOW_WS` reader - See RAM0_CTL."]
pub type SLOW_WS_R = crate::FieldReader;
#[doc = "Field `SLOW_WS` writer - See RAM0_CTL."]
pub type SLOW_WS_W<'a, const O: u8> = crate::FieldWriter<'a, RAM2_CTL0_SPEC, 2, O>;
#[doc = "Field `FAST_WS` reader - See RAM0_CTL."]
pub type FAST_WS_R = crate::FieldReader;
#[doc = "Field `FAST_WS` writer - See RAM0_CTL."]
pub type FAST_WS_W<'a, const O: u8> = crate::FieldWriter<'a, RAM2_CTL0_SPEC, 2, O>;
#[doc = "Field `ECC_EN` reader - See RAM0_CTL."]
pub type ECC_EN_R = crate::BitReader;
#[doc = "Field `ECC_EN` writer - See RAM0_CTL."]
pub type ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, RAM2_CTL0_SPEC, O>;
#[doc = "Field `ECC_AUTO_CORRECT` reader - See RAM0_CTL."]
pub type ECC_AUTO_CORRECT_R = crate::BitReader;
#[doc = "Field `ECC_AUTO_CORRECT` writer - See RAM0_CTL."]
pub type ECC_AUTO_CORRECT_W<'a, const O: u8> = crate::BitWriter<'a, RAM2_CTL0_SPEC, O>;
#[doc = "Field `ECC_INJ_EN` reader - See RAM0_CTL."]
pub type ECC_INJ_EN_R = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - See RAM0_CTL."]
pub type ECC_INJ_EN_W<'a, const O: u8> = crate::BitWriter<'a, RAM2_CTL0_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - See RAM0_CTL."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SLOW_WS_R {
        SLOW_WS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - See RAM0_CTL."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FAST_WS_R {
        FAST_WS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - See RAM0_CTL."]
    #[inline(always)]
    pub fn ecc_en(&self) -> ECC_EN_R {
        ECC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See RAM0_CTL."]
    #[inline(always)]
    pub fn ecc_auto_correct(&self) -> ECC_AUTO_CORRECT_R {
        ECC_AUTO_CORRECT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - See RAM0_CTL."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> ECC_INJ_EN_R {
        ECC_INJ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn slow_ws(&mut self) -> SLOW_WS_W<0> {
        SLOW_WS_W::new(self)
    }
    #[doc = "Bits 8:9 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn fast_ws(&mut self) -> FAST_WS_W<8> {
        FAST_WS_W::new(self)
    }
    #[doc = "Bit 16 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> ECC_EN_W<16> {
        ECC_EN_W::new(self)
    }
    #[doc = "Bit 17 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_auto_correct(&mut self) -> ECC_AUTO_CORRECT_W<17> {
        ECC_AUTO_CORRECT_W::new(self)
    }
    #[doc = "Bit 18 - See RAM0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> ECC_INJ_EN_W<18> {
        ECC_INJ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM 2 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram2_ctl0](index.html) module"]
pub struct RAM2_CTL0_SPEC;
impl crate::RegisterSpec for RAM2_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram2_ctl0::R](R) reader structure"]
impl crate::Readable for RAM2_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram2_ctl0::W](W) writer structure"]
impl crate::Writable for RAM2_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM2_CTL0 to value 0x0003_0001"]
impl crate::Resettable for RAM2_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0001;
}
