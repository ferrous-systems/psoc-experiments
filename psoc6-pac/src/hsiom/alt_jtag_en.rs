#[doc = "Register `ALT_JTAG_EN` reader"]
pub struct R(crate::R<ALT_JTAG_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALT_JTAG_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALT_JTAG_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALT_JTAG_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALT_JTAG_EN` writer"]
pub struct W(crate::W<ALT_JTAG_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALT_JTAG_EN_SPEC>;
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
impl From<crate::W<ALT_JTAG_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALT_JTAG_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, ALT_JTAG_EN_SPEC, O>;
impl R {
    #[doc = "Bit 31 - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate JTAG IF selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alt_jtag_en](index.html) module"]
pub struct ALT_JTAG_EN_SPEC;
impl crate::RegisterSpec for ALT_JTAG_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alt_jtag_en::R](R) reader structure"]
impl crate::Readable for ALT_JTAG_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alt_jtag_en::W](W) writer structure"]
impl crate::Writable for ALT_JTAG_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALT_JTAG_EN to value 0"]
impl crate::Resettable for ALT_JTAG_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
