#[doc = "Register `CWA` reader"]
pub struct R(crate::R<CWA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWA` writer"]
pub struct W(crate::W<CWA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWA_SPEC>;
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
impl From<crate::W<CWA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWA` reader - Write Address for Common Area"]
pub type CWA_R = crate::FieldReader;
#[doc = "Field `CWA` writer - Write Address for Common Area"]
pub type CWA_W<'a, const O: u8> = crate::FieldWriter<'a, CWA_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa(&self) -> CWA_R {
        CWA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    #[must_use]
    pub fn cwa(&mut self) -> CWA_W<0> {
        CWA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common Area Write Address *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwa](index.html) module"]
pub struct CWA_SPEC;
impl crate::RegisterSpec for CWA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwa::R](R) reader structure"]
impl crate::Readable for CWA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwa::W](W) writer structure"]
impl crate::Writable for CWA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWA to value 0"]
impl crate::Resettable for CWA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
