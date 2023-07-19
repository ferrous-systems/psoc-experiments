#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Profile control"]
    pub ctl: CTL,
    #[doc = "0x04 - Profile status"]
    pub status: STATUS,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Profile command"]
    pub cmd: CMD,
    _reserved3: [u8; 0x07ac],
    #[doc = "0x7c0 - Profile interrupt"]
    pub intr: INTR,
    #[doc = "0x7c4 - Profile interrupt set"]
    pub intr_set: INTR_SET,
    #[doc = "0x7c8 - Profile interrupt mask"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x7cc - Profile interrupt masked"]
    pub intr_masked: INTR_MASKED,
    _reserved7: [u8; 0x30],
    #[doc = "0x800..0x80c - Profile counter structure"]
    pub cnt_struct0: CNT_STRUCT,
    _reserved8: [u8; 0x04],
    #[doc = "0x810..0x81c - Profile counter structure"]
    pub cnt_struct1: CNT_STRUCT,
    _reserved9: [u8; 0x04],
    #[doc = "0x820..0x82c - Profile counter structure"]
    pub cnt_struct2: CNT_STRUCT,
    _reserved10: [u8; 0x04],
    #[doc = "0x830..0x83c - Profile counter structure"]
    pub cnt_struct3: CNT_STRUCT,
    _reserved11: [u8; 0x04],
    #[doc = "0x840..0x84c - Profile counter structure"]
    pub cnt_struct4: CNT_STRUCT,
    _reserved12: [u8; 0x04],
    #[doc = "0x850..0x85c - Profile counter structure"]
    pub cnt_struct5: CNT_STRUCT,
    _reserved13: [u8; 0x04],
    #[doc = "0x860..0x86c - Profile counter structure"]
    pub cnt_struct6: CNT_STRUCT,
    _reserved14: [u8; 0x04],
    #[doc = "0x870..0x87c - Profile counter structure"]
    pub cnt_struct7: CNT_STRUCT,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Profile control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Profile status"]
pub mod status;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Profile command"]
pub mod cmd;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Profile interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Profile interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Profile interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Profile interrupt masked"]
pub mod intr_masked;
#[doc = "Profile counter structure"]
pub use self::cnt_struct::CNT_STRUCT;
#[doc = r"Cluster"]
#[doc = "Profile counter structure"]
pub mod cnt_struct;
