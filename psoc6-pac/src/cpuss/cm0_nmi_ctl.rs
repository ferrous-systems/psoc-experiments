#[doc = "Register `CM0_NMI_CTL[%s]` reader"]
pub struct R(crate::R<CM0_NMI_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_NMI_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_NMI_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_NMI_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0_NMI_CTL[%s]` writer"]
pub struct W(crate::W<CM0_NMI_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0_NMI_CTL_SPEC>;
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
impl From<crate::W<CM0_NMI_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0_NMI_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTEM_INT_IDX` reader - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
pub type SYSTEM_INT_IDX_R = crate::FieldReader<u16>;
#[doc = "Field `SYSTEM_INT_IDX` writer - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
pub type SYSTEM_INT_IDX_W<'a, const O: u8> = crate::FieldWriter<'a, CM0_NMI_CTL_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SYSTEM_INT_IDX_R {
        SYSTEM_INT_IDX_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    #[must_use]
    pub fn system_int_idx(&mut self) -> SYSTEM_INT_IDX_W<0> {
        SYSTEM_INT_IDX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM0+ NMI control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_nmi_ctl](index.html) module"]
pub struct CM0_NMI_CTL_SPEC;
impl crate::RegisterSpec for CM0_NMI_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_nmi_ctl::R](R) reader structure"]
impl crate::Readable for CM0_NMI_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0_nmi_ctl::W](W) writer structure"]
impl crate::Writable for CM0_NMI_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM0_NMI_CTL[%s]
to value 0x03ff"]
impl crate::Resettable for CM0_NMI_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
