#[doc = "Register `CAL_CTL5` reader"]
pub struct R(crate::R<CAL_CTL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_CTL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_CTL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_CTL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_CTL5` writer"]
pub struct W(crate::W<CAL_CTL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_CTL5_SPEC>;
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
impl From<crate::W<CAL_CTL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_CTL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLIM_TRIM_LP_HV` reader - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
pub type VLIM_TRIM_LP_HV_R = crate::FieldReader;
#[doc = "Field `VLIM_TRIM_LP_HV` writer - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
pub type VLIM_TRIM_LP_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL5_SPEC, 2, O>;
#[doc = "Field `IDAC_LP_HV` reader - Sets the sense current reference offset value. Refer to trim tables for details."]
pub type IDAC_LP_HV_R = crate::FieldReader;
#[doc = "Field `IDAC_LP_HV` writer - Sets the sense current reference offset value. Refer to trim tables for details."]
pub type IDAC_LP_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL5_SPEC, 4, O>;
#[doc = "Field `SDAC_LP_HV` reader - Sets the sense current reference temp slope. Refer to trim tables for details."]
pub type SDAC_LP_HV_R = crate::FieldReader;
#[doc = "Field `SDAC_LP_HV` writer - Sets the sense current reference temp slope. Refer to trim tables for details."]
pub type SDAC_LP_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL5_SPEC, 2, O>;
#[doc = "Field `ITIM_LP_HV` reader - Trimming of timing current"]
pub type ITIM_LP_HV_R = crate::FieldReader;
#[doc = "Field `ITIM_LP_HV` writer - Trimming of timing current"]
pub type ITIM_LP_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL5_SPEC, 5, O>;
#[doc = "Field `FM_READY_DEL_LP_HV` reader - 00: Delayed by 1us 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
pub type FM_READY_DEL_LP_HV_R = crate::FieldReader;
#[doc = "Field `FM_READY_DEL_LP_HV` writer - 00: Delayed by 1us 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
pub type FM_READY_DEL_LP_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL5_SPEC, 2, O>;
#[doc = "Field `SPARE451_LP_HV` reader - N/A"]
pub type SPARE451_LP_HV_R = crate::BitReader;
#[doc = "Field `SPARE451_LP_HV` writer - N/A"]
pub type SPARE451_LP_HV_W<'a, const O: u8> = crate::BitWriter<'a, CAL_CTL5_SPEC, O>;
#[doc = "Field `SPARE52_HV` reader - N/A"]
pub type SPARE52_HV_R = crate::FieldReader;
#[doc = "Field `SPARE52_HV` writer - N/A"]
pub type SPARE52_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL5_SPEC, 2, O>;
#[doc = "Field `AMUX_SEL_HV` reader - Amux Select in AMUX_UGB 00: Bypass UGB for both amuxbusa and amuxbusb 01: Bypass UGB for amuxbusb while passing amuxbusa through UGB. 10: Bypass UGB for amuxbusa while passing amuxbusb through UGB. 11: UGB Calibrate mode"]
pub type AMUX_SEL_HV_R = crate::FieldReader;
#[doc = "Field `AMUX_SEL_HV` writer - Amux Select in AMUX_UGB 00: Bypass UGB for both amuxbusa and amuxbusb 01: Bypass UGB for amuxbusb while passing amuxbusa through UGB. 10: Bypass UGB for amuxbusa while passing amuxbusb through UGB. 11: UGB Calibrate mode"]
pub type AMUX_SEL_HV_W<'a, const O: u8> = crate::FieldWriter<'a, CAL_CTL5_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_lp_hv(&self) -> VLIM_TRIM_LP_HV_R {
        VLIM_TRIM_LP_HV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    pub fn idac_lp_hv(&self) -> IDAC_LP_HV_R {
        IDAC_LP_HV_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    pub fn sdac_lp_hv(&self) -> SDAC_LP_HV_R {
        SDAC_LP_HV_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    pub fn itim_lp_hv(&self) -> ITIM_LP_HV_R {
        ITIM_LP_HV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - 00: Delayed by 1us 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    pub fn fm_ready_del_lp_hv(&self) -> FM_READY_DEL_LP_HV_R {
        FM_READY_DEL_LP_HV_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn spare451_lp_hv(&self) -> SPARE451_LP_HV_R {
        SPARE451_LP_HV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn spare52_hv(&self) -> SPARE52_HV_R {
        SPARE52_HV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Amux Select in AMUX_UGB 00: Bypass UGB for both amuxbusa and amuxbusb 01: Bypass UGB for amuxbusb while passing amuxbusa through UGB. 10: Bypass UGB for amuxbusa while passing amuxbusb through UGB. 11: UGB Calibrate mode"]
    #[inline(always)]
    pub fn amux_sel_hv(&self) -> AMUX_SEL_HV_R {
        AMUX_SEL_HV_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VLIM_TRIM\\[1:0\\]: 00: V2 = 650mV 01: V2 = 600mV 10: V2 = 750mV 11: V2 = 700mV"]
    #[inline(always)]
    #[must_use]
    pub fn vlim_trim_lp_hv(&mut self) -> VLIM_TRIM_LP_HV_W<0> {
        VLIM_TRIM_LP_HV_W::new(self)
    }
    #[doc = "Bits 2:5 - Sets the sense current reference offset value. Refer to trim tables for details."]
    #[inline(always)]
    #[must_use]
    pub fn idac_lp_hv(&mut self) -> IDAC_LP_HV_W<2> {
        IDAC_LP_HV_W::new(self)
    }
    #[doc = "Bits 6:7 - Sets the sense current reference temp slope. Refer to trim tables for details."]
    #[inline(always)]
    #[must_use]
    pub fn sdac_lp_hv(&mut self) -> SDAC_LP_HV_W<6> {
        SDAC_LP_HV_W::new(self)
    }
    #[doc = "Bits 8:12 - Trimming of timing current"]
    #[inline(always)]
    #[must_use]
    pub fn itim_lp_hv(&mut self) -> ITIM_LP_HV_W<8> {
        ITIM_LP_HV_W::new(self)
    }
    #[doc = "Bits 13:14 - 00: Delayed by 1us 01: Delayed by 1.5us 10: Delayed by 2.0us 11: Delayed by 2.5us"]
    #[inline(always)]
    #[must_use]
    pub fn fm_ready_del_lp_hv(&mut self) -> FM_READY_DEL_LP_HV_W<13> {
        FM_READY_DEL_LP_HV_W::new(self)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn spare451_lp_hv(&mut self) -> SPARE451_LP_HV_W<15> {
        SPARE451_LP_HV_W::new(self)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn spare52_hv(&mut self) -> SPARE52_HV_W<16> {
        SPARE52_HV_W::new(self)
    }
    #[doc = "Bits 18:19 - Amux Select in AMUX_UGB 00: Bypass UGB for both amuxbusa and amuxbusb 01: Bypass UGB for amuxbusb while passing amuxbusa through UGB. 10: Bypass UGB for amuxbusa while passing amuxbusb through UGB. 11: UGB Calibrate mode"]
    #[inline(always)]
    #[must_use]
    pub fn amux_sel_hv(&mut self) -> AMUX_SEL_HV_W<18> {
        AMUX_SEL_HV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cal control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl5](index.html) module"]
pub struct CAL_CTL5_SPEC;
impl crate::RegisterSpec for CAL_CTL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_ctl5::R](R) reader structure"]
impl crate::Readable for CAL_CTL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_ctl5::W](W) writer structure"]
impl crate::Writable for CAL_CTL5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL5 to value 0x2ae0"]
impl crate::Resettable for CAL_CTL5_SPEC {
    const RESET_VALUE: Self::Ux = 0x2ae0;
}
