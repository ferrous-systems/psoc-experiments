#[doc = "Register `ACT_DESCR_Y_CTL` reader"]
pub struct R(crate::R<ACT_DESCR_Y_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACT_DESCR_Y_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACT_DESCR_Y_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACT_DESCR_Y_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Copy of DESCR_Y_CTL of the currently active descriptor. \\[11:0\\]
SRC_Y_INCR Specifies increment of source address for each Y loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. \\[23:12\\]
DST_Y_INCR Specifies increment of destination address for each Y loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. \\[31:24\\]
Y_COUNT Number of iterations (minus 1) of the 'Y loop' (X_COUNT+1)*(Y_COUNT+1) is the number of single transfers in a 2D transfer). This field is an unsigned number in the range \\[0, 255\\], representing 1 through 256 iterations. For single, 1D and CRC transfer descriptor types, descriptor will not have Y_CTL."]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Copy of DESCR_Y_CTL of the currently active descriptor. \\[11:0\\]
SRC_Y_INCR Specifies increment of source address for each Y loop iteration (in multiples of SRC_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. \\[23:12\\]
DST_Y_INCR Specifies increment of destination address for each Y loop iteration (in multiples of DST_TRANSFER_SIZE). This field is a signed number in the range \\[-2048, 2047\\]. \\[31:24\\]
Y_COUNT Number of iterations (minus 1) of the 'Y loop' (X_COUNT+1)*(Y_COUNT+1) is the number of single transfers in a 2D transfer). This field is an unsigned number in the range \\[0, 255\\], representing 1 through 256 iterations. For single, 1D and CRC transfer descriptor types, descriptor will not have Y_CTL."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Active descriptor Y loop control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [act_descr_y_ctl](index.html) module"]
pub struct ACT_DESCR_Y_CTL_SPEC;
impl crate::RegisterSpec for ACT_DESCR_Y_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [act_descr_y_ctl::R](R) reader structure"]
impl crate::Readable for ACT_DESCR_Y_CTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACT_DESCR_Y_CTL to value 0"]
impl crate::Resettable for ACT_DESCR_Y_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
