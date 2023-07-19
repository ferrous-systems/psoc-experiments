#[doc = "Register `FORCE_ERROR_INT_STAT_R` reader"]
pub struct R(crate::R<FORCE_ERROR_INT_STAT_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FORCE_ERROR_INT_STAT_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FORCE_ERROR_INT_STAT_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FORCE_ERROR_INT_STAT_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FORCE_ERROR_INT_STAT_R` writer"]
pub struct W(crate::W<FORCE_ERROR_INT_STAT_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCE_ERROR_INT_STAT_R_SPEC>;
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
impl From<crate::W<FORCE_ERROR_INT_STAT_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCE_ERROR_INT_STAT_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_CMD_TOUT_ERR` reader - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
pub type FORCE_CMD_TOUT_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_CMD_TOUT_ERR` writer - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
pub type FORCE_CMD_TOUT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_CMD_CRC_ERR` reader - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
pub type FORCE_CMD_CRC_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_CMD_CRC_ERR` writer - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
pub type FORCE_CMD_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_CMD_END_BIT_ERR` reader - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
pub type FORCE_CMD_END_BIT_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_CMD_END_BIT_ERR` writer - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
pub type FORCE_CMD_END_BIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_CMD_IDX_ERR` reader - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
pub type FORCE_CMD_IDX_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_CMD_IDX_ERR` writer - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
pub type FORCE_CMD_IDX_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_DATA_TOUT_ERR` reader - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
pub type FORCE_DATA_TOUT_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_DATA_TOUT_ERR` writer - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
pub type FORCE_DATA_TOUT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_DATA_CRC_ERR` reader - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
pub type FORCE_DATA_CRC_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_DATA_CRC_ERR` writer - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
pub type FORCE_DATA_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_DATA_END_BIT_ERR` reader - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
pub type FORCE_DATA_END_BIT_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_DATA_END_BIT_ERR` writer - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
pub type FORCE_DATA_END_BIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_CUR_LMT_ERR` reader - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
pub type FORCE_CUR_LMT_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_CUR_LMT_ERR` writer - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
pub type FORCE_CUR_LMT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_AUTO_CMD_ERR` reader - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
pub type FORCE_AUTO_CMD_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_AUTO_CMD_ERR` writer - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
pub type FORCE_AUTO_CMD_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_ADMA_ERR` reader - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
pub type FORCE_ADMA_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_ADMA_ERR` writer - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
pub type FORCE_ADMA_ERR_W<'a, const O: u8> = crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_TUNING_ERR` reader - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
pub type FORCE_TUNING_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_TUNING_ERR` writer - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
pub type FORCE_TUNING_ERR_W<'a, const O: u8> = crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_RESP_ERR` reader - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
pub type FORCE_RESP_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_RESP_ERR` writer - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
pub type FORCE_RESP_ERR_W<'a, const O: u8> = crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_BOOT_ACK_ERR` reader - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
pub type FORCE_BOOT_ACK_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_BOOT_ACK_ERR` writer - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
pub type FORCE_BOOT_ACK_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_VENDOR_ERR1` reader - N/A"]
pub type FORCE_VENDOR_ERR1_R = crate::BitReader;
#[doc = "Field `FORCE_VENDOR_ERR1` writer - N/A"]
pub type FORCE_VENDOR_ERR1_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_VENDOR_ERR2` reader - N/A"]
pub type FORCE_VENDOR_ERR2_R = crate::BitReader;
#[doc = "Field `FORCE_VENDOR_ERR2` writer - N/A"]
pub type FORCE_VENDOR_ERR2_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `FORCE_VENDOR_ERR3` reader - N/A"]
pub type FORCE_VENDOR_ERR3_R = crate::BitReader;
#[doc = "Field `FORCE_VENDOR_ERR3` writer - N/A"]
pub type FORCE_VENDOR_ERR3_W<'a, const O: u8> =
    crate::BitWriter<'a, FORCE_ERROR_INT_STAT_R_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_tout_err(&self) -> FORCE_CMD_TOUT_ERR_R {
        FORCE_CMD_TOUT_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_crc_err(&self) -> FORCE_CMD_CRC_ERR_R {
        FORCE_CMD_CRC_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_end_bit_err(&self) -> FORCE_CMD_END_BIT_ERR_R {
        FORCE_CMD_END_BIT_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_idx_err(&self) -> FORCE_CMD_IDX_ERR_R {
        FORCE_CMD_IDX_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_data_tout_err(&self) -> FORCE_DATA_TOUT_ERR_R {
        FORCE_DATA_TOUT_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
    #[inline(always)]
    pub fn force_data_crc_err(&self) -> FORCE_DATA_CRC_ERR_R {
        FORCE_DATA_CRC_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_data_end_bit_err(&self) -> FORCE_DATA_END_BIT_ERR_R {
        FORCE_DATA_END_BIT_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
    #[inline(always)]
    pub fn force_cur_lmt_err(&self) -> FORCE_CUR_LMT_ERR_R {
        FORCE_CUR_LMT_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
    #[inline(always)]
    pub fn force_auto_cmd_err(&self) -> FORCE_AUTO_CMD_ERR_R {
        FORCE_AUTO_CMD_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
    #[inline(always)]
    pub fn force_adma_err(&self) -> FORCE_ADMA_ERR_R {
        FORCE_ADMA_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
    #[inline(always)]
    pub fn force_tuning_err(&self) -> FORCE_TUNING_ERR_R {
        FORCE_TUNING_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
    #[inline(always)]
    pub fn force_resp_err(&self) -> FORCE_RESP_ERR_R {
        FORCE_RESP_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
    #[inline(always)]
    pub fn force_boot_ack_err(&self) -> FORCE_BOOT_ACK_ERR_R {
        FORCE_BOOT_ACK_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err1(&self) -> FORCE_VENDOR_ERR1_R {
        FORCE_VENDOR_ERR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err2(&self) -> FORCE_VENDOR_ERR2_R {
        FORCE_VENDOR_ERR2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err3(&self) -> FORCE_VENDOR_ERR3_R {
        FORCE_VENDOR_ERR3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_tout_err(&mut self) -> FORCE_CMD_TOUT_ERR_W<0> {
        FORCE_CMD_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_crc_err(&mut self) -> FORCE_CMD_CRC_ERR_W<1> {
        FORCE_CMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_end_bit_err(&mut self) -> FORCE_CMD_END_BIT_ERR_W<2> {
        FORCE_CMD_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_idx_err(&mut self) -> FORCE_CMD_IDX_ERR_W<3> {
        FORCE_CMD_IDX_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_tout_err(&mut self) -> FORCE_DATA_TOUT_ERR_W<4> {
        FORCE_DATA_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_crc_err(&mut self) -> FORCE_DATA_CRC_ERR_W<5> {
        FORCE_DATA_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_end_bit_err(&mut self) -> FORCE_DATA_END_BIT_ERR_W<6> {
        FORCE_DATA_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cur_lmt_err(&mut self) -> FORCE_CUR_LMT_ERR_W<7> {
        FORCE_CUR_LMT_ERR_W::new(self)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_err(&mut self) -> FORCE_AUTO_CMD_ERR_W<8> {
        FORCE_AUTO_CMD_ERR_W::new(self)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_adma_err(&mut self) -> FORCE_ADMA_ERR_W<9> {
        FORCE_ADMA_ERR_W::new(self)
    }
    #[doc = "Bit 10 - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_tuning_err(&mut self) -> FORCE_TUNING_ERR_W<10> {
        FORCE_TUNING_ERR_W::new(self)
    }
    #[doc = "Bit 11 - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_resp_err(&mut self) -> FORCE_RESP_ERR_W<11> {
        FORCE_RESP_ERR_W::new(self)
    }
    #[doc = "Bit 12 - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_boot_ack_err(&mut self) -> FORCE_BOOT_ACK_ERR_W<12> {
        FORCE_BOOT_ACK_ERR_W::new(self)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn force_vendor_err1(&mut self) -> FORCE_VENDOR_ERR1_W<13> {
        FORCE_VENDOR_ERR1_W::new(self)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn force_vendor_err2(&mut self) -> FORCE_VENDOR_ERR2_W<14> {
        FORCE_VENDOR_ERR2_W::new(self)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn force_vendor_err3(&mut self) -> FORCE_VENDOR_ERR3_W<15> {
        FORCE_VENDOR_ERR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event Register for Error Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_error_int_stat_r](index.html) module"]
pub struct FORCE_ERROR_INT_STAT_R_SPEC;
impl crate::RegisterSpec for FORCE_ERROR_INT_STAT_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [force_error_int_stat_r::R](R) reader structure"]
impl crate::Readable for FORCE_ERROR_INT_STAT_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [force_error_int_stat_r::W](W) writer structure"]
impl crate::Writable for FORCE_ERROR_INT_STAT_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FORCE_ERROR_INT_STAT_R to value 0"]
impl crate::Resettable for FORCE_ERROR_INT_STAT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
