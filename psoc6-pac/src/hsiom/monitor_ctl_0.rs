#[doc = "Register `MONITOR_CTL_0` reader"]
pub struct R(crate::R<MONITOR_CTL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONITOR_CTL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONITOR_CTL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONITOR_CTL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MONITOR_CTL_0` writer"]
pub struct W(crate::W<MONITOR_CTL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MONITOR_CTL_0_SPEC>;
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
impl From<crate::W<MONITOR_CTL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MONITOR_CTL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MONITOR_EN` reader - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
pub type MONITOR_EN_R = crate::FieldReader<u32>;
#[doc = "Field `MONITOR_EN` writer - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
pub type MONITOR_EN_W<'a, const O: u8> = crate::FieldWriter<'a, MONITOR_CTL_0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn monitor_en(&self) -> MONITOR_EN_R {
        MONITOR_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
    #[inline(always)]
    #[must_use]
    pub fn monitor_en(&mut self) -> MONITOR_EN_W<0> {
        MONITOR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power/Ground Monitor cell control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monitor_ctl_0](index.html) module"]
pub struct MONITOR_CTL_0_SPEC;
impl crate::RegisterSpec for MONITOR_CTL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monitor_ctl_0::R](R) reader structure"]
impl crate::Readable for MONITOR_CTL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [monitor_ctl_0::W](W) writer structure"]
impl crate::Writable for MONITOR_CTL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MONITOR_CTL_0 to value 0"]
impl crate::Resettable for MONITOR_CTL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
