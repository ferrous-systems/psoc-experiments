#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Control register"]
    pub ctl: CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Synchronization control register"]
    pub sync_ctl: SYNC_CTL,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20..0x40 - LUT component input selection"]
    pub lut_sel: [LUT_SEL; 8],
    #[doc = "0x40..0x60 - LUT component control register"]
    pub lut_ctl: [LUT_CTL; 8],
    _reserved4: [u8; 0x60],
    #[doc = "0xc0 - Data unit component input selection"]
    pub du_sel: DU_SEL,
    #[doc = "0xc4 - Data unit component control register"]
    pub du_ctl: DU_CTL,
    _reserved6: [u8; 0x28],
    #[doc = "0xf0 - Data register"]
    pub data: DATA,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "SYNC_CTL (rw) register accessor: an alias for `Reg<SYNC_CTL_SPEC>`"]
pub type SYNC_CTL = crate::Reg<sync_ctl::SYNC_CTL_SPEC>;
#[doc = "Synchronization control register"]
pub mod sync_ctl;
#[doc = "LUT_SEL (rw) register accessor: an alias for `Reg<LUT_SEL_SPEC>`"]
pub type LUT_SEL = crate::Reg<lut_sel::LUT_SEL_SPEC>;
#[doc = "LUT component input selection"]
pub mod lut_sel;
#[doc = "LUT_CTL (rw) register accessor: an alias for `Reg<LUT_CTL_SPEC>`"]
pub type LUT_CTL = crate::Reg<lut_ctl::LUT_CTL_SPEC>;
#[doc = "LUT component control register"]
pub mod lut_ctl;
#[doc = "DU_SEL (rw) register accessor: an alias for `Reg<DU_SEL_SPEC>`"]
pub type DU_SEL = crate::Reg<du_sel::DU_SEL_SPEC>;
#[doc = "Data unit component input selection"]
pub mod du_sel;
#[doc = "DU_CTL (rw) register accessor: an alias for `Reg<DU_CTL_SPEC>`"]
pub type DU_CTL = crate::Reg<du_ctl::DU_CTL_SPEC>;
#[doc = "Data unit component control register"]
pub mod du_ctl;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;
