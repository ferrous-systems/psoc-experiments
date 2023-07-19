#[doc = "Register `CMD_R` reader"]
pub struct R(crate::R<CMD_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_R` writer"]
pub struct W(crate::W<CMD_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_R_SPEC>;
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
impl From<crate::W<CMD_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESP_TYPE_SELECT` reader - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
pub type RESP_TYPE_SELECT_R = crate::FieldReader;
#[doc = "Field `RESP_TYPE_SELECT` writer - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
pub type RESP_TYPE_SELECT_W<'a, const O: u8> = crate::FieldWriter<'a, CMD_R_SPEC, 2, O>;
#[doc = "Field `SUB_CMD_FLAG` reader - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
pub type SUB_CMD_FLAG_R = crate::BitReader;
#[doc = "Field `SUB_CMD_FLAG` writer - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
pub type SUB_CMD_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, CMD_R_SPEC, O>;
#[doc = "Field `CMD_CRC_CHK_ENABLE` reader - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type CMD_CRC_CHK_ENABLE_R = crate::BitReader;
#[doc = "Field `CMD_CRC_CHK_ENABLE` writer - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type CMD_CRC_CHK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, CMD_R_SPEC, O>;
#[doc = "Field `CMD_IDX_CHK_ENABLE` reader - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type CMD_IDX_CHK_ENABLE_R = crate::BitReader;
#[doc = "Field `CMD_IDX_CHK_ENABLE` writer - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type CMD_IDX_CHK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, CMD_R_SPEC, O>;
#[doc = "Field `DATA_PRESENT_SEL` reader - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
pub type DATA_PRESENT_SEL_R = crate::BitReader;
#[doc = "Field `DATA_PRESENT_SEL` writer - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
pub type DATA_PRESENT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, CMD_R_SPEC, O>;
#[doc = "Field `CMD_TYPE` reader - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
pub type CMD_TYPE_R = crate::FieldReader;
#[doc = "Field `CMD_TYPE` writer - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
pub type CMD_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, CMD_R_SPEC, 2, O>;
#[doc = "Field `CMD_INDEX` reader - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
pub type CMD_INDEX_R = crate::FieldReader;
#[doc = "Field `CMD_INDEX` writer - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
pub type CMD_INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, CMD_R_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:1 - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
    #[inline(always)]
    pub fn resp_type_select(&self) -> RESP_TYPE_SELECT_R {
        RESP_TYPE_SELECT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
    #[inline(always)]
    pub fn sub_cmd_flag(&self) -> SUB_CMD_FLAG_R {
        SUB_CMD_FLAG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_crc_chk_enable(&self) -> CMD_CRC_CHK_ENABLE_R {
        CMD_CRC_CHK_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_idx_chk_enable(&self) -> CMD_IDX_CHK_ENABLE_R {
        CMD_IDX_CHK_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
    #[inline(always)]
    pub fn data_present_sel(&self) -> DATA_PRESENT_SEL_R {
        DATA_PRESENT_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CMD_TYPE_R {
        CMD_TYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
    #[inline(always)]
    #[must_use]
    pub fn resp_type_select(&mut self) -> RESP_TYPE_SELECT_W<0> {
        RESP_TYPE_SELECT_W::new(self)
    }
    #[doc = "Bit 2 - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
    #[inline(always)]
    #[must_use]
    pub fn sub_cmd_flag(&mut self) -> SUB_CMD_FLAG_W<2> {
        SUB_CMD_FLAG_W::new(self)
    }
    #[doc = "Bit 3 - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_chk_enable(&mut self) -> CMD_CRC_CHK_ENABLE_W<3> {
        CMD_CRC_CHK_ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx_chk_enable(&mut self) -> CMD_IDX_CHK_ENABLE_W<4> {
        CMD_IDX_CHK_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
    #[inline(always)]
    #[must_use]
    pub fn data_present_sel(&mut self) -> DATA_PRESENT_SEL_W<5> {
        DATA_PRESENT_SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CMD_TYPE_W<6> {
        CMD_TYPE_W::new(self)
    }
    #[doc = "Bits 8:13 - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CMD_INDEX_W<8> {
        CMD_INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_r](index.html) module"]
pub struct CMD_R_SPEC;
impl crate::RegisterSpec for CMD_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cmd_r::R](R) reader structure"]
impl crate::Readable for CMD_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_r::W](W) writer structure"]
impl crate::Writable for CMD_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_R to value 0"]
impl crate::Resettable for CMD_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
