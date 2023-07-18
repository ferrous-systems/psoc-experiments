#[doc = "Register `MS_SIZE` reader"]
pub struct R(crate::R<MS_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REGION_SIZE` reader - This field specifies the size of the master region: '5': 64 B region The master region includes the SL_ADDR, SL_SIZE, SL_ATT0, ..., SL_ATT3, MS_ADDR, MS_SIZE, MS_ATT0, ..., MS_ATT3 registers. Therefore, the access privileges for all these registers is determined by MS_ATT0, ..., MS_ATT3."]
pub type REGION_SIZE_R = crate::FieldReader;
#[doc = "Field `VALID` reader - Master region enable: '1': Enabled."]
pub type VALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 24:28 - This field specifies the size of the master region: '5': 64 B region The master region includes the SL_ADDR, SL_SIZE, SL_ATT0, ..., SL_ATT3, MS_ADDR, MS_SIZE, MS_ATT0, ..., MS_ATT3 registers. Therefore, the access privileges for all these registers is determined by MS_ATT0, ..., MS_ATT3."]
    #[inline(always)]
    pub fn region_size(&self) -> REGION_SIZE_R {
        REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Master region enable: '1': Enabled."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Master region, size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_size](index.html) module"]
pub struct MS_SIZE_SPEC;
impl crate::RegisterSpec for MS_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms_size::R](R) reader structure"]
impl crate::Readable for MS_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MS_SIZE to value 0x8500_0000"]
impl crate::Resettable for MS_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x8500_0000;
}
