#[doc = "Register `TR_CTL[%s]` reader"]
pub struct R(crate::R<TR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTL[%s]` writer"]
pub struct W(crate::W<TR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTL_SPEC>;
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
impl From<crate::W<TR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR_SEL` reader - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
pub type TR_SEL_R = crate::FieldReader;
#[doc = "Field `TR_SEL` writer - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
pub type TR_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, TR_CTL_SPEC, 8, O>;
#[doc = "Field `TR_INV` reader - Specifies if the output trigger is inverted."]
pub type TR_INV_R = crate::BitReader;
#[doc = "Field `TR_INV` writer - Specifies if the output trigger is inverted."]
pub type TR_INV_W<'a, const O: u8> = crate::BitWriter<'a, TR_CTL_SPEC, O>;
#[doc = "Field `TR_EDGE` reader - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
pub type TR_EDGE_R = crate::BitReader;
#[doc = "Field `TR_EDGE` writer - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
pub type TR_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, TR_CTL_SPEC, O>;
#[doc = "Field `DBG_FREEZE_EN` reader - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
pub type DBG_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `DBG_FREEZE_EN` writer - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
pub type DBG_FREEZE_EN_W<'a, const O: u8> = crate::BitWriter<'a, TR_CTL_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TR_SEL_R {
        TR_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    pub fn tr_inv(&self) -> TR_INV_R {
        TR_INV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    pub fn tr_edge(&self) -> TR_EDGE_R {
        TR_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DBG_FREEZE_EN_R {
        DBG_FREEZE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    #[must_use]
    pub fn tr_sel(&mut self) -> TR_SEL_W<0> {
        TR_SEL_W::new(self)
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn tr_inv(&mut self) -> TR_INV_W<8> {
        TR_INV_W::new(self)
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    #[must_use]
    pub fn tr_edge(&mut self) -> TR_EDGE_W<9> {
        TR_EDGE_W::new(self)
    }
    #[doc = "Bit 12 - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_freeze_en(&mut self) -> DBG_FREEZE_EN_W<12> {
        DBG_FREEZE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctl](index.html) module"]
pub struct TR_CTL_SPEC;
impl crate::RegisterSpec for TR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctl::R](R) reader structure"]
impl crate::Readable for TR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctl::W](W) writer structure"]
impl crate::Writable for TR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CTL[%s]
to value 0"]
impl crate::Resettable for TR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
