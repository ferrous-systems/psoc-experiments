#[doc = "Register `WR_DATA_CTL` reader"]
pub struct R(crate::R<WR_DATA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_DATA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_DATA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_DATA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_DATA_CTL` writer"]
pub struct W(crate::W<WR_DATA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_DATA_CTL_SPEC>;
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
impl From<crate::W<WR_DATA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_DATA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub type WIDTH_R = crate::FieldReader;
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, WR_DATA_CTL_SPEC, 2, O>;
impl R {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<16> {
        WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write data control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_data_ctl](index.html) module"]
pub struct WR_DATA_CTL_SPEC;
impl crate::RegisterSpec for WR_DATA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_data_ctl::R](R) reader structure"]
impl crate::Readable for WR_DATA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_data_ctl::W](W) writer structure"]
impl crate::Writable for WR_DATA_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_DATA_CTL to value 0"]
impl crate::Resettable for WR_DATA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
