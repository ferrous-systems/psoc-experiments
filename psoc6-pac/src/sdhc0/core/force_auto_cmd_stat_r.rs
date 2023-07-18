#[doc = "Register `FORCE_AUTO_CMD_STAT_R` writer"]
pub struct W(crate::W<FORCE_AUTO_CMD_STAT_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCE_AUTO_CMD_STAT_R_SPEC>;
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
impl From<crate::W<FORCE_AUTO_CMD_STAT_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCE_AUTO_CMD_STAT_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_AUTO_CMD12_NOT_EXEC` writer - Force Event for Auto CMD12 Not Executed Values: - 0x1 (TRUE): Auto CMD12 Not Executed Status is set - 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD12_NOT_EXEC_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_AUTO_CMD_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_AUTO_CMD_TOUT_ERR` writer - Force Event for Auto CMD Timeout Error Values: - 0x1 (TRUE): Auto CMD Timeout Error Status is set - 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_TOUT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_AUTO_CMD_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_AUTO_CMD_CRC_ERR` writer - Force Event for Auto CMD CRC Error Values: - 0x1 (TRUE): Auto CMD CRC Error Status is set - 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_AUTO_CMD_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_AUTO_CMD_EBIT_ERR` writer - Force Event for Auto CMD End Bit Error Values: - 0x1 (TRUE): Auto CMD End Bit Error Status is set - 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_EBIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_AUTO_CMD_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_AUTO_CMD_IDX_ERR` writer - Force Event for Auto CMD Index Error Values: - 0x1 (TRUE): Auto CMD Index Error Status is set - 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_IDX_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_AUTO_CMD_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_AUTO_CMD_RESP_ERR` writer - Force Event for Auto CMD Response Error Values: - 0x1 (TRUE): Auto CMD Response Error Status is set - 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_RESP_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_AUTO_CMD_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_CMD_NOT_ISSUED_AUTO_CMD12` writer - Force Event for Command Not Issued By Auto CMD12 Error Values: - 0x1 (TRUE): Command Not Issued By Auto CMD12 Error Status is set - 0x0 (FALSE): Not Affected"]
pub type FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_AUTO_CMD_STAT_R_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 Not Executed Values: - 0x1 (TRUE): Auto CMD12 Not Executed Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd12_not_exec(&mut self) -> FORCE_AUTO_CMD12_NOT_EXEC_W<0> {
        FORCE_AUTO_CMD12_NOT_EXEC_W::new(self)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error Values: - 0x1 (TRUE): Auto CMD Timeout Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_tout_err(&mut self) -> FORCE_AUTO_CMD_TOUT_ERR_W<1> {
        FORCE_AUTO_CMD_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error Values: - 0x1 (TRUE): Auto CMD CRC Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_crc_err(&mut self) -> FORCE_AUTO_CMD_CRC_ERR_W<2> {
        FORCE_AUTO_CMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error Values: - 0x1 (TRUE): Auto CMD End Bit Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_ebit_err(&mut self) -> FORCE_AUTO_CMD_EBIT_ERR_W<3> {
        FORCE_AUTO_CMD_EBIT_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error Values: - 0x1 (TRUE): Auto CMD Index Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_idx_err(&mut self) -> FORCE_AUTO_CMD_IDX_ERR_W<4> {
        FORCE_AUTO_CMD_IDX_ERR_W::new(self)
    }
    #[doc = "Bit 5 - Force Event for Auto CMD Response Error Values: - 0x1 (TRUE): Auto CMD Response Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_resp_err(&mut self) -> FORCE_AUTO_CMD_RESP_ERR_W<5> {
        FORCE_AUTO_CMD_RESP_ERR_W::new(self)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error Values: - 0x1 (TRUE): Command Not Issued By Auto CMD12 Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_not_issued_auto_cmd12(&mut self) -> FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W<7> {
        FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event Register for Auto CMD Error Status register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_auto_cmd_stat_r](index.html) module"]
pub struct FORCE_AUTO_CMD_STAT_R_SPEC;
impl crate::RegisterSpec for FORCE_AUTO_CMD_STAT_R_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [force_auto_cmd_stat_r::W](W) writer structure"]
impl crate::Writable for FORCE_AUTO_CMD_STAT_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FORCE_AUTO_CMD_STAT_R to value 0"]
impl crate::Resettable for FORCE_AUTO_CMD_STAT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
