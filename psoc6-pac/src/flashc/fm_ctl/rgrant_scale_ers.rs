#[doc = "Register `RGRANT_SCALE_ERS` reader"]
pub struct R(crate::R<RGRANT_SCALE_ERS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGRANT_SCALE_ERS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGRANT_SCALE_ERS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGRANT_SCALE_ERS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGRANT_SCALE_ERS` writer"]
pub struct W(crate::W<RGRANT_SCALE_ERS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGRANT_SCALE_ERS_SPEC>;
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
impl From<crate::W<RGRANT_SCALE_ERS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGRANT_SCALE_ERS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCALE_ERS_SEQ01` reader - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ01_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_SEQ01` writer - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ01_W<'a, const O: u8> = crate::FieldWriter<'a, RGRANT_SCALE_ERS_SPEC, 2, O>;
#[doc = "Field `SCALE_ERS_SEQ12` reader - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ12_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_SEQ12` writer - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ12_W<'a, const O: u8> = crate::FieldWriter<'a, RGRANT_SCALE_ERS_SPEC, 2, O>;
#[doc = "Field `SCALE_ERS_SEQ23` reader - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ23_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_SEQ23` writer - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_SEQ23_W<'a, const O: u8> = crate::FieldWriter<'a, RGRANT_SCALE_ERS_SPEC, 2, O>;
#[doc = "Field `SCALE_ERS_PEON` reader - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_PEON_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_PEON` writer - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_PEON_W<'a, const O: u8> = crate::FieldWriter<'a, RGRANT_SCALE_ERS_SPEC, 2, O>;
#[doc = "Field `SCALE_ERS_PEOFF` reader - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_PEOFF_R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_PEOFF` writer - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type SCALE_ERS_PEOFF_W<'a, const O: u8> = crate::FieldWriter<'a, RGRANT_SCALE_ERS_SPEC, 2, O>;
#[doc = "Field `RGRANT_DELAY_ERS_PEON` reader - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_PEON_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_PEON` writer - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_PEON_W<'a, const O: u8> =
    crate::FieldWriter<'a, RGRANT_SCALE_ERS_SPEC, 8, O>;
#[doc = "Field `RGRANT_DELAY_ERS_PEOFF` reader - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_PEOFF_R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_PEOFF` writer - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RGRANT_DELAY_ERS_PEOFF_W<'a, const O: u8> =
    crate::FieldWriter<'a, RGRANT_SCALE_ERS_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:1 - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq01(&self) -> SCALE_ERS_SEQ01_R {
        SCALE_ERS_SEQ01_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq12(&self) -> SCALE_ERS_SEQ12_R {
        SCALE_ERS_SEQ12_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq23(&self) -> SCALE_ERS_SEQ23_R {
        SCALE_ERS_SEQ23_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peon(&self) -> SCALE_ERS_PEON_R {
        SCALE_ERS_PEON_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peoff(&self) -> SCALE_ERS_PEOFF_R {
        SCALE_ERS_PEOFF_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peon(&self) -> RGRANT_DELAY_ERS_PEON_R {
        RGRANT_DELAY_ERS_PEON_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peoff(&self) -> RGRANT_DELAY_ERS_PEOFF_R {
        RGRANT_DELAY_ERS_PEOFF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_seq01(&mut self) -> SCALE_ERS_SEQ01_W<0> {
        SCALE_ERS_SEQ01_W::new(self)
    }
    #[doc = "Bits 2:3 - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_seq12(&mut self) -> SCALE_ERS_SEQ12_W<2> {
        SCALE_ERS_SEQ12_W::new(self)
    }
    #[doc = "Bits 4:5 - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_seq23(&mut self) -> SCALE_ERS_SEQ23_W<4> {
        SCALE_ERS_SEQ23_W::new(self)
    }
    #[doc = "Bits 6:7 - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_peon(&mut self) -> SCALE_ERS_PEON_W<6> {
        SCALE_ERS_PEON_W::new(self)
    }
    #[doc = "Bits 8:9 - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_peoff(&mut self) -> SCALE_ERS_PEOFF_W<8> {
        SCALE_ERS_PEOFF_W::new(self)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_peon(&mut self) -> RGRANT_DELAY_ERS_PEON_W<16> {
        RGRANT_DELAY_ERS_PEON_W::new(self)
    }
    #[doc = "Bits 24:31 - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_peoff(&mut self) -> RGRANT_DELAY_ERS_PEOFF_W<24> {
        RGRANT_DELAY_ERS_PEOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "R-grant delay scale for erase\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgrant_scale_ers](index.html) module"]
pub struct RGRANT_SCALE_ERS_SPEC;
impl crate::RegisterSpec for RGRANT_SCALE_ERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgrant_scale_ers::R](R) reader structure"]
impl crate::Readable for RGRANT_SCALE_ERS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgrant_scale_ers::W](W) writer structure"]
impl crate::Writable for RGRANT_SCALE_ERS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGRANT_SCALE_ERS to value 0"]
impl crate::Resettable for RGRANT_SCALE_ERS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
