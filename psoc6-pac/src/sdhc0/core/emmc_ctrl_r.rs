#[doc = "Register `EMMC_CTRL_R` reader"]
pub struct R(crate::R<EMMC_CTRL_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_CTRL_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_CTRL_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_CTRL_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_CTRL_R` writer"]
pub struct W(crate::W<EMMC_CTRL_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_CTRL_R_SPEC>;
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
impl From<crate::W<EMMC_CTRL_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_CTRL_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_IS_EMMC` reader - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
pub type CARD_IS_EMMC_R = crate::BitReader;
#[doc = "Field `CARD_IS_EMMC` writer - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
pub type CARD_IS_EMMC_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_CTRL_R_SPEC, O>;
#[doc = "Field `DISABLE_DATA_CRC_CHK` reader - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
pub type DISABLE_DATA_CRC_CHK_R = crate::BitReader;
#[doc = "Field `DISABLE_DATA_CRC_CHK` writer - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
pub type DISABLE_DATA_CRC_CHK_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_CTRL_R_SPEC, O>;
#[doc = "Field `EMMC_RST_N` reader - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
pub type EMMC_RST_N_R = crate::BitReader;
#[doc = "Field `EMMC_RST_N` writer - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
pub type EMMC_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_CTRL_R_SPEC, O>;
#[doc = "Field `EMMC_RST_N_OE` reader - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
pub type EMMC_RST_N_OE_R = crate::BitReader;
#[doc = "Field `EMMC_RST_N_OE` writer - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
pub type EMMC_RST_N_OE_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_CTRL_R_SPEC, O>;
#[doc = "Field `CQE_ALGO_SEL` reader - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
pub type CQE_ALGO_SEL_R = crate::BitReader;
#[doc = "Field `CQE_ALGO_SEL` writer - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
pub type CQE_ALGO_SEL_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_CTRL_R_SPEC, O>;
#[doc = "Field `CQE_PREFETCH_DISABLE` reader - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
pub type CQE_PREFETCH_DISABLE_R = crate::BitReader;
#[doc = "Field `CQE_PREFETCH_DISABLE` writer - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
pub type CQE_PREFETCH_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, EMMC_CTRL_R_SPEC, O>;
impl R {
    #[doc = "Bit 0 - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
    #[inline(always)]
    pub fn card_is_emmc(&self) -> CARD_IS_EMMC_R {
        CARD_IS_EMMC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
    #[inline(always)]
    pub fn disable_data_crc_chk(&self) -> DISABLE_DATA_CRC_CHK_R {
        DISABLE_DATA_CRC_CHK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    #[inline(always)]
    pub fn emmc_rst_n(&self) -> EMMC_RST_N_R {
        EMMC_RST_N_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
    #[inline(always)]
    pub fn emmc_rst_n_oe(&self) -> EMMC_RST_N_OE_R {
        EMMC_RST_N_OE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
    #[inline(always)]
    pub fn cqe_algo_sel(&self) -> CQE_ALGO_SEL_R {
        CQE_ALGO_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
    #[inline(always)]
    pub fn cqe_prefetch_disable(&self) -> CQE_PREFETCH_DISABLE_R {
        CQE_PREFETCH_DISABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDHC. Values: - 0x1 (EMMC_CARD): Card connected to SDHC is an eMMC card - 0x0 (NON_EMMC_CARD): Card connected to SDHC is a non-eMMC card"]
    #[inline(always)]
    #[must_use]
    pub fn card_is_emmc(&mut self) -> CARD_IS_EMMC_W<0> {
        CARD_IS_EMMC_W::new(self)
    }
    #[doc = "Bit 1 - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: - 0x1 (DISABLE): DATA CRC check is disabled - 0x0 (ENABLE): DATA CRC check is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn disable_data_crc_chk(&mut self) -> DISABLE_DATA_CRC_CHK_W<1> {
        DISABLE_DATA_CRC_CHK_W::new(self)
    }
    #[doc = "Bit 2 - EMMC Device Reset signal control. This register field controls the card_emmc_reset_n output of SDHC Values: - 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted - 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_rst_n(&mut self) -> EMMC_RST_N_W<2> {
        EMMC_RST_N_W::new(self)
    }
    #[doc = "Bit 3 - Output Enable (OE) control for EMMC Device Reset signal (card_emmc_reset_n). Values: - 0x1 (ENABLE): OE for card_emmc_reset_n is 1 - 0x0 (DISABLE): OE for card_emmc_reset_n is 0"]
    #[inline(always)]
    #[must_use]
    pub fn emmc_rst_n_oe(&mut self) -> EMMC_RST_N_OE_W<3> {
        EMMC_RST_N_OE_W::new(self)
    }
    #[doc = "Bit 9 - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: - 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks - 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_algo_sel(&mut self) -> CQE_ALGO_SEL_W<9> {
        CQE_ALGO_SEL_W::new(self)
    }
    #[doc = "Bit 10 - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: - 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers - 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_prefetch_disable(&mut self) -> CQE_PREFETCH_DISABLE_W<10> {
        CQE_PREFETCH_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eMMC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_ctrl_r](index.html) module"]
pub struct EMMC_CTRL_R_SPEC;
impl crate::RegisterSpec for EMMC_CTRL_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [emmc_ctrl_r::R](R) reader structure"]
impl crate::Readable for EMMC_CTRL_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_ctrl_r::W](W) writer structure"]
impl crate::Writable for EMMC_CTRL_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMMC_CTRL_R to value 0x0c"]
impl crate::Resettable for EMMC_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
