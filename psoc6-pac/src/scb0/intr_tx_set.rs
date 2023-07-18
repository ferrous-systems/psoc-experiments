#[doc = "Register `INTR_TX_SET` reader"]
pub struct R(crate::R<INTR_TX_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_TX_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_TX_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_TX_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_TX_SET` writer"]
pub struct W(crate::W<INTR_TX_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_TX_SET_SPEC>;
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
impl From<crate::W<INTR_TX_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_TX_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TRIGGER_R = crate::BitReader;
#[doc = "Field `TRIGGER` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, INTR_TX_SET_SPEC, O>;
#[doc = "Field `NOT_FULL` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type NOT_FULL_R = crate::BitReader;
#[doc = "Field `NOT_FULL` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type NOT_FULL_W<'a, const O: u8> = crate::BitWriter<'a, INTR_TX_SET_SPEC, O>;
#[doc = "Field `EMPTY` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type EMPTY_R = crate::BitReader;
#[doc = "Field `EMPTY` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, INTR_TX_SET_SPEC, O>;
#[doc = "Field `OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type OVERFLOW_R = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, INTR_TX_SET_SPEC, O>;
#[doc = "Field `UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type UNDERFLOW_R = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, INTR_TX_SET_SPEC, O>;
#[doc = "Field `BLOCKED` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type BLOCKED_R = crate::BitReader;
#[doc = "Field `BLOCKED` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type BLOCKED_W<'a, const O: u8> = crate::BitWriter<'a, INTR_TX_SET_SPEC, O>;
#[doc = "Field `UART_NACK` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type UART_NACK_R = crate::BitReader;
#[doc = "Field `UART_NACK` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type UART_NACK_W<'a, const O: u8> = crate::BitWriter<'a, INTR_TX_SET_SPEC, O>;
#[doc = "Field `UART_DONE` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type UART_DONE_R = crate::BitReader;
#[doc = "Field `UART_DONE` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type UART_DONE_W<'a, const O: u8> = crate::BitWriter<'a, INTR_TX_SET_SPEC, O>;
#[doc = "Field `UART_ARB_LOST` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type UART_ARB_LOST_R = crate::BitReader;
#[doc = "Field `UART_ARB_LOST` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type UART_ARB_LOST_W<'a, const O: u8> = crate::BitWriter<'a, INTR_TX_SET_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn not_full(&self) -> NOT_FULL_R {
        NOT_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_nack(&self) -> UART_NACK_R {
        UART_NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_done(&self) -> UART_DONE_R {
        UART_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UART_ARB_LOST_R {
        UART_ARB_LOST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TRIGGER_W<0> {
        TRIGGER_W::new(self)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn not_full(&mut self) -> NOT_FULL_W<1> {
        NOT_FULL_W::new(self)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EMPTY_W<4> {
        EMPTY_W::new(self)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<5> {
        OVERFLOW_W::new(self)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UNDERFLOW_W<6> {
        UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BLOCKED_W<7> {
        BLOCKED_W::new(self)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn uart_nack(&mut self) -> UART_NACK_W<8> {
        UART_NACK_W::new(self)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn uart_done(&mut self) -> UART_DONE_W<9> {
        UART_DONE_W::new(self)
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn uart_arb_lost(&mut self) -> UART_ARB_LOST_W<10> {
        UART_ARB_LOST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter interrupt set request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_tx_set](index.html) module"]
pub struct INTR_TX_SET_SPEC;
impl crate::RegisterSpec for INTR_TX_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_tx_set::R](R) reader structure"]
impl crate::Readable for INTR_TX_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_tx_set::W](W) writer structure"]
impl crate::Writable for INTR_TX_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_TX_SET to value 0"]
impl crate::Resettable for INTR_TX_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
