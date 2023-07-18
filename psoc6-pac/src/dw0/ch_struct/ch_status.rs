#[doc = "Register `CH_STATUS` reader"]
pub struct R(crate::R<CH_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTR_CAUSE` reader - Specifies the source of the interrupt cause: '0': No interrupt generated '1': Interrupt based on transfer complettion configuration based on INTR_TYPE '2': Source transfer bus error '3': Destination transfer bus error '4': Source address misalignment '5': Destination address misalignment '6': Current descriptor pointer is null '7': Active channel is disabled '8': Descriptor bus error '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
pub type INTR_CAUSE_R = crate::FieldReader;
#[doc = "Field `PENDING` reader - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
pub type PENDING_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Specifies the source of the interrupt cause: '0': No interrupt generated '1': Interrupt based on transfer complettion configuration based on INTR_TYPE '2': Source transfer bus error '3': Destination transfer bus error '4': Source address misalignment '5': Destination address misalignment '6': Current descriptor pointer is null '7': Active channel is disabled '8': Descriptor bus error '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
    #[inline(always)]
    pub fn intr_cause(&self) -> INTR_CAUSE_R {
        INTR_CAUSE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_status](index.html) module"]
pub struct CH_STATUS_SPEC;
impl crate::RegisterSpec for CH_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_status::R](R) reader structure"]
impl crate::Readable for CH_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH_STATUS to value 0"]
impl crate::Resettable for CH_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
