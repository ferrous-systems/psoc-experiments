#[doc = r"Register block"]
#[repr(C)]
pub struct AREF {
    #[doc = "0x00 - global AREF control"]
    pub aref_ctrl: AREF_CTRL,
}
#[doc = "AREF_CTRL (rw) register accessor: an alias for `Reg<AREF_CTRL_SPEC>`"]
pub type AREF_CTRL = crate::Reg<aref_ctrl::AREF_CTRL_SPEC>;
#[doc = "global AREF control"]
pub mod aref_ctrl;
