#[doc = r"Register block"]
#[repr(C)]
pub struct GR {
    #[doc = "0x00 - Clock control"]
    pub clock_ctl: CLOCK_CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Slave control"]
    pub sl_ctl: SL_CTL,
}
#[doc = "CLOCK_CTL (rw) register accessor: an alias for `Reg<CLOCK_CTL_SPEC>`"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "SL_CTL (rw) register accessor: an alias for `Reg<SL_CTL_SPEC>`"]
pub type SL_CTL = crate::Reg<sl_ctl::SL_CTL_SPEC>;
#[doc = "Slave control"]
pub mod sl_ctl;
