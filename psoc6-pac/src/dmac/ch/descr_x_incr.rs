#[doc = "Register `DESCR_X_INCR` reader"]
pub struct R(crate::R<DESCR_X_INCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESCR_X_INCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESCR_X_INCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESCR_X_INCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC_X` reader - Specifies increment of source address for each X loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number (sign-magnitude format) in the range \\[-32768, 32767\\]. If this field is '0', the source address is not incremented. This is useful for reading from RX FIFO structures."]
pub type SRC_X_R = crate::FieldReader<u16>;
#[doc = "Field `DST_X` reader - Specifies increment of destination address for each X loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number (sign-magnitude format) in the range \\[-32768, 32767\\]. If this field is '0', the destination address is not incremented. This is useful for writing to TX FIFO structures."]
pub type DST_X_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Specifies increment of source address for each X loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number (sign-magnitude format) in the range \\[-32768, 32767\\]. If this field is '0', the source address is not incremented. This is useful for reading from RX FIFO structures."]
    #[inline(always)]
    pub fn src_x(&self) -> SRC_X_R {
        SRC_X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies increment of destination address for each X loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number (sign-magnitude format) in the range \\[-32768, 32767\\]. If this field is '0', the destination address is not incremented. This is useful for writing to TX FIFO structures."]
    #[inline(always)]
    pub fn dst_x(&self) -> DST_X_R {
        DST_X_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Channel descriptor X increment\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [descr_x_incr](index.html) module"]
pub struct DESCR_X_INCR_SPEC;
impl crate::RegisterSpec for DESCR_X_INCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [descr_x_incr::R](R) reader structure"]
impl crate::Readable for DESCR_X_INCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DESCR_X_INCR to value 0"]
impl crate::Resettable for DESCR_X_INCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
