#[doc = "Register `ACLK_CTL` writer"]
pub struct W(crate::W<ACLK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACLK_CTL_SPEC>;
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
impl From<crate::W<ACLK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACLK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACLK_GEN` writer - A write to this register generates the clock pulse for HV control registers (mpcon outputs)"]
pub type ACLK_GEN_W<'a, const O: u8> = crate::BitWriter<'a, ACLK_CTL_SPEC, O>;
impl W {
    #[doc = "Bit 0 - A write to this register generates the clock pulse for HV control registers (mpcon outputs)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gen(&mut self) -> ACLK_GEN_W<0> {
        ACLK_GEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCON clock\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aclk_ctl](index.html) module"]
pub struct ACLK_CTL_SPEC;
impl crate::RegisterSpec for ACLK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aclk_ctl::W](W) writer structure"]
impl crate::Writable for ACLK_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACLK_CTL to value 0"]
impl crate::Resettable for ACLK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
