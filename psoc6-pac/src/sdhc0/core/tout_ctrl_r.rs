#[doc = "Register `TOUT_CTRL_R` reader"]
pub struct R(crate::R<TOUT_CTRL_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUT_CTRL_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUT_CTRL_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUT_CTRL_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUT_CTRL_R` writer"]
pub struct W(crate::W<TOUT_CTRL_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUT_CTRL_R_SPEC>;
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
impl From<crate::W<TOUT_CTRL_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUT_CTRL_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUT_CNT` reader - N/A"]
pub type TOUT_CNT_R = crate::FieldReader;
#[doc = "Field `TOUT_CNT` writer - N/A"]
pub type TOUT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, TOUT_CTRL_R_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn tout_cnt(&self) -> TOUT_CNT_R {
        TOUT_CNT_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn tout_cnt(&mut self) -> TOUT_CNT_W<0> {
        TOUT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tout_ctrl_r](index.html) module"]
pub struct TOUT_CTRL_R_SPEC;
impl crate::RegisterSpec for TOUT_CTRL_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tout_ctrl_r::R](R) reader structure"]
impl crate::Readable for TOUT_CTRL_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tout_ctrl_r::W](W) writer structure"]
impl crate::Writable for TOUT_CTRL_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUT_CTRL_R to value 0"]
impl crate::Resettable for TOUT_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
