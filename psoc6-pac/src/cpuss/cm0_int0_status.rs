#[doc = "Register `CM0_INT0_STATUS` reader"]
pub struct R(crate::R<CM0_INT0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_INT0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_INT0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_INT0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYSTEM_INT_IDX` reader - Lowest CM0+ activated system interrupt index for CPU interrupt 0. Multiple system interrupts can be mapped on the same CPU interrupt. The selected system interrupt is the system interrupt with the lowest system interrupt index that has an activated interrupt request at the time of the fetch (system_interrupts\\[SYSTEM_INT_IDX\\]
is '1'). The CPU interrupt handler SW can read SYSTEM_INT_IDX to determine the system interrupt that activated the handler."]
pub type SYSTEM_INT_IDX_R = crate::FieldReader<u16>;
#[doc = "Field `SYSTEM_INT_VALID` reader - Valid indication for SYSTEM_INT_IDX. When '0', no system interrupt for CPU interrupt 0 is valid/activated."]
pub type SYSTEM_INT_VALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Lowest CM0+ activated system interrupt index for CPU interrupt 0. Multiple system interrupts can be mapped on the same CPU interrupt. The selected system interrupt is the system interrupt with the lowest system interrupt index that has an activated interrupt request at the time of the fetch (system_interrupts\\[SYSTEM_INT_IDX\\]
is '1'). The CPU interrupt handler SW can read SYSTEM_INT_IDX to determine the system interrupt that activated the handler."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SYSTEM_INT_IDX_R {
        SYSTEM_INT_IDX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Valid indication for SYSTEM_INT_IDX. When '0', no system interrupt for CPU interrupt 0 is valid/activated."]
    #[inline(always)]
    pub fn system_int_valid(&self) -> SYSTEM_INT_VALID_R {
        SYSTEM_INT_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "CM0+ interrupt 0 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_int0_status](index.html) module"]
pub struct CM0_INT0_STATUS_SPEC;
impl crate::RegisterSpec for CM0_INT0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_int0_status::R](R) reader structure"]
impl crate::Readable for CM0_INT0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CM0_INT0_STATUS to value 0"]
impl crate::Resettable for CM0_INT0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
