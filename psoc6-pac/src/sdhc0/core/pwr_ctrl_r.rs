#[doc = "Register `PWR_CTRL_R` reader"]
pub struct R(crate::R<PWR_CTRL_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRL_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRL_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRL_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTRL_R` writer"]
pub struct W(crate::W<PWR_CTRL_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTRL_R_SPEC>;
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
impl From<crate::W<PWR_CTRL_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTRL_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD_BUS_PWR_VDD1` reader - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
pub type SD_BUS_PWR_VDD1_R = crate::BitReader;
#[doc = "Field `SD_BUS_PWR_VDD1` writer - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
pub type SD_BUS_PWR_VDD1_W<'a, const O: u8> = crate::BitWriter<'a, PWR_CTRL_R_SPEC, O>;
#[doc = "Field `SD_BUS_VOL_VDD1` reader - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
pub type SD_BUS_VOL_VDD1_R = crate::FieldReader;
#[doc = "Field `SD_BUS_VOL_VDD1` writer - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
pub type SD_BUS_VOL_VDD1_W<'a, const O: u8> = crate::FieldWriter<'a, PWR_CTRL_R_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
    #[inline(always)]
    pub fn sd_bus_pwr_vdd1(&self) -> SD_BUS_PWR_VDD1_R {
        SD_BUS_PWR_VDD1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
    #[inline(always)]
    pub fn sd_bus_vol_vdd1(&self) -> SD_BUS_VOL_VDD1_R {
        SD_BUS_VOL_VDD1_R::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power for VDD1 This bit enables VDD1 power of the card. This setting is available on the card_if_pwr_en output so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared. In SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register. Values: - 0x0 (OFF): Power off - 0x1 (ON): Power on"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_pwr_vdd1(&mut self) -> SD_BUS_PWR_VDD1_W<0> {
        SD_BUS_PWR_VDD1_W::new(self)
    }
    #[doc = "Bits 1:3 - These bits are NON-operational (they can be written and read but they have no effect). In a generic HCI host these would select the card supply voltage. But, for the applications targeted for this block it is assumed that the card supply voltage is always fixed at the board level. If for some reason there is a variable power supply then that can be managed through normal GPIO programming separately."]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_vol_vdd1(&mut self) -> SD_BUS_VOL_VDD1_W<1> {
        SD_BUS_VOL_VDD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl_r](index.html) module"]
pub struct PWR_CTRL_R_SPEC;
impl crate::RegisterSpec for PWR_CTRL_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pwr_ctrl_r::R](R) reader structure"]
impl crate::Readable for PWR_CTRL_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctrl_r::W](W) writer structure"]
impl crate::Writable for PWR_CTRL_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CTRL_R to value 0"]
impl crate::Resettable for PWR_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
