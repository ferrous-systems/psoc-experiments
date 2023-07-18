#[doc = "Register `SL_SIZE` reader"]
pub struct R(crate::R<SL_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SL_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SL_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SL_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SL_SIZE` writer"]
pub struct W(crate::W<SL_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SL_SIZE_SPEC>;
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
impl From<crate::W<SL_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SL_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION_SIZE` reader - This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
pub type REGION_SIZE_R = crate::FieldReader;
#[doc = "Field `REGION_SIZE` writer - This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
pub type REGION_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, SL_SIZE_SPEC, 5, O>;
#[doc = "Field `VALID` reader - Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
pub type VALID_R = crate::BitReader;
#[doc = "Field `VALID` writer - Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, SL_SIZE_SPEC, O>;
impl R {
    #[doc = "Bits 24:28 - This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub fn region_size(&self) -> REGION_SIZE_R {
        REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:28 - This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    #[must_use]
    pub fn region_size(&mut self) -> REGION_SIZE_W<24> {
        REGION_SIZE_W::new(self)
    }
    #[doc = "Bit 31 - Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
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
#[doc = "Slave region, size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_size](index.html) module"]
pub struct SL_SIZE_SPEC;
impl crate::RegisterSpec for SL_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sl_size::R](R) reader structure"]
impl crate::Readable for SL_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sl_size::W](W) writer structure"]
impl crate::Writable for SL_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SL_SIZE to value 0"]
impl crate::Resettable for SL_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
