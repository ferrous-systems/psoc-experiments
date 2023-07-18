#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    #[doc = "0x00 - Fault control"]
    pub ctl: CTL,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - Fault status"]
    pub status: STATUS,
    #[doc = "0x10..0x20 - Fault data"]
    pub data: [DATA; 4],
    _reserved3: [u8; 0x20],
    #[doc = "0x40 - Fault pending 0"]
    pub pending0: PENDING0,
    #[doc = "0x44 - Fault pending 1"]
    pub pending1: PENDING1,
    #[doc = "0x48 - Fault pending 2"]
    pub pending2: PENDING2,
    _reserved6: [u8; 0x04],
    #[doc = "0x50 - Fault mask 0"]
    pub mask0: MASK0,
    #[doc = "0x54 - Fault mask 1"]
    pub mask1: MASK1,
    #[doc = "0x58 - Fault mask 2"]
    pub mask2: MASK2,
    _reserved9: [u8; 0x64],
    #[doc = "0xc0 - Interrupt"]
    pub intr: INTR,
    #[doc = "0xc4 - Interrupt set"]
    pub intr_set: INTR_SET,
    #[doc = "0xc8 - Interrupt mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0xcc - Interrupt masked"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Fault control"]
pub mod ctl;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Fault status"]
pub mod status;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Fault data"]
pub mod data;
#[doc = "PENDING0 (r) register accessor: an alias for `Reg<PENDING0_SPEC>`"]
pub type PENDING0 = crate::Reg<pending0::PENDING0_SPEC>;
#[doc = "Fault pending 0"]
pub mod pending0;
#[doc = "PENDING1 (r) register accessor: an alias for `Reg<PENDING1_SPEC>`"]
pub type PENDING1 = crate::Reg<pending1::PENDING1_SPEC>;
#[doc = "Fault pending 1"]
pub mod pending1;
#[doc = "PENDING2 (r) register accessor: an alias for `Reg<PENDING2_SPEC>`"]
pub type PENDING2 = crate::Reg<pending2::PENDING2_SPEC>;
#[doc = "Fault pending 2"]
pub mod pending2;
#[doc = "MASK0 (rw) register accessor: an alias for `Reg<MASK0_SPEC>`"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "Fault mask 0"]
pub mod mask0;
#[doc = "MASK1 (rw) register accessor: an alias for `Reg<MASK1_SPEC>`"]
pub type MASK1 = crate::Reg<mask1::MASK1_SPEC>;
#[doc = "Fault mask 1"]
pub mod mask1;
#[doc = "MASK2 (rw) register accessor: an alias for `Reg<MASK2_SPEC>`"]
pub type MASK2 = crate::Reg<mask2::MASK2_SPEC>;
#[doc = "Fault mask 2"]
pub mod mask2;
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
