#[doc = "Register `CQCFG` reader"]
pub struct R(crate::R<CQCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQCFG` writer"]
pub struct W(crate::W<CQCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CQCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQ_EN` reader - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
pub type CQ_EN_R = crate::BitReader;
#[doc = "Field `CQ_EN` writer - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
pub type CQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, CQCFG_SPEC, O>;
#[doc = "Field `CR_GENERAL_EN` reader - N/A"]
pub type CR_GENERAL_EN_R = crate::BitReader;
#[doc = "Field `CR_GENERAL_EN` writer - N/A"]
pub type CR_GENERAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, CQCFG_SPEC, O>;
#[doc = "Field `TASK_DESC_SIZE` reader - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
pub type TASK_DESC_SIZE_R = crate::BitReader;
#[doc = "Field `TASK_DESC_SIZE` writer - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
pub type TASK_DESC_SIZE_W<'a, const O: u8> = crate::BitWriter<'a, CQCFG_SPEC, O>;
#[doc = "Field `DCMD_EN` reader - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
pub type DCMD_EN_R = crate::BitReader;
#[doc = "Field `DCMD_EN` writer - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
pub type DCMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, CQCFG_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
    #[inline(always)]
    pub fn cq_en(&self) -> CQ_EN_R {
        CQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn cr_general_en(&self) -> CR_GENERAL_EN_R {
        CR_GENERAL_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
    #[inline(always)]
    pub fn task_desc_size(&self) -> TASK_DESC_SIZE_R {
        TASK_DESC_SIZE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    pub fn dcmd_en(&self) -> DCMD_EN_R {
        DCMD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
    #[inline(always)]
    #[must_use]
    pub fn cq_en(&mut self) -> CQ_EN_W<0> {
        CQ_EN_W::new(self)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cr_general_en(&mut self) -> CR_GENERAL_EN_W<1> {
        CR_GENERAL_EN_W::new(self)
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
    #[inline(always)]
    #[must_use]
    pub fn task_desc_size(&mut self) -> TASK_DESC_SIZE_W<8> {
        TASK_DESC_SIZE_W::new(self)
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn dcmd_en(&mut self) -> DCMD_EN_W<12> {
        DCMD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Queuing Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcfg](index.html) module"]
pub struct CQCFG_SPEC;
impl crate::RegisterSpec for CQCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqcfg::R](R) reader structure"]
impl crate::Readable for CQCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqcfg::W](W) writer structure"]
impl crate::Writable for CQCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQCFG to value 0"]
impl crate::Resettable for CQCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
