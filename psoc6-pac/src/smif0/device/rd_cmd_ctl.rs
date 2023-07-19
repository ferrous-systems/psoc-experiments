#[doc = "Register `RD_CMD_CTL` reader"]
pub struct R(crate::R<RD_CMD_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_CMD_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_CMD_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_CMD_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_CMD_CTL` writer"]
pub struct W(crate::W<RD_CMD_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_CMD_CTL_SPEC>;
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
impl From<crate::W<RD_CMD_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_CMD_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE` reader - Command byte code."]
pub type CODE_R = crate::FieldReader;
#[doc = "Field `CODE` writer - Command byte code."]
pub type CODE_W<'a, const O: u8> = crate::FieldWriter<'a, RD_CMD_CTL_SPEC, 8, O>;
#[doc = "Field `WIDTH` reader - Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
pub type WIDTH_R = crate::FieldReader;
#[doc = "Field `WIDTH` writer - Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, RD_CMD_CTL_SPEC, 2, O>;
#[doc = "Field `PRESENT` reader - Presence of command field: '0': not present '1': present"]
pub type PRESENT_R = crate::BitReader;
#[doc = "Field `PRESENT` writer - Presence of command field: '0': not present '1': present"]
pub type PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, RD_CMD_CTL_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - Command byte code."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command byte code."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<0> {
        CODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<16> {
        WIDTH_W::new(self)
    }
    #[doc = "Bit 31 - Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PRESENT_W<31> {
        PRESENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read command control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_cmd_ctl](index.html) module"]
pub struct RD_CMD_CTL_SPEC;
impl crate::RegisterSpec for RD_CMD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_cmd_ctl::R](R) reader structure"]
impl crate::Readable for RD_CMD_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_cmd_ctl::W](W) writer structure"]
impl crate::Writable for RD_CMD_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD_CMD_CTL to value 0"]
impl crate::Resettable for RD_CMD_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
