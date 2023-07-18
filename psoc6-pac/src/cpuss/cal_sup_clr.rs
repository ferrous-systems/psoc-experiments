#[doc = "Register `CAL_SUP_CLR` reader"]
pub struct R(crate::R<CAL_SUP_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_SUP_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_SUP_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_SUP_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_SUP_CLR` writer"]
pub struct W(crate::W<CAL_SUP_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_SUP_CLR_SPEC>;
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
impl From<crate::W<CAL_SUP_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_SUP_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_SUP_CLR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration support clear and reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_sup_clr](index.html) module"]
pub struct CAL_SUP_CLR_SPEC;
impl crate::RegisterSpec for CAL_SUP_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_sup_clr::R](R) reader structure"]
impl crate::Readable for CAL_SUP_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_sup_clr::W](W) writer structure"]
impl crate::Writable for CAL_SUP_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_SUP_CLR to value 0"]
impl crate::Resettable for CAL_SUP_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
