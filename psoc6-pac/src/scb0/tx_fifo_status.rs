#[doc = "Register `TX_FIFO_STATUS` reader"]
pub struct R(crate::R<TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - Amount of entries in the transmitter FIFO. The value of this field ranges from 0 to FF_DATA_NR (EZ_DATA_NR/2)."]
pub type USED_R = crate::FieldReader<u16>;
#[doc = "Field `SR_VALID` reader - Indicates whether the TX shift registers holds a valid data frame ('1') or not ('0'). The shift register can be considered the top of the TX FIFO (the data frame is not included in the USED field of the TX FIFO). The shift register is a working register and holds the data frame that is currently transmitted (when the protocol state machine is transmitting a data frame) or the data frame that is transmitted next (when the protocol state machine is not transmitting a data frame)."]
pub type SR_VALID_R = crate::BitReader;
#[doc = "Field `RD_PTR` reader - FIFO read pointer: FIFO location from which a data frame is read by the hardware."]
pub type RD_PTR_R = crate::FieldReader;
#[doc = "Field `WR_PTR` reader - FIFO write pointer: FIFO location at which a new data frame is written."]
pub type WR_PTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - Amount of entries in the transmitter FIFO. The value of this field ranges from 0 to FF_DATA_NR (EZ_DATA_NR/2)."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Indicates whether the TX shift registers holds a valid data frame ('1') or not ('0'). The shift register can be considered the top of the TX FIFO (the data frame is not included in the USED field of the TX FIFO). The shift register is a working register and holds the data frame that is currently transmitted (when the protocol state machine is transmitting a data frame) or the data frame that is transmitted next (when the protocol state machine is not transmitting a data frame)."]
    #[inline(always)]
    pub fn sr_valid(&self) -> SR_VALID_R {
        SR_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - FIFO read pointer: FIFO location from which a data frame is read by the hardware."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FIFO write pointer: FIFO location at which a new data frame is written."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Transmitter FIFO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_status](index.html) module"]
pub struct TX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for TX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_fifo_status::R](R) reader structure"]
impl crate::Readable for TX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_FIFO_STATUS to value 0"]
impl crate::Resettable for TX_FIFO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
