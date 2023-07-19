#[doc = "Register `PRODUCT_ID` reader"]
pub struct R(crate::R<PRODUCT_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRODUCT_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRODUCT_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRODUCT_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FAMILY_ID` reader - Family ID. Common ID for a product family."]
pub type FAMILY_ID_R = crate::FieldReader<u16>;
#[doc = "Field `MAJOR_REV` reader - Major Revision, starts with 1, increments with all layer tape-out (implemented with metal ECO-able tie-off)"]
pub type MAJOR_REV_R = crate::FieldReader;
#[doc = "Field `MINOR_REV` reader - Minor Revision, starts with 1, increments with metal layer only tape-out (implemented with metal ECO-able tie-off)"]
pub type MINOR_REV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Family ID. Common ID for a product family."]
    #[inline(always)]
    pub fn family_id(&self) -> FAMILY_ID_R {
        FAMILY_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Major Revision, starts with 1, increments with all layer tape-out (implemented with metal ECO-able tie-off)"]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Minor Revision, starts with 1, increments with metal layer only tape-out (implemented with metal ECO-able tie-off)"]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "Product identifier and version (same as CoreSight RomTables)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [product_id](index.html) module"]
pub struct PRODUCT_ID_SPEC;
impl crate::RegisterSpec for PRODUCT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [product_id::R](R) reader structure"]
impl crate::Readable for PRODUCT_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRODUCT_ID to value 0"]
impl crate::Resettable for PRODUCT_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
