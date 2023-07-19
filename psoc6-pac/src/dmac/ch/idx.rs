#[doc = "Register `IDX` reader"]
pub struct R(crate::R<IDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `X` reader - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor."]
pub type X_R = crate::FieldReader<u16>;
#[doc = "Field `Y` reader - Specifies the Y loop index, with Y_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor.."]
pub type Y_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies the Y loop index, with Y_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor.."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Channel current indices\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idx](index.html) module"]
pub struct IDX_SPEC;
impl crate::RegisterSpec for IDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idx::R](R) reader structure"]
impl crate::Readable for IDX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDX to value 0"]
impl crate::Resettable for IDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
