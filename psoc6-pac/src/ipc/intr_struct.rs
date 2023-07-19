#[doc = r"Register block"]
#[repr(C)]
pub struct INTR_STRUCT {
    #[doc = "0x00 - Interrupt"]
    pub intr: INTR,
    #[doc = "0x04 - Interrupt set"]
    pub intr_set: INTR_SET,
    #[doc = "0x08 - Interrupt mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x0c - Interrupt masked"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked"]
pub mod intr_masked;
