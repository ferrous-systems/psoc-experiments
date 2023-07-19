#[doc = "Register `CLOCK_CTL` reader"]
pub struct R(crate::R<CLOCK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CTL` writer"]
pub struct W(crate::W<CLOCK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CTL_SPEC>;
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
impl From<crate::W<CLOCK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT8_DIV` reader - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT8_DIV_R = crate::FieldReader;
#[doc = "Field `INT8_DIV` writer - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type INT8_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, CLOCK_CTL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 8:15 - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&self) -> INT8_DIV_R {
        INT8_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn int8_div(&mut self) -> INT8_DIV_W<8> {
        INT8_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctl](index.html) module"]
pub struct CLOCK_CTL_SPEC;
impl crate::RegisterSpec for CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_ctl::R](R) reader structure"]
impl crate::Readable for CLOCK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_ctl::W](W) writer structure"]
impl crate::Writable for CLOCK_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_CTL to value 0"]
impl crate::Resettable for CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
