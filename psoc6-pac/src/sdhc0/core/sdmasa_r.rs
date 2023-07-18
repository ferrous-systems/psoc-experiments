#[doc = "Register `SDMASA_R` reader"]
pub struct R(crate::R<SDMASA_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMASA_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMASA_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMASA_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMASA_R` writer"]
pub struct W(crate::W<SDMASA_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMASA_R_SPEC>;
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
impl From<crate::W<SDMASA_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMASA_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCKCNT_SDMASA` reader - 32-bit Block Count (SDMA System Address) - SDMA System Address (Host Version 4 Enable = 0): This register contains the system memory address for an SDMA transfer in the 32-bit addressing mode. When the Host Controller stops an SDMA transfer, this register points to the system address of the next contiguous data position. It can be accessed only if no transaction is executing. Reading this register during data transfers may return an invalid value. - 32-bit Block Count (Host Version 4 Enable = 1): From the Host Controller Version 4.10 specification, this register is redefined as 32-bit Block Count. The Host Controller decrements the block count of this register for every block transfer and the data transfer stops when the count reaches zero. This register must be accessed when no transaction is executing. Reading this register during data transfers may return invalid value. Following are the values for BLOCKCNT_SDMASA: - 0xFFFF_FFFF - 4G - 1 Block - ...... - 0x0000_0002 - 2 Blocks - 0x0000_0001 - 1 Block - 0x0000_0000 - Stop Count Note: - When Host Version 4 Enable = 0, SDMA uses this register as system address and hence Auto CMD23 cannot be used with SDMA since this register is assigned for Auto CMD23 as 32-bit Block Count register. -When Host Version 4 Enable = 1, SDMA uses ADMA system address register and this register is reassigned to 32-bit Block Count. This register must be programmed with a non-zero value for data transfer if the 32-bit Block count register is used instead of the 16-bit Block count register. SDMA may use Auto CMD23 if 32-bit Block Count register is used."]
pub type BLOCKCNT_SDMASA_R = crate::FieldReader<u32>;
#[doc = "Field `BLOCKCNT_SDMASA` writer - 32-bit Block Count (SDMA System Address) - SDMA System Address (Host Version 4 Enable = 0): This register contains the system memory address for an SDMA transfer in the 32-bit addressing mode. When the Host Controller stops an SDMA transfer, this register points to the system address of the next contiguous data position. It can be accessed only if no transaction is executing. Reading this register during data transfers may return an invalid value. - 32-bit Block Count (Host Version 4 Enable = 1): From the Host Controller Version 4.10 specification, this register is redefined as 32-bit Block Count. The Host Controller decrements the block count of this register for every block transfer and the data transfer stops when the count reaches zero. This register must be accessed when no transaction is executing. Reading this register during data transfers may return invalid value. Following are the values for BLOCKCNT_SDMASA: - 0xFFFF_FFFF - 4G - 1 Block - ...... - 0x0000_0002 - 2 Blocks - 0x0000_0001 - 1 Block - 0x0000_0000 - Stop Count Note: - When Host Version 4 Enable = 0, SDMA uses this register as system address and hence Auto CMD23 cannot be used with SDMA since this register is assigned for Auto CMD23 as 32-bit Block Count register. -When Host Version 4 Enable = 1, SDMA uses ADMA system address register and this register is reassigned to 32-bit Block Count. This register must be programmed with a non-zero value for data transfer if the 32-bit Block count register is used instead of the 16-bit Block count register. SDMA may use Auto CMD23 if 32-bit Block Count register is used."]
pub type BLOCKCNT_SDMASA_W<'a, const O: u8> = crate::FieldWriter<'a, SDMASA_R_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit Block Count (SDMA System Address) - SDMA System Address (Host Version 4 Enable = 0): This register contains the system memory address for an SDMA transfer in the 32-bit addressing mode. When the Host Controller stops an SDMA transfer, this register points to the system address of the next contiguous data position. It can be accessed only if no transaction is executing. Reading this register during data transfers may return an invalid value. - 32-bit Block Count (Host Version 4 Enable = 1): From the Host Controller Version 4.10 specification, this register is redefined as 32-bit Block Count. The Host Controller decrements the block count of this register for every block transfer and the data transfer stops when the count reaches zero. This register must be accessed when no transaction is executing. Reading this register during data transfers may return invalid value. Following are the values for BLOCKCNT_SDMASA: - 0xFFFF_FFFF - 4G - 1 Block - ...... - 0x0000_0002 - 2 Blocks - 0x0000_0001 - 1 Block - 0x0000_0000 - Stop Count Note: - When Host Version 4 Enable = 0, SDMA uses this register as system address and hence Auto CMD23 cannot be used with SDMA since this register is assigned for Auto CMD23 as 32-bit Block Count register. -When Host Version 4 Enable = 1, SDMA uses ADMA system address register and this register is reassigned to 32-bit Block Count. This register must be programmed with a non-zero value for data transfer if the 32-bit Block count register is used instead of the 16-bit Block count register. SDMA may use Auto CMD23 if 32-bit Block Count register is used."]
    #[inline(always)]
    pub fn blockcnt_sdmasa(&self) -> BLOCKCNT_SDMASA_R {
        BLOCKCNT_SDMASA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit Block Count (SDMA System Address) - SDMA System Address (Host Version 4 Enable = 0): This register contains the system memory address for an SDMA transfer in the 32-bit addressing mode. When the Host Controller stops an SDMA transfer, this register points to the system address of the next contiguous data position. It can be accessed only if no transaction is executing. Reading this register during data transfers may return an invalid value. - 32-bit Block Count (Host Version 4 Enable = 1): From the Host Controller Version 4.10 specification, this register is redefined as 32-bit Block Count. The Host Controller decrements the block count of this register for every block transfer and the data transfer stops when the count reaches zero. This register must be accessed when no transaction is executing. Reading this register during data transfers may return invalid value. Following are the values for BLOCKCNT_SDMASA: - 0xFFFF_FFFF - 4G - 1 Block - ...... - 0x0000_0002 - 2 Blocks - 0x0000_0001 - 1 Block - 0x0000_0000 - Stop Count Note: - When Host Version 4 Enable = 0, SDMA uses this register as system address and hence Auto CMD23 cannot be used with SDMA since this register is assigned for Auto CMD23 as 32-bit Block Count register. -When Host Version 4 Enable = 1, SDMA uses ADMA system address register and this register is reassigned to 32-bit Block Count. This register must be programmed with a non-zero value for data transfer if the 32-bit Block count register is used instead of the 16-bit Block count register. SDMA may use Auto CMD23 if 32-bit Block Count register is used."]
    #[inline(always)]
    #[must_use]
    pub fn blockcnt_sdmasa(&mut self) -> BLOCKCNT_SDMASA_W<0> {
        BLOCKCNT_SDMASA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMA System Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmasa_r](index.html) module"]
pub struct SDMASA_R_SPEC;
impl crate::RegisterSpec for SDMASA_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmasa_r::R](R) reader structure"]
impl crate::Readable for SDMASA_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmasa_r::W](W) writer structure"]
impl crate::Writable for SDMASA_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMASA_R to value 0"]
impl crate::Resettable for SDMASA_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
