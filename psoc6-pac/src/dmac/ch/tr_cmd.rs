#[doc = "Register `TR_CMD` reader"]
pub struct R(crate::R<TR_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CMD` writer"]
pub struct W(crate::W<TR_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CMD_SPEC>;
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
impl From<crate::W<TR_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVATE` reader - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
pub type ACTIVATE_R = crate::BitReader;
#[doc = "Field `ACTIVATE` writer - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
pub type ACTIVATE_W<'a, const O: u8> = crate::BitWriter<'a, TR_CMD_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    pub fn activate(&self) -> ACTIVATE_R {
        ACTIVATE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    #[must_use]
    pub fn activate(&mut self) -> ACTIVATE_W<0> {
        ACTIVATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channle software trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_cmd](index.html) module"]
pub struct TR_CMD_SPEC;
impl crate::RegisterSpec for TR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_cmd::R](R) reader structure"]
impl crate::Readable for TR_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_cmd::W](W) writer structure"]
impl crate::Writable for TR_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CMD to value 0"]
impl crate::Resettable for TR_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
