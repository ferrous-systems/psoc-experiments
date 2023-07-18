#[doc = "Register `MS_ATT3` reader"]
pub struct R(crate::R<MS_ATT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS_ATT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS_ATT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS_ATT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS_ATT3` writer"]
pub struct W(crate::W<MS_ATT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS_ATT3_SPEC>;
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
impl From<crate::W<MS_ATT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MS_ATT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC12_UR` reader - Protection context 12, user read enable."]
pub type PC12_UR_R = crate::BitReader;
#[doc = "Field `PC12_UW` reader - Protection context 12, user write enable."]
pub type PC12_UW_R = crate::BitReader;
#[doc = "Field `PC12_UW` writer - Protection context 12, user write enable."]
pub type PC12_UW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC12_PR` reader - Protection context 12, privileged read enable."]
pub type PC12_PR_R = crate::BitReader;
#[doc = "Field `PC12_PW` reader - Protection context 12, privileged write enable."]
pub type PC12_PW_R = crate::BitReader;
#[doc = "Field `PC12_PW` writer - Protection context 12, privileged write enable."]
pub type PC12_PW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC12_NS` reader - Protection context 12, non-secure."]
pub type PC12_NS_R = crate::BitReader;
#[doc = "Field `PC12_NS` writer - Protection context 12, non-secure."]
pub type PC12_NS_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC13_UR` reader - Protection context 13, user read enable."]
pub type PC13_UR_R = crate::BitReader;
#[doc = "Field `PC13_UW` reader - Protection context 13, user write enable."]
pub type PC13_UW_R = crate::BitReader;
#[doc = "Field `PC13_UW` writer - Protection context 13, user write enable."]
pub type PC13_UW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC13_PR` reader - Protection context 13, privileged read enable."]
pub type PC13_PR_R = crate::BitReader;
#[doc = "Field `PC13_PW` reader - Protection context 13, privileged write enable."]
pub type PC13_PW_R = crate::BitReader;
#[doc = "Field `PC13_PW` writer - Protection context 13, privileged write enable."]
pub type PC13_PW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC13_NS` reader - Protection context 13, non-secure."]
pub type PC13_NS_R = crate::BitReader;
#[doc = "Field `PC13_NS` writer - Protection context 13, non-secure."]
pub type PC13_NS_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC14_UR` reader - Protection context 14, user read enable."]
pub type PC14_UR_R = crate::BitReader;
#[doc = "Field `PC14_UW` reader - Protection context 14, user write enable."]
pub type PC14_UW_R = crate::BitReader;
#[doc = "Field `PC14_UW` writer - Protection context 14, user write enable."]
pub type PC14_UW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC14_PR` reader - Protection context 14, privileged read enable."]
pub type PC14_PR_R = crate::BitReader;
#[doc = "Field `PC14_PW` reader - Protection context 14, privileged write enable."]
pub type PC14_PW_R = crate::BitReader;
#[doc = "Field `PC14_PW` writer - Protection context 14, privileged write enable."]
pub type PC14_PW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC14_NS` reader - Protection context 14, non-secure."]
pub type PC14_NS_R = crate::BitReader;
#[doc = "Field `PC14_NS` writer - Protection context 14, non-secure."]
pub type PC14_NS_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC15_UR` reader - Protection context 15, user read enable."]
pub type PC15_UR_R = crate::BitReader;
#[doc = "Field `PC15_UW` reader - Protection context 15, user write enable."]
pub type PC15_UW_R = crate::BitReader;
#[doc = "Field `PC15_UW` writer - Protection context 15, user write enable."]
pub type PC15_UW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC15_PR` reader - Protection context 15, privileged read enable."]
pub type PC15_PR_R = crate::BitReader;
#[doc = "Field `PC15_PW` reader - Protection context 15, privileged write enable."]
pub type PC15_PW_R = crate::BitReader;
#[doc = "Field `PC15_PW` writer - Protection context 15, privileged write enable."]
pub type PC15_PW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
#[doc = "Field `PC15_NS` reader - Protection context 15, non-secure."]
pub type PC15_NS_R = crate::BitReader;
#[doc = "Field `PC15_NS` writer - Protection context 15, non-secure."]
pub type PC15_NS_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT3_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Protection context 12, user read enable."]
    #[inline(always)]
    pub fn pc12_ur(&self) -> PC12_UR_R {
        PC12_UR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protection context 12, user write enable."]
    #[inline(always)]
    pub fn pc12_uw(&self) -> PC12_UW_R {
        PC12_UW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protection context 12, privileged read enable."]
    #[inline(always)]
    pub fn pc12_pr(&self) -> PC12_PR_R {
        PC12_PR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection context 12, privileged write enable."]
    #[inline(always)]
    pub fn pc12_pw(&self) -> PC12_PW_R {
        PC12_PW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection context 12, non-secure."]
    #[inline(always)]
    pub fn pc12_ns(&self) -> PC12_NS_R {
        PC12_NS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Protection context 13, user read enable."]
    #[inline(always)]
    pub fn pc13_ur(&self) -> PC13_UR_R {
        PC13_UR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protection context 13, user write enable."]
    #[inline(always)]
    pub fn pc13_uw(&self) -> PC13_UW_R {
        PC13_UW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protection context 13, privileged read enable."]
    #[inline(always)]
    pub fn pc13_pr(&self) -> PC13_PR_R {
        PC13_PR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection context 13, privileged write enable."]
    #[inline(always)]
    pub fn pc13_pw(&self) -> PC13_PW_R {
        PC13_PW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protection context 13, non-secure."]
    #[inline(always)]
    pub fn pc13_ns(&self) -> PC13_NS_R {
        PC13_NS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection context 14, user read enable."]
    #[inline(always)]
    pub fn pc14_ur(&self) -> PC14_UR_R {
        PC14_UR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protection context 14, user write enable."]
    #[inline(always)]
    pub fn pc14_uw(&self) -> PC14_UW_R {
        PC14_UW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protection context 14, privileged read enable."]
    #[inline(always)]
    pub fn pc14_pr(&self) -> PC14_PR_R {
        PC14_PR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protection context 14, privileged write enable."]
    #[inline(always)]
    pub fn pc14_pw(&self) -> PC14_PW_R {
        PC14_PW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protection context 14, non-secure."]
    #[inline(always)]
    pub fn pc14_ns(&self) -> PC14_NS_R {
        PC14_NS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Protection context 15, user read enable."]
    #[inline(always)]
    pub fn pc15_ur(&self) -> PC15_UR_R {
        PC15_UR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protection context 15, user write enable."]
    #[inline(always)]
    pub fn pc15_uw(&self) -> PC15_UW_R {
        PC15_UW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection context 15, privileged read enable."]
    #[inline(always)]
    pub fn pc15_pr(&self) -> PC15_PR_R {
        PC15_PR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protection context 15, privileged write enable."]
    #[inline(always)]
    pub fn pc15_pw(&self) -> PC15_PW_R {
        PC15_PW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protection context 15, non-secure."]
    #[inline(always)]
    pub fn pc15_ns(&self) -> PC15_NS_R {
        PC15_NS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Protection context 12, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc12_uw(&mut self) -> PC12_UW_W<1> {
        PC12_UW_W::new(self)
    }
    #[doc = "Bit 3 - Protection context 12, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc12_pw(&mut self) -> PC12_PW_W<3> {
        PC12_PW_W::new(self)
    }
    #[doc = "Bit 4 - Protection context 12, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc12_ns(&mut self) -> PC12_NS_W<4> {
        PC12_NS_W::new(self)
    }
    #[doc = "Bit 9 - Protection context 13, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc13_uw(&mut self) -> PC13_UW_W<9> {
        PC13_UW_W::new(self)
    }
    #[doc = "Bit 11 - Protection context 13, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc13_pw(&mut self) -> PC13_PW_W<11> {
        PC13_PW_W::new(self)
    }
    #[doc = "Bit 12 - Protection context 13, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc13_ns(&mut self) -> PC13_NS_W<12> {
        PC13_NS_W::new(self)
    }
    #[doc = "Bit 17 - Protection context 14, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc14_uw(&mut self) -> PC14_UW_W<17> {
        PC14_UW_W::new(self)
    }
    #[doc = "Bit 19 - Protection context 14, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc14_pw(&mut self) -> PC14_PW_W<19> {
        PC14_PW_W::new(self)
    }
    #[doc = "Bit 20 - Protection context 14, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc14_ns(&mut self) -> PC14_NS_W<20> {
        PC14_NS_W::new(self)
    }
    #[doc = "Bit 25 - Protection context 15, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc15_uw(&mut self) -> PC15_UW_W<25> {
        PC15_UW_W::new(self)
    }
    #[doc = "Bit 27 - Protection context 15, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc15_pw(&mut self) -> PC15_PW_W<27> {
        PC15_PW_W::new(self)
    }
    #[doc = "Bit 28 - Protection context 15, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc15_ns(&mut self) -> PC15_NS_W<28> {
        PC15_NS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master attributes 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_att3](index.html) module"]
pub struct MS_ATT3_SPEC;
impl crate::RegisterSpec for MS_ATT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms_att3::R](R) reader structure"]
impl crate::Readable for MS_ATT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms_att3::W](W) writer structure"]
impl crate::Writable for MS_ATT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MS_ATT3 to value 0x1f1f_1f1f"]
impl crate::Resettable for MS_ATT3_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f1f_1f1f;
}
