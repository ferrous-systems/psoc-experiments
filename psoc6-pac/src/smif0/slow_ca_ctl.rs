#[doc = "Register `SLOW_CA_CTL` reader"]
pub struct R(crate::R<SLOW_CA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOW_CA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOW_CA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOW_CA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLOW_CA_CTL` writer"]
pub struct W(crate::W<SLOW_CA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLOW_CA_CTL_SPEC>;
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
impl From<crate::W<SLOW_CA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLOW_CA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAY` reader - Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
pub type WAY_R = crate::FieldReader;
#[doc = "Field `WAY` writer - Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
pub type WAY_W<'a, const O: u8> = crate::FieldWriter<'a, SLOW_CA_CTL_SPEC, 2, O>;
#[doc = "Field `SET_ADDR` reader - Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
pub type SET_ADDR_R = crate::FieldReader;
#[doc = "Field `SET_ADDR` writer - Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
pub type SET_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, SLOW_CA_CTL_SPEC, 2, O>;
#[doc = "Field `PREF_EN` reader - Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PREF_EN_R = crate::BitReader;
#[doc = "Field `PREF_EN` writer - Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PREF_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLOW_CA_CTL_SPEC, O>;
#[doc = "Field `ENABLED` reader - Cache enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - Cache enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, SLOW_CA_CTL_SPEC, O>;
impl R {
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn way(&self) -> WAY_R {
        WAY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_addr(&self) -> SET_ADDR_R {
        SET_ADDR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cache enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    #[must_use]
    pub fn way(&mut self) -> WAY_W<16> {
        WAY_W::new(self)
    }
    #[doc = "Bits 24:25 - Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    #[must_use]
    pub fn set_addr(&mut self) -> SET_ADDR_W<24> {
        SET_ADDR_W::new(self)
    }
    #[doc = "Bit 30 - Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PREF_EN_W<30> {
        PREF_EN_W::new(self)
    }
    #[doc = "Bit 31 - Cache enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_ca_ctl](index.html) module"]
pub struct SLOW_CA_CTL_SPEC;
impl crate::RegisterSpec for SLOW_CA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slow_ca_ctl::R](R) reader structure"]
impl crate::Readable for SLOW_CA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slow_ca_ctl::W](W) writer structure"]
impl crate::Writable for SLOW_CA_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLOW_CA_CTL to value 0xc000_0000"]
impl crate::Resettable for SLOW_CA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
