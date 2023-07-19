#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Active channels"]
    pub active: ACTIVE,
    _reserved2: [u8; 0x0ff4],
    #[doc = "0x1000..0x1090 - DMA controller channel"]
    pub ch0: CH,
    _reserved3: [u8; 0x70],
    #[doc = "0x1100..0x1190 - DMA controller channel"]
    pub ch1: CH,
    _reserved4: [u8; 0x70],
    #[doc = "0x1200..0x1290 - DMA controller channel"]
    pub ch2: CH,
    _reserved5: [u8; 0x70],
    #[doc = "0x1300..0x1390 - DMA controller channel"]
    pub ch3: CH,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "ACTIVE (r) register accessor: an alias for `Reg<ACTIVE_SPEC>`"]
pub type ACTIVE = crate::Reg<active::ACTIVE_SPEC>;
#[doc = "Active channels"]
pub mod active;
#[doc = "DMA controller channel"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "DMA controller channel"]
pub mod ch;
