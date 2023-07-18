#[doc = "Register `CAL_CTL2` reader"]
pub struct R(crate::R<CAL_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_CTL2` writer"]
pub struct W(crate::W<CAL_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_CTL2_SPEC>;
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
impl From<crate::W<CAL_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICREF_TRIM_LO_HV` reader - LO Bandgap Current trim control."]
pub type ICREF_TRIM_LO_HV_R = crate::FieldReader;
#[doc = "Field `ICREF_TRIM_LO_HV` writer - LO Bandgap Current trim control."]
pub type ICREF_TRIM_LO_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL2_SPEC, 5, O>;
#[doc = "Field `ICREF_TRIM_HI_HV` reader - HI Bandgap Current trim control."]
pub type ICREF_TRIM_HI_HV_R = crate::FieldReader;
#[doc = "Field `ICREF_TRIM_HI_HV` writer - HI Bandgap Current trim control."]
pub type ICREF_TRIM_HI_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL2_SPEC, 5, O>;
#[doc = "Field `IPREF_TRIM_LO_HV` reader - LO Bandgap IPTAT trim control."]
pub type IPREF_TRIM_LO_HV_R = crate::FieldReader;
#[doc = "Field `IPREF_TRIM_LO_HV` writer - LO Bandgap IPTAT trim control."]
pub type IPREF_TRIM_LO_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL2_SPEC, 5, O>;
#[doc = "Field `IPREF_TRIM_HI_HV` reader - HI Bandgap IPTAT trim control."]
pub type IPREF_TRIM_HI_HV_R = crate::FieldReader;
#[doc = "Field `IPREF_TRIM_HI_HV` writer - HI Bandgap IPTAT trim control."]
pub type IPREF_TRIM_HI_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL2_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_lo_hv(&self) -> ICREF_TRIM_LO_HV_R {
        ICREF_TRIM_LO_HV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - HI Bandgap Current trim control."]
    #[inline(always)]
    pub fn icref_trim_hi_hv(&self) -> ICREF_TRIM_HI_HV_R {
        ICREF_TRIM_HI_HV_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - LO Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_lo_hv(&self) -> IPREF_TRIM_LO_HV_R {
        IPREF_TRIM_LO_HV_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    pub fn ipref_trim_hi_hv(&self) -> IPREF_TRIM_HI_HV_R {
        IPREF_TRIM_HI_HV_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - LO Bandgap Current trim control."]
    #[inline(always)]
    #[must_use]
    pub fn icref_trim_lo_hv(&mut self) -> ICREF_TRIM_LO_HV_W<0> {
        ICREF_TRIM_LO_HV_W::new(self)
    }
    #[doc = "Bits 5:9 - HI Bandgap Current trim control."]
    #[inline(always)]
    #[must_use]
    pub fn icref_trim_hi_hv(&mut self) -> ICREF_TRIM_HI_HV_W<5> {
        ICREF_TRIM_HI_HV_W::new(self)
    }
    #[doc = "Bits 10:14 - LO Bandgap IPTAT trim control."]
    #[inline(always)]
    #[must_use]
    pub fn ipref_trim_lo_hv(&mut self) -> IPREF_TRIM_LO_HV_W<10> {
        IPREF_TRIM_LO_HV_W::new(self)
    }
    #[doc = "Bits 15:19 - HI Bandgap IPTAT trim control."]
    #[inline(always)]
    #[must_use]
    pub fn ipref_trim_hi_hv(&mut self) -> IPREF_TRIM_HI_HV_W<15> {
        IPREF_TRIM_HI_HV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cal control BG LO&amp;HI trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl2](index.html) module"]
pub struct CAL_CTL2_SPEC;
impl crate::RegisterSpec for CAL_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_ctl2::R](R) reader structure"]
impl crate::Readable for CAL_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_ctl2::W](W) writer structure"]
impl crate::Writable for CAL_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL2 to value 0x0007_be10"]
impl crate::Resettable for CAL_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_be10;
}
