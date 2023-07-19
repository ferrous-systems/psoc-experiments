#[doc = "Register `GP_OUT_R` reader"]
pub struct R(crate::R<GP_OUT_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_OUT_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_OUT_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_OUT_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_OUT_R` writer"]
pub struct W(crate::W<GP_OUT_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_OUT_R_SPEC>;
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
impl From<crate::W<GP_OUT_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_OUT_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_DETECT_EN` reader - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
pub type CARD_DETECT_EN_R = crate::BitReader;
#[doc = "Field `CARD_DETECT_EN` writer - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
pub type CARD_DETECT_EN_W<'a, const O: u8> = crate::BitWriter<'a, GP_OUT_R_SPEC, O>;
#[doc = "Field `CARD_MECH_WRITE_PROT_EN` reader - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
pub type CARD_MECH_WRITE_PROT_EN_R = crate::BitReader;
#[doc = "Field `CARD_MECH_WRITE_PROT_EN` writer - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
pub type CARD_MECH_WRITE_PROT_EN_W<'a, const O: u8> = crate::BitWriter<'a, GP_OUT_R_SPEC, O>;
#[doc = "Field `LED_CTRL_OE` reader - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
pub type LED_CTRL_OE_R = crate::BitReader;
#[doc = "Field `LED_CTRL_OE` writer - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
pub type LED_CTRL_OE_W<'a, const O: u8> = crate::BitWriter<'a, GP_OUT_R_SPEC, O>;
#[doc = "Field `CARD_CLOCK_OE` reader - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
pub type CARD_CLOCK_OE_R = crate::BitReader;
#[doc = "Field `CARD_CLOCK_OE` writer - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
pub type CARD_CLOCK_OE_W<'a, const O: u8> = crate::BitWriter<'a, GP_OUT_R_SPEC, O>;
#[doc = "Field `CARD_IF_PWR_EN_OE` reader - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
pub type CARD_IF_PWR_EN_OE_R = crate::BitReader;
#[doc = "Field `CARD_IF_PWR_EN_OE` writer - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
pub type CARD_IF_PWR_EN_OE_W<'a, const O: u8> = crate::BitWriter<'a, GP_OUT_R_SPEC, O>;
#[doc = "Field `IO_VOLT_SEL_OE` reader - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
pub type IO_VOLT_SEL_OE_R = crate::BitReader;
#[doc = "Field `IO_VOLT_SEL_OE` writer - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
pub type IO_VOLT_SEL_OE_W<'a, const O: u8> = crate::BitWriter<'a, GP_OUT_R_SPEC, O>;
#[doc = "Field `CARD_CLOCK_OUT_DLY` reader - N/A"]
pub type CARD_CLOCK_OUT_DLY_R = crate::FieldReader;
#[doc = "Field `CARD_CLOCK_OUT_DLY` writer - N/A"]
pub type CARD_CLOCK_OUT_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, GP_OUT_R_SPEC, 2, O>;
#[doc = "Field `CARD_CLOCK_IN_DLY` reader - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
pub type CARD_CLOCK_IN_DLY_R = crate::FieldReader;
#[doc = "Field `CARD_CLOCK_IN_DLY` writer - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
pub type CARD_CLOCK_IN_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, GP_OUT_R_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
    #[inline(always)]
    pub fn card_detect_en(&self) -> CARD_DETECT_EN_R {
        CARD_DETECT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
    #[inline(always)]
    pub fn card_mech_write_prot_en(&self) -> CARD_MECH_WRITE_PROT_EN_R {
        CARD_MECH_WRITE_PROT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
    #[inline(always)]
    pub fn led_ctrl_oe(&self) -> LED_CTRL_OE_R {
        LED_CTRL_OE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
    #[inline(always)]
    pub fn card_clock_oe(&self) -> CARD_CLOCK_OE_R {
        CARD_CLOCK_OE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
    #[inline(always)]
    pub fn card_if_pwr_en_oe(&self) -> CARD_IF_PWR_EN_OE_R {
        CARD_IF_PWR_EN_OE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
    #[inline(always)]
    pub fn io_volt_sel_oe(&self) -> IO_VOLT_SEL_OE_R {
        IO_VOLT_SEL_OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn card_clock_out_dly(&self) -> CARD_CLOCK_OUT_DLY_R {
        CARD_CLOCK_OUT_DLY_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
    #[inline(always)]
    pub fn card_clock_in_dly(&self) -> CARD_CLOCK_IN_DLY_R {
        CARD_CLOCK_IN_DLY_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Force card_detect_n input to 0 1: Normal card_detect_n operation allowing card detection from a device pin"]
    #[inline(always)]
    #[must_use]
    pub fn card_detect_en(&mut self) -> CARD_DETECT_EN_W<0> {
        CARD_DETECT_EN_W::new(self)
    }
    #[doc = "Bit 1 - card_mech_write_prot, despite its name, is an active low signal (per the SD Host Controller Standard spec it is officially called SDWP#). Consider that in the following: 0: Force card_mech_write_prot input to 0 internally; this forces write protection to be active 1: Allow card_mech_write_prot to work normally per the device's pin state"]
    #[inline(always)]
    #[must_use]
    pub fn card_mech_write_prot_en(&mut self) -> CARD_MECH_WRITE_PROT_EN_W<1> {
        CARD_MECH_WRITE_PROT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Active high output enable for the LED output signal (led_ctrl) controlled through HOST_CTRL1_R.LED_CTRL: 0: disable OE associated with the led_ctrl output 1: enable OE associated with the led_ctrl output"]
    #[inline(always)]
    #[must_use]
    pub fn led_ctrl_oe(&mut self) -> LED_CTRL_OE_W<2> {
        LED_CTRL_OE_W::new(self)
    }
    #[doc = "Bit 3 - Active high output enable for the card clock output (clk_card) which is gated by CLK_CTRL_R.SD_CLK_EN: 0: disable OE to the clk_card output 1: enable OE to the clk_card output"]
    #[inline(always)]
    #[must_use]
    pub fn card_clock_oe(&mut self) -> CARD_CLOCK_OE_W<3> {
        CARD_CLOCK_OE_W::new(self)
    }
    #[doc = "Bit 4 - Active high output enable for the card interface power enable output (card_if_pwr_en) controlled through PWR_CTRL_R.SD_BUS_PWR_VDD1: 0: disable OE to the card_if_pwr_en output 1: enable OE to the card_if_pwr_en output"]
    #[inline(always)]
    #[must_use]
    pub fn card_if_pwr_en_oe(&mut self) -> CARD_IF_PWR_EN_OE_W<4> {
        CARD_IF_PWR_EN_OE_W::new(self)
    }
    #[doc = "Bit 5 - Active high output enable for the IO voltage selection signal (io_volt_sel) controlled through HOST_CTRL_2.SIGNALING_EN: 0: disable OE to the io_volt_sel output 1: enable OE to the io_volt_sel output"]
    #[inline(always)]
    #[must_use]
    pub fn io_volt_sel_oe(&mut self) -> IO_VOLT_SEL_OE_W<5> {
        IO_VOLT_SEL_OE_W::new(self)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn card_clock_out_dly(&mut self) -> CARD_CLOCK_OUT_DLY_W<6> {
        CARD_CLOCK_OUT_DLY_W::new(self)
    }
    #[doc = "Bits 8:9 - Delay CARD_CLOCK input internally to optimally sample CMD/DAT; set according to interface mode: 00: SD Default Speed, SD SDR12, eMMC Legacy 01: SD SDR25, SD SDR50 10: SD High Speed, eMMC High Speed SDR 11: SD DDR50, eMMC DDR"]
    #[inline(always)]
    #[must_use]
    pub fn card_clock_in_dly(&mut self) -> CARD_CLOCK_IN_DLY_W<8> {
        CARD_CLOCK_IN_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_out_r](index.html) module"]
pub struct GP_OUT_R_SPEC;
impl crate::RegisterSpec for GP_OUT_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_out_r::R](R) reader structure"]
impl crate::Readable for GP_OUT_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_out_r::W](W) writer structure"]
impl crate::Writable for GP_OUT_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GP_OUT_R to value 0"]
impl crate::Resettable for GP_OUT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
