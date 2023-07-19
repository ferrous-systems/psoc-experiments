#[doc = "Register `CM4_INT1_STATUS` reader"]
pub struct R(crate::R<CM4_INT1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_INT1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_INT1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_INT1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYSTEM_INT_IDX` reader - Lowest CM4 activated system interrupt index for CPU interrupt 1. See description of CM0_INT0_STATUS."]
pub type SYSTEM_INT_IDX_R = crate::FieldReader<u16>;
#[doc = "Field `SYSTEM_INT_VALID` reader - See description of CM0_INT0_STATUS."]
pub type SYSTEM_INT_VALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Lowest CM4 activated system interrupt index for CPU interrupt 1. See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SYSTEM_INT_IDX_R {
        SYSTEM_INT_IDX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_valid(&self) -> SYSTEM_INT_VALID_R {
        SYSTEM_INT_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "CM4 interrupt 1 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_int1_status](index.html) module"]
pub struct CM4_INT1_STATUS_SPEC;
impl crate::RegisterSpec for CM4_INT1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_int1_status::R](R) reader structure"]
impl crate::Readable for CM4_INT1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM4_INT1_STATUS to value 0"]
impl crate::Resettable for CM4_INT1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
