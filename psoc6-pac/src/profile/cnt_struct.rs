#[doc = r"Register block"]
#[repr(C)]
pub struct CNT_STRUCT {
    #[doc = "0x00 - Profile counter configuration"]
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Profile counter value"]
    pub cnt: CNT,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Profile counter configuration"]
pub mod ctl;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Profile counter value"]
pub mod cnt;
