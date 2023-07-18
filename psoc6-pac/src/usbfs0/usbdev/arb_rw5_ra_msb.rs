#[doc = "Register `ARB_RW5_RA_MSB` reader"]
pub struct R(crate::R<ARB_RW5_RA_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_RW5_RA_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_RW5_RA_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_RW5_RA_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_RW5_RA_MSB` writer"]
pub struct W(crate::W<ARB_RW5_RA_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_RW5_RA_MSB_SPEC>;
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
impl From<crate::W<ARB_RW5_RA_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_RW5_RA_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA_MSB` reader - Read Address for EP"]
pub type RA_MSB_R = crate::BitReader;
#[doc = "Field `RA_MSB` writer - Read Address for EP"]
pub type RA_MSB_W<'a, const O: u8> = crate::BitWriter<'a, ARB_RW5_RA_MSB_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Read Address for EP"]
    #[inline(always)]
    pub fn ra_msb(&self) -> RA_MSB_R {
        RA_MSB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn ra_msb(&mut self) -> RA_MSB_W<0> {
        RA_MSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Read Address value *1, *2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw5_ra_msb](index.html) module"]
pub struct ARB_RW5_RA_MSB_SPEC;
impl crate::RegisterSpec for ARB_RW5_RA_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_rw5_ra_msb::R](R) reader structure"]
impl crate::Readable for ARB_RW5_RA_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_rw5_ra_msb::W](W) writer structure"]
impl crate::Writable for ARB_RW5_RA_MSB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_RW5_RA_MSB to value 0"]
impl crate::Resettable for ARB_RW5_RA_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
