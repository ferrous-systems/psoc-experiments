#[doc = r"Register block"]
#[repr(C)]
pub struct PPU_PR {
    #[doc = "0x00 - Slave region, base address"]
    pub sl_addr: SL_ADDR,
    #[doc = "0x04 - Slave region, size"]
    pub sl_size: SL_SIZE,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Slave attributes 0"]
    pub sl_att0: SL_ATT0,
    #[doc = "0x14 - Slave attributes 1"]
    pub sl_att1: SL_ATT1,
    #[doc = "0x18 - Slave attributes 2"]
    pub sl_att2: SL_ATT2,
    #[doc = "0x1c - Slave attributes 3"]
    pub sl_att3: SL_ATT3,
    #[doc = "0x20 - Master region, base address"]
    pub ms_addr: MS_ADDR,
    #[doc = "0x24 - Master region, size"]
    pub ms_size: MS_SIZE,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - Master attributes 0"]
    pub ms_att0: MS_ATT0,
    #[doc = "0x34 - Master attributes 1"]
    pub ms_att1: MS_ATT1,
    #[doc = "0x38 - Master attributes 2"]
    pub ms_att2: MS_ATT2,
    #[doc = "0x3c - Master attributes 3"]
    pub ms_att3: MS_ATT3,
}
#[doc = "SL_ADDR (rw) register accessor: an alias for `Reg<SL_ADDR_SPEC>`"]
pub type SL_ADDR = crate::Reg<sl_addr::SL_ADDR_SPEC>;
#[doc = "Slave region, base address"]
pub mod sl_addr;
#[doc = "SL_SIZE (rw) register accessor: an alias for `Reg<SL_SIZE_SPEC>`"]
pub type SL_SIZE = crate::Reg<sl_size::SL_SIZE_SPEC>;
#[doc = "Slave region, size"]
pub mod sl_size;
#[doc = "SL_ATT0 (rw) register accessor: an alias for `Reg<SL_ATT0_SPEC>`"]
pub type SL_ATT0 = crate::Reg<sl_att0::SL_ATT0_SPEC>;
#[doc = "Slave attributes 0"]
pub mod sl_att0;
#[doc = "SL_ATT1 (rw) register accessor: an alias for `Reg<SL_ATT1_SPEC>`"]
pub type SL_ATT1 = crate::Reg<sl_att1::SL_ATT1_SPEC>;
#[doc = "Slave attributes 1"]
pub mod sl_att1;
#[doc = "SL_ATT2 (rw) register accessor: an alias for `Reg<SL_ATT2_SPEC>`"]
pub type SL_ATT2 = crate::Reg<sl_att2::SL_ATT2_SPEC>;
#[doc = "Slave attributes 2"]
pub mod sl_att2;
#[doc = "SL_ATT3 (rw) register accessor: an alias for `Reg<SL_ATT3_SPEC>`"]
pub type SL_ATT3 = crate::Reg<sl_att3::SL_ATT3_SPEC>;
#[doc = "Slave attributes 3"]
pub mod sl_att3;
#[doc = "MS_ADDR (r) register accessor: an alias for `Reg<MS_ADDR_SPEC>`"]
pub type MS_ADDR = crate::Reg<ms_addr::MS_ADDR_SPEC>;
#[doc = "Master region, base address"]
pub mod ms_addr;
#[doc = "MS_SIZE (r) register accessor: an alias for `Reg<MS_SIZE_SPEC>`"]
pub type MS_SIZE = crate::Reg<ms_size::MS_SIZE_SPEC>;
#[doc = "Master region, size"]
pub mod ms_size;
#[doc = "MS_ATT0 (rw) register accessor: an alias for `Reg<MS_ATT0_SPEC>`"]
pub type MS_ATT0 = crate::Reg<ms_att0::MS_ATT0_SPEC>;
#[doc = "Master attributes 0"]
pub mod ms_att0;
#[doc = "MS_ATT1 (rw) register accessor: an alias for `Reg<MS_ATT1_SPEC>`"]
pub type MS_ATT1 = crate::Reg<ms_att1::MS_ATT1_SPEC>;
#[doc = "Master attributes 1"]
pub mod ms_att1;
#[doc = "MS_ATT2 (rw) register accessor: an alias for `Reg<MS_ATT2_SPEC>`"]
pub type MS_ATT2 = crate::Reg<ms_att2::MS_ATT2_SPEC>;
#[doc = "Master attributes 2"]
pub mod ms_att2;
#[doc = "MS_ATT3 (rw) register accessor: an alias for `Reg<MS_ATT3_SPEC>`"]
pub type MS_ATT3 = crate::Reg<ms_att3::MS_ATT3_SPEC>;
#[doc = "Master attributes 3"]
pub mod ms_att3;
