#[doc = r"Register block"]
#[repr(C)]
pub struct MPU_STRUCT {
    #[doc = "0x00 - MPU region address"]
    pub addr: ADDR,
    #[doc = "0x04 - MPU region attrributes"]
    pub att: ATT,
}
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "MPU region address"]
pub mod addr;
#[doc = "ATT (rw) register accessor: an alias for `Reg<ATT_SPEC>`"]
pub type ATT = crate::Reg<att::ATT_SPEC>;
#[doc = "MPU region attrributes"]
pub mod att;
