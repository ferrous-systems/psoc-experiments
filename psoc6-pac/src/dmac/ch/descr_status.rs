#[doc = "Register `DESCR_STATUS` reader"]
pub struct R(crate::R<DESCR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALID` reader - Indicates whether the descriptor information present in DESCR_CTL, DESCR_SRC, DESCR_DST, DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE, DESCR_Y_INCR, DESCR_NEXT status registers is valid or not."]
pub type VALID_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - Indicates whether the descriptor information present in DESCR_CTL, DESCR_SRC, DESCR_DST, DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE, DESCR_Y_INCR, DESCR_NEXT status registers is valid or not."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel descriptor status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_status](index.html) module"]
pub struct DESCR_STATUS_SPEC;
impl crate::RegisterSpec for DESCR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr_status::R](R) reader structure"]
impl crate::Readable for DESCR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESCR_STATUS to value 0"]
impl crate::Resettable for DESCR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
