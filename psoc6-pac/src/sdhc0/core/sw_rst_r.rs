#[doc = "Register `SW_RST_R` reader"]
pub struct R(crate::R<SW_RST_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_RST_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_RST_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_RST_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_RST_R` writer"]
pub struct W(crate::W<SW_RST_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_RST_R_SPEC>;
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
impl From<crate::W<SW_RST_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_RST_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_RST_ALL` reader - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_ALL_R = crate::BitReader;
#[doc = "Field `SW_RST_ALL` writer - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_ALL_W<'a, const O: u8> = crate::BitWriter<'a, SW_RST_R_SPEC, O>;
#[doc = "Field `SW_RST_CMD` reader - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_CMD_R = crate::BitReader;
#[doc = "Field `SW_RST_CMD` writer - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_CMD_W<'a, const O: u8> = crate::BitWriter<'a, SW_RST_R_SPEC, O>;
#[doc = "Field `SW_RST_DAT` reader - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_DAT_R = crate::BitReader;
#[doc = "Field `SW_RST_DAT` writer - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_DAT_W<'a, const O: u8> = crate::BitWriter<'a, SW_RST_R_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SW_RST_ALL_R {
        SW_RST_ALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_cmd(&self) -> SW_RST_CMD_R {
        SW_RST_CMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_dat(&self) -> SW_RST_DAT_R {
        SW_RST_DAT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_all(&mut self) -> SW_RST_ALL_W<0> {
        SW_RST_ALL_W::new(self)
    }
    #[doc = "Bit 1 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_cmd(&mut self) -> SW_RST_CMD_W<1> {
        SW_RST_CMD_W::new(self)
    }
    #[doc = "Bit 2 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_dat(&mut self) -> SW_RST_DAT_W<2> {
        SW_RST_DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_rst_r](index.html) module"]
pub struct SW_RST_R_SPEC;
impl crate::RegisterSpec for SW_RST_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sw_rst_r::R](R) reader structure"]
impl crate::Readable for SW_RST_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_rst_r::W](W) writer structure"]
impl crate::Writable for SW_RST_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_RST_R to value 0"]
impl crate::Resettable for SW_RST_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
