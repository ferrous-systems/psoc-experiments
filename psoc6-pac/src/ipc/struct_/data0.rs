#[doc = "Register `DATA0` reader"]
pub struct R(crate::R<DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA0` writer"]
pub struct W(crate::W<DATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0_SPEC>;
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
impl From<crate::W<DATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - This field holds a 32-bit data element that is associated with the IPC structure."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - This field holds a 32-bit data element that is associated with the IPC structure."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, DATA0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This field holds a 32-bit data element that is associated with the IPC structure."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field holds a 32-bit data element that is associated with the IPC structure."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC data 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0](index.html) module"]
pub struct DATA0_SPEC;
impl crate::RegisterSpec for DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0::R](R) reader structure"]
impl crate::Readable for DATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0::W](W) writer structure"]
impl crate::Writable for DATA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
