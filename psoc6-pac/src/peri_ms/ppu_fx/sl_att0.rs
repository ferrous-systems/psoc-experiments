#[doc = "Register `SL_ATT0` reader"]
pub struct R(crate::R<SL_ATT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SL_ATT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SL_ATT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SL_ATT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SL_ATT0` writer"]
pub struct W(crate::W<SL_ATT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SL_ATT0_SPEC>;
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
impl From<crate::W<SL_ATT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SL_ATT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC0_UR` reader - Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
pub type PC0_UR_R = crate::BitReader;
#[doc = "Field `PC0_UW` reader - Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
pub type PC0_UW_R = crate::BitReader;
#[doc = "Field `PC0_PR` reader - Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
pub type PC0_PR_R = crate::BitReader;
#[doc = "Field `PC0_PW` reader - Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
pub type PC0_PW_R = crate::BitReader;
#[doc = "Field `PC0_NS` reader - Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
pub type PC0_NS_R = crate::BitReader;
#[doc = "Field `PC1_UR` reader - Protection context 1, user read enable."]
pub type PC1_UR_R = crate::BitReader;
#[doc = "Field `PC1_UR` writer - Protection context 1, user read enable."]
pub type PC1_UR_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC1_UW` reader - Protection context 1, user write enable."]
pub type PC1_UW_R = crate::BitReader;
#[doc = "Field `PC1_UW` writer - Protection context 1, user write enable."]
pub type PC1_UW_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC1_PR` reader - Protection context 1, privileged read enable."]
pub type PC1_PR_R = crate::BitReader;
#[doc = "Field `PC1_PR` writer - Protection context 1, privileged read enable."]
pub type PC1_PR_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC1_PW` reader - Protection context 1, privileged write enable."]
pub type PC1_PW_R = crate::BitReader;
#[doc = "Field `PC1_PW` writer - Protection context 1, privileged write enable."]
pub type PC1_PW_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC1_NS` reader - Protection context 1, non-secure."]
pub type PC1_NS_R = crate::BitReader;
#[doc = "Field `PC1_NS` writer - Protection context 1, non-secure."]
pub type PC1_NS_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC2_UR` reader - Protection context 2, user read enable."]
pub type PC2_UR_R = crate::BitReader;
#[doc = "Field `PC2_UR` writer - Protection context 2, user read enable."]
pub type PC2_UR_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC2_UW` reader - Protection context 2, user write enable."]
pub type PC2_UW_R = crate::BitReader;
#[doc = "Field `PC2_UW` writer - Protection context 2, user write enable."]
pub type PC2_UW_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC2_PR` reader - Protection context 2, privileged read enable."]
pub type PC2_PR_R = crate::BitReader;
#[doc = "Field `PC2_PR` writer - Protection context 2, privileged read enable."]
pub type PC2_PR_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC2_PW` reader - Protection context 2, privileged write enable."]
pub type PC2_PW_R = crate::BitReader;
#[doc = "Field `PC2_PW` writer - Protection context 2, privileged write enable."]
pub type PC2_PW_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC2_NS` reader - Protection context 2, non-secure."]
pub type PC2_NS_R = crate::BitReader;
#[doc = "Field `PC2_NS` writer - Protection context 2, non-secure."]
pub type PC2_NS_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC3_UR` reader - Protection context 3, user read enable."]
pub type PC3_UR_R = crate::BitReader;
#[doc = "Field `PC3_UR` writer - Protection context 3, user read enable."]
pub type PC3_UR_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC3_UW` reader - Protection context 3, user write enable."]
pub type PC3_UW_R = crate::BitReader;
#[doc = "Field `PC3_UW` writer - Protection context 3, user write enable."]
pub type PC3_UW_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC3_PR` reader - Protection context 3, privileged read enable."]
pub type PC3_PR_R = crate::BitReader;
#[doc = "Field `PC3_PR` writer - Protection context 3, privileged read enable."]
pub type PC3_PR_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC3_PW` reader - Protection context 3, privileged write enable."]
pub type PC3_PW_R = crate::BitReader;
#[doc = "Field `PC3_PW` writer - Protection context 3, privileged write enable."]
pub type PC3_PW_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
#[doc = "Field `PC3_NS` reader - Protection context 3, non-secure."]
pub type PC3_NS_R = crate::BitReader;
#[doc = "Field `PC3_NS` writer - Protection context 3, non-secure."]
pub type PC3_NS_W<'a, const O: u8> = crate::BitWriter<'a, SL_ATT0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Protection context 0, user read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_ur(&self) -> PC0_UR_R {
        PC0_UR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protection context 0, user write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_uw(&self) -> PC0_UW_R {
        PC0_UW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protection context 0, privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_pr(&self) -> PC0_PR_R {
        PC0_PR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection context 0, privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn pc0_pw(&self) -> PC0_PW_R {
        PC0_PW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection context 0, non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn pc0_ns(&self) -> PC0_NS_R {
        PC0_NS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Protection context 1, user read enable."]
    #[inline(always)]
    pub fn pc1_ur(&self) -> PC1_UR_R {
        PC1_UR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protection context 1, user write enable."]
    #[inline(always)]
    pub fn pc1_uw(&self) -> PC1_UW_R {
        PC1_UW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protection context 1, privileged read enable."]
    #[inline(always)]
    pub fn pc1_pr(&self) -> PC1_PR_R {
        PC1_PR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection context 1, privileged write enable."]
    #[inline(always)]
    pub fn pc1_pw(&self) -> PC1_PW_R {
        PC1_PW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protection context 1, non-secure."]
    #[inline(always)]
    pub fn pc1_ns(&self) -> PC1_NS_R {
        PC1_NS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection context 2, user read enable."]
    #[inline(always)]
    pub fn pc2_ur(&self) -> PC2_UR_R {
        PC2_UR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protection context 2, user write enable."]
    #[inline(always)]
    pub fn pc2_uw(&self) -> PC2_UW_R {
        PC2_UW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protection context 2, privileged read enable."]
    #[inline(always)]
    pub fn pc2_pr(&self) -> PC2_PR_R {
        PC2_PR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protection context 2, privileged write enable."]
    #[inline(always)]
    pub fn pc2_pw(&self) -> PC2_PW_R {
        PC2_PW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protection context 2, non-secure."]
    #[inline(always)]
    pub fn pc2_ns(&self) -> PC2_NS_R {
        PC2_NS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Protection context 3, user read enable."]
    #[inline(always)]
    pub fn pc3_ur(&self) -> PC3_UR_R {
        PC3_UR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protection context 3, user write enable."]
    #[inline(always)]
    pub fn pc3_uw(&self) -> PC3_UW_R {
        PC3_UW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection context 3, privileged read enable."]
    #[inline(always)]
    pub fn pc3_pr(&self) -> PC3_PR_R {
        PC3_PR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protection context 3, privileged write enable."]
    #[inline(always)]
    pub fn pc3_pw(&self) -> PC3_PW_R {
        PC3_PW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protection context 3, non-secure."]
    #[inline(always)]
    pub fn pc3_ns(&self) -> PC3_NS_R {
        PC3_NS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Protection context 1, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc1_ur(&mut self) -> PC1_UR_W<8> {
        PC1_UR_W::new(self)
    }
    #[doc = "Bit 9 - Protection context 1, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc1_uw(&mut self) -> PC1_UW_W<9> {
        PC1_UW_W::new(self)
    }
    #[doc = "Bit 10 - Protection context 1, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc1_pr(&mut self) -> PC1_PR_W<10> {
        PC1_PR_W::new(self)
    }
    #[doc = "Bit 11 - Protection context 1, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc1_pw(&mut self) -> PC1_PW_W<11> {
        PC1_PW_W::new(self)
    }
    #[doc = "Bit 12 - Protection context 1, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc1_ns(&mut self) -> PC1_NS_W<12> {
        PC1_NS_W::new(self)
    }
    #[doc = "Bit 16 - Protection context 2, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc2_ur(&mut self) -> PC2_UR_W<16> {
        PC2_UR_W::new(self)
    }
    #[doc = "Bit 17 - Protection context 2, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc2_uw(&mut self) -> PC2_UW_W<17> {
        PC2_UW_W::new(self)
    }
    #[doc = "Bit 18 - Protection context 2, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc2_pr(&mut self) -> PC2_PR_W<18> {
        PC2_PR_W::new(self)
    }
    #[doc = "Bit 19 - Protection context 2, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc2_pw(&mut self) -> PC2_PW_W<19> {
        PC2_PW_W::new(self)
    }
    #[doc = "Bit 20 - Protection context 2, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc2_ns(&mut self) -> PC2_NS_W<20> {
        PC2_NS_W::new(self)
    }
    #[doc = "Bit 24 - Protection context 3, user read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc3_ur(&mut self) -> PC3_UR_W<24> {
        PC3_UR_W::new(self)
    }
    #[doc = "Bit 25 - Protection context 3, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc3_uw(&mut self) -> PC3_UW_W<25> {
        PC3_UW_W::new(self)
    }
    #[doc = "Bit 26 - Protection context 3, privileged read enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc3_pr(&mut self) -> PC3_PR_W<26> {
        PC3_PR_W::new(self)
    }
    #[doc = "Bit 27 - Protection context 3, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc3_pw(&mut self) -> PC3_PW_W<27> {
        PC3_PW_W::new(self)
    }
    #[doc = "Bit 28 - Protection context 3, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc3_ns(&mut self) -> PC3_NS_W<28> {
        PC3_NS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave attributes 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_att0](index.html) module"]
pub struct SL_ATT0_SPEC;
impl crate::RegisterSpec for SL_ATT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sl_att0::R](R) reader structure"]
impl crate::Readable for SL_ATT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sl_att0::W](W) writer structure"]
impl crate::Writable for SL_ATT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SL_ATT0 to value 0x1f1f_1f1f"]
impl crate::Resettable for SL_ATT0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f1f_1f1f;
}
