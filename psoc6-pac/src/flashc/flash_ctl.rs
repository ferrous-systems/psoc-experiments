#[doc = "Register `FLASH_CTL` reader"]
pub struct R(crate::R<FLASH_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_CTL` writer"]
pub struct W(crate::W<FLASH_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_CTL_SPEC>;
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
impl From<crate::W<FLASH_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAIN_WS` reader - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
pub type MAIN_WS_R = crate::FieldReader;
#[doc = "Field `MAIN_WS` writer - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
pub type MAIN_WS_W<'a, const O: u8> = crate::FieldWriter<'a, FLASH_CTL_SPEC, 4, O>;
#[doc = "Field `MAIN_MAP` reader - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
pub type MAIN_MAP_R = crate::BitReader;
#[doc = "Field `MAIN_MAP` writer - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
pub type MAIN_MAP_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
#[doc = "Field `WORK_MAP` reader - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
pub type WORK_MAP_R = crate::BitReader;
#[doc = "Field `WORK_MAP` writer - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
pub type WORK_MAP_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
#[doc = "Field `MAIN_BANK_MODE` reader - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
pub type MAIN_BANK_MODE_R = crate::BitReader;
#[doc = "Field `MAIN_BANK_MODE` writer - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
pub type MAIN_BANK_MODE_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
#[doc = "Field `WORK_BANK_MODE` reader - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
pub type WORK_BANK_MODE_R = crate::BitReader;
#[doc = "Field `WORK_BANK_MODE` writer - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
pub type WORK_BANK_MODE_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
#[doc = "Field `MAIN_ECC_EN` reader - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
pub type MAIN_ECC_EN_R = crate::BitReader;
#[doc = "Field `MAIN_ECC_EN` writer - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
pub type MAIN_ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
#[doc = "Field `MAIN_ECC_INJ_EN` reader - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type MAIN_ECC_INJ_EN_R = crate::BitReader;
#[doc = "Field `MAIN_ECC_INJ_EN` writer - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type MAIN_ECC_INJ_EN_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
#[doc = "Field `MAIN_ERR_SILENT` reader - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
pub type MAIN_ERR_SILENT_R = crate::BitReader;
#[doc = "Field `MAIN_ERR_SILENT` writer - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
pub type MAIN_ERR_SILENT_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
#[doc = "Field `WORK_ECC_EN` reader - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
pub type WORK_ECC_EN_R = crate::BitReader;
#[doc = "Field `WORK_ECC_EN` writer - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
pub type WORK_ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
#[doc = "Field `WORK_ECC_INJ_EN` reader - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type WORK_ECC_INJ_EN_R = crate::BitReader;
#[doc = "Field `WORK_ECC_INJ_EN` writer - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type WORK_ECC_INJ_EN_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
#[doc = "Field `WORK_ERR_SILENT` reader - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
pub type WORK_ERR_SILENT_R = crate::BitReader;
#[doc = "Field `WORK_ERR_SILENT` writer - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
pub type WORK_ERR_SILENT_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_CTL_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    pub fn main_ws(&self) -> MAIN_WS_R {
        MAIN_WS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn main_map(&self) -> MAIN_MAP_R {
        MAIN_MAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    pub fn work_map(&self) -> WORK_MAP_R {
        WORK_MAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn main_bank_mode(&self) -> MAIN_BANK_MODE_R {
        MAIN_BANK_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    pub fn work_bank_mode(&self) -> WORK_BANK_MODE_R {
        WORK_BANK_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn main_ecc_en(&self) -> MAIN_ECC_EN_R {
        MAIN_ECC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn main_ecc_inj_en(&self) -> MAIN_ECC_INJ_EN_R {
        MAIN_ECC_INJ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
    #[inline(always)]
    pub fn main_err_silent(&self) -> MAIN_ERR_SILENT_R {
        MAIN_ERR_SILENT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    pub fn work_ecc_en(&self) -> WORK_ECC_EN_R {
        WORK_ECC_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn work_ecc_inj_en(&self) -> WORK_ECC_INJ_EN_R {
        WORK_ECC_INJ_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
    #[inline(always)]
    pub fn work_err_silent(&self) -> WORK_ERR_SILENT_R {
        WORK_ERR_SILENT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    #[must_use]
    pub fn main_ws(&mut self) -> MAIN_WS_W<0> {
        MAIN_WS_W::new(self)
    }
    #[doc = "Bit 8 - Specifies mapping of FLASH macro main array. 0: Mapping A. 1: Mapping B. This field is only used when MAIN_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    #[must_use]
    pub fn main_map(&mut self) -> MAIN_MAP_W<8> {
        MAIN_MAP_W::new(self)
    }
    #[doc = "Bit 9 - Specifies mapping of FLASH macro work array. 0: Mapping A. 1: Mapping B. This field is only used when WORK_BANK_MODE is '1' (dual bank mode)."]
    #[inline(always)]
    #[must_use]
    pub fn work_map(&mut self) -> WORK_MAP_W<9> {
        WORK_MAP_W::new(self)
    }
    #[doc = "Bit 12 - Specifies bank mode of FLASH macro main array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    #[must_use]
    pub fn main_bank_mode(&mut self) -> MAIN_BANK_MODE_W<12> {
        MAIN_BANK_MODE_W::new(self)
    }
    #[doc = "Bit 13 - Specifies bank mode of FLASH macro work array. 0: Single bank mode. 1: Dual bank mode."]
    #[inline(always)]
    #[must_use]
    pub fn work_bank_mode(&mut self) -> WORK_BANK_MODE_W<13> {
        WORK_BANK_MODE_W::new(self)
    }
    #[doc = "Bit 16 - Enable ECC checking for FLASH main interface: 0: Disabled. ECC checking/reporting on FLASH main interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn main_ecc_en(&mut self) -> MAIN_ECC_EN_W<16> {
        MAIN_ECC_EN_W::new(self)
    }
    #[doc = "Bit 17 - Enable error injection for FLASH main interface. When'1', the parity (ECC_CTL.PARITY\\[7:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    #[must_use]
    pub fn main_ecc_inj_en(&mut self) -> MAIN_ECC_INJ_EN_W<17> {
        MAIN_ECC_INJ_EN_W::new(self)
    }
    #[doc = "Bit 18 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro main interface (either a non-correctable ECC error, a FLASH macro main interface internal error, a FLASH macro main interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro main interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro main interface internal error. - FLASH macro main interface non-recoverable ECC error. - FLASH macro main interface recoverable ECC error. - FLASH macro main interface memory hole error."]
    #[inline(always)]
    #[must_use]
    pub fn main_err_silent(&mut self) -> MAIN_ERR_SILENT_W<18> {
        MAIN_ERR_SILENT_W::new(self)
    }
    #[doc = "Bit 20 - Enable ECC checking for FLASH work interface: 0: Disabled. ECC checking/reporting on FLASH work interface is disabled. No correctable or non-correctable faults are reported. 1: Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn work_ecc_en(&mut self) -> WORK_ECC_EN_W<20> {
        WORK_ECC_EN_W::new(self)
    }
    #[doc = "Bit 21 - Enable error injection for FLASH work interface. When'1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used for a load from the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    #[must_use]
    pub fn work_ecc_inj_en(&mut self) -> WORK_ECC_INJ_EN_W<21> {
        WORK_ECC_INJ_EN_W::new(self)
    }
    #[doc = "Bit 22 - Specifies bus transfer behavior for a non-recoverable error on the FLASH macro work interface (either a non-correctable ECC error, a FLASH macro work interface internal error, a FLASH macro work interface memory hole access): 0: Bus transfer has a bus error. 1: Bus transfer does NOT have a bus error; i.e. the error is 'silent' In either case, the erroneous FLASH macro data is returned by the bus master interface. The erroneous data is NOT placed in a bus master interface's cache and/or buffer. This field is ONLY used by CPU (and debug i.e. SYS_AP/CM0_AP/CM4_AP) bus transfers. Non-CPU bus transfers always have a bus transfer with a bus error, in case of a non-recoverable error. Note: All CPU bus masters have dedicated status registers (CM0_STATUS and CM4_STATUS) to register the occurrence of FLASH macro work interface internal errors (non-correctable ECC errors and memory hole errors are NOT registered). Note: fault reporting can be used to identify the error that occurred: - FLASH macro work interface internal error. - FLASH macro work interface non-recoverable ECC error. - FLASH macro work interface recoverable ECC error. - FLASH macro work interface memory hole error."]
    #[inline(always)]
    #[must_use]
    pub fn work_err_silent(&mut self) -> WORK_ERR_SILENT_W<22> {
        WORK_ERR_SILENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ctl](index.html) module"]
pub struct FLASH_CTL_SPEC;
impl crate::RegisterSpec for FLASH_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ctl::R](R) reader structure"]
impl crate::Readable for FLASH_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ctl::W](W) writer structure"]
impl crate::Writable for FLASH_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_CTL to value 0x0011_0000"]
impl crate::Resettable for FLASH_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_0000;
}
