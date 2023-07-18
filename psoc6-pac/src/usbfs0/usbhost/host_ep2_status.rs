#[doc = "Register `HOST_EP2_STATUS` reader"]
pub struct R(crate::R<HOST_EP2_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_EP2_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_EP2_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_EP2_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIZE2` reader - These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP2 has finished. The indication range is from 0x000 to 0x40. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
pub type SIZE2_R = crate::FieldReader;
#[doc = "Field `VAL_DATA` reader - This bit shows that there is valid data in the EP2 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
pub type VAL_DATA_R = crate::BitReader;
#[doc = "Field `INI_ST` reader - This bit shows that EP2 is initialized. If the BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' and EP2 is initialized, this bit is to '1'. '0' : Not Initialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately evne if BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '0' or '1'."]
pub type INI_ST_R = crate::BitReader;
#[doc = "Field `RSVD_18` reader - N/A"]
pub type RSVD_18_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - These bits indicate the number of data bytes written to the receive buffer when IN packet transfer of EP2 has finished. The indication range is from 0x000 to 0x40. Note : - These bits are set to the data size transferred in the IN direction and written to the buffer. Therefore, a value read during transfer in the OUT direction has no effect."]
    #[inline(always)]
    pub fn size2(&self) -> SIZE2_R {
        SIZE2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - This bit shows that there is valid data in the EP2 buffer. '0' : Invalid data in the buffer '1' : Valid data in the buffer"]
    #[inline(always)]
    pub fn val_data(&self) -> VAL_DATA_R {
        VAL_DATA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit shows that EP2 is initialized. If the BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' and EP2 is initialized, this bit is to '1'. '0' : Not Initialized '1' : Initialized Note: - This bit isn't set to '0' or '1' immediately evne if BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '0' or '1'."]
    #[inline(always)]
    pub fn ini_st(&self) -> INI_ST_R {
        INI_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn rsvd_18(&self) -> RSVD_18_R {
        RSVD_18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Host Endpoint 2 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep2_status](index.html) module"]
pub struct HOST_EP2_STATUS_SPEC;
impl crate::RegisterSpec for HOST_EP2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_ep2_status::R](R) reader structure"]
impl crate::Readable for HOST_EP2_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_EP2_STATUS to value 0x0006_0000"]
impl crate::Resettable for HOST_EP2_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_0000;
}
