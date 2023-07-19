#[doc = "Register `DESCR_Y_INCR` reader"]
pub struct R(crate::R<DESCR_Y_INCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_Y_INCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_Y_INCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_Y_INCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC_Y` reader - Specifies increment of source address for each Y loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-32768, 32767\\]."]
pub type SRC_Y_R = crate::FieldReader<u16>;
#[doc = "Field `DST_Y` reader - Specifies increment of destination address for each Y loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-32768, 32767\\]."]
pub type DST_Y_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Specifies increment of source address for each Y loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-32768, 32767\\]."]
    #[inline(always)]
    pub fn src_y(&self) -> SRC_Y_R {
        SRC_Y_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies increment of destination address for each Y loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-32768, 32767\\]."]
    #[inline(always)]
    pub fn dst_y(&self) -> DST_Y_R {
        DST_Y_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor Y increment\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_y_incr](index.html) module"]
pub struct DESCR_Y_INCR_SPEC;
impl crate::RegisterSpec for DESCR_Y_INCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr_y_incr::R](R) reader structure"]
impl crate::Readable for DESCR_Y_INCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESCR_Y_INCR to value 0"]
impl crate::Resettable for DESCR_Y_INCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
