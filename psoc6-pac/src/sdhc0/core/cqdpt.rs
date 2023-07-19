#[doc = "Register `CQDPT` reader"]
pub struct R(crate::R<CQDPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQDPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQDPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQDPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DPT` reader - Device-Pending Tasks Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has been successfully queued into the device and is awaiting execution - Bit-N(0): Task-N is not yet queued. Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task has not been executed. The controller sets this bit after receiving a successful response for CMD45. CQE clears this bit after the task has completed execution. Software reads this register in the task-discard procedure to determine if the task is queued in the device."]
pub type DPT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device-Pending Tasks Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has been successfully queued into the device and is awaiting execution - Bit-N(0): Task-N is not yet queued. Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task has not been executed. The controller sets this bit after receiving a successful response for CMD45. CQE clears this bit after the task has completed execution. Software reads this register in the task-discard procedure to determine if the task is queued in the device."]
    #[inline(always)]
    pub fn dpt(&self) -> DPT_R {
        DPT_R::new(self.bits)
    }
}
#[doc = "Device pending tasks register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqdpt](index.html) module"]
pub struct CQDPT_SPEC;
impl crate::RegisterSpec for CQDPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqdpt::R](R) reader structure"]
impl crate::Readable for CQDPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CQDPT to value 0"]
impl crate::Resettable for CQDPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
