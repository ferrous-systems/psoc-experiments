#[doc = "Register `SPI_STATUS` reader"]
pub struct R(crate::R<SPI_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUS_BUSY` reader - SPI bus is busy. The bus is considered busy ('1') during an ongoing transaction. For Motorola and National submodes, the busy bit is '1', when the slave selection is activated. For TI submode, the busy bit is '1' from the time the preceding/coinciding slave select is activated for the first transmitted data frame, till the last MOSI/MISO bit of the last data frame is transmitted."]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `SPI_EC_BUSY` reader - Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_ADDR or CURR_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_ADDR and CURR_ADDR are reliable."]
pub type SPI_EC_BUSY_R = crate::BitReader;
#[doc = "Field `CURR_EZ_ADDR` reader - SPI current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when SPI_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
pub type CURR_EZ_ADDR_R = crate::FieldReader;
#[doc = "Field `BASE_EZ_ADDR` reader - SPI base EZ address. Address as provided by a SPI write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
pub type BASE_EZ_ADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SPI bus is busy. The bus is considered busy ('1') during an ongoing transaction. For Motorola and National submodes, the busy bit is '1', when the slave selection is activated. For TI submode, the busy bit is '1' from the time the preceding/coinciding slave select is activated for the first transmitted data frame, till the last MOSI/MISO bit of the last data frame is transmitted."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_ADDR or CURR_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_ADDR and CURR_ADDR are reliable."]
    #[inline(always)]
    pub fn spi_ec_busy(&self) -> SPI_EC_BUSY_R {
        SPI_EC_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SPI current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when SPI_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn curr_ez_addr(&self) -> CURR_EZ_ADDR_R {
        CURR_EZ_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI base EZ address. Address as provided by a SPI write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn base_ez_addr(&self) -> BASE_EZ_ADDR_R {
        BASE_EZ_ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SPI status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_status](index.html) module"]
pub struct SPI_STATUS_SPEC;
impl crate::RegisterSpec for SPI_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_status::R](R) reader structure"]
impl crate::Readable for SPI_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_STATUS to value 0"]
impl crate::Resettable for SPI_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
