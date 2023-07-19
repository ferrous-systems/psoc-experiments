#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Port selection 0"]
    pub port_sel0: PORT_SEL0,
    #[doc = "0x04 - Port selection 1"]
    pub port_sel1: PORT_SEL1,
}
#[doc = "PORT_SEL0 (rw) register accessor: an alias for `Reg<PORT_SEL0_SPEC>`"]
pub type PORT_SEL0 = crate::Reg<port_sel0::PORT_SEL0_SPEC>;
#[doc = "Port selection 0"]
pub mod port_sel0;
#[doc = "PORT_SEL1 (rw) register accessor: an alias for `Reg<PORT_SEL1_SPEC>`"]
pub type PORT_SEL1 = crate::Reg<port_sel1::PORT_SEL1_SPEC>;
#[doc = "Port selection 1"]
pub mod port_sel1;
