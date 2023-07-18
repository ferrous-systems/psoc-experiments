#[doc = r"Register block"]
#[repr(C)]
pub struct USBLPM {
    #[doc = "0x00 - Power Control Register"]
    pub power_ctl: POWER_CTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - USB IO Control Register"]
    pub usbio_ctl: USBIO_CTL,
    #[doc = "0x0c - Flow Control Register"]
    pub flow_ctl: FLOW_CTL,
    #[doc = "0x10 - LPM Control Register"]
    pub lpm_ctl: LPM_CTL,
    #[doc = "0x14 - LPM Status register"]
    pub lpm_stat: LPM_STAT,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - USB SOF, BUS RESET and EP0 Interrupt Status"]
    pub intr_sie: INTR_SIE,
    #[doc = "0x24 - USB SOF, BUS RESET and EP0 Interrupt Set"]
    pub intr_sie_set: INTR_SIE_SET,
    #[doc = "0x28 - USB SOF, BUS RESET and EP0 Interrupt Mask"]
    pub intr_sie_mask: INTR_SIE_MASK,
    #[doc = "0x2c - USB SOF, BUS RESET and EP0 Interrupt Masked"]
    pub intr_sie_masked: INTR_SIE_MASKED,
    #[doc = "0x30 - Select interrupt level for each interrupt source"]
    pub intr_lvl_sel: INTR_LVL_SEL,
    #[doc = "0x34 - High priority interrupt Cause register"]
    pub intr_cause_hi: INTR_CAUSE_HI,
    #[doc = "0x38 - Medium priority interrupt Cause register"]
    pub intr_cause_med: INTR_CAUSE_MED,
    #[doc = "0x3c - Low priority interrupt Cause register"]
    pub intr_cause_lo: INTR_CAUSE_LO,
    _reserved13: [u8; 0x30],
    #[doc = "0x70 - DFT control"]
    pub dft_ctl: DFT_CTL,
}
#[doc = "POWER_CTL (rw) register accessor: an alias for `Reg<POWER_CTL_SPEC>`"]
pub type POWER_CTL = crate::Reg<power_ctl::POWER_CTL_SPEC>;
#[doc = "Power Control Register"]
pub mod power_ctl;
#[doc = "USBIO_CTL (rw) register accessor: an alias for `Reg<USBIO_CTL_SPEC>`"]
pub type USBIO_CTL = crate::Reg<usbio_ctl::USBIO_CTL_SPEC>;
#[doc = "USB IO Control Register"]
pub mod usbio_ctl;
#[doc = "FLOW_CTL (rw) register accessor: an alias for `Reg<FLOW_CTL_SPEC>`"]
pub type FLOW_CTL = crate::Reg<flow_ctl::FLOW_CTL_SPEC>;
#[doc = "Flow Control Register"]
pub mod flow_ctl;
#[doc = "LPM_CTL (rw) register accessor: an alias for `Reg<LPM_CTL_SPEC>`"]
pub type LPM_CTL = crate::Reg<lpm_ctl::LPM_CTL_SPEC>;
#[doc = "LPM Control Register"]
pub mod lpm_ctl;
#[doc = "LPM_STAT (r) register accessor: an alias for `Reg<LPM_STAT_SPEC>`"]
pub type LPM_STAT = crate::Reg<lpm_stat::LPM_STAT_SPEC>;
#[doc = "LPM Status register"]
pub mod lpm_stat;
#[doc = "INTR_SIE (rw) register accessor: an alias for `Reg<INTR_SIE_SPEC>`"]
pub type INTR_SIE = crate::Reg<intr_sie::INTR_SIE_SPEC>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Status"]
pub mod intr_sie;
#[doc = "INTR_SIE_SET (rw) register accessor: an alias for `Reg<INTR_SIE_SET_SPEC>`"]
pub type INTR_SIE_SET = crate::Reg<intr_sie_set::INTR_SIE_SET_SPEC>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Set"]
pub mod intr_sie_set;
#[doc = "INTR_SIE_MASK (rw) register accessor: an alias for `Reg<INTR_SIE_MASK_SPEC>`"]
pub type INTR_SIE_MASK = crate::Reg<intr_sie_mask::INTR_SIE_MASK_SPEC>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask"]
pub mod intr_sie_mask;
#[doc = "INTR_SIE_MASKED (r) register accessor: an alias for `Reg<INTR_SIE_MASKED_SPEC>`"]
pub type INTR_SIE_MASKED = crate::Reg<intr_sie_masked::INTR_SIE_MASKED_SPEC>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Masked"]
pub mod intr_sie_masked;
#[doc = "INTR_LVL_SEL (rw) register accessor: an alias for `Reg<INTR_LVL_SEL_SPEC>`"]
pub type INTR_LVL_SEL = crate::Reg<intr_lvl_sel::INTR_LVL_SEL_SPEC>;
#[doc = "Select interrupt level for each interrupt source"]
pub mod intr_lvl_sel;
#[doc = "INTR_CAUSE_HI (r) register accessor: an alias for `Reg<INTR_CAUSE_HI_SPEC>`"]
pub type INTR_CAUSE_HI = crate::Reg<intr_cause_hi::INTR_CAUSE_HI_SPEC>;
#[doc = "High priority interrupt Cause register"]
pub mod intr_cause_hi;
#[doc = "INTR_CAUSE_MED (r) register accessor: an alias for `Reg<INTR_CAUSE_MED_SPEC>`"]
pub type INTR_CAUSE_MED = crate::Reg<intr_cause_med::INTR_CAUSE_MED_SPEC>;
#[doc = "Medium priority interrupt Cause register"]
pub mod intr_cause_med;
#[doc = "INTR_CAUSE_LO (r) register accessor: an alias for `Reg<INTR_CAUSE_LO_SPEC>`"]
pub type INTR_CAUSE_LO = crate::Reg<intr_cause_lo::INTR_CAUSE_LO_SPEC>;
#[doc = "Low priority interrupt Cause register"]
pub mod intr_cause_lo;
#[doc = "DFT_CTL (rw) register accessor: an alias for `Reg<DFT_CTL_SPEC>`"]
pub type DFT_CTL = crate::Reg<dft_ctl::DFT_CTL_SPEC>;
#[doc = "DFT control"]
pub mod dft_ctl;
