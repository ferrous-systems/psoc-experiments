#[doc = "Register `XFER_MODE_R` reader"]
pub struct R(crate::R<XFER_MODE_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XFER_MODE_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XFER_MODE_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XFER_MODE_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XFER_MODE_R` writer"]
pub struct W(crate::W<XFER_MODE_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XFER_MODE_R_SPEC>;
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
impl From<crate::W<XFER_MODE_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XFER_MODE_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_ENABLE` reader - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: - 0x1 (ENABLED): DMA Data transfer - 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
pub type DMA_ENABLE_R = crate::BitReader;
#[doc = "Field `DMA_ENABLE` writer - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: - 0x1 (ENABLED): DMA Data transfer - 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
pub type DMA_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, XFER_MODE_R_SPEC, O>;
#[doc = "Field `BLOCK_COUNT_ENABLE` reader - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. When 16-bit Block Count register is used, the Host Driver can set this bit to 0 in ADMA2 mode to enable larger data transfer than the maximum of 65535 block counts supported by the 16-bit Block Count register."]
pub type BLOCK_COUNT_ENABLE_R = crate::BitReader;
#[doc = "Field `BLOCK_COUNT_ENABLE` writer - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. When 16-bit Block Count register is used, the Host Driver can set this bit to 0 in ADMA2 mode to enable larger data transfer than the maximum of 65535 block counts supported by the 16-bit Block Count register."]
pub type BLOCK_COUNT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, XFER_MODE_R_SPEC, O>;
#[doc = "Field `AUTO_CMD_ENABLE` reader - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: - 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled - 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable - 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable - 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Select"]
pub type AUTO_CMD_ENABLE_R = crate::FieldReader;
#[doc = "Field `AUTO_CMD_ENABLE` writer - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: - 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled - 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable - 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable - 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Select"]
pub type AUTO_CMD_ENABLE_W<'a, const O: u8> = crate::FieldWriter<'a, XFER_MODE_R_SPEC, 2, O>;
#[doc = "Field `DATA_XFER_DIR` reader - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: - 0x1 (READ): Read (Card to Host) - 0x0 (WRITE): Write (Host to Card)"]
pub type DATA_XFER_DIR_R = crate::BitReader;
#[doc = "Field `DATA_XFER_DIR` writer - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: - 0x1 (READ): Read (Card to Host) - 0x0 (WRITE): Write (Host to Card)"]
pub type DATA_XFER_DIR_W<'a, const O: u8> = crate::BitWriter<'a, XFER_MODE_R_SPEC, O>;
#[doc = "Field `MULTI_BLK_SEL` reader - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register."]
pub type MULTI_BLK_SEL_R = crate::BitReader;
#[doc = "Field `MULTI_BLK_SEL` writer - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register."]
pub type MULTI_BLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, XFER_MODE_R_SPEC, O>;
#[doc = "Field `RESP_TYPE` reader - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: - OUT_OF_RANGE - ADDRESS_ERROR - BLOCK_LEN_ERROR - WP_VIOLATION - CARD_IS_LOCKED - COM_CRC_ERROR - CARD_ECC_FAILED - CC_ERROR - ERROR Response Flags checked in R5: - COM_CRC_ERROR - ERROR - FUNCTION_NUMBER - OUT_OF_RANGE Values: - 0x0 (RESP_R1): R1 (Memory) - 0x1 (RESP_R5): R5 (SDIO)"]
pub type RESP_TYPE_R = crate::BitReader;
#[doc = "Field `RESP_TYPE` writer - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: - OUT_OF_RANGE - ADDRESS_ERROR - BLOCK_LEN_ERROR - WP_VIOLATION - CARD_IS_LOCKED - COM_CRC_ERROR - CARD_ECC_FAILED - CC_ERROR - ERROR Response Flags checked in R5: - COM_CRC_ERROR - ERROR - FUNCTION_NUMBER - OUT_OF_RANGE Values: - 0x0 (RESP_R1): R1 (Memory) - 0x1 (RESP_R5): R5 (SDIO)"]
pub type RESP_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, XFER_MODE_R_SPEC, O>;
#[doc = "Field `RESP_ERR_CHK_ENABLE` reader - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
pub type RESP_ERR_CHK_ENABLE_R = crate::BitReader;
#[doc = "Field `RESP_ERR_CHK_ENABLE` writer - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
pub type RESP_ERR_CHK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, XFER_MODE_R_SPEC, O>;
#[doc = "Field `RESP_INT_DISABLE` reader - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
pub type RESP_INT_DISABLE_R = crate::BitReader;
#[doc = "Field `RESP_INT_DISABLE` writer - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
pub type RESP_INT_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, XFER_MODE_R_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: - 0x1 (ENABLED): DMA Data transfer - 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
    #[inline(always)]
    pub fn dma_enable(&self) -> DMA_ENABLE_R {
        DMA_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. When 16-bit Block Count register is used, the Host Driver can set this bit to 0 in ADMA2 mode to enable larger data transfer than the maximum of 65535 block counts supported by the 16-bit Block Count register."]
    #[inline(always)]
    pub fn block_count_enable(&self) -> BLOCK_COUNT_ENABLE_R {
        BLOCK_COUNT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: - 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled - 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable - 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable - 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Select"]
    #[inline(always)]
    pub fn auto_cmd_enable(&self) -> AUTO_CMD_ENABLE_R {
        AUTO_CMD_ENABLE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: - 0x1 (READ): Read (Card to Host) - 0x0 (WRITE): Write (Host to Card)"]
    #[inline(always)]
    pub fn data_xfer_dir(&self) -> DATA_XFER_DIR_R {
        DATA_XFER_DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register."]
    #[inline(always)]
    pub fn multi_blk_sel(&self) -> MULTI_BLK_SEL_R {
        MULTI_BLK_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: - OUT_OF_RANGE - ADDRESS_ERROR - BLOCK_LEN_ERROR - WP_VIOLATION - CARD_IS_LOCKED - COM_CRC_ERROR - CARD_ECC_FAILED - CC_ERROR - ERROR Response Flags checked in R5: - COM_CRC_ERROR - ERROR - FUNCTION_NUMBER - OUT_OF_RANGE Values: - 0x0 (RESP_R1): R1 (Memory) - 0x1 (RESP_R5): R5 (SDIO)"]
    #[inline(always)]
    pub fn resp_type(&self) -> RESP_TYPE_R {
        RESP_TYPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
    #[inline(always)]
    pub fn resp_err_chk_enable(&self) -> RESP_ERR_CHK_ENABLE_R {
        RESP_ERR_CHK_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
    #[inline(always)]
    pub fn resp_int_disable(&self) -> RESP_INT_DISABLE_R {
        RESP_INT_DISABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: - 0x1 (ENABLED): DMA Data transfer - 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dma_enable(&mut self) -> DMA_ENABLE_W<0> {
        DMA_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. When 16-bit Block Count register is used, the Host Driver can set this bit to 0 in ADMA2 mode to enable larger data transfer than the maximum of 65535 block counts supported by the 16-bit Block Count register."]
    #[inline(always)]
    #[must_use]
    pub fn block_count_enable(&mut self) -> BLOCK_COUNT_ENABLE_W<1> {
        BLOCK_COUNT_ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: - 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled - 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable - 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable - 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Select"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_enable(&mut self) -> AUTO_CMD_ENABLE_W<2> {
        AUTO_CMD_ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: - 0x1 (READ): Read (Card to Host) - 0x0 (WRITE): Write (Host to Card)"]
    #[inline(always)]
    #[must_use]
    pub fn data_xfer_dir(&mut self) -> DATA_XFER_DIR_W<4> {
        DATA_XFER_DIR_W::new(self)
    }
    #[doc = "Bit 5 - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register."]
    #[inline(always)]
    #[must_use]
    pub fn multi_blk_sel(&mut self) -> MULTI_BLK_SEL_W<5> {
        MULTI_BLK_SEL_W::new(self)
    }
    #[doc = "Bit 6 - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: - OUT_OF_RANGE - ADDRESS_ERROR - BLOCK_LEN_ERROR - WP_VIOLATION - CARD_IS_LOCKED - COM_CRC_ERROR - CARD_ECC_FAILED - CC_ERROR - ERROR Response Flags checked in R5: - COM_CRC_ERROR - ERROR - FUNCTION_NUMBER - OUT_OF_RANGE Values: - 0x0 (RESP_R1): R1 (Memory) - 0x1 (RESP_R5): R5 (SDIO)"]
    #[inline(always)]
    #[must_use]
    pub fn resp_type(&mut self) -> RESP_TYPE_W<6> {
        RESP_TYPE_W::new(self)
    }
    #[doc = "Bit 7 - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_chk_enable(&mut self) -> RESP_ERR_CHK_ENABLE_W<7> {
        RESP_ERR_CHK_ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn resp_int_disable(&mut self) -> RESP_INT_DISABLE_W<8> {
        RESP_INT_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xfer_mode_r](index.html) module"]
pub struct XFER_MODE_R_SPEC;
impl crate::RegisterSpec for XFER_MODE_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xfer_mode_r::R](R) reader structure"]
impl crate::Readable for XFER_MODE_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xfer_mode_r::W](W) writer structure"]
impl crate::Writable for XFER_MODE_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XFER_MODE_R to value 0"]
impl crate::Resettable for XFER_MODE_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
