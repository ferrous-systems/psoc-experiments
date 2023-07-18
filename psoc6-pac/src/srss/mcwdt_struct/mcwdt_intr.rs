#[doc = "Register `MCWDT_INTR` reader"]
pub struct R(crate::R<MCWDT_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCWDT_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCWDT_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCWDT_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCWDT_INTR` writer"]
pub struct W(crate::W<MCWDT_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCWDT_INTR_SPEC>;
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
impl From<crate::W<MCWDT_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCWDT_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCWDT_INT0` reader - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
pub type MCWDT_INT0_R = crate::BitReader;
#[doc = "Field `MCWDT_INT0` writer - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
pub type MCWDT_INT0_W<'a, const O: u8> = crate::BitWriter<'a, MCWDT_INTR_SPEC, O>;
#[doc = "Field `MCWDT_INT1` reader - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
pub type MCWDT_INT1_R = crate::BitReader;
#[doc = "Field `MCWDT_INT1` writer - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
pub type MCWDT_INT1_W<'a, const O: u8> = crate::BitWriter<'a, MCWDT_INTR_SPEC, O>;
#[doc = "Field `MCWDT_INT2` reader - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
pub type MCWDT_INT2_R = crate::BitReader;
#[doc = "Field `MCWDT_INT2` writer - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
pub type MCWDT_INT2_W<'a, const O: u8> = crate::BitWriter<'a, MCWDT_INTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
    #[inline(always)]
    pub fn mcwdt_int0(&self) -> MCWDT_INT0_R {
        MCWDT_INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
    #[inline(always)]
    pub fn mcwdt_int1(&self) -> MCWDT_INT1_R {
        MCWDT_INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
    #[inline(always)]
    pub fn mcwdt_int2(&self) -> MCWDT_INT2_R {
        MCWDT_INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int0(&mut self) -> MCWDT_INT0_W<0> {
        MCWDT_INT0_W::new(self)
    }
    #[doc = "Bit 1 - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int1(&mut self) -> MCWDT_INT1_W<1> {
        MCWDT_INT1_W::new(self)
    }
    #[doc = "Bit 2 - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int2(&mut self) -> MCWDT_INT2_W<2> {
        MCWDT_INT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_intr](index.html) module"]
pub struct MCWDT_INTR_SPEC;
impl crate::RegisterSpec for MCWDT_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcwdt_intr::R](R) reader structure"]
impl crate::Readable for MCWDT_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcwdt_intr::W](W) writer structure"]
impl crate::Writable for MCWDT_INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCWDT_INTR to value 0"]
impl crate::Resettable for MCWDT_INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
