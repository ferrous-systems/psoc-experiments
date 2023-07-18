#[doc = "Register `HOST_CNTRL_VERS_R` reader"]
pub struct R(crate::R<HOST_CNTRL_VERS_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_CNTRL_VERS_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_CNTRL_VERS_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_CNTRL_VERS_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPEC_VERSION_NUM` reader - N/A"]
pub type SPEC_VERSION_NUM_R = crate::FieldReader;
#[doc = "Field `VENDOR_VERSION_NUM` reader - N/A"]
pub type VENDOR_VERSION_NUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn spec_version_num(&self) -> SPEC_VERSION_NUM_R {
        SPEC_VERSION_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - N/A"]
    #[inline(always)]
    pub fn vendor_version_num(&self) -> VENDOR_VERSION_NUM_R {
        VENDOR_VERSION_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_cntrl_vers_r](index.html) module"]
pub struct HOST_CNTRL_VERS_R_SPEC;
impl crate::RegisterSpec for HOST_CNTRL_VERS_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [host_cntrl_vers_r::R](R) reader structure"]
impl crate::Readable for HOST_CNTRL_VERS_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_CNTRL_VERS_R to value 0x05"]
impl crate::Resettable for HOST_CNTRL_VERS_R_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
