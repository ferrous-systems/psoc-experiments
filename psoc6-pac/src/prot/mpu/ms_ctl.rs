#[doc = "Register `MS_CTL` reader"]
pub struct R(crate::R<MS_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS_CTL` writer"]
pub struct W(crate::W<MS_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS_CTL_SPEC>;
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
impl From<crate::W<MS_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MS_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC` reader - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
pub type PC_R = crate::FieldReader;
#[doc = "Field `PC` writer - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
pub type PC_W<'a, const O: u8> = crate::FieldWriter<'a, MS_CTL_SPEC, 4, O>;
#[doc = "Field `PC_SAVED` reader - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
pub type PC_SAVED_R = crate::FieldReader;
#[doc = "Field `PC_SAVED` writer - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
pub type PC_SAVED_W<'a, const O: u8> = crate::FieldWriter<'a, MS_CTL_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
    #[inline(always)]
    pub fn pc_saved(&self) -> PC_SAVED_R {
        PC_SAVED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active protection context (PC). Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. In addition, a write transfer with protection context '0' can change this field (protection context 0 has unrestricted access). The CM0+ MPU MS_CTL register is special: the PC field is modifiable by BOTH HW and SW (for all other masters, the MPU MS_CTL.PC field is modifiable by SW ONLY. For CM0+ PC field HW modifications, the following holds: * On entry of a CM0_PC0/1/2/3_HANDLER exception/interrupt handler: IF (the new PC is the same as MS_CTL.PC) PC is not affected; PC_SAVED is not affected. ELSE IF (CM0_PC_CTL.VALID\\[MS_CTL.PC\\]) An AHB-Lite bus error is generated for the exception handler fetch; PC is not affected; PC_SAVED is not affected. ELSE PC = 'new PC'; PC_SAVED = PC (push operation). * On entry of any other exception/interrupt handler: PC = PC_SAVED; PC_SAVED is not affected (pop operation). Note that the CM0_PC0/1/2/3_HANDLER and CM0_PC_CTL registers are part of repecitve CPUSS MMIO registers. Note: this field is NOT used by the DW controllers, DMA controller, AXI DMA controller, CRYPTO component and VIDEOSS."]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<0> {
        PC_W::new(self)
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated SMPU MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\]
fields. Note: this field is ONLY used by the CM0+."]
    #[inline(always)]
    #[must_use]
    pub fn pc_saved(&mut self) -> PC_SAVED_W<16> {
        PC_SAVED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_ctl](index.html) module"]
pub struct MS_CTL_SPEC;
impl crate::RegisterSpec for MS_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms_ctl::R](R) reader structure"]
impl crate::Readable for MS_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms_ctl::W](W) writer structure"]
impl crate::Writable for MS_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MS_CTL to value 0"]
impl crate::Resettable for MS_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
