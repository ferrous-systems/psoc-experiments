#[doc = "Register `RX_MATCH` reader"]
pub struct R(crate::R<RX_MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_MATCH` writer"]
pub struct W(crate::W<RX_MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_MATCH_SPEC>;
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
impl From<crate::W<RX_MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_MATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, RX_MATCH_SPEC, 8, O>;
#[doc = "Field `MASK` reader - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR &amp; MASK) == ('slave address' &amp; MASK))."]
pub type MASK_R = crate::FieldReader;
#[doc = "Field `MASK` writer - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR &amp; MASK) == ('slave address' &amp; MASK))."]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, RX_MATCH_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR &amp; MASK) == ('slave address' &amp; MASK))."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 16:23 - Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR &amp; MASK) == ('slave address' &amp; MASK))."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<16> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave address and mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_match](index.html) module"]
pub struct RX_MATCH_SPEC;
impl crate::RegisterSpec for RX_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_match::R](R) reader structure"]
impl crate::Readable for RX_MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_match::W](W) writer structure"]
impl crate::Writable for RX_MATCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_MATCH to value 0"]
impl crate::Resettable for RX_MATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
