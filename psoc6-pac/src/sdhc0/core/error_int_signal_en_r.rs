#[doc = "Register `ERROR_INT_SIGNAL_EN_R` reader"]
pub struct R(crate::R<ERROR_INT_SIGNAL_EN_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_INT_SIGNAL_EN_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_INT_SIGNAL_EN_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_INT_SIGNAL_EN_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_INT_SIGNAL_EN_R` writer"]
pub struct W(crate::W<ERROR_INT_SIGNAL_EN_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_INT_SIGNAL_EN_R_SPEC>;
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
impl From<crate::W<ERROR_INT_SIGNAL_EN_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_INT_SIGNAL_EN_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_TOUT_ERR_SIGNAL_EN` reader - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_TOUT_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CMD_TOUT_ERR_SIGNAL_EN` writer - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_TOUT_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `CMD_CRC_ERR_SIGNAL_EN` reader - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_CRC_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CMD_CRC_ERR_SIGNAL_EN` writer - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_CRC_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `CMD_END_BIT_ERR_SIGNAL_EN` reader - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_END_BIT_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CMD_END_BIT_ERR_SIGNAL_EN` writer - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_END_BIT_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `CMD_IDX_ERR_SIGNAL_EN` reader - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type CMD_IDX_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CMD_IDX_ERR_SIGNAL_EN` writer - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type CMD_IDX_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `DATA_TOUT_ERR_SIGNAL_EN` reader - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_TOUT_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `DATA_TOUT_ERR_SIGNAL_EN` writer - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_TOUT_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `DATA_CRC_ERR_SIGNAL_EN` reader - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_CRC_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `DATA_CRC_ERR_SIGNAL_EN` writer - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_CRC_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `DATA_END_BIT_ERR_SIGNAL_EN` reader - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_END_BIT_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `DATA_END_BIT_ERR_SIGNAL_EN` writer - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_END_BIT_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `CUR_LMT_ERR_SIGNAL_EN` reader - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CUR_LMT_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `CUR_LMT_ERR_SIGNAL_EN` writer - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CUR_LMT_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `AUTO_CMD_ERR_SIGNAL_EN` reader - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AUTO_CMD_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `AUTO_CMD_ERR_SIGNAL_EN` writer - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AUTO_CMD_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `ADMA_ERR_SIGNAL_EN` reader - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type ADMA_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `ADMA_ERR_SIGNAL_EN` writer - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type ADMA_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `TUNING_ERR_SIGNAL_EN` reader - N/A"]
pub type TUNING_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `TUNING_ERR_SIGNAL_EN` writer - N/A"]
pub type TUNING_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `RESP_ERR_SIGNAL_EN` reader - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type RESP_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `RESP_ERR_SIGNAL_EN` writer - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type RESP_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `BOOT_ACK_ERR_SIGNAL_EN` reader - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BOOT_ACK_ERR_SIGNAL_EN_R = crate::BitReader;
#[doc = "Field `BOOT_ACK_ERR_SIGNAL_EN` writer - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BOOT_ACK_ERR_SIGNAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN1` reader - N/A"]
pub type VENDOR_ERR_SIGNAL_EN1_R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN1` writer - N/A"]
pub type VENDOR_ERR_SIGNAL_EN1_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN2` reader - N/A"]
pub type VENDOR_ERR_SIGNAL_EN2_R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN2` writer - N/A"]
pub type VENDOR_ERR_SIGNAL_EN2_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN3` reader - N/A"]
pub type VENDOR_ERR_SIGNAL_EN3_R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN3` writer - N/A"]
pub type VENDOR_ERR_SIGNAL_EN3_W<'a, const O: u8> =
    crate::BitWriter<'a, ERROR_INT_SIGNAL_EN_R_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_tout_err_signal_en(&self) -> CMD_TOUT_ERR_SIGNAL_EN_R {
        CMD_TOUT_ERR_SIGNAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_crc_err_signal_en(&self) -> CMD_CRC_ERR_SIGNAL_EN_R {
        CMD_CRC_ERR_SIGNAL_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_end_bit_err_signal_en(&self) -> CMD_END_BIT_ERR_SIGNAL_EN_R {
        CMD_END_BIT_ERR_SIGNAL_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err_signal_en(&self) -> CMD_IDX_ERR_SIGNAL_EN_R {
        CMD_IDX_ERR_SIGNAL_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_tout_err_signal_en(&self) -> DATA_TOUT_ERR_SIGNAL_EN_R {
        DATA_TOUT_ERR_SIGNAL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_crc_err_signal_en(&self) -> DATA_CRC_ERR_SIGNAL_EN_R {
        DATA_CRC_ERR_SIGNAL_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_end_bit_err_signal_en(&self) -> DATA_END_BIT_ERR_SIGNAL_EN_R {
        DATA_END_BIT_ERR_SIGNAL_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cur_lmt_err_signal_en(&self) -> CUR_LMT_ERR_SIGNAL_EN_R {
        CUR_LMT_ERR_SIGNAL_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn auto_cmd_err_signal_en(&self) -> AUTO_CMD_ERR_SIGNAL_EN_R {
        AUTO_CMD_ERR_SIGNAL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn adma_err_signal_en(&self) -> ADMA_ERR_SIGNAL_EN_R {
        ADMA_ERR_SIGNAL_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tuning_err_signal_en(&self) -> TUNING_ERR_SIGNAL_EN_R {
        TUNING_ERR_SIGNAL_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn resp_err_signal_en(&self) -> RESP_ERR_SIGNAL_EN_R {
        RESP_ERR_SIGNAL_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn boot_ack_err_signal_en(&self) -> BOOT_ACK_ERR_SIGNAL_EN_R {
        BOOT_ACK_ERR_SIGNAL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en1(&self) -> VENDOR_ERR_SIGNAL_EN1_R {
        VENDOR_ERR_SIGNAL_EN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en2(&self) -> VENDOR_ERR_SIGNAL_EN2_R {
        VENDOR_ERR_SIGNAL_EN2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en3(&self) -> VENDOR_ERR_SIGNAL_EN3_R {
        VENDOR_ERR_SIGNAL_EN3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_tout_err_signal_en(&mut self) -> CMD_TOUT_ERR_SIGNAL_EN_W<0> {
        CMD_TOUT_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err_signal_en(&mut self) -> CMD_CRC_ERR_SIGNAL_EN_W<1> {
        CMD_CRC_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err_signal_en(&mut self) -> CMD_END_BIT_ERR_SIGNAL_EN_W<2> {
        CMD_END_BIT_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx_err_signal_en(&mut self) -> CMD_IDX_ERR_SIGNAL_EN_W<3> {
        CMD_IDX_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_tout_err_signal_en(&mut self) -> DATA_TOUT_ERR_SIGNAL_EN_W<4> {
        DATA_TOUT_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_err_signal_en(&mut self) -> DATA_CRC_ERR_SIGNAL_EN_W<5> {
        DATA_CRC_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_end_bit_err_signal_en(&mut self) -> DATA_END_BIT_ERR_SIGNAL_EN_W<6> {
        DATA_END_BIT_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cur_lmt_err_signal_en(&mut self) -> CUR_LMT_ERR_SIGNAL_EN_W<7> {
        CUR_LMT_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_err_signal_en(&mut self) -> AUTO_CMD_ERR_SIGNAL_EN_W<8> {
        AUTO_CMD_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn adma_err_signal_en(&mut self) -> ADMA_ERR_SIGNAL_EN_W<9> {
        ADMA_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_err_signal_en(&mut self) -> TUNING_ERR_SIGNAL_EN_W<10> {
        TUNING_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 11 - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_signal_en(&mut self) -> RESP_ERR_SIGNAL_EN_W<11> {
        RESP_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_err_signal_en(&mut self) -> BOOT_ACK_ERR_SIGNAL_EN_W<12> {
        BOOT_ACK_ERR_SIGNAL_EN_W::new(self)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_signal_en1(&mut self) -> VENDOR_ERR_SIGNAL_EN1_W<13> {
        VENDOR_ERR_SIGNAL_EN1_W::new(self)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_signal_en2(&mut self) -> VENDOR_ERR_SIGNAL_EN2_W<14> {
        VENDOR_ERR_SIGNAL_EN2_W::new(self)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_signal_en3(&mut self) -> VENDOR_ERR_SIGNAL_EN3_W<15> {
        VENDOR_ERR_SIGNAL_EN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_int_signal_en_r](index.html) module"]
pub struct ERROR_INT_SIGNAL_EN_R_SPEC;
impl crate::RegisterSpec for ERROR_INT_SIGNAL_EN_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [error_int_signal_en_r::R](R) reader structure"]
impl crate::Readable for ERROR_INT_SIGNAL_EN_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_int_signal_en_r::W](W) writer structure"]
impl crate::Writable for ERROR_INT_SIGNAL_EN_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERROR_INT_SIGNAL_EN_R to value 0"]
impl crate::Resettable for ERROR_INT_SIGNAL_EN_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
