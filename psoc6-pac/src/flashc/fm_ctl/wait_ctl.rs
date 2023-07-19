#[doc = "Register `WAIT_CTL` reader"]
pub struct R(crate::R<WAIT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAIT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAIT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAIT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAIT_CTL` writer"]
pub struct W(crate::W<WAIT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAIT_CTL_SPEC>;
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
impl From<crate::W<WAIT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAIT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT_FM_MEM_RD` reader - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
pub type WAIT_FM_MEM_RD_R = crate::FieldReader;
#[doc = "Field `WAIT_FM_MEM_RD` writer - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
pub type WAIT_FM_MEM_RD_W<'a, const O: u8> = crate::FieldWriter<'a, WAIT_CTL_SPEC, 4, O>;
#[doc = "Field `WAIT_FM_HV_RD` reader - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
pub type WAIT_FM_HV_RD_R = crate::FieldReader;
#[doc = "Field `WAIT_FM_HV_RD` writer - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
pub type WAIT_FM_HV_RD_W<'a, const O: u8> = crate::FieldWriter<'a, WAIT_CTL_SPEC, 4, O>;
#[doc = "Field `WAIT_FM_HV_WR` reader - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
pub type WAIT_FM_HV_WR_R = crate::FieldReader;
#[doc = "Field `WAIT_FM_HV_WR` writer - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
pub type WAIT_FM_HV_WR_W<'a, const O: u8> = crate::FieldWriter<'a, WAIT_CTL_SPEC, 3, O>;
#[doc = "Field `FM_RWW_MODE` reader - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
pub type FM_RWW_MODE_R = crate::FieldReader;
#[doc = "Field `FM_RWW_MODE` writer - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
pub type FM_RWW_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, WAIT_CTL_SPEC, 2, O>;
#[doc = "Field `LV_SPARE_1` reader - Spare register"]
pub type LV_SPARE_1_R = crate::BitReader;
#[doc = "Field `LV_SPARE_1` writer - Spare register"]
pub type LV_SPARE_1_W<'a, const O: u8> = crate::BitWriter<'a, WAIT_CTL_SPEC, O>;
#[doc = "Field `DRMM` reader - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
pub type DRMM_R = crate::BitReader;
#[doc = "Field `DRMM` writer - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
pub type DRMM_W<'a, const O: u8> = crate::BitWriter<'a, WAIT_CTL_SPEC, O>;
#[doc = "Field `MBA` reader - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
pub type MBA_R = crate::BitReader;
#[doc = "Field `MBA` writer - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
pub type MBA_W<'a, const O: u8> = crate::BitWriter<'a, WAIT_CTL_SPEC, O>;
#[doc = "Field `PL_SOFT_SET_EN` reader - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
pub type PL_SOFT_SET_EN_R = crate::BitReader;
#[doc = "Field `PL_SOFT_SET_EN` writer - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
pub type PL_SOFT_SET_EN_W<'a, const O: u8> = crate::BitWriter<'a, WAIT_CTL_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub fn wait_fm_mem_rd(&self) -> WAIT_FM_MEM_RD_R {
        WAIT_FM_MEM_RD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub fn wait_fm_hv_rd(&self) -> WAIT_FM_HV_RD_R {
        WAIT_FM_HV_RD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub fn wait_fm_hv_wr(&self) -> WAIT_FM_HV_WR_R {
        WAIT_FM_HV_WR_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25 - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
    #[inline(always)]
    pub fn fm_rww_mode(&self) -> FM_RWW_MODE_R {
        FM_RWW_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Spare register"]
    #[inline(always)]
    pub fn lv_spare_1(&self) -> LV_SPARE_1_R {
        LV_SPARE_1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
    #[inline(always)]
    pub fn drmm(&self) -> DRMM_R {
        DRMM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
    #[inline(always)]
    pub fn mba(&self) -> MBA_R {
        MBA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
    #[inline(always)]
    pub fn pl_soft_set_en(&self) -> PL_SOFT_SET_EN_R {
        PL_SOFT_SET_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    #[must_use]
    pub fn wait_fm_mem_rd(&mut self) -> WAIT_FM_MEM_RD_W<0> {
        WAIT_FM_MEM_RD_W::new(self)
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    #[must_use]
    pub fn wait_fm_hv_rd(&mut self) -> WAIT_FM_HV_RD_W<8> {
        WAIT_FM_HV_RD_W::new(self)
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    #[must_use]
    pub fn wait_fm_hv_wr(&mut self) -> WAIT_FM_HV_WR_W<16> {
        WAIT_FM_HV_WR_W::new(self)
    }
    #[doc = "Bits 24:25 - 00: Full CBUS MODE 01: RWW 10: RWW. R_GRANT is stalling r_bus for the whole program/erase duration"]
    #[inline(always)]
    #[must_use]
    pub fn fm_rww_mode(&mut self) -> FM_RWW_MODE_W<24> {
        FM_RWW_MODE_W::new(self)
    }
    #[doc = "Bit 26 - Spare register"]
    #[inline(always)]
    #[must_use]
    pub fn lv_spare_1(&mut self) -> LV_SPARE_1_W<26> {
        LV_SPARE_1_W::new(self)
    }
    #[doc = "Bit 27 - 0: Normal 1: Test mode to enable Margin mode for 2 rows at a time"]
    #[inline(always)]
    #[must_use]
    pub fn drmm(&mut self) -> DRMM_W<27> {
        DRMM_W::new(self)
    }
    #[doc = "Bit 28 - 0: Normal 1: Test mode to enable Master Bulk Access which allows both normal rows and redundant rows to be erased / programmed in one HV cycle (Bulk / Sector Erase and Sector Program)."]
    #[inline(always)]
    #[must_use]
    pub fn mba(&mut self) -> MBA_W<28> {
        MBA_W::new(self)
    }
    #[doc = "Bit 29 - Page latch soft set enable, 0 = disabled, 1 = enabled (at end of seq_2), taken care in API"]
    #[inline(always)]
    #[must_use]
    pub fn pl_soft_set_en(&mut self) -> PL_SOFT_SET_EN_W<29> {
        PL_SOFT_SET_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wait State control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wait_ctl](index.html) module"]
pub struct WAIT_CTL_SPEC;
impl crate::RegisterSpec for WAIT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wait_ctl::R](R) reader structure"]
impl crate::Readable for WAIT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wait_ctl::W](W) writer structure"]
impl crate::Writable for WAIT_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAIT_CTL to value 0x0003_0b09"]
impl crate::Resettable for WAIT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0b09;
}
