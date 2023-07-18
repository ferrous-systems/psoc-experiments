#[doc = r"Register block"]
#[repr(C)]
pub struct WRAP {
    #[doc = "0x00 - Top level wrapper control"]
    pub ctl: CTL,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Top level wrapper control"]
pub mod ctl;
