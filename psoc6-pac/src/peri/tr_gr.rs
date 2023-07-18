#[doc = r"Register block"]
#[repr(C)]
pub struct TR_GR {
    #[doc = "0x00..0x400 - Trigger control register"]
    pub tr_ctl: [TR_CTL; 256],
}
#[doc = "TR_CTL (rw) register accessor: an alias for `Reg<TR_CTL_SPEC>`"]
pub type TR_CTL = crate::Reg<tr_ctl::TR_CTL_SPEC>;
#[doc = "Trigger control register"]
pub mod tr_ctl;
