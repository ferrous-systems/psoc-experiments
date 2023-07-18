#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX` reader - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
pub type IDX_R = crate::FieldReader;
#[doc = "Field `IDX` writer - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
pub type IDX_W<'a, const O: u8> = crate::FieldWriter<'a, STATUS_SPEC, 7, O>;
#[doc = "Field `VALID` reader - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
pub type VALID_R = crate::BitReader;
#[doc = "Field `VALID` writer - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, STATUS_SPEC, O>;
impl R {
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IDX_W<0> {
        IDX_W::new(self)
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<31> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
