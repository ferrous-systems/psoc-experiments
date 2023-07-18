#[doc = "Register `RX_FIFO_RD_SILENT` reader"]
pub struct R(crate::R<RX_FIFO_RD_SILENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_RD_SILENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_RD_SILENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_RD_SILENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data read from the receiver FIFO. Reading a data frame will NOT remove the data frame from the FIFO; i.e. behavior is similar to that of a PEEK operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\]
are used. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data read from the receiver FIFO. Reading a data frame will NOT remove the data frame from the FIFO; i.e. behavior is similar to that of a PEEK operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\]
are used. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver FIFO read silent\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_rd_silent](index.html) module"]
pub struct RX_FIFO_RD_SILENT_SPEC;
impl crate::RegisterSpec for RX_FIFO_RD_SILENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_rd_silent::R](R) reader structure"]
impl crate::Readable for RX_FIFO_RD_SILENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_RD_SILENT to value 0"]
impl crate::Resettable for RX_FIFO_RD_SILENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
