#[doc = "Register `CM4_STATUS` reader"]
pub struct R(crate::R<CM4_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM4_STATUS` writer"]
pub struct W(crate::W<CM4_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM4_STATUS_SPEC>;
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
impl From<crate::W<CM4_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM4_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAIN_INTERNAL_ERR` reader - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM4 access (or debug access via SYS_AP/CM4_AP). SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
pub type MAIN_INTERNAL_ERR_R = crate::BitReader;
#[doc = "Field `MAIN_INTERNAL_ERR` writer - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM4 access (or debug access via SYS_AP/CM4_AP). SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
pub type MAIN_INTERNAL_ERR_W<'a, const O: u8> = crate::BitWriter<'a, CM4_STATUS_SPEC, O>;
#[doc = "Field `WORK_INTERNAL_ERR` reader - See CM4_STATUS.MAIN_INTERNAL_ERROR."]
pub type WORK_INTERNAL_ERR_R = crate::BitReader;
#[doc = "Field `WORK_INTERNAL_ERR` writer - See CM4_STATUS.MAIN_INTERNAL_ERROR."]
pub type WORK_INTERNAL_ERR_W<'a, const O: u8> = crate::BitWriter<'a, CM4_STATUS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM4 access (or debug access via SYS_AP/CM4_AP). SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
    #[inline(always)]
    pub fn main_internal_err(&self) -> MAIN_INTERNAL_ERR_R {
        MAIN_INTERNAL_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See CM4_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn work_internal_err(&self) -> WORK_INTERNAL_ERR_R {
        WORK_INTERNAL_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM4 access (or debug access via SYS_AP/CM4_AP). SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
    #[inline(always)]
    #[must_use]
    pub fn main_internal_err(&mut self) -> MAIN_INTERNAL_ERR_W<0> {
        MAIN_INTERNAL_ERR_W::new(self)
    }
    #[doc = "Bit 1 - See CM4_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    #[must_use]
    pub fn work_internal_err(&mut self) -> WORK_INTERNAL_ERR_W<1> {
        WORK_INTERNAL_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM4 interface status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_status](index.html) module"]
pub struct CM4_STATUS_SPEC;
impl crate::RegisterSpec for CM4_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_status::R](R) reader structure"]
impl crate::Readable for CM4_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm4_status::W](W) writer structure"]
impl crate::Writable for CM4_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM4_STATUS to value 0"]
impl crate::Resettable for CM4_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
