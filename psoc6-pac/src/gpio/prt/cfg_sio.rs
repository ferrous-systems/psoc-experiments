#[doc = "Register `CFG_SIO` reader"]
pub struct R(crate::R<CFG_SIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_SIO` writer"]
pub struct W(crate::W<CFG_SIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SIO_SPEC>;
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
impl From<crate::W<CFG_SIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREG_EN01` reader - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub type VREG_EN01_R = crate::BitReader;
#[doc = "Field `VREG_EN01` writer - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub type VREG_EN01_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `IBUF_SEL01` reader - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
pub type IBUF_SEL01_R = crate::BitReader;
#[doc = "Field `IBUF_SEL01` writer - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
pub type IBUF_SEL01_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `VTRIP_SEL01` reader - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
pub type VTRIP_SEL01_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL01` writer - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
pub type VTRIP_SEL01_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `VREF_SEL01` reader - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
pub type VREF_SEL01_R = crate::FieldReader;
#[doc = "Field `VREF_SEL01` writer - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
pub type VREF_SEL01_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SIO_SPEC, 2, O>;
#[doc = "Field `VOH_SEL01` reader - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
pub type VOH_SEL01_R = crate::FieldReader;
#[doc = "Field `VOH_SEL01` writer - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
pub type VOH_SEL01_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SIO_SPEC, 3, O>;
#[doc = "Field `VREG_EN23` reader - See corresponding definition for IO pins 0 and 1"]
pub type VREG_EN23_R = crate::BitReader;
#[doc = "Field `VREG_EN23` writer - See corresponding definition for IO pins 0 and 1"]
pub type VREG_EN23_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `IBUF_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub type IBUF_SEL23_R = crate::BitReader;
#[doc = "Field `IBUF_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub type IBUF_SEL23_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `VTRIP_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub type VTRIP_SEL23_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub type VTRIP_SEL23_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `VREF_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub type VREF_SEL23_R = crate::FieldReader;
#[doc = "Field `VREF_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub type VREF_SEL23_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SIO_SPEC, 2, O>;
#[doc = "Field `VOH_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub type VOH_SEL23_R = crate::FieldReader;
#[doc = "Field `VOH_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub type VOH_SEL23_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SIO_SPEC, 3, O>;
#[doc = "Field `VREG_EN45` reader - See corresponding definition for IO pins 0 and 1"]
pub type VREG_EN45_R = crate::BitReader;
#[doc = "Field `VREG_EN45` writer - See corresponding definition for IO pins 0 and 1"]
pub type VREG_EN45_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `IBUF_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub type IBUF_SEL45_R = crate::BitReader;
#[doc = "Field `IBUF_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub type IBUF_SEL45_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `VTRIP_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub type VTRIP_SEL45_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub type VTRIP_SEL45_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `VREF_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub type VREF_SEL45_R = crate::FieldReader;
#[doc = "Field `VREF_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub type VREF_SEL45_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SIO_SPEC, 2, O>;
#[doc = "Field `VOH_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub type VOH_SEL45_R = crate::FieldReader;
#[doc = "Field `VOH_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub type VOH_SEL45_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SIO_SPEC, 3, O>;
#[doc = "Field `VREG_EN67` reader - See corresponding definition for IO pins 0 and 1"]
pub type VREG_EN67_R = crate::BitReader;
#[doc = "Field `VREG_EN67` writer - See corresponding definition for IO pins 0 and 1"]
pub type VREG_EN67_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `IBUF_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub type IBUF_SEL67_R = crate::BitReader;
#[doc = "Field `IBUF_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub type IBUF_SEL67_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `VTRIP_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub type VTRIP_SEL67_R = crate::BitReader;
#[doc = "Field `VTRIP_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub type VTRIP_SEL67_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SIO_SPEC, O>;
#[doc = "Field `VREF_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub type VREF_SEL67_R = crate::FieldReader;
#[doc = "Field `VREF_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub type VREF_SEL67_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SIO_SPEC, 2, O>;
#[doc = "Field `VOH_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub type VOH_SEL67_R = crate::FieldReader;
#[doc = "Field `VOH_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub type VOH_SEL67_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SIO_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn vreg_en01(&self) -> VREG_EN01_R {
        VREG_EN01_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
    #[inline(always)]
    pub fn ibuf_sel01(&self) -> IBUF_SEL01_R {
        IBUF_SEL01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
    #[inline(always)]
    pub fn vtrip_sel01(&self) -> VTRIP_SEL01_R {
        VTRIP_SEL01_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
    #[inline(always)]
    pub fn vref_sel01(&self) -> VREF_SEL01_R {
        VREF_SEL01_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
    #[inline(always)]
    pub fn voh_sel01(&self) -> VOH_SEL01_R {
        VOH_SEL01_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en23(&self) -> VREG_EN23_R {
        VREG_EN23_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel23(&self) -> IBUF_SEL23_R {
        IBUF_SEL23_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel23(&self) -> VTRIP_SEL23_R {
        VTRIP_SEL23_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel23(&self) -> VREF_SEL23_R {
        VREF_SEL23_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel23(&self) -> VOH_SEL23_R {
        VOH_SEL23_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en45(&self) -> VREG_EN45_R {
        VREG_EN45_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel45(&self) -> IBUF_SEL45_R {
        IBUF_SEL45_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel45(&self) -> VTRIP_SEL45_R {
        VTRIP_SEL45_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel45(&self) -> VREF_SEL45_R {
        VREF_SEL45_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel45(&self) -> VOH_SEL45_R {
        VOH_SEL45_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en67(&self) -> VREG_EN67_R {
        VREG_EN67_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel67(&self) -> IBUF_SEL67_R {
        IBUF_SEL67_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel67(&self) -> VTRIP_SEL67_R {
        VTRIP_SEL67_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel67(&self) -> VREF_SEL67_R {
        VREF_SEL67_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:31 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel67(&self) -> VOH_SEL67_R {
        VOH_SEL67_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en01(&mut self) -> VREG_EN01_W<0> {
        VREG_EN01_W::new(self)
    }
    #[doc = "Bit 1 - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel01(&mut self) -> IBUF_SEL01_W<1> {
        IBUF_SEL01_W::new(self)
    }
    #[doc = "Bit 2 - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel01(&mut self) -> VTRIP_SEL01_W<2> {
        VTRIP_SEL01_W::new(self)
    }
    #[doc = "Bits 3:4 - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel01(&mut self) -> VREF_SEL01_W<3> {
        VREF_SEL01_W::new(self)
    }
    #[doc = "Bits 5:7 - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel01(&mut self) -> VOH_SEL01_W<5> {
        VOH_SEL01_W::new(self)
    }
    #[doc = "Bit 8 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en23(&mut self) -> VREG_EN23_W<8> {
        VREG_EN23_W::new(self)
    }
    #[doc = "Bit 9 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel23(&mut self) -> IBUF_SEL23_W<9> {
        IBUF_SEL23_W::new(self)
    }
    #[doc = "Bit 10 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel23(&mut self) -> VTRIP_SEL23_W<10> {
        VTRIP_SEL23_W::new(self)
    }
    #[doc = "Bits 11:12 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel23(&mut self) -> VREF_SEL23_W<11> {
        VREF_SEL23_W::new(self)
    }
    #[doc = "Bits 13:15 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel23(&mut self) -> VOH_SEL23_W<13> {
        VOH_SEL23_W::new(self)
    }
    #[doc = "Bit 16 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en45(&mut self) -> VREG_EN45_W<16> {
        VREG_EN45_W::new(self)
    }
    #[doc = "Bit 17 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel45(&mut self) -> IBUF_SEL45_W<17> {
        IBUF_SEL45_W::new(self)
    }
    #[doc = "Bit 18 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel45(&mut self) -> VTRIP_SEL45_W<18> {
        VTRIP_SEL45_W::new(self)
    }
    #[doc = "Bits 19:20 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel45(&mut self) -> VREF_SEL45_W<19> {
        VREF_SEL45_W::new(self)
    }
    #[doc = "Bits 21:23 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel45(&mut self) -> VOH_SEL45_W<21> {
        VOH_SEL45_W::new(self)
    }
    #[doc = "Bit 24 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en67(&mut self) -> VREG_EN67_W<24> {
        VREG_EN67_W::new(self)
    }
    #[doc = "Bit 25 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel67(&mut self) -> IBUF_SEL67_W<25> {
        IBUF_SEL67_W::new(self)
    }
    #[doc = "Bit 26 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel67(&mut self) -> VTRIP_SEL67_W<26> {
        VTRIP_SEL67_W::new(self)
    }
    #[doc = "Bits 27:28 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel67(&mut self) -> VREF_SEL67_W<27> {
        VREF_SEL67_W::new(self)
    }
    #[doc = "Bits 29:31 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel67(&mut self) -> VOH_SEL67_W<29> {
        VOH_SEL67_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port SIO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_sio](index.html) module"]
pub struct CFG_SIO_SPEC;
impl crate::RegisterSpec for CFG_SIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_sio::R](R) reader structure"]
impl crate::Readable for CFG_SIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_sio::W](W) writer structure"]
impl crate::Writable for CFG_SIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_SIO to value 0"]
impl crate::Resettable for CFG_SIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
