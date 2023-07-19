#[doc = r"Register block"]
#[repr(C)]
pub struct MCWDT_STRUCT {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Multi-Counter Watchdog Sub-counters 0/1"]
    pub mcwdt_cntlow: MCWDT_CNTLOW,
    #[doc = "0x08 - Multi-Counter Watchdog Sub-counter 2"]
    pub mcwdt_cnthigh: MCWDT_CNTHIGH,
    #[doc = "0x0c - Multi-Counter Watchdog Counter Match Register"]
    pub mcwdt_match: MCWDT_MATCH,
    #[doc = "0x10 - Multi-Counter Watchdog Counter Configuration"]
    pub mcwdt_config: MCWDT_CONFIG,
    #[doc = "0x14 - Multi-Counter Watchdog Counter Control"]
    pub mcwdt_ctl: MCWDT_CTL,
    #[doc = "0x18 - Multi-Counter Watchdog Counter Interrupt Register"]
    pub mcwdt_intr: MCWDT_INTR,
    #[doc = "0x1c - Multi-Counter Watchdog Counter Interrupt Set Register"]
    pub mcwdt_intr_set: MCWDT_INTR_SET,
    #[doc = "0x20 - Multi-Counter Watchdog Counter Interrupt Mask Register"]
    pub mcwdt_intr_mask: MCWDT_INTR_MASK,
    #[doc = "0x24 - Multi-Counter Watchdog Counter Interrupt Masked Register"]
    pub mcwdt_intr_masked: MCWDT_INTR_MASKED,
    #[doc = "0x28 - Multi-Counter Watchdog Counter Lock Register"]
    pub mcwdt_lock: MCWDT_LOCK,
}
#[doc = "MCWDT_CNTLOW (rw) register accessor: an alias for `Reg<MCWDT_CNTLOW_SPEC>`"]
pub type MCWDT_CNTLOW = crate::Reg<mcwdt_cntlow::MCWDT_CNTLOW_SPEC>;
#[doc = "Multi-Counter Watchdog Sub-counters 0/1"]
pub mod mcwdt_cntlow;
#[doc = "MCWDT_CNTHIGH (rw) register accessor: an alias for `Reg<MCWDT_CNTHIGH_SPEC>`"]
pub type MCWDT_CNTHIGH = crate::Reg<mcwdt_cnthigh::MCWDT_CNTHIGH_SPEC>;
#[doc = "Multi-Counter Watchdog Sub-counter 2"]
pub mod mcwdt_cnthigh;
#[doc = "MCWDT_MATCH (rw) register accessor: an alias for `Reg<MCWDT_MATCH_SPEC>`"]
pub type MCWDT_MATCH = crate::Reg<mcwdt_match::MCWDT_MATCH_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Match Register"]
pub mod mcwdt_match;
#[doc = "MCWDT_CONFIG (rw) register accessor: an alias for `Reg<MCWDT_CONFIG_SPEC>`"]
pub type MCWDT_CONFIG = crate::Reg<mcwdt_config::MCWDT_CONFIG_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Configuration"]
pub mod mcwdt_config;
#[doc = "MCWDT_CTL (rw) register accessor: an alias for `Reg<MCWDT_CTL_SPEC>`"]
pub type MCWDT_CTL = crate::Reg<mcwdt_ctl::MCWDT_CTL_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Control"]
pub mod mcwdt_ctl;
#[doc = "MCWDT_INTR (rw) register accessor: an alias for `Reg<MCWDT_INTR_SPEC>`"]
pub type MCWDT_INTR = crate::Reg<mcwdt_intr::MCWDT_INTR_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Register"]
pub mod mcwdt_intr;
#[doc = "MCWDT_INTR_SET (rw) register accessor: an alias for `Reg<MCWDT_INTR_SET_SPEC>`"]
pub type MCWDT_INTR_SET = crate::Reg<mcwdt_intr_set::MCWDT_INTR_SET_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Set Register"]
pub mod mcwdt_intr_set;
#[doc = "MCWDT_INTR_MASK (rw) register accessor: an alias for `Reg<MCWDT_INTR_MASK_SPEC>`"]
pub type MCWDT_INTR_MASK = crate::Reg<mcwdt_intr_mask::MCWDT_INTR_MASK_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Mask Register"]
pub mod mcwdt_intr_mask;
#[doc = "MCWDT_INTR_MASKED (r) register accessor: an alias for `Reg<MCWDT_INTR_MASKED_SPEC>`"]
pub type MCWDT_INTR_MASKED = crate::Reg<mcwdt_intr_masked::MCWDT_INTR_MASKED_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Masked Register"]
pub mod mcwdt_intr_masked;
#[doc = "MCWDT_LOCK (rw) register accessor: an alias for `Reg<MCWDT_LOCK_SPEC>`"]
pub type MCWDT_LOCK = crate::Reg<mcwdt_lock::MCWDT_LOCK_SPEC>;
#[doc = "Multi-Counter Watchdog Counter Lock Register"]
pub mod mcwdt_lock;
