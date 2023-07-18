#[doc = "Register `RED_CTL45` reader"]
pub struct R(crate::R<RED_CTL45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RED_CTL45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RED_CTL45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RED_CTL45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RED_CTL45` writer"]
pub struct W(crate::W<RED_CTL45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RED_CTL45_SPEC>;
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
impl From<crate::W<RED_CTL45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RED_CTL45_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RED_ADDR_4` reader - Bad Row Pair Address for Sector 4"]
pub type RED_ADDR_4_R = crate::FieldReader;
#[doc = "Field `RED_ADDR_4` writer - Bad Row Pair Address for Sector 4"]
pub type RED_ADDR_4_W<'a, const O: u8> = crate::FieldWriter<'a, RED_CTL45_SPEC, 8, O>;
#[doc = "Field `RED_EN_4` reader - 1: Redundancy Enable for Sector 4"]
pub type RED_EN_4_R = crate::BitReader;
#[doc = "Field `RED_EN_4` writer - 1: Redundancy Enable for Sector 4"]
pub type RED_EN_4_W<'a, const O: u8> = crate::BitWriter<'a, RED_CTL45_SPEC, O>;
#[doc = "Field `RED_ADDR_5` reader - Bad Row Pair Address for Sector 5"]
pub type RED_ADDR_5_R = crate::FieldReader;
#[doc = "Field `RED_ADDR_5` writer - Bad Row Pair Address for Sector 5"]
pub type RED_ADDR_5_W<'a, const O: u8> = crate::FieldWriter<'a, RED_CTL45_SPEC, 8, O>;
#[doc = "Field `RED_EN_5` reader - 1: Redundancy Enable for Sector 5"]
pub type RED_EN_5_R = crate::BitReader;
#[doc = "Field `RED_EN_5` writer - 1: Redundancy Enable for Sector 5"]
pub type RED_EN_5_W<'a, const O: u8> = crate::BitWriter<'a, RED_CTL45_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 4"]
    #[inline(always)]
    pub fn red_addr_4(&self) -> RED_ADDR_4_R {
        RED_ADDR_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 4"]
    #[inline(always)]
    pub fn red_en_4(&self) -> RED_EN_4_R {
        RED_EN_4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 5"]
    #[inline(always)]
    pub fn red_addr_5(&self) -> RED_ADDR_5_R {
        RED_ADDR_5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 5"]
    #[inline(always)]
    pub fn red_en_5(&self) -> RED_EN_5_R {
        RED_EN_5_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 4"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_4(&mut self) -> RED_ADDR_4_W<0> {
        RED_ADDR_4_W::new(self)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 4"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_4(&mut self) -> RED_EN_4_W<8> {
        RED_EN_4_W::new(self)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 5"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_5(&mut self) -> RED_ADDR_5_W<16> {
        RED_ADDR_5_W::new(self)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 5"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_5(&mut self) -> RED_EN_5_W<24> {
        RED_EN_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Redundancy Control normal sectors 4,5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl45](index.html) module"]
pub struct RED_CTL45_SPEC;
impl crate::RegisterSpec for RED_CTL45_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [red_ctl45::R](R) reader structure"]
impl crate::Readable for RED_CTL45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [red_ctl45::W](W) writer structure"]
impl crate::Writable for RED_CTL45_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RED_CTL45 to value 0"]
impl crate::Resettable for RED_CTL45_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
