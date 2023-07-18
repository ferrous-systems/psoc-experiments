#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMPLETION` reader - Logical and of corresponding INTR.COMPLETION and INTR_MASK.COMPLETION fields."]
pub type COMPLETION_R = crate::BitReader;
#[doc = "Field `SRC_BUS_ERROR` reader - Logical and of corresponding INTR.SRC_BUS_ERROR and INTR_MASK.SRC_BUS_ERROR fields."]
pub type SRC_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `DST_BUS_ERROR` reader - Logical and of corresponding INTR.DST_BUS_ERROR and INTR_MASK.DST_BUS_ERROR fields."]
pub type DST_BUS_ERROR_R = crate::BitReader;
#[doc = "Field `SRC_MISAL` reader - Logical and of corresponding INTR.SRC_MISAL and INTR_MASK.SRC_MISAL fields."]
pub type SRC_MISAL_R = crate::BitReader;
#[doc = "Field `DST_MISAL` reader - Logical and of corresponding INTR.DST_MISAL and INTR_MASK.DST_MISAL fields."]
pub type DST_MISAL_R = crate::BitReader;
#[doc = "Field `CURR_PTR_NULL` reader - Logical and of corresponding INTR.CURR_PTR_NULL and INTR_MASK.CURR_PTR_NULL fields."]
pub type CURR_PTR_NULL_R = crate::BitReader;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - Logical and of corresponding INTR.ACTIVE_CH_DISABLED and INTR_MASK.ACTIVE_CH_DISABLED fields."]
pub type ACTIVE_CH_DISABLED_R = crate::BitReader;
#[doc = "Field `DESCR_BUS_ERROR` reader - Logical and of corresponding INTR.DESCR_BUS_ERROR and INTR_MASK.DESCR_BUS_ERROR fields."]
pub type DESCR_BUS_ERROR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding INTR.COMPLETION and INTR_MASK.COMPLETION fields."]
    #[inline(always)]
    pub fn completion(&self) -> COMPLETION_R {
        COMPLETION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding INTR.SRC_BUS_ERROR and INTR_MASK.SRC_BUS_ERROR fields."]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SRC_BUS_ERROR_R {
        SRC_BUS_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding INTR.DST_BUS_ERROR and INTR_MASK.DST_BUS_ERROR fields."]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DST_BUS_ERROR_R {
        DST_BUS_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding INTR.SRC_MISAL and INTR_MASK.SRC_MISAL fields."]
    #[inline(always)]
    pub fn src_misal(&self) -> SRC_MISAL_R {
        SRC_MISAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding INTR.DST_MISAL and INTR_MASK.DST_MISAL fields."]
    #[inline(always)]
    pub fn dst_misal(&self) -> DST_MISAL_R {
        DST_MISAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding INTR.CURR_PTR_NULL and INTR_MASK.CURR_PTR_NULL fields."]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CURR_PTR_NULL_R {
        CURR_PTR_NULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding INTR.ACTIVE_CH_DISABLED and INTR_MASK.ACTIVE_CH_DISABLED fields."]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ACTIVE_CH_DISABLED_R {
        ACTIVE_CH_DISABLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding INTR.DESCR_BUS_ERROR and INTR_MASK.DESCR_BUS_ERROR fields."]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DESCR_BUS_ERROR_R {
        DESCR_BUS_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
