#[doc = "Register `CM0_SYSTEM_INT_CTL[%s]` reader"]
pub struct R(crate::R<CM0_SYSTEM_INT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_SYSTEM_INT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_SYSTEM_INT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_SYSTEM_INT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0_SYSTEM_INT_CTL[%s]` writer"]
pub struct W(crate::W<CM0_SYSTEM_INT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0_SYSTEM_INT_CTL_SPEC>;
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
impl From<crate::W<CM0_SYSTEM_INT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0_SYSTEM_INT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_INT_IDX` reader - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
pub type CPU_INT_IDX_R = crate::FieldReader;
#[doc = "Field `CPU_INT_IDX` writer - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
pub type CPU_INT_IDX_W<'a, const O: u8> = crate::FieldWriter<'a, CM0_SYSTEM_INT_CTL_SPEC, 3, O>;
#[doc = "Field `CPU_INT_VALID` reader - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
pub type CPU_INT_VALID_R = crate::BitReader;
#[doc = "Field `CPU_INT_VALID` writer - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
pub type CPU_INT_VALID_W<'a, const O: u8> = crate::BitWriter<'a, CM0_SYSTEM_INT_CTL_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
    #[inline(always)]
    pub fn cpu_int_idx(&self) -> CPU_INT_IDX_R {
        CPU_INT_IDX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
    #[inline(always)]
    pub fn cpu_int_valid(&self) -> CPU_INT_VALID_R {
        CPU_INT_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_idx(&mut self) -> CPU_INT_IDX_W<0> {
        CPU_INT_IDX_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_valid(&mut self) -> CPU_INT_VALID_W<31> {
        CPU_INT_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM0+ system interrupt control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_system_int_ctl](index.html) module"]
pub struct CM0_SYSTEM_INT_CTL_SPEC;
impl crate::RegisterSpec for CM0_SYSTEM_INT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_system_int_ctl::R](R) reader structure"]
impl crate::Readable for CM0_SYSTEM_INT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0_system_int_ctl::W](W) writer structure"]
impl crate::Writable for CM0_SYSTEM_INT_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM0_SYSTEM_INT_CTL[%s]
to value 0"]
impl crate::Resettable for CM0_SYSTEM_INT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
