#[doc = "Register `IDACA` reader"]
pub struct R(crate::R<IDACA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDACA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDACA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDACA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDACA` writer"]
pub struct W(crate::W<IDACA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDACA_SPEC>;
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
impl From<crate::W<IDACA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDACA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - Current value setting for this IDAC (7 bits)."]
pub type VAL_R = crate::FieldReader;
#[doc = "Field `VAL` writer - Current value setting for this IDAC (7 bits)."]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, IDACA_SPEC, 7, O>;
#[doc = "Field `POL_DYN` reader - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
pub type POL_DYN_R = crate::BitReader<POL_DYN_A>;
#[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_DYN_A {
    #[doc = "0: Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    STATIC = 0,
    #[doc = "1: Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    DYNAMIC = 1,
}
impl From<POL_DYN_A> for bool {
    #[inline(always)]
    fn from(variant: POL_DYN_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_DYN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_DYN_A {
        match self.bits {
            false => POL_DYN_A::STATIC,
            true => POL_DYN_A::DYNAMIC,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == POL_DYN_A::STATIC
    }
    #[doc = "Checks if the value of the field is `DYNAMIC`"]
    #[inline(always)]
    pub fn is_dynamic(&self) -> bool {
        *self == POL_DYN_A::DYNAMIC
    }
}
#[doc = "Field `POL_DYN` writer - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
pub type POL_DYN_W<'a, const O: u8> = crate::BitWriter<'a, IDACA_SPEC, O, POL_DYN_A>;
impl<'a, const O: u8> POL_DYN_W<'a, O> {
    #[doc = "Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(POL_DYN_A::STATIC)
    }
    #[doc = "Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    #[inline(always)]
    pub fn dynamic(self) -> &'a mut W {
        self.variant(POL_DYN_A::DYNAMIC)
    }
}
#[doc = "Field `POLARITY` reader - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
pub type POLARITY_R = crate::FieldReader<POLARITY_A>;
#[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLARITY_A {
    #[doc = "0: Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    VSSA_SRC = 0,
    #[doc = "1: Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    VDDA_SNK = 1,
    #[doc = "2: The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE = 2,
    #[doc = "3: The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE_INV = 3,
}
impl From<POLARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POLARITY_A {
    type Ux = u8;
}
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            0 => POLARITY_A::VSSA_SRC,
            1 => POLARITY_A::VDDA_SNK,
            2 => POLARITY_A::SENSE,
            3 => POLARITY_A::SENSE_INV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VSSA_SRC`"]
    #[inline(always)]
    pub fn is_vssa_src(&self) -> bool {
        *self == POLARITY_A::VSSA_SRC
    }
    #[doc = "Checks if the value of the field is `VDDA_SNK`"]
    #[inline(always)]
    pub fn is_vdda_snk(&self) -> bool {
        *self == POLARITY_A::VDDA_SNK
    }
    #[doc = "Checks if the value of the field is `SENSE`"]
    #[inline(always)]
    pub fn is_sense(&self) -> bool {
        *self == POLARITY_A::SENSE
    }
    #[doc = "Checks if the value of the field is `SENSE_INV`"]
    #[inline(always)]
    pub fn is_sense_inv(&self) -> bool {
        *self == POLARITY_A::SENSE_INV
    }
}
#[doc = "Field `POLARITY` writer - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
pub type POLARITY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, IDACA_SPEC, 2, O, POLARITY_A>;
impl<'a, const O: u8> POLARITY_W<'a, O> {
    #[doc = "Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    #[inline(always)]
    pub fn vssa_src(self) -> &'a mut W {
        self.variant(POLARITY_A::VSSA_SRC)
    }
    #[doc = "Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    #[inline(always)]
    pub fn vdda_snk(self) -> &'a mut W {
        self.variant(POLARITY_A::VDDA_SNK)
    }
    #[doc = "The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    #[inline(always)]
    pub fn sense(self) -> &'a mut W {
        self.variant(POLARITY_A::SENSE)
    }
    #[doc = "The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    #[inline(always)]
    pub fn sense_inv(self) -> &'a mut W {
        self.variant(POLARITY_A::SENSE_INV)
    }
}
#[doc = "Field `BAL_MODE` reader - Balancing mode: only applies to legs configured as CSD."]
pub type BAL_MODE_R = crate::FieldReader<BAL_MODE_A>;
#[doc = "Balancing mode: only applies to legs configured as CSD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BAL_MODE_A {
    #[doc = "0: enabled from start of Phi2 until disabled by CSDCMP. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    FULL = 0,
    #[doc = "1: enabled from start of Phi1 and disabled by CSDCMP or at end of Phi1. Enables dual IDAC CSX or Full-Wave, one for sourcing and the other for sinking."]
    PHI1 = 1,
    #[doc = "2: enabled from start of Phi2 and disabled by CSDCMP or at end of Phi2. Intended usage: CSD Low EMI or dual IDAC CSX or Full-Wave."]
    PHI2 = 2,
    #[doc = "3: enabled from start of both Phi1 and Phi2 and disabled by CSDCMP or at end of Phi1 or Phi2 (if non-overlap enabled). Intended usage: single IDAC CSX, or Full-Wave."]
    PHI1_2 = 3,
}
impl From<BAL_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BAL_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BAL_MODE_A {
    type Ux = u8;
}
impl BAL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BAL_MODE_A {
        match self.bits {
            0 => BAL_MODE_A::FULL,
            1 => BAL_MODE_A::PHI1,
            2 => BAL_MODE_A::PHI2,
            3 => BAL_MODE_A::PHI1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == BAL_MODE_A::FULL
    }
    #[doc = "Checks if the value of the field is `PHI1`"]
    #[inline(always)]
    pub fn is_phi1(&self) -> bool {
        *self == BAL_MODE_A::PHI1
    }
    #[doc = "Checks if the value of the field is `PHI2`"]
    #[inline(always)]
    pub fn is_phi2(&self) -> bool {
        *self == BAL_MODE_A::PHI2
    }
    #[doc = "Checks if the value of the field is `PHI1_2`"]
    #[inline(always)]
    pub fn is_phi1_2(&self) -> bool {
        *self == BAL_MODE_A::PHI1_2
    }
}
#[doc = "Field `BAL_MODE` writer - Balancing mode: only applies to legs configured as CSD."]
pub type BAL_MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, IDACA_SPEC, 2, O, BAL_MODE_A>;
impl<'a, const O: u8> BAL_MODE_W<'a, O> {
    #[doc = "enabled from start of Phi2 until disabled by CSDCMP. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(BAL_MODE_A::FULL)
    }
    #[doc = "enabled from start of Phi1 and disabled by CSDCMP or at end of Phi1. Enables dual IDAC CSX or Full-Wave, one for sourcing and the other for sinking."]
    #[inline(always)]
    pub fn phi1(self) -> &'a mut W {
        self.variant(BAL_MODE_A::PHI1)
    }
    #[doc = "enabled from start of Phi2 and disabled by CSDCMP or at end of Phi2. Intended usage: CSD Low EMI or dual IDAC CSX or Full-Wave."]
    #[inline(always)]
    pub fn phi2(self) -> &'a mut W {
        self.variant(BAL_MODE_A::PHI2)
    }
    #[doc = "enabled from start of both Phi1 and Phi2 and disabled by CSDCMP or at end of Phi1 or Phi2 (if non-overlap enabled). Intended usage: single IDAC CSX, or Full-Wave."]
    #[inline(always)]
    pub fn phi1_2(self) -> &'a mut W {
        self.variant(BAL_MODE_A::PHI1_2)
    }
}
#[doc = "Field `LEG1_MODE` reader - Controls the usage mode of LEG1 and the Polarity bit"]
pub type LEG1_MODE_R = crate::FieldReader<LEG1_MODE_A>;
#[doc = "Controls the usage mode of LEG1 and the Polarity bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEG1_MODE_A {
    #[doc = "0: General Purpose static mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    GP_STATIC = 0,
    #[doc = "1: General Purpose dynamic mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    GP = 1,
    #[doc = "2: CSD static mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG1 is controlled by LEG1_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    CSD_STATIC = 2,
    #[doc = "3: CSD dynamic mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In thoses states LEG1 is controlled by LEG1_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    CSD = 3,
}
impl From<LEG1_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LEG1_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEG1_MODE_A {
    type Ux = u8;
}
impl LEG1_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEG1_MODE_A {
        match self.bits {
            0 => LEG1_MODE_A::GP_STATIC,
            1 => LEG1_MODE_A::GP,
            2 => LEG1_MODE_A::CSD_STATIC,
            3 => LEG1_MODE_A::CSD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GP_STATIC`"]
    #[inline(always)]
    pub fn is_gp_static(&self) -> bool {
        *self == LEG1_MODE_A::GP_STATIC
    }
    #[doc = "Checks if the value of the field is `GP`"]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        *self == LEG1_MODE_A::GP
    }
    #[doc = "Checks if the value of the field is `CSD_STATIC`"]
    #[inline(always)]
    pub fn is_csd_static(&self) -> bool {
        *self == LEG1_MODE_A::CSD_STATIC
    }
    #[doc = "Checks if the value of the field is `CSD`"]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        *self == LEG1_MODE_A::CSD
    }
}
#[doc = "Field `LEG1_MODE` writer - Controls the usage mode of LEG1 and the Polarity bit"]
pub type LEG1_MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, IDACA_SPEC, 2, O, LEG1_MODE_A>;
impl<'a, const O: u8> LEG1_MODE_W<'a, O> {
    #[doc = "General Purpose static mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn gp_static(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::GP_STATIC)
    }
    #[doc = "General Purpose dynamic mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn gp(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::GP)
    }
    #[doc = "CSD static mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG1 is controlled by LEG1_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn csd_static(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::CSD_STATIC)
    }
    #[doc = "CSD dynamic mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In thoses states LEG1 is controlled by LEG1_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn csd(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::CSD)
    }
}
#[doc = "Field `LEG2_MODE` reader - Controls the usage mode of LEG2"]
pub type LEG2_MODE_R = crate::FieldReader<LEG2_MODE_A>;
#[doc = "Controls the usage mode of LEG2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEG2_MODE_A {
    #[doc = "0: General Purpose static mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    GP_STATIC = 0,
    #[doc = "1: General Purpose dynamic mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    GP = 1,
    #[doc = "2: CSD static mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg2 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    CSD_STATIC = 2,
    #[doc = "3: CSD dynamic mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). In addition leg2 enable can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    CSD = 3,
}
impl From<LEG2_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LEG2_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEG2_MODE_A {
    type Ux = u8;
}
impl LEG2_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEG2_MODE_A {
        match self.bits {
            0 => LEG2_MODE_A::GP_STATIC,
            1 => LEG2_MODE_A::GP,
            2 => LEG2_MODE_A::CSD_STATIC,
            3 => LEG2_MODE_A::CSD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GP_STATIC`"]
    #[inline(always)]
    pub fn is_gp_static(&self) -> bool {
        *self == LEG2_MODE_A::GP_STATIC
    }
    #[doc = "Checks if the value of the field is `GP`"]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        *self == LEG2_MODE_A::GP
    }
    #[doc = "Checks if the value of the field is `CSD_STATIC`"]
    #[inline(always)]
    pub fn is_csd_static(&self) -> bool {
        *self == LEG2_MODE_A::CSD_STATIC
    }
    #[doc = "Checks if the value of the field is `CSD`"]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        *self == LEG2_MODE_A::CSD
    }
}
#[doc = "Field `LEG2_MODE` writer - Controls the usage mode of LEG2"]
pub type LEG2_MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, IDACA_SPEC, 2, O, LEG2_MODE_A>;
impl<'a, const O: u8> LEG2_MODE_W<'a, O> {
    #[doc = "General Purpose static mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn gp_static(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::GP_STATIC)
    }
    #[doc = "General Purpose dynamic mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn gp(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::GP)
    }
    #[doc = "CSD static mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg2 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn csd_static(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::CSD_STATIC)
    }
    #[doc = "CSD dynamic mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). In addition leg2 enable can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn csd(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::CSD)
    }
}
#[doc = "Field `DSI_CTRL_EN` reader - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
pub type DSI_CTRL_EN_R = crate::BitReader;
#[doc = "Field `DSI_CTRL_EN` writer - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
pub type DSI_CTRL_EN_W<'a, const O: u8> = crate::BitWriter<'a, IDACA_SPEC, O>;
#[doc = "Field `RANGE` reader - IDAC multiplier"]
pub type RANGE_R = crate::FieldReader<RANGE_A>;
#[doc = "IDAC multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RANGE_A {
    #[doc = "0: 1 LSB = 37.5 nA"]
    IDAC_LO = 0,
    #[doc = "1: 1 LSB = 300 nA"]
    IDAC_MED = 1,
    #[doc = "2: 1 LSB = 2400 nA"]
    IDAC_HI = 2,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RANGE_A {
    type Ux = u8;
}
impl RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RANGE_A> {
        match self.bits {
            0 => Some(RANGE_A::IDAC_LO),
            1 => Some(RANGE_A::IDAC_MED),
            2 => Some(RANGE_A::IDAC_HI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDAC_LO`"]
    #[inline(always)]
    pub fn is_idac_lo(&self) -> bool {
        *self == RANGE_A::IDAC_LO
    }
    #[doc = "Checks if the value of the field is `IDAC_MED`"]
    #[inline(always)]
    pub fn is_idac_med(&self) -> bool {
        *self == RANGE_A::IDAC_MED
    }
    #[doc = "Checks if the value of the field is `IDAC_HI`"]
    #[inline(always)]
    pub fn is_idac_hi(&self) -> bool {
        *self == RANGE_A::IDAC_HI
    }
}
#[doc = "Field `RANGE` writer - IDAC multiplier"]
pub type RANGE_W<'a, const O: u8> = crate::FieldWriter<'a, IDACA_SPEC, 2, O, RANGE_A>;
impl<'a, const O: u8> RANGE_W<'a, O> {
    #[doc = "1 LSB = 37.5 nA"]
    #[inline(always)]
    pub fn idac_lo(self) -> &'a mut W {
        self.variant(RANGE_A::IDAC_LO)
    }
    #[doc = "1 LSB = 300 nA"]
    #[inline(always)]
    pub fn idac_med(self) -> &'a mut W {
        self.variant(RANGE_A::IDAC_MED)
    }
    #[doc = "1 LSB = 2400 nA"]
    #[inline(always)]
    pub fn idac_hi(self) -> &'a mut W {
        self.variant(RANGE_A::IDAC_HI)
    }
}
#[doc = "Field `LEG1_EN` reader - output enable for leg 1 to CSDBUSA"]
pub type LEG1_EN_R = crate::BitReader;
#[doc = "Field `LEG1_EN` writer - output enable for leg 1 to CSDBUSA"]
pub type LEG1_EN_W<'a, const O: u8> = crate::BitWriter<'a, IDACA_SPEC, O>;
#[doc = "Field `LEG2_EN` reader - output enable for leg 2 to CSDBUSA"]
pub type LEG2_EN_R = crate::BitReader;
#[doc = "Field `LEG2_EN` writer - output enable for leg 2 to CSDBUSA"]
pub type LEG2_EN_W<'a, const O: u8> = crate::BitWriter<'a, IDACA_SPEC, O>;
impl R {
    #[doc = "Bits 0:6 - Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub fn pol_dyn(&self) -> POL_DYN_R {
        POL_DYN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Balancing mode: only applies to legs configured as CSD."]
    #[inline(always)]
    pub fn bal_mode(&self) -> BAL_MODE_R {
        BAL_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub fn leg1_mode(&self) -> LEG1_MODE_R {
        LEG1_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Controls the usage mode of LEG2"]
    #[inline(always)]
    pub fn leg2_mode(&self) -> LEG2_MODE_R {
        LEG2_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
    #[inline(always)]
    pub fn dsi_ctrl_en(&self) -> DSI_CTRL_EN_R {
        DSI_CTRL_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - IDAC multiplier"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - output enable for leg 1 to CSDBUSA"]
    #[inline(always)]
    pub fn leg1_en(&self) -> LEG1_EN_R {
        LEG1_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - output enable for leg 2 to CSDBUSA"]
    #[inline(always)]
    pub fn leg2_en(&self) -> LEG2_EN_R {
        LEG2_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Bit 7 - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    #[must_use]
    pub fn pol_dyn(&mut self) -> POL_DYN_W<7> {
        POL_DYN_W::new(self)
    }
    #[doc = "Bits 8:9 - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<8> {
        POLARITY_W::new(self)
    }
    #[doc = "Bits 10:11 - Balancing mode: only applies to legs configured as CSD."]
    #[inline(always)]
    #[must_use]
    pub fn bal_mode(&mut self) -> BAL_MODE_W<10> {
        BAL_MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    #[must_use]
    pub fn leg1_mode(&mut self) -> LEG1_MODE_W<16> {
        LEG1_MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Controls the usage mode of LEG2"]
    #[inline(always)]
    #[must_use]
    pub fn leg2_mode(&mut self) -> LEG2_MODE_W<18> {
        LEG2_MODE_W::new(self)
    }
    #[doc = "Bit 21 - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_ctrl_en(&mut self) -> DSI_CTRL_EN_W<21> {
        DSI_CTRL_EN_W::new(self)
    }
    #[doc = "Bits 22:23 - IDAC multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn range(&mut self) -> RANGE_W<22> {
        RANGE_W::new(self)
    }
    #[doc = "Bit 24 - output enable for leg 1 to CSDBUSA"]
    #[inline(always)]
    #[must_use]
    pub fn leg1_en(&mut self) -> LEG1_EN_W<24> {
        LEG1_EN_W::new(self)
    }
    #[doc = "Bit 25 - output enable for leg 2 to CSDBUSA"]
    #[inline(always)]
    #[must_use]
    pub fn leg2_en(&mut self) -> LEG2_EN_W<25> {
        LEG2_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDACA Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idaca](index.html) module"]
pub struct IDACA_SPEC;
impl crate::RegisterSpec for IDACA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idaca::R](R) reader structure"]
impl crate::Readable for IDACA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idaca::W](W) writer structure"]
impl crate::Writable for IDACA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDACA to value 0"]
impl crate::Resettable for IDACA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
