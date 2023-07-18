#[doc = "Register `RX_FIFO_CTRL` reader"]
pub struct R(crate::R<RX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_FIFO_CTRL` writer"]
pub struct W(crate::W<RX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_FIFO_CTRL_SPEC>;
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
impl From<crate::W<RX_FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
pub type TRIGGER_LEVEL_R = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
pub type TRIGGER_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, RX_FIFO_CTRL_SPEC, 8, O>;
#[doc = "Field `CLEAR` reader - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type CLEAR_R = crate::BitReader;
#[doc = "Field `CLEAR` writer - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, RX_FIFO_CTRL_SPEC, O>;
#[doc = "Field `FREEZE` reader - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
pub type FREEZE_R = crate::BitReader;
#[doc = "Field `FREEZE` writer - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
pub type FREEZE_W<'a, const O: u8> = crate::BitWriter<'a, RX_FIFO_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<0> {
        TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<16> {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FREEZE_W<17> {
        FREEZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_ctrl](index.html) module"]
pub struct RX_FIFO_CTRL_SPEC;
impl crate::RegisterSpec for RX_FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_ctrl::R](R) reader structure"]
impl crate::Readable for RX_FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_fifo_ctrl::W](W) writer structure"]
impl crate::Writable for RX_FIFO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_FIFO_CTRL to value 0"]
impl crate::Resettable for RX_FIFO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
