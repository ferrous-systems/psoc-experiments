#[doc = r"Register block"]
#[repr(C)]
pub struct SMPU_STRUCT {
    #[doc = "0x00 - SMPU region address 0 (slave structure)"]
    pub addr0: ADDR0,
    #[doc = "0x04 - SMPU region attributes 0 (slave structure)"]
    pub att0: ATT0,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - SMPU region address 1 (master structure)"]
    pub addr1: ADDR1,
    #[doc = "0x24 - SMPU region attributes 1 (master structure)"]
    pub att1: ATT1,
}
#[doc = "ADDR0 (rw) register accessor: an alias for `Reg<ADDR0_SPEC>`"]
pub type ADDR0 = crate::Reg<addr0::ADDR0_SPEC>;
#[doc = "SMPU region address 0 (slave structure)"]
pub mod addr0;
#[doc = "ATT0 (rw) register accessor: an alias for `Reg<ATT0_SPEC>`"]
pub type ATT0 = crate::Reg<att0::ATT0_SPEC>;
#[doc = "SMPU region attributes 0 (slave structure)"]
pub mod att0;
#[doc = "ADDR1 (r) register accessor: an alias for `Reg<ADDR1_SPEC>`"]
pub type ADDR1 = crate::Reg<addr1::ADDR1_SPEC>;
#[doc = "SMPU region address 1 (master structure)"]
pub mod addr1;
#[doc = "ATT1 (rw) register accessor: an alias for `Reg<ATT1_SPEC>`"]
pub type ATT1 = crate::Reg<att1::ATT1_SPEC>;
#[doc = "SMPU region attributes 1 (master structure)"]
pub mod att1;
