#[doc = "Register `SL_CTL` reader"]
pub struct R(crate::R<SL_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SL_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SL_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SL_CTL` writer"]
pub struct W(crate::W<SL_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SL_CTL_SPEC>;
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
impl From<crate::W<SL_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SL_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLED_0` reader - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
pub type ENABLED_0_R = crate::BitReader;
#[doc = "Field `ENABLED_0` writer - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
pub type ENABLED_0_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_1` reader - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
pub type ENABLED_1_R = crate::BitReader;
#[doc = "Field `ENABLED_1` writer - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
pub type ENABLED_1_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_2` reader - N/A"]
pub type ENABLED_2_R = crate::BitReader;
#[doc = "Field `ENABLED_2` writer - N/A"]
pub type ENABLED_2_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_3` reader - N/A"]
pub type ENABLED_3_R = crate::BitReader;
#[doc = "Field `ENABLED_3` writer - N/A"]
pub type ENABLED_3_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_4` reader - N/A"]
pub type ENABLED_4_R = crate::BitReader;
#[doc = "Field `ENABLED_4` writer - N/A"]
pub type ENABLED_4_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_5` reader - N/A"]
pub type ENABLED_5_R = crate::BitReader;
#[doc = "Field `ENABLED_5` writer - N/A"]
pub type ENABLED_5_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_6` reader - N/A"]
pub type ENABLED_6_R = crate::BitReader;
#[doc = "Field `ENABLED_6` writer - N/A"]
pub type ENABLED_6_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_7` reader - N/A"]
pub type ENABLED_7_R = crate::BitReader;
#[doc = "Field `ENABLED_7` writer - N/A"]
pub type ENABLED_7_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_8` reader - N/A"]
pub type ENABLED_8_R = crate::BitReader;
#[doc = "Field `ENABLED_8` writer - N/A"]
pub type ENABLED_8_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_9` reader - N/A"]
pub type ENABLED_9_R = crate::BitReader;
#[doc = "Field `ENABLED_9` writer - N/A"]
pub type ENABLED_9_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_10` reader - N/A"]
pub type ENABLED_10_R = crate::BitReader;
#[doc = "Field `ENABLED_10` writer - N/A"]
pub type ENABLED_10_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_11` reader - N/A"]
pub type ENABLED_11_R = crate::BitReader;
#[doc = "Field `ENABLED_11` writer - N/A"]
pub type ENABLED_11_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_12` reader - N/A"]
pub type ENABLED_12_R = crate::BitReader;
#[doc = "Field `ENABLED_12` writer - N/A"]
pub type ENABLED_12_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_13` reader - N/A"]
pub type ENABLED_13_R = crate::BitReader;
#[doc = "Field `ENABLED_13` writer - N/A"]
pub type ENABLED_13_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_14` reader - N/A"]
pub type ENABLED_14_R = crate::BitReader;
#[doc = "Field `ENABLED_14` writer - N/A"]
pub type ENABLED_14_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `ENABLED_15` reader - N/A"]
pub type ENABLED_15_R = crate::BitReader;
#[doc = "Field `ENABLED_15` writer - N/A"]
pub type ENABLED_15_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_0` reader - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
pub type DISABLED_0_R = crate::BitReader;
#[doc = "Field `DISABLED_0` writer - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
pub type DISABLED_0_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_1` reader - N/A"]
pub type DISABLED_1_R = crate::BitReader;
#[doc = "Field `DISABLED_1` writer - N/A"]
pub type DISABLED_1_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_2` reader - N/A"]
pub type DISABLED_2_R = crate::BitReader;
#[doc = "Field `DISABLED_2` writer - N/A"]
pub type DISABLED_2_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_3` reader - N/A"]
pub type DISABLED_3_R = crate::BitReader;
#[doc = "Field `DISABLED_3` writer - N/A"]
pub type DISABLED_3_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_4` reader - N/A"]
pub type DISABLED_4_R = crate::BitReader;
#[doc = "Field `DISABLED_4` writer - N/A"]
pub type DISABLED_4_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_5` reader - N/A"]
pub type DISABLED_5_R = crate::BitReader;
#[doc = "Field `DISABLED_5` writer - N/A"]
pub type DISABLED_5_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_6` reader - N/A"]
pub type DISABLED_6_R = crate::BitReader;
#[doc = "Field `DISABLED_6` writer - N/A"]
pub type DISABLED_6_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_7` reader - N/A"]
pub type DISABLED_7_R = crate::BitReader;
#[doc = "Field `DISABLED_7` writer - N/A"]
pub type DISABLED_7_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_8` reader - N/A"]
pub type DISABLED_8_R = crate::BitReader;
#[doc = "Field `DISABLED_8` writer - N/A"]
pub type DISABLED_8_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_9` reader - N/A"]
pub type DISABLED_9_R = crate::BitReader;
#[doc = "Field `DISABLED_9` writer - N/A"]
pub type DISABLED_9_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_10` reader - N/A"]
pub type DISABLED_10_R = crate::BitReader;
#[doc = "Field `DISABLED_10` writer - N/A"]
pub type DISABLED_10_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_11` reader - N/A"]
pub type DISABLED_11_R = crate::BitReader;
#[doc = "Field `DISABLED_11` writer - N/A"]
pub type DISABLED_11_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_12` reader - N/A"]
pub type DISABLED_12_R = crate::BitReader;
#[doc = "Field `DISABLED_12` writer - N/A"]
pub type DISABLED_12_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_13` reader - N/A"]
pub type DISABLED_13_R = crate::BitReader;
#[doc = "Field `DISABLED_13` writer - N/A"]
pub type DISABLED_13_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_14` reader - N/A"]
pub type DISABLED_14_R = crate::BitReader;
#[doc = "Field `DISABLED_14` writer - N/A"]
pub type DISABLED_14_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
#[doc = "Field `DISABLED_15` reader - N/A"]
pub type DISABLED_15_R = crate::BitReader;
#[doc = "Field `DISABLED_15` writer - N/A"]
pub type DISABLED_15_W<'a, const O: u8> = crate::BitWriter<'a, SL_CTL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn enabled_0(&self) -> ENABLED_0_R {
        ENABLED_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    pub fn enabled_1(&self) -> ENABLED_1_R {
        ENABLED_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn enabled_2(&self) -> ENABLED_2_R {
        ENABLED_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn enabled_3(&self) -> ENABLED_3_R {
        ENABLED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn enabled_4(&self) -> ENABLED_4_R {
        ENABLED_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn enabled_5(&self) -> ENABLED_5_R {
        ENABLED_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn enabled_6(&self) -> ENABLED_6_R {
        ENABLED_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn enabled_7(&self) -> ENABLED_7_R {
        ENABLED_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn enabled_8(&self) -> ENABLED_8_R {
        ENABLED_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn enabled_9(&self) -> ENABLED_9_R {
        ENABLED_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn enabled_10(&self) -> ENABLED_10_R {
        ENABLED_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn enabled_11(&self) -> ENABLED_11_R {
        ENABLED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn enabled_12(&self) -> ENABLED_12_R {
        ENABLED_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn enabled_13(&self) -> ENABLED_13_R {
        ENABLED_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn enabled_14(&self) -> ENABLED_14_R {
        ENABLED_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn enabled_15(&self) -> ENABLED_15_R {
        ENABLED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
    #[inline(always)]
    pub fn disabled_0(&self) -> DISABLED_0_R {
        DISABLED_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn disabled_1(&self) -> DISABLED_1_R {
        DISABLED_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn disabled_2(&self) -> DISABLED_2_R {
        DISABLED_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn disabled_3(&self) -> DISABLED_3_R {
        DISABLED_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn disabled_4(&self) -> DISABLED_4_R {
        DISABLED_4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn disabled_5(&self) -> DISABLED_5_R {
        DISABLED_5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    pub fn disabled_6(&self) -> DISABLED_6_R {
        DISABLED_6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn disabled_7(&self) -> DISABLED_7_R {
        DISABLED_7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn disabled_8(&self) -> DISABLED_8_R {
        DISABLED_8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn disabled_9(&self) -> DISABLED_9_R {
        DISABLED_9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn disabled_10(&self) -> DISABLED_10_R {
        DISABLED_10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - N/A"]
    #[inline(always)]
    pub fn disabled_11(&self) -> DISABLED_11_R {
        DISABLED_11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    pub fn disabled_12(&self) -> DISABLED_12_R {
        DISABLED_12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    pub fn disabled_13(&self) -> DISABLED_13_R {
        DISABLED_13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn disabled_14(&self) -> DISABLED_14_R {
        DISABLED_14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn disabled_15(&self) -> DISABLED_15_R {
        DISABLED_15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral group, slave 0 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_0(&mut self) -> ENABLED_0_W<0> {
        ENABLED_0_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral group, slave 1 enable. If the slave is disabled, its clock is gated off (constant '0') and its resets are activated. Note: For peripheral group 0 (the peripheral interconnect, master interface MMIO registers), this field is a constant '1' (SW: R): the slave can NOT be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_1(&mut self) -> ENABLED_1_W<1> {
        ENABLED_1_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_2(&mut self) -> ENABLED_2_W<2> {
        ENABLED_2_W::new(self)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_3(&mut self) -> ENABLED_3_W<3> {
        ENABLED_3_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_4(&mut self) -> ENABLED_4_W<4> {
        ENABLED_4_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_5(&mut self) -> ENABLED_5_W<5> {
        ENABLED_5_W::new(self)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_6(&mut self) -> ENABLED_6_W<6> {
        ENABLED_6_W::new(self)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_7(&mut self) -> ENABLED_7_W<7> {
        ENABLED_7_W::new(self)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_8(&mut self) -> ENABLED_8_W<8> {
        ENABLED_8_W::new(self)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_9(&mut self) -> ENABLED_9_W<9> {
        ENABLED_9_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_10(&mut self) -> ENABLED_10_W<10> {
        ENABLED_10_W::new(self)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_11(&mut self) -> ENABLED_11_W<11> {
        ENABLED_11_W::new(self)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_12(&mut self) -> ENABLED_12_W<12> {
        ENABLED_12_W::new(self)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_13(&mut self) -> ENABLED_13_W<13> {
        ENABLED_13_W::new(self)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_14(&mut self) -> ENABLED_14_W<14> {
        ENABLED_14_W::new(self)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_15(&mut self) -> ENABLED_15_W<15> {
        ENABLED_15_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral group, slave 0 permanent disable. Setting this bit to 1 has the same effect as setting ENABLED_0 to 0. However, once set to 1, this bit cannot be changed back to 0 anymore."]
    #[inline(always)]
    #[must_use]
    pub fn disabled_0(&mut self) -> DISABLED_0_W<16> {
        DISABLED_0_W::new(self)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_1(&mut self) -> DISABLED_1_W<17> {
        DISABLED_1_W::new(self)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_2(&mut self) -> DISABLED_2_W<18> {
        DISABLED_2_W::new(self)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_3(&mut self) -> DISABLED_3_W<19> {
        DISABLED_3_W::new(self)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_4(&mut self) -> DISABLED_4_W<20> {
        DISABLED_4_W::new(self)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_5(&mut self) -> DISABLED_5_W<21> {
        DISABLED_5_W::new(self)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_6(&mut self) -> DISABLED_6_W<22> {
        DISABLED_6_W::new(self)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_7(&mut self) -> DISABLED_7_W<23> {
        DISABLED_7_W::new(self)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_8(&mut self) -> DISABLED_8_W<24> {
        DISABLED_8_W::new(self)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_9(&mut self) -> DISABLED_9_W<25> {
        DISABLED_9_W::new(self)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_10(&mut self) -> DISABLED_10_W<26> {
        DISABLED_10_W::new(self)
    }
    #[doc = "Bit 27 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_11(&mut self) -> DISABLED_11_W<27> {
        DISABLED_11_W::new(self)
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_12(&mut self) -> DISABLED_12_W<28> {
        DISABLED_12_W::new(self)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_13(&mut self) -> DISABLED_13_W<29> {
        DISABLED_13_W::new(self)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_14(&mut self) -> DISABLED_14_W<30> {
        DISABLED_14_W::new(self)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_15(&mut self) -> DISABLED_15_W<31> {
        DISABLED_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl_ctl](index.html) module"]
pub struct SL_CTL_SPEC;
impl crate::RegisterSpec for SL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sl_ctl::R](R) reader structure"]
impl crate::Readable for SL_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sl_ctl::W](W) writer structure"]
impl crate::Writable for SL_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SL_CTL to value 0xffff"]
impl crate::Resettable for SL_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
