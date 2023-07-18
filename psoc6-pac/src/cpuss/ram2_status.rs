#[doc = "Register `RAM2_STATUS` reader"]
pub struct R(crate::R<RAM2_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM2_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM2_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM2_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WB_EMPTY` reader - See RAM0_STATUS."]
pub type WB_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - See RAM0_STATUS."]
    #[inline(always)]
    pub fn wb_empty(&self) -> WB_EMPTY_R {
        WB_EMPTY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "RAM 2 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram2_status](index.html) module"]
pub struct RAM2_STATUS_SPEC;
impl crate::RegisterSpec for RAM2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram2_status::R](R) reader structure"]
impl crate::Readable for RAM2_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAM2_STATUS to value 0x01"]
impl crate::Resettable for RAM2_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
