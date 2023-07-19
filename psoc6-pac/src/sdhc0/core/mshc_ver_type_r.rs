#[doc = "Register `MSHC_VER_TYPE_R` reader"]
pub struct R(crate::R<MSHC_VER_TYPE_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSHC_VER_TYPE_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSHC_VER_TYPE_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSHC_VER_TYPE_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSHC_VER_TYPE` reader - Current release type This field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release type that is read by an application. For example, release type is 'ga' is represented in ASCII as 0x6761. Lower 16 bits read from this register can be ignored by the application. An application reading this register in conjunction with the MSHC_VER_ID_R register, gathers details of the current release."]
pub type MSHC_VER_TYPE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current release type This field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release type that is read by an application. For example, release type is 'ga' is represented in ASCII as 0x6761. Lower 16 bits read from this register can be ignored by the application. An application reading this register in conjunction with the MSHC_VER_ID_R register, gathers details of the current release."]
    #[inline(always)]
    pub fn mshc_ver_type(&self) -> MSHC_VER_TYPE_R {
        MSHC_VER_TYPE_R::new(self.bits)
    }
}
#[doc = "MSHC version type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mshc_ver_type_r](index.html) module"]
pub struct MSHC_VER_TYPE_R_SPEC;
impl crate::RegisterSpec for MSHC_VER_TYPE_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mshc_ver_type_r::R](R) reader structure"]
impl crate::Readable for MSHC_VER_TYPE_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSHC_VER_TYPE_R to value 0x6761_2a2a"]
impl crate::Resettable for MSHC_VER_TYPE_R_SPEC {
    const RESET_VALUE: Self::Ux = 0x6761_2a2a;
}
