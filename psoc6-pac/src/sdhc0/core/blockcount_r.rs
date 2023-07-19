#[doc = "Register `BLOCKCOUNT_R` reader"]
pub struct R(crate::R<BLOCKCOUNT_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLOCKCOUNT_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLOCKCOUNT_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLOCKCOUNT_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLOCKCOUNT_R` writer"]
pub struct W(crate::W<BLOCKCOUNT_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLOCKCOUNT_R_SPEC>;
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
impl From<crate::W<BLOCKCOUNT_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLOCKCOUNT_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_CNT` reader - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
pub type BLOCK_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_CNT` writer - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
pub type BLOCK_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, BLOCKCOUNT_R_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    pub fn block_cnt(&self) -> BLOCK_CNT_R {
        BLOCK_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    #[must_use]
    pub fn block_cnt(&mut self) -> BLOCK_CNT_W<0> {
        BLOCK_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "16-bit Block Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blockcount_r](index.html) module"]
pub struct BLOCKCOUNT_R_SPEC;
impl crate::RegisterSpec for BLOCKCOUNT_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [blockcount_r::R](R) reader structure"]
impl crate::Readable for BLOCKCOUNT_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blockcount_r::W](W) writer structure"]
impl crate::Writable for BLOCKCOUNT_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLOCKCOUNT_R to value 0"]
impl crate::Resettable for BLOCKCOUNT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
