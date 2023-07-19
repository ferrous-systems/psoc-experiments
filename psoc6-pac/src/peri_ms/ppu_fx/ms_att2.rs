#[doc = "Register `MS_ATT2` reader"]
pub struct R(crate::R<MS_ATT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS_ATT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS_ATT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS_ATT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS_ATT2` writer"]
pub struct W(crate::W<MS_ATT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS_ATT2_SPEC>;
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
impl From<crate::W<MS_ATT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MS_ATT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC8_UR` reader - Protection context 8, user read enable."]
pub type PC8_UR_R = crate::BitReader;
#[doc = "Field `PC8_UW` reader - Protection context 8, user write enable."]
pub type PC8_UW_R = crate::BitReader;
#[doc = "Field `PC8_UW` writer - Protection context 8, user write enable."]
pub type PC8_UW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC8_PR` reader - Protection context 8, privileged read enable."]
pub type PC8_PR_R = crate::BitReader;
#[doc = "Field `PC8_PW` reader - Protection context 8, privileged write enable."]
pub type PC8_PW_R = crate::BitReader;
#[doc = "Field `PC8_PW` writer - Protection context 8, privileged write enable."]
pub type PC8_PW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC8_NS` reader - Protection context 8, non-secure."]
pub type PC8_NS_R = crate::BitReader;
#[doc = "Field `PC8_NS` writer - Protection context 8, non-secure."]
pub type PC8_NS_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC9_UR` reader - Protection context 9, user read enable."]
pub type PC9_UR_R = crate::BitReader;
#[doc = "Field `PC9_UW` reader - Protection context 9, user write enable."]
pub type PC9_UW_R = crate::BitReader;
#[doc = "Field `PC9_UW` writer - Protection context 9, user write enable."]
pub type PC9_UW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC9_PR` reader - Protection context 9, privileged read enable."]
pub type PC9_PR_R = crate::BitReader;
#[doc = "Field `PC9_PW` reader - Protection context 9, privileged write enable."]
pub type PC9_PW_R = crate::BitReader;
#[doc = "Field `PC9_PW` writer - Protection context 9, privileged write enable."]
pub type PC9_PW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC9_NS` reader - Protection context 9, non-secure."]
pub type PC9_NS_R = crate::BitReader;
#[doc = "Field `PC9_NS` writer - Protection context 9, non-secure."]
pub type PC9_NS_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC10_UR` reader - Protection context 10, user read enable."]
pub type PC10_UR_R = crate::BitReader;
#[doc = "Field `PC10_UW` reader - Protection context 10, user write enable."]
pub type PC10_UW_R = crate::BitReader;
#[doc = "Field `PC10_UW` writer - Protection context 10, user write enable."]
pub type PC10_UW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC10_PR` reader - Protection context 10, privileged read enable."]
pub type PC10_PR_R = crate::BitReader;
#[doc = "Field `PC10_PW` reader - Protection context 10, privileged write enable."]
pub type PC10_PW_R = crate::BitReader;
#[doc = "Field `PC10_PW` writer - Protection context 10, privileged write enable."]
pub type PC10_PW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC10_NS` reader - Protection context 10, non-secure."]
pub type PC10_NS_R = crate::BitReader;
#[doc = "Field `PC10_NS` writer - Protection context 10, non-secure."]
pub type PC10_NS_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC11_UR` reader - Protection context 11, user read enable."]
pub type PC11_UR_R = crate::BitReader;
#[doc = "Field `PC11_UW` reader - Protection context 11, user write enable."]
pub type PC11_UW_R = crate::BitReader;
#[doc = "Field `PC11_UW` writer - Protection context 11, user write enable."]
pub type PC11_UW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC11_PR` reader - Protection context 11, privileged read enable."]
pub type PC11_PR_R = crate::BitReader;
#[doc = "Field `PC11_PW` reader - Protection context 11, privileged write enable."]
pub type PC11_PW_R = crate::BitReader;
#[doc = "Field `PC11_PW` writer - Protection context 11, privileged write enable."]
pub type PC11_PW_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
#[doc = "Field `PC11_NS` reader - Protection context 11, non-secure."]
pub type PC11_NS_R = crate::BitReader;
#[doc = "Field `PC11_NS` writer - Protection context 11, non-secure."]
pub type PC11_NS_W<'a, const O: u8> = crate::BitWriter<'a, MS_ATT2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Protection context 8, user read enable."]
    #[inline(always)]
    pub fn pc8_ur(&self) -> PC8_UR_R {
        PC8_UR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protection context 8, user write enable."]
    #[inline(always)]
    pub fn pc8_uw(&self) -> PC8_UW_R {
        PC8_UW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protection context 8, privileged read enable."]
    #[inline(always)]
    pub fn pc8_pr(&self) -> PC8_PR_R {
        PC8_PR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection context 8, privileged write enable."]
    #[inline(always)]
    pub fn pc8_pw(&self) -> PC8_PW_R {
        PC8_PW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection context 8, non-secure."]
    #[inline(always)]
    pub fn pc8_ns(&self) -> PC8_NS_R {
        PC8_NS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Protection context 9, user read enable."]
    #[inline(always)]
    pub fn pc9_ur(&self) -> PC9_UR_R {
        PC9_UR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protection context 9, user write enable."]
    #[inline(always)]
    pub fn pc9_uw(&self) -> PC9_UW_R {
        PC9_UW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protection context 9, privileged read enable."]
    #[inline(always)]
    pub fn pc9_pr(&self) -> PC9_PR_R {
        PC9_PR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection context 9, privileged write enable."]
    #[inline(always)]
    pub fn pc9_pw(&self) -> PC9_PW_R {
        PC9_PW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protection context 9, non-secure."]
    #[inline(always)]
    pub fn pc9_ns(&self) -> PC9_NS_R {
        PC9_NS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection context 10, user read enable."]
    #[inline(always)]
    pub fn pc10_ur(&self) -> PC10_UR_R {
        PC10_UR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protection context 10, user write enable."]
    #[inline(always)]
    pub fn pc10_uw(&self) -> PC10_UW_R {
        PC10_UW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protection context 10, privileged read enable."]
    #[inline(always)]
    pub fn pc10_pr(&self) -> PC10_PR_R {
        PC10_PR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protection context 10, privileged write enable."]
    #[inline(always)]
    pub fn pc10_pw(&self) -> PC10_PW_R {
        PC10_PW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protection context 10, non-secure."]
    #[inline(always)]
    pub fn pc10_ns(&self) -> PC10_NS_R {
        PC10_NS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Protection context 11, user read enable."]
    #[inline(always)]
    pub fn pc11_ur(&self) -> PC11_UR_R {
        PC11_UR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protection context 11, user write enable."]
    #[inline(always)]
    pub fn pc11_uw(&self) -> PC11_UW_R {
        PC11_UW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection context 11, privileged read enable."]
    #[inline(always)]
    pub fn pc11_pr(&self) -> PC11_PR_R {
        PC11_PR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protection context 11, privileged write enable."]
    #[inline(always)]
    pub fn pc11_pw(&self) -> PC11_PW_R {
        PC11_PW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protection context 11, non-secure."]
    #[inline(always)]
    pub fn pc11_ns(&self) -> PC11_NS_R {
        PC11_NS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Protection context 8, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc8_uw(&mut self) -> PC8_UW_W<1> {
        PC8_UW_W::new(self)
    }
    #[doc = "Bit 3 - Protection context 8, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc8_pw(&mut self) -> PC8_PW_W<3> {
        PC8_PW_W::new(self)
    }
    #[doc = "Bit 4 - Protection context 8, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc8_ns(&mut self) -> PC8_NS_W<4> {
        PC8_NS_W::new(self)
    }
    #[doc = "Bit 9 - Protection context 9, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc9_uw(&mut self) -> PC9_UW_W<9> {
        PC9_UW_W::new(self)
    }
    #[doc = "Bit 11 - Protection context 9, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc9_pw(&mut self) -> PC9_PW_W<11> {
        PC9_PW_W::new(self)
    }
    #[doc = "Bit 12 - Protection context 9, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc9_ns(&mut self) -> PC9_NS_W<12> {
        PC9_NS_W::new(self)
    }
    #[doc = "Bit 17 - Protection context 10, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc10_uw(&mut self) -> PC10_UW_W<17> {
        PC10_UW_W::new(self)
    }
    #[doc = "Bit 19 - Protection context 10, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc10_pw(&mut self) -> PC10_PW_W<19> {
        PC10_PW_W::new(self)
    }
    #[doc = "Bit 20 - Protection context 10, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc10_ns(&mut self) -> PC10_NS_W<20> {
        PC10_NS_W::new(self)
    }
    #[doc = "Bit 25 - Protection context 11, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc11_uw(&mut self) -> PC11_UW_W<25> {
        PC11_UW_W::new(self)
    }
    #[doc = "Bit 27 - Protection context 11, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc11_pw(&mut self) -> PC11_PW_W<27> {
        PC11_PW_W::new(self)
    }
    #[doc = "Bit 28 - Protection context 11, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc11_ns(&mut self) -> PC11_NS_W<28> {
        PC11_NS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master attributes 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_att2](index.html) module"]
pub struct MS_ATT2_SPEC;
impl crate::RegisterSpec for MS_ATT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms_att2::R](R) reader structure"]
impl crate::Readable for MS_ATT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms_att2::W](W) writer structure"]
impl crate::Writable for MS_ATT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MS_ATT2 to value 0x1f1f_1f1f"]
impl crate::Resettable for MS_ATT2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f1f_1f1f;
}
