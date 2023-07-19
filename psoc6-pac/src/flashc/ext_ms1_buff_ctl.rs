#[doc = "Register `EXT_MS1_BUFF_CTL` reader"]
pub struct R(crate::R<EXT_MS1_BUFF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_MS1_BUFF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_MS1_BUFF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_MS1_BUFF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_MS1_BUFF_CTL` writer"]
pub struct W(crate::W<EXT_MS1_BUFF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_MS1_BUFF_CTL_SPEC>;
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
impl From<crate::W<EXT_MS1_BUFF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_MS1_BUFF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREF_EN` reader - See CRYPTO_BUFF_CTL."]
pub type PREF_EN_R = crate::BitReader;
#[doc = "Field `PREF_EN` writer - See CRYPTO_BUFF_CTL."]
pub type PREF_EN_W<'a, const O: u8> = crate::BitWriter<'a, EXT_MS1_BUFF_CTL_SPEC, O>;
impl R {
    #[doc = "Bit 30 - See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PREF_EN_W<30> {
        PREF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External master 1 buffer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_ms1_buff_ctl](index.html) module"]
pub struct EXT_MS1_BUFF_CTL_SPEC;
impl crate::RegisterSpec for EXT_MS1_BUFF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_ms1_buff_ctl::R](R) reader structure"]
impl crate::Readable for EXT_MS1_BUFF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_ms1_buff_ctl::W](W) writer structure"]
impl crate::Writable for EXT_MS1_BUFF_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_MS1_BUFF_CTL to value 0x4000_0000"]
impl crate::Resettable for EXT_MS1_BUFF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
