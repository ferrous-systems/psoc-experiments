#[doc = "Register `ACT_DESCR_CTL` reader"]
pub struct R(crate::R<ACT_DESCR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACT_DESCR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACT_DESCR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACT_DESCR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - N/A"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Active descriptor control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_ctl](index.html) module"]
pub struct ACT_DESCR_CTL_SPEC;
impl crate::RegisterSpec for ACT_DESCR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [act_descr_ctl::R](R) reader structure"]
impl crate::Readable for ACT_DESCR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACT_DESCR_CTL to value 0"]
impl crate::Resettable for ACT_DESCR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
