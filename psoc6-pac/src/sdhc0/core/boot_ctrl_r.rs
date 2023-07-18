#[doc = "Register `BOOT_CTRL_R` reader"]
pub struct R(crate::R<BOOT_CTRL_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_CTRL_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_CTRL_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_CTRL_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOT_CTRL_R` writer"]
pub struct W(crate::W<BOOT_CTRL_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT_CTRL_R_SPEC>;
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
impl From<crate::W<BOOT_CTRL_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT_CTRL_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAN_BOOT_EN` reader - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
pub type MAN_BOOT_EN_R = crate::BitReader;
#[doc = "Field `MAN_BOOT_EN` writer - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
pub type MAN_BOOT_EN_W<'a, const O: u8> = crate::BitWriter<'a, BOOT_CTRL_R_SPEC, O>;
#[doc = "Field `VALIDATE_BOOT` writer - Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: - 0x1 (TRUE): Validate Mandatory boot enable bit - 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
pub type VALIDATE_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, BOOT_CTRL_R_SPEC, O>;
#[doc = "Field `BOOT_ACK_ENABLE` reader - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
pub type BOOT_ACK_ENABLE_R = crate::BitReader;
#[doc = "Field `BOOT_ACK_ENABLE` writer - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
pub type BOOT_ACK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, BOOT_CTRL_R_SPEC, O>;
#[doc = "Field `BOOT_TOUT_CNT` reader - N/A"]
pub type BOOT_TOUT_CNT_R = crate::FieldReader;
#[doc = "Field `BOOT_TOUT_CNT` writer - N/A"]
pub type BOOT_TOUT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, BOOT_CTRL_R_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    pub fn man_boot_en(&self) -> MAN_BOOT_EN_R {
        MAN_BOOT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    pub fn boot_ack_enable(&self) -> BOOT_ACK_ENABLE_R {
        BOOT_ACK_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    pub fn boot_tout_cnt(&self) -> BOOT_TOUT_CNT_R {
        BOOT_TOUT_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    #[must_use]
    pub fn man_boot_en(&mut self) -> MAN_BOOT_EN_W<0> {
        MAN_BOOT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: - 0x1 (TRUE): Validate Mandatory boot enable bit - 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn validate_boot(&mut self) -> VALIDATE_BOOT_W<7> {
        VALIDATE_BOOT_W::new(self)
    }
    #[doc = "Bit 8 - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_enable(&mut self) -> BOOT_ACK_ENABLE_W<8> {
        BOOT_ACK_ENABLE_W::new(self)
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn boot_tout_cnt(&mut self) -> BOOT_TOUT_CNT_W<12> {
        BOOT_TOUT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eMMC Boot Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_ctrl_r](index.html) module"]
pub struct BOOT_CTRL_R_SPEC;
impl crate::RegisterSpec for BOOT_CTRL_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [boot_ctrl_r::R](R) reader structure"]
impl crate::Readable for BOOT_CTRL_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boot_ctrl_r::W](W) writer structure"]
impl crate::Writable for BOOT_CTRL_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOT_CTRL_R to value 0"]
impl crate::Resettable for BOOT_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
