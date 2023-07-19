#[doc = "Register `CURR` reader"]
pub struct R(crate::R<CURR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CURR` writer"]
pub struct W(crate::W<CURR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CURR_SPEC>;
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
impl From<crate::W<CURR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CURR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTR` reader - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
pub type PTR_R = crate::FieldReader<u32>;
#[doc = "Field `PTR` writer - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
pub type PTR_W<'a, const O: u8> = crate::FieldWriter<'a, CURR_SPEC, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PTR_W<2> {
        PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current descriptor pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curr](index.html) module"]
pub struct CURR_SPEC;
impl crate::RegisterSpec for CURR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [curr::R](R) reader structure"]
impl crate::Readable for CURR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [curr::W](W) writer structure"]
impl crate::Writable for CURR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CURR to value 0"]
impl crate::Resettable for CURR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
