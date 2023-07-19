#[doc = "Register `PWR_CTL` reader"]
pub struct R(crate::R<PWR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTL` writer"]
pub struct W(crate::W<PWR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTL_SPEC>;
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
impl From<crate::W<PWR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER_MODE` reader - Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
pub type POWER_MODE_R = crate::FieldReader<POWER_MODE_A>;
#[doc = "Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POWER_MODE_A {
    #[doc = "0: System is resetting."]
    RESET = 0,
    #[doc = "1: At least one CPU is running."]
    ACTIVE = 1,
    #[doc = "2: No CPUs are running. Peripherals may be running."]
    SLEEP = 2,
    #[doc = "3: Main high-frequency clock is off; low speed clocks are available. Communication interface clocks may be present."]
    DEEPSLEEP = 3,
}
impl From<POWER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: POWER_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POWER_MODE_A {
    type Ux = u8;
}
impl POWER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_MODE_A {
        match self.bits {
            0 => POWER_MODE_A::RESET,
            1 => POWER_MODE_A::ACTIVE,
            2 => POWER_MODE_A::SLEEP,
            3 => POWER_MODE_A::DEEPSLEEP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == POWER_MODE_A::RESET
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == POWER_MODE_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == POWER_MODE_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == POWER_MODE_A::DEEPSLEEP
    }
}
#[doc = "Field `DEBUG_SESSION` reader - Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
pub type DEBUG_SESSION_R = crate::BitReader<DEBUG_SESSION_A>;
#[doc = "Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBUG_SESSION_A {
    #[doc = "0: No debug session active"]
    NO_SESSION = 0,
    #[doc = "1: Debug session is active. Power modes behave differently to keep the debug session active."]
    SESSION_ACTIVE = 1,
}
impl From<DEBUG_SESSION_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_SESSION_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBUG_SESSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUG_SESSION_A {
        match self.bits {
            false => DEBUG_SESSION_A::NO_SESSION,
            true => DEBUG_SESSION_A::SESSION_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SESSION`"]
    #[inline(always)]
    pub fn is_no_session(&self) -> bool {
        *self == DEBUG_SESSION_A::NO_SESSION
    }
    #[doc = "Checks if the value of the field is `SESSION_ACTIVE`"]
    #[inline(always)]
    pub fn is_session_active(&self) -> bool {
        *self == DEBUG_SESSION_A::SESSION_ACTIVE
    }
}
#[doc = "Field `LPM_READY` reader - Indicates whether certain low power functions are ready. The low current circuits take longer to startup after XRES/POR/BOD/HIBERNATE wakeup than the normal mode circuits. HIBERNATE mode may be entered regardless of this bit. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: If a low power circuit operation is requested, it will stay in its normal operating mode until it is ready. If DEEPSLEEP is requested by all processors WFI/WFE, the device will instead enter SLEEP. When low power circuits are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP and low power circuits operate as requested in other registers."]
pub type LPM_READY_R = crate::BitReader;
#[doc = "Field `IREF_LPMODE` reader - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
pub type IREF_LPMODE_R = crate::BitReader;
#[doc = "Field `IREF_LPMODE` writer - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
pub type IREF_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `VREFBUF_OK` reader - Indicates that the voltage reference buffer is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting VREFBUF_DIS=1."]
pub type VREFBUF_OK_R = crate::BitReader;
#[doc = "Field `DPSLP_REG_DIS` reader - Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
pub type DPSLP_REG_DIS_R = crate::BitReader;
#[doc = "Field `DPSLP_REG_DIS` writer - Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
pub type DPSLP_REG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `RET_REG_DIS` reader - Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
pub type RET_REG_DIS_R = crate::BitReader;
#[doc = "Field `RET_REG_DIS` writer - Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
pub type RET_REG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `NWELL_REG_DIS` reader - Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
pub type NWELL_REG_DIS_R = crate::BitReader;
#[doc = "Field `NWELL_REG_DIS` writer - Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
pub type NWELL_REG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `LINREG_DIS` reader - Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
pub type LINREG_DIS_R = crate::BitReader;
#[doc = "Field `LINREG_DIS` writer - Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
pub type LINREG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `LINREG_LPMODE` reader - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
pub type LINREG_LPMODE_R = crate::BitReader;
#[doc = "Field `LINREG_LPMODE` writer - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
pub type LINREG_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `PORBOD_LPMODE` reader - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type PORBOD_LPMODE_R = crate::BitReader;
#[doc = "Field `PORBOD_LPMODE` writer - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type PORBOD_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `BGREF_LPMODE` reader - Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
pub type BGREF_LPMODE_R = crate::BitReader;
#[doc = "Field `BGREF_LPMODE` writer - Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
pub type BGREF_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `PLL_LS_BYPASS` reader - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
pub type PLL_LS_BYPASS_R = crate::BitReader;
#[doc = "Field `PLL_LS_BYPASS` writer - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
pub type PLL_LS_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `VREFBUF_LPMODE` reader - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type VREFBUF_LPMODE_R = crate::BitReader;
#[doc = "Field `VREFBUF_LPMODE` writer - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
pub type VREFBUF_LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `VREFBUF_DIS` reader - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type VREFBUF_DIS_R = crate::BitReader;
#[doc = "Field `VREFBUF_DIS` writer - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type VREFBUF_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `ACT_REF_DIS` reader - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
pub type ACT_REF_DIS_R = crate::BitReader;
#[doc = "Field `ACT_REF_DIS` writer - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
pub type ACT_REF_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTL_SPEC, O>;
#[doc = "Field `ACT_REF_OK` reader - Indicates that the normal mode of the Active Reference is ready."]
pub type ACT_REF_OK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
    #[inline(always)]
    pub fn power_mode(&self) -> POWER_MODE_R {
        POWER_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
    #[inline(always)]
    pub fn debug_session(&self) -> DEBUG_SESSION_R {
        DEBUG_SESSION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates whether certain low power functions are ready. The low current circuits take longer to startup after XRES/POR/BOD/HIBERNATE wakeup than the normal mode circuits. HIBERNATE mode may be entered regardless of this bit. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: If a low power circuit operation is requested, it will stay in its normal operating mode until it is ready. If DEEPSLEEP is requested by all processors WFI/WFE, the device will instead enter SLEEP. When low power circuits are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP and low power circuits operate as requested in other registers."]
    #[inline(always)]
    pub fn lpm_ready(&self) -> LPM_READY_R {
        LPM_READY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 18 - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn iref_lpmode(&self) -> IREF_LPMODE_R {
        IREF_LPMODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates that the voltage reference buffer is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting VREFBUF_DIS=1."]
    #[inline(always)]
    pub fn vrefbuf_ok(&self) -> VREFBUF_OK_R {
        VREFBUF_OK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
    #[inline(always)]
    pub fn dpslp_reg_dis(&self) -> DPSLP_REG_DIS_R {
        DPSLP_REG_DIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    pub fn ret_reg_dis(&self) -> RET_REG_DIS_R {
        RET_REG_DIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    pub fn nwell_reg_dis(&self) -> NWELL_REG_DIS_R {
        NWELL_REG_DIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
    #[inline(always)]
    pub fn linreg_dis(&self) -> LINREG_DIS_R {
        LINREG_DIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    pub fn linreg_lpmode(&self) -> LINREG_LPMODE_R {
        LINREG_LPMODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn porbod_lpmode(&self) -> PORBOD_LPMODE_R {
        PORBOD_LPMODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
    #[inline(always)]
    pub fn bgref_lpmode(&self) -> BGREF_LPMODE_R {
        BGREF_LPMODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    pub fn pll_ls_bypass(&self) -> PLL_LS_BYPASS_R {
        PLL_LS_BYPASS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn vrefbuf_lpmode(&self) -> VREFBUF_LPMODE_R {
        VREFBUF_LPMODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn vrefbuf_dis(&self) -> VREFBUF_DIS_R {
        VREFBUF_DIS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
    #[inline(always)]
    pub fn act_ref_dis(&self) -> ACT_REF_DIS_R {
        ACT_REF_DIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates that the normal mode of the Active Reference is ready."]
    #[inline(always)]
    pub fn act_ref_ok(&self) -> ACT_REF_OK_R {
        ACT_REF_OK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    #[must_use]
    pub fn iref_lpmode(&mut self) -> IREF_LPMODE_W<18> {
        IREF_LPMODE_W::new(self)
    }
    #[doc = "Bit 20 - Disable the DeepSleep regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn dpslp_reg_dis(&mut self) -> DPSLP_REG_DIS_W<20> {
        DPSLP_REG_DIS_W::new(self)
    }
    #[doc = "Bit 21 - Disable the Retention regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn ret_reg_dis(&mut self) -> RET_REG_DIS_W<21> {
        RET_REG_DIS_W::new(self)
    }
    #[doc = "Bit 22 - Disable the Nwell regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn nwell_reg_dis(&mut self) -> NWELL_REG_DIS_W<22> {
        NWELL_REG_DIS_W::new(self)
    }
    #[doc = "Bit 23 - Disable the linear Core Regulator. This is only legal when the on-chip buck regulator supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
    #[inline(always)]
    #[must_use]
    pub fn linreg_dis(&mut self) -> LINREG_DIS_W<23> {
        LINREG_DIS_W::new(self)
    }
    #[doc = "Bit 24 - Control the power mode of the Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    #[must_use]
    pub fn linreg_lpmode(&mut self) -> LINREG_LPMODE_W<24> {
        LINREG_LPMODE_W::new(self)
    }
    #[doc = "Bit 25 - Control the power mode of the POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    #[must_use]
    pub fn porbod_lpmode(&mut self) -> PORBOD_LPMODE_W<25> {
        PORBOD_LPMODE_W::new(self)
    }
    #[doc = "Bit 26 - Control the power mode of the Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
    #[inline(always)]
    #[must_use]
    pub fn bgref_lpmode(&mut self) -> BGREF_LPMODE_W<26> {
        BGREF_LPMODE_W::new(self)
    }
    #[doc = "Bit 27 - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    #[must_use]
    pub fn pll_ls_bypass(&mut self) -> PLL_LS_BYPASS_W<27> {
        PLL_LS_BYPASS_W::new(self)
    }
    #[doc = "Bit 28 - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/HIBERNATE. 1: Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    #[must_use]
    pub fn vrefbuf_lpmode(&mut self) -> VREFBUF_LPMODE_W<28> {
        VREFBUF_LPMODE_W::new(self)
    }
    #[doc = "Bit 29 - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn vrefbuf_dis(&mut self) -> VREFBUF_DIS_W<29> {
        VREFBUF_DIS_W::new(self)
    }
    #[doc = "Bit 30 - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn act_ref_dis(&mut self) -> ACT_REF_DIS_W<30> {
        ACT_REF_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctl](index.html) module"]
pub struct PWR_CTL_SPEC;
impl crate::RegisterSpec for PWR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctl::R](R) reader structure"]
impl crate::Readable for PWR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctl::W](W) writer structure"]
impl crate::Writable for PWR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CTL to value 0"]
impl crate::Resettable for PWR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
