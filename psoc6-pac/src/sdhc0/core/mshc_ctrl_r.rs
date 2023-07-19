#[doc = "Register `MSHC_CTRL_R` reader"]
pub struct R(crate::R<MSHC_CTRL_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSHC_CTRL_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSHC_CTRL_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSHC_CTRL_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSHC_CTRL_R` writer"]
pub struct W(crate::W<MSHC_CTRL_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSHC_CTRL_R_SPEC>;
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
impl From<crate::W<MSHC_CTRL_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSHC_CTRL_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_CONFLICT_CHECK` reader - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
pub type CMD_CONFLICT_CHECK_R = crate::BitReader;
#[doc = "Field `CMD_CONFLICT_CHECK` writer - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
pub type CMD_CONFLICT_CHECK_W<'a, const O: u8> = crate::BitWriter<'a, MSHC_CTRL_R_SPEC, O>;
#[doc = "Field `SW_CG_DIS` reader - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
pub type SW_CG_DIS_R = crate::BitReader;
#[doc = "Field `SW_CG_DIS` writer - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
pub type SW_CG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, MSHC_CTRL_R_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
    #[inline(always)]
    pub fn cmd_conflict_check(&self) -> CMD_CONFLICT_CHECK_R {
        CMD_CONFLICT_CHECK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
    #[inline(always)]
    pub fn sw_cg_dis(&self) -> SW_CG_DIS_R {
        SW_CG_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_conflict_check(&mut self) -> CMD_CONFLICT_CHECK_W<0> {
        CMD_CONFLICT_CHECK_W::new(self)
    }
    #[doc = "Bit 4 - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cg_dis(&mut self) -> SW_CG_DIS_W<4> {
        SW_CG_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSHC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mshc_ctrl_r](index.html) module"]
pub struct MSHC_CTRL_R_SPEC;
impl crate::RegisterSpec for MSHC_CTRL_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mshc_ctrl_r::R](R) reader structure"]
impl crate::Readable for MSHC_CTRL_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mshc_ctrl_r::W](W) writer structure"]
impl crate::Writable for MSHC_CTRL_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSHC_CTRL_R to value 0x01"]
impl crate::Resettable for MSHC_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
