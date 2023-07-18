#[doc = "Register `FM_CTL` reader"]
pub struct R(crate::R<FM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM_CTL` writer"]
pub struct W(crate::W<FM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_CTL_SPEC>;
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
impl From<crate::W<FM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FM_MODE` reader - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
pub type FM_MODE_R = crate::FieldReader;
#[doc = "Field `FM_MODE` writer - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
pub type FM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, FM_CTL_SPEC, 4, O>;
#[doc = "Field `FM_SEQ` reader - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
pub type FM_SEQ_R = crate::FieldReader;
#[doc = "Field `FM_SEQ` writer - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
pub type FM_SEQ_W<'a, const O: u8> = crate::FieldWriter<'a, FM_CTL_SPEC, 2, O>;
#[doc = "Field `DAA_MUX_SEL` reader - Direct memory cell access address."]
pub type DAA_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `DAA_MUX_SEL` writer - Direct memory cell access address."]
pub type DAA_MUX_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, FM_CTL_SPEC, 7, O>;
#[doc = "Field `IF_SEL` reader - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type IF_SEL_R = crate::BitReader;
#[doc = "Field `IF_SEL` writer - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type IF_SEL_W<'a, const O: u8> = crate::BitWriter<'a, FM_CTL_SPEC, O>;
#[doc = "Field `WR_EN` reader - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type WR_EN_R = crate::BitReader;
#[doc = "Field `WR_EN` writer - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type WR_EN_W<'a, const O: u8> = crate::BitWriter<'a, FM_CTL_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
    #[inline(always)]
    pub fn fm_mode(&self) -> FM_MODE_R {
        FM_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
    #[inline(always)]
    pub fn fm_seq(&self) -> FM_SEQ_R {
        FM_SEQ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    pub fn daa_mux_sel(&self) -> DAA_MUX_SEL_R {
        DAA_MUX_SEL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub fn if_sel(&self) -> IF_SEL_R {
        IF_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn fm_mode(&mut self) -> FM_MODE_W<0> {
        FM_MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
    #[inline(always)]
    #[must_use]
    pub fn fm_seq(&mut self) -> FM_SEQ_W<8> {
        FM_SEQ_W::new(self)
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    #[must_use]
    pub fn daa_mux_sel(&mut self) -> DAA_MUX_SEL_W<16> {
        DAA_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn if_sel(&mut self) -> IF_SEL_W<24> {
        IF_SEL_W::new(self)
    }
    #[doc = "Bit 25 - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WR_EN_W<25> {
        WR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash macro control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_ctl](index.html) module"]
pub struct FM_CTL_SPEC;
impl crate::RegisterSpec for FM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm_ctl::R](R) reader structure"]
impl crate::Readable for FM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm_ctl::W](W) writer structure"]
impl crate::Writable for FM_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_CTL to value 0"]
impl crate::Resettable for FM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
