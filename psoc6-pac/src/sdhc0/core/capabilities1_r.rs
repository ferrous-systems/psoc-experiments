#[doc = "Register `CAPABILITIES1_R` reader"]
pub struct R(crate::R<CAPABILITIES1_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPABILITIES1_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPABILITIES1_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPABILITIES1_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUT_CLK_FREQ` reader - Timeout Clock Frequency This bit shows the base clock frequency used to detect Data Timeout Error. The Timeout Clock unit defines the unit of timeout clock frequency. It can be KHz or MHz. - 0x00 - Get information through another method - 0x01 - 1KHz / 1MHz - 0x02 - 2KHz / 2MHz - 0x03 - 3KHz / 3MHz - ........... - 0x3F - 63KHz / 63MHz"]
pub type TOUT_CLK_FREQ_R = crate::FieldReader;
#[doc = "Field `TOUT_CLK_UNIT` reader - Timeout Clock Unit This bit shows the unit of base clock frequency used to detect Data TImeout Error. Values: - 0x0 (KHZ): KHz - 0x1 (MHZ): MHz"]
pub type TOUT_CLK_UNIT_R = crate::BitReader;
#[doc = "Field `BASE_CLK_FREQ` reader - Base Clock Frequency for SD clock These bits indicate the base (maximum) clock frequency for the SD Clock. The definition of these bits depend on the Host Controller Version. - 6-Bit Base Clock Frequency: This mode is supported by the Host Controller version 1.00 and 2.00. The upper 2 bits are not effective and are always 0. The unit values are 1 MHz. The supported clock range is 10 MHz to 63 MHz. - 0x00 - Get information through another method - 0x01 - 1 MHz - 0x02 - 2 MHz - ............. - 0x3F - 63 MHz - 0x40-0xFF - Not Supported - 8-Bit Base Clock Frequency: This mode is supported by the Host Controller version 3.00. The unit values are 1 MHz. The supported clock range is 10 MHz to 255 MHz. - 0x00 - Get information through another method - 0x01 - 1 MHz - 0x02 - 2 MHz - ............ - 0xFF - 255 MHz If the frequency is 16.5 MHz, the larger value is set to 0001001b (17 MHz) because the Host Driver uses this value to calculate the clock divider value and it does not exceed the upper limit of the SD Clock frequency. If these bits are all 0, the Host system has to get information using a different method."]
pub type BASE_CLK_FREQ_R = crate::FieldReader;
#[doc = "Field `MAX_BLK_LEN` reader - N/A"]
pub type MAX_BLK_LEN_R = crate::FieldReader;
#[doc = "Field `EMBEDDED_8_BIT` reader - 8-bit Support for Embedded Device This bit indicates whether the Host Controller is capable of using an 8-bit bus width mode. This bit is not effective when the Slot Type is set to 10b. Values: - 0x0 (FALSE): 8-bit Bus Width not Supported - 0x1 (TRUE): 8-bit Bus Width Supported"]
pub type EMBEDDED_8_BIT_R = crate::BitReader;
#[doc = "Field `ADMA2_SUPPORT` reader - ADMA2 Support This bit indicates whether the Host Controller is capable of using ADMA2. Values: - 0x0 (FALSE): ADMA2 not Supported - 0x1 (TRUE): ADMA2 Supported"]
pub type ADMA2_SUPPORT_R = crate::BitReader;
#[doc = "Field `HIGH_SPEED_SUPPORT` reader - High Speed Support This bit indicates whether the Host Controller and the Host System supports High Speed mode and they can supply the SD Clock frequency from 25 MHz to 50 MHz. Values: - 0x0 (FALSE): High Speed not Supported - 0x1 (TRUE): High Speed Supported"]
pub type HIGH_SPEED_SUPPORT_R = crate::BitReader;
#[doc = "Field `SDMA_SUPPORT` reader - SDMA Support This bit indicates whether the Host Controller is capable of using SDMA to transfer data between the system memory and the Host Controller directly. Values: - 0x0 (FALSE): SDMA not Supported - 0x1 (TRUE): SDMA Supported"]
pub type SDMA_SUPPORT_R = crate::BitReader;
#[doc = "Field `SUS_RES_SUPPORT` reader - Suspense/Resume Support This bit indicates whether the Host Controller supports Suspend/Resume functionality. If this bit is 0, the Host Driver does not issue either Suspend or Resume commands because the Suspend and Resume mechanism is not supported. Values: - 0x0 (FALSE): Not Supported - 0x1 (TRUE): Supported"]
pub type SUS_RES_SUPPORT_R = crate::BitReader;
#[doc = "Field `VOLT_33` reader - Voltage Support 3.3V Values: - 0x0 (FALSE): 3.3V Not Supported - 0x1 (TRUE): 3.3V Supported"]
pub type VOLT_33_R = crate::BitReader;
#[doc = "Field `VOLT_30` reader - Voltage Support 3.0V Values: - 0x0 (FALSE): 3.0V Not Supported - 0x1 (TRUE): 3.0V Supported"]
pub type VOLT_30_R = crate::BitReader;
#[doc = "Field `VOLT_18` reader - Voltage Support 1.8V Values: - 0x0 (FALSE): 1.8V Not Supported - 0x1 (TRUE): 1.8V Supported"]
pub type VOLT_18_R = crate::BitReader;
#[doc = "Field `SYS_ADDR_64_V4` reader - 64-bit System Address Support for V4 This bit sets the Host Controller to support 64-bit System Addressing of V4 mode. When this bit is set to 1, full or part of 64-bit address must be used to decode the Host Controller Registers so that Host Controller Registers can be placed above system memory area. 64-bit address decode of Host Controller registers is effective regardless of setting to 64-bit Addressing in Host Control 2. If this bit is set to 1, 64-bit DMA Addressing for version 4 is enabled by setting Host Version 4 Enable (HOST_VER4_ENABLE = 1) and by setting 64-bit Addressing (ADDRESSING =1) in the Host Control 2 register. SDMA can be used and ADMA2 uses 128-bit Descriptor. Values: - 0x0 (FALSE): 64-bit System Address for V4 is Not Supported - 0x1 (TRUE): 64-bit System Address for V4 is Supported"]
pub type SYS_ADDR_64_V4_R = crate::BitReader;
#[doc = "Field `SYS_ADDR_64_V3` reader - 64-bit System Address Support for V3 This bit sets the Host controller to support 64-bit System Addressing of V3 mode. SDMA cannot be used in 64-bit Addressing in Version 3 Mode. If this bit is set to 1, 64-bit ADMA2 with using 96-bit Descriptor can be enabled by setting Host Version 4 Enable (HOST_VER4_ENABLE = 0) and DMA select (DMA_SEL = 11b). Values: - 0x0 (FALSE): 64-bit System Address for V3 is Not Supported - 0x1 (TRUE): 64-bit System Address for V3 is Supported"]
pub type SYS_ADDR_64_V3_R = crate::BitReader;
#[doc = "Field `ASYNC_INT_SUPPORT` reader - Asynchronous Interrupt Support (SD Mode only) Values: - 0x0 (FALSE): Asynchronous Interrupt Not Supported - 0x1 (TRUE): Asynchronous Interrupt Supported"]
pub type ASYNC_INT_SUPPORT_R = crate::BitReader;
#[doc = "Field `SLOT_TYPE_R` reader - Slot Type These bits indicate usage of a slot by a specific Host System. Values: - 0x0 (REMOVABLE_SLOT): Removable Card Slot - 0x1 (EMBEDDED_SLOT): Embedded Slot for one Device - 0x2 (SHARED_SLOT): Shared Bus Slot (SD mode) - 0x3 (UHS2_EMBEDDED_SLOT): UHS-II Multiple Embedded Devices"]
pub type SLOT_TYPE_R_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency This bit shows the base clock frequency used to detect Data Timeout Error. The Timeout Clock unit defines the unit of timeout clock frequency. It can be KHz or MHz. - 0x00 - Get information through another method - 0x01 - 1KHz / 1MHz - 0x02 - 2KHz / 2MHz - 0x03 - 3KHz / 3MHz - ........... - 0x3F - 63KHz / 63MHz"]
    #[inline(always)]
    pub fn tout_clk_freq(&self) -> TOUT_CLK_FREQ_R {
        TOUT_CLK_FREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit This bit shows the unit of base clock frequency used to detect Data TImeout Error. Values: - 0x0 (KHZ): KHz - 0x1 (MHZ): MHz"]
    #[inline(always)]
    pub fn tout_clk_unit(&self) -> TOUT_CLK_UNIT_R {
        TOUT_CLK_UNIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD clock These bits indicate the base (maximum) clock frequency for the SD Clock. The definition of these bits depend on the Host Controller Version. - 6-Bit Base Clock Frequency: This mode is supported by the Host Controller version 1.00 and 2.00. The upper 2 bits are not effective and are always 0. The unit values are 1 MHz. The supported clock range is 10 MHz to 63 MHz. - 0x00 - Get information through another method - 0x01 - 1 MHz - 0x02 - 2 MHz - ............. - 0x3F - 63 MHz - 0x40-0xFF - Not Supported - 8-Bit Base Clock Frequency: This mode is supported by the Host Controller version 3.00. The unit values are 1 MHz. The supported clock range is 10 MHz to 255 MHz. - 0x00 - Get information through another method - 0x01 - 1 MHz - 0x02 - 2 MHz - ............ - 0xFF - 255 MHz If the frequency is 16.5 MHz, the larger value is set to 0001001b (17 MHz) because the Host Driver uses this value to calculate the clock divider value and it does not exceed the upper limit of the SD Clock frequency. If these bits are all 0, the Host system has to get information using a different method."]
    #[inline(always)]
    pub fn base_clk_freq(&self) -> BASE_CLK_FREQ_R {
        BASE_CLK_FREQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn max_blk_len(&self) -> MAX_BLK_LEN_R {
        MAX_BLK_LEN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 8-bit Support for Embedded Device This bit indicates whether the Host Controller is capable of using an 8-bit bus width mode. This bit is not effective when the Slot Type is set to 10b. Values: - 0x0 (FALSE): 8-bit Bus Width not Supported - 0x1 (TRUE): 8-bit Bus Width Supported"]
    #[inline(always)]
    pub fn embedded_8_bit(&self) -> EMBEDDED_8_BIT_R {
        EMBEDDED_8_BIT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support This bit indicates whether the Host Controller is capable of using ADMA2. Values: - 0x0 (FALSE): ADMA2 not Supported - 0x1 (TRUE): ADMA2 Supported"]
    #[inline(always)]
    pub fn adma2_support(&self) -> ADMA2_SUPPORT_R {
        ADMA2_SUPPORT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support This bit indicates whether the Host Controller and the Host System supports High Speed mode and they can supply the SD Clock frequency from 25 MHz to 50 MHz. Values: - 0x0 (FALSE): High Speed not Supported - 0x1 (TRUE): High Speed Supported"]
    #[inline(always)]
    pub fn high_speed_support(&self) -> HIGH_SPEED_SUPPORT_R {
        HIGH_SPEED_SUPPORT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMA Support This bit indicates whether the Host Controller is capable of using SDMA to transfer data between the system memory and the Host Controller directly. Values: - 0x0 (FALSE): SDMA not Supported - 0x1 (TRUE): SDMA Supported"]
    #[inline(always)]
    pub fn sdma_support(&self) -> SDMA_SUPPORT_R {
        SDMA_SUPPORT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspense/Resume Support This bit indicates whether the Host Controller supports Suspend/Resume functionality. If this bit is 0, the Host Driver does not issue either Suspend or Resume commands because the Suspend and Resume mechanism is not supported. Values: - 0x0 (FALSE): Not Supported - 0x1 (TRUE): Supported"]
    #[inline(always)]
    pub fn sus_res_support(&self) -> SUS_RES_SUPPORT_R {
        SUS_RES_SUPPORT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V Values: - 0x0 (FALSE): 3.3V Not Supported - 0x1 (TRUE): 3.3V Supported"]
    #[inline(always)]
    pub fn volt_33(&self) -> VOLT_33_R {
        VOLT_33_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V Values: - 0x0 (FALSE): 3.0V Not Supported - 0x1 (TRUE): 3.0V Supported"]
    #[inline(always)]
    pub fn volt_30(&self) -> VOLT_30_R {
        VOLT_30_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V Values: - 0x0 (FALSE): 1.8V Not Supported - 0x1 (TRUE): 1.8V Supported"]
    #[inline(always)]
    pub fn volt_18(&self) -> VOLT_18_R {
        VOLT_18_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 64-bit System Address Support for V4 This bit sets the Host Controller to support 64-bit System Addressing of V4 mode. When this bit is set to 1, full or part of 64-bit address must be used to decode the Host Controller Registers so that Host Controller Registers can be placed above system memory area. 64-bit address decode of Host Controller registers is effective regardless of setting to 64-bit Addressing in Host Control 2. If this bit is set to 1, 64-bit DMA Addressing for version 4 is enabled by setting Host Version 4 Enable (HOST_VER4_ENABLE = 1) and by setting 64-bit Addressing (ADDRESSING =1) in the Host Control 2 register. SDMA can be used and ADMA2 uses 128-bit Descriptor. Values: - 0x0 (FALSE): 64-bit System Address for V4 is Not Supported - 0x1 (TRUE): 64-bit System Address for V4 is Supported"]
    #[inline(always)]
    pub fn sys_addr_64_v4(&self) -> SYS_ADDR_64_V4_R {
        SYS_ADDR_64_V4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 64-bit System Address Support for V3 This bit sets the Host controller to support 64-bit System Addressing of V3 mode. SDMA cannot be used in 64-bit Addressing in Version 3 Mode. If this bit is set to 1, 64-bit ADMA2 with using 96-bit Descriptor can be enabled by setting Host Version 4 Enable (HOST_VER4_ENABLE = 0) and DMA select (DMA_SEL = 11b). Values: - 0x0 (FALSE): 64-bit System Address for V3 is Not Supported - 0x1 (TRUE): 64-bit System Address for V3 is Supported"]
    #[inline(always)]
    pub fn sys_addr_64_v3(&self) -> SYS_ADDR_64_V3_R {
        SYS_ADDR_64_V3_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support (SD Mode only) Values: - 0x0 (FALSE): Asynchronous Interrupt Not Supported - 0x1 (TRUE): Asynchronous Interrupt Supported"]
    #[inline(always)]
    pub fn async_int_support(&self) -> ASYNC_INT_SUPPORT_R {
        ASYNC_INT_SUPPORT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Slot Type These bits indicate usage of a slot by a specific Host System. Values: - 0x0 (REMOVABLE_SLOT): Removable Card Slot - 0x1 (EMBEDDED_SLOT): Embedded Slot for one Device - 0x2 (SHARED_SLOT): Shared Bus Slot (SD mode) - 0x3 (UHS2_EMBEDDED_SLOT): UHS-II Multiple Embedded Devices"]
    #[inline(always)]
    pub fn slot_type_r(&self) -> SLOT_TYPE_R_R {
        SLOT_TYPE_R_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Capabilities 1 Register - 0 to 31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capabilities1_r](index.html) module"]
pub struct CAPABILITIES1_R_SPEC;
impl crate::RegisterSpec for CAPABILITIES1_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capabilities1_r::R](R) reader structure"]
impl crate::Readable for CAPABILITIES1_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPABILITIES1_R to value 0x276c_6481"]
impl crate::Resettable for CAPABILITIES1_R_SPEC {
    const RESET_VALUE: Self::Ux = 0x276c_6481;
}
