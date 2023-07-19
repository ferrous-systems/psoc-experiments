#[doc = "Register `ACT_DESCR_X_CTL` reader"]
pub struct R(crate::R<ACT_DESCR_X_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACT_DESCR_X_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACT_DESCR_X_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACT_DESCR_X_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Copy of DESCR_X_CTL of the currently active descriptor. \\[11:0\\]
SRC_X_INCR Specifies increment of source address for each X loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. If this field is '0', the source address is not incremented. This is useful for reading from RX FIFO structures. \\[23:12\\]
DST_X_INCR Specifies increment of destination address for each X loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. If this field is '0', the destination address is not incremented. This is useful for writing to TX FIFO structures. Note: this field is not used for CRC transfer descriptors and must be set to '0'. \\[31:24\\]
X_COUNT Number of iterations (minus 1) of the 'X loop' (X_COUNT+1 is the number of single transfers in a 1D transfer). This field is an unsigned number in the range \\[0, 255\\], representing 1 through 256 iterations. For a single transfer descriptor type, descriptor will not have X_CTL."]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Copy of DESCR_X_CTL of the currently active descriptor. \\[11:0\\]
SRC_X_INCR Specifies increment of source address for each X loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. If this field is '0', the source address is not incremented. This is useful for reading from RX FIFO structures. \\[23:12\\]
DST_X_INCR Specifies increment of destination address for each X loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. If this field is '0', the destination address is not incremented. This is useful for writing to TX FIFO structures. Note: this field is not used for CRC transfer descriptors and must be set to '0'. \\[31:24\\]
X_COUNT Number of iterations (minus 1) of the 'X loop' (X_COUNT+1 is the number of single transfers in a 1D transfer). This field is an unsigned number in the range \\[0, 255\\], representing 1 through 256 iterations. For a single transfer descriptor type, descriptor will not have X_CTL."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Active descriptor X loop control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_x_ctl](index.html) module"]
pub struct ACT_DESCR_X_CTL_SPEC;
impl crate::RegisterSpec for ACT_DESCR_X_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [act_descr_x_ctl::R](R) reader structure"]
impl crate::Readable for ACT_DESCR_X_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACT_DESCR_X_CTL to value 0"]
impl crate::Resettable for ACT_DESCR_X_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
