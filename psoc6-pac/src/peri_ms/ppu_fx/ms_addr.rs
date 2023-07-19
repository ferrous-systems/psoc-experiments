#[doc = "Register `MS_ADDR` reader"]
pub struct R(crate::R<MS_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR26` reader - This field specifies the base address of the master region. The base address of the region is the address of the SL_ADDR register."]
pub type ADDR26_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 6:31 - This field specifies the base address of the master region. The base address of the region is the address of the SL_ADDR register."]
    #[inline(always)]
    pub fn addr26(&self) -> ADDR26_R {
        ADDR26_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
#[doc = "Master region, base address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_addr](index.html) module"]
pub struct MS_ADDR_SPEC;
impl crate::RegisterSpec for MS_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms_addr::R](R) reader structure"]
impl crate::Readable for MS_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MS_ADDR to value 0"]
impl crate::Resettable for MS_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
