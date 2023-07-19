#[doc = "Register `SL_ADDR` reader"]
pub struct R(crate::R<SL_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SL_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SL_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SL_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SL_ADDR` writer"]
pub struct W(crate::W<SL_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SL_ADDR_SPEC>;
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
impl From<crate::W<SL_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SL_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR30` reader - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
pub type ADDR30_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR30` writer - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
pub type ADDR30_W<'a, const O: u8> = crate::FieldWriter<'a, SL_ADDR_SPEC, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
    #[inline(always)]
    pub fn addr30(&self) -> ADDR30_R {
        ADDR30_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
    #[inline(always)]
    #[must_use]
    pub fn addr30(&mut self) -> ADDR30_W<2> {
        ADDR30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave region, base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_addr](index.html) module"]
pub struct SL_ADDR_SPEC;
impl crate::RegisterSpec for SL_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sl_addr::R](R) reader structure"]
impl crate::Readable for SL_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sl_addr::W](W) writer structure"]
impl crate::Writable for SL_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SL_ADDR to value 0"]
impl crate::Resettable for SL_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
