#[doc = "Register `ERROR_INT_STAT_R` reader"]
pub struct R(crate::R<ERROR_INT_STAT_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_INT_STAT_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_INT_STAT_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_INT_STAT_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_INT_STAT_R` writer"]
pub struct W(crate::W<ERROR_INT_STAT_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_INT_STAT_R_SPEC>;
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
impl From<crate::W<ERROR_INT_STAT_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_INT_STAT_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_TOUT_ERR` reader - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
pub type CMD_TOUT_ERR_R = crate::BitReader;
#[doc = "Field `CMD_TOUT_ERR` writer - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
pub type CMD_TOUT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `CMD_CRC_ERR` reader - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
pub type CMD_CRC_ERR_R = crate::BitReader;
#[doc = "Field `CMD_CRC_ERR` writer - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
pub type CMD_CRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `CMD_END_BIT_ERR` reader - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
pub type CMD_END_BIT_ERR_R = crate::BitReader;
#[doc = "Field `CMD_END_BIT_ERR` writer - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
pub type CMD_END_BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `CMD_IDX_ERR` reader - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type CMD_IDX_ERR_R = crate::BitReader;
#[doc = "Field `CMD_IDX_ERR` writer - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type CMD_IDX_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `DATA_TOUT_ERR` reader - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
pub type DATA_TOUT_ERR_R = crate::BitReader;
#[doc = "Field `DATA_TOUT_ERR` writer - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
pub type DATA_TOUT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `DATA_CRC_ERR` reader - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type DATA_CRC_ERR_R = crate::BitReader;
#[doc = "Field `DATA_CRC_ERR` writer - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type DATA_CRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `DATA_END_BIT_ERR` reader - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type DATA_END_BIT_ERR_R = crate::BitReader;
#[doc = "Field `DATA_END_BIT_ERR` writer - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type DATA_END_BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `CUR_LMT_ERR` reader - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
pub type CUR_LMT_ERR_R = crate::BitReader;
#[doc = "Field `CUR_LMT_ERR` writer - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
pub type CUR_LMT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `AUTO_CMD_ERR` reader - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type AUTO_CMD_ERR_R = crate::BitReader;
#[doc = "Field `AUTO_CMD_ERR` writer - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type AUTO_CMD_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `ADMA_ERR` reader - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type ADMA_ERR_R = crate::BitReader;
#[doc = "Field `ADMA_ERR` writer - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type ADMA_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `TUNING_ERR` reader - N/A"]
pub type TUNING_ERR_R = crate::BitReader;
#[doc = "Field `TUNING_ERR` writer - N/A"]
pub type TUNING_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `RESP_ERR` reader - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type RESP_ERR_R = crate::BitReader;
#[doc = "Field `RESP_ERR` writer - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type RESP_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
#[doc = "Field `BOOT_ACK_ERR` reader - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type BOOT_ACK_ERR_R = crate::BitReader;
#[doc = "Field `BOOT_ACK_ERR` writer - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type BOOT_ACK_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ERROR_INT_STAT_R_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn cmd_tout_err(&self) -> CMD_TOUT_ERR_R {
        CMD_TOUT_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
    #[inline(always)]
    pub fn cmd_crc_err(&self) -> CMD_CRC_ERR_R {
        CMD_CRC_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&self) -> CMD_END_BIT_ERR_R {
        CMD_END_BIT_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err(&self) -> CMD_IDX_ERR_R {
        CMD_IDX_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn data_tout_err(&self) -> DATA_TOUT_ERR_R {
        DATA_TOUT_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_crc_err(&self) -> DATA_CRC_ERR_R {
        DATA_CRC_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&self) -> DATA_END_BIT_ERR_R {
        DATA_END_BIT_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
    #[inline(always)]
    pub fn cur_lmt_err(&self) -> CUR_LMT_ERR_R {
        CUR_LMT_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn auto_cmd_err(&self) -> AUTO_CMD_ERR_R {
        AUTO_CMD_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn adma_err(&self) -> ADMA_ERR_R {
        ADMA_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tuning_err(&self) -> TUNING_ERR_R {
        TUNING_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn resp_err(&self) -> RESP_ERR_R {
        RESP_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn boot_ack_err(&self) -> BOOT_ACK_ERR_R {
        BOOT_ACK_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_tout_err(&mut self) -> CMD_TOUT_ERR_W<0> {
        CMD_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 1 - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err(&mut self) -> CMD_CRC_ERR_W<1> {
        CMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 2 - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err(&mut self) -> CMD_END_BIT_ERR_W<2> {
        CMD_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 3 - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx_err(&mut self) -> CMD_IDX_ERR_W<3> {
        CMD_IDX_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    #[must_use]
    pub fn data_tout_err(&mut self) -> DATA_TOUT_ERR_W<4> {
        DATA_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 5 - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_err(&mut self) -> DATA_CRC_ERR_W<5> {
        DATA_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 6 - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn data_end_bit_err(&mut self) -> DATA_END_BIT_ERR_W<6> {
        DATA_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 7 - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
    #[inline(always)]
    #[must_use]
    pub fn cur_lmt_err(&mut self) -> CUR_LMT_ERR_W<7> {
        CUR_LMT_ERR_W::new(self)
    }
    #[doc = "Bit 8 - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_err(&mut self) -> AUTO_CMD_ERR_W<8> {
        AUTO_CMD_ERR_W::new(self)
    }
    #[doc = "Bit 9 - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn adma_err(&mut self) -> ADMA_ERR_W<9> {
        ADMA_ERR_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_err(&mut self) -> TUNING_ERR_W<10> {
        TUNING_ERR_W::new(self)
    }
    #[doc = "Bit 11 - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn resp_err(&mut self) -> RESP_ERR_W<11> {
        RESP_ERR_W::new(self)
    }
    #[doc = "Bit 12 - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_err(&mut self) -> BOOT_ACK_ERR_W<12> {
        BOOT_ACK_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_int_stat_r](index.html) module"]
pub struct ERROR_INT_STAT_R_SPEC;
impl crate::RegisterSpec for ERROR_INT_STAT_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [error_int_stat_r::R](R) reader structure"]
impl crate::Readable for ERROR_INT_STAT_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_int_stat_r::W](W) writer structure"]
impl crate::Writable for ERROR_INT_STAT_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERROR_INT_STAT_R to value 0"]
impl crate::Resettable for ERROR_INT_STAT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
