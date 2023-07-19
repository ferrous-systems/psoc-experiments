#[doc = "Register `RGRANT_DELAY_PRG` reader"]
pub struct R(crate::R<RGRANT_DELAY_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGRANT_DELAY_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGRANT_DELAY_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGRANT_DELAY_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGRANT_DELAY_PRG` writer"]
pub struct W(crate::W<RGRANT_DELAY_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGRANT_DELAY_PRG_SPEC>;
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
impl From<crate::W<RGRANT_DELAY_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGRANT_DELAY_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RGRANT_DELAY_PRG_SEQ12` reader - PROG&amp;PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_SEQ12_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ12` writer - PROG&amp;PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_SEQ12_W<'a, const O: u8> =
    crate::FieldWriter<'a, RGRANT_DELAY_PRG_SPEC, 8, O>;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ23` reader - PROG&amp;PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_SEQ23_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ23` writer - PROG&amp;PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_PRG_SEQ23_W<'a, const O: u8> =
    crate::FieldWriter<'a, RGRANT_DELAY_PRG_SPEC, 8, O>;
#[doc = "Field `RGRANT_DELAY_SEQ30` reader - PROG&amp;PRE_PROG &amp; ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_SEQ30_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_SEQ30` writer - PROG&amp;PRE_PROG &amp; ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_SEQ30_W<'a, const O: u8> =
    crate::FieldWriter<'a, RGRANT_DELAY_PRG_SPEC, 8, O>;
#[doc = "Field `RGRANT_DELAY_CLK` reader - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
pub type RGRANT_DELAY_CLK_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_CLK` writer - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
pub type RGRANT_DELAY_CLK_W<'a, const O: u8> = crate::FieldWriter<'a, RGRANT_DELAY_PRG_SPEC, 4, O>;
#[doc = "Field `HV_PARAMS_LOADED` reader - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
pub type HV_PARAMS_LOADED_R = crate::BitReader;
#[doc = "Field `HV_PARAMS_LOADED` writer - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
pub type HV_PARAMS_LOADED_W<'a, const O: u8> = crate::BitWriter<'a, RGRANT_DELAY_PRG_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - PROG&amp;PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq12(&self) -> RGRANT_DELAY_PRG_SEQ12_R {
        RGRANT_DELAY_PRG_SEQ12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PROG&amp;PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq23(&self) -> RGRANT_DELAY_PRG_SEQ23_R {
        RGRANT_DELAY_PRG_SEQ23_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PROG&amp;PRE_PROG &amp; ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_seq30(&self) -> RGRANT_DELAY_SEQ30_R {
        RGRANT_DELAY_SEQ30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
    #[inline(always)]
    pub fn rgrant_delay_clk(&self) -> RGRANT_DELAY_CLK_R {
        RGRANT_DELAY_CLK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
    #[inline(always)]
    pub fn hv_params_loaded(&self) -> HV_PARAMS_LOADED_R {
        HV_PARAMS_LOADED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PROG&amp;PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_seq12(&mut self) -> RGRANT_DELAY_PRG_SEQ12_W<0> {
        RGRANT_DELAY_PRG_SEQ12_W::new(self)
    }
    #[doc = "Bits 8:15 - PROG&amp;PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_seq23(&mut self) -> RGRANT_DELAY_PRG_SEQ23_W<8> {
        RGRANT_DELAY_PRG_SEQ23_W::new(self)
    }
    #[doc = "Bits 16:23 - PROG&amp;PRE_PROG &amp; ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_seq30(&mut self) -> RGRANT_DELAY_SEQ30_W<16> {
        RGRANT_DELAY_SEQ30_W::new(self)
    }
    #[doc = "Bits 24:27 - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_clk(&mut self) -> RGRANT_DELAY_CLK_W<24> {
        RGRANT_DELAY_CLK_W::new(self)
    }
    #[doc = "Bit 31 - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
    #[inline(always)]
    #[must_use]
    pub fn hv_params_loaded(&mut self) -> HV_PARAMS_LOADED_W<31> {
        HV_PARAMS_LOADED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "R-grant delay for program\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgrant_delay_prg](index.html) module"]
pub struct RGRANT_DELAY_PRG_SPEC;
impl crate::RegisterSpec for RGRANT_DELAY_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgrant_delay_prg::R](R) reader structure"]
impl crate::Readable for RGRANT_DELAY_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgrant_delay_prg::W](W) writer structure"]
impl crate::Writable for RGRANT_DELAY_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGRANT_DELAY_PRG to value 0x0100_0000"]
impl crate::Resettable for RGRANT_DELAY_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
