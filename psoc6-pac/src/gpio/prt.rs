#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Port output data register"]
    pub out: OUT,
    #[doc = "0x04 - Port output data clear register"]
    pub out_clr: OUT_CLR,
    #[doc = "0x08 - Port output data set register"]
    pub out_set: OUT_SET,
    #[doc = "0x0c - Port output data invert register"]
    pub out_inv: OUT_INV,
    #[doc = "0x10 - Port input state register"]
    pub in_: IN,
    #[doc = "0x14 - Port interrupt status register"]
    pub intr: INTR,
    #[doc = "0x18 - Port interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x1c - Port interrupt masked status register"]
    pub intr_masked: INTR_MASKED,
    #[doc = "0x20 - Port interrupt set register"]
    pub intr_set: INTR_SET,
    _reserved9: [u8; 0x1c],
    #[doc = "0x40 - Port interrupt configuration register"]
    pub intr_cfg: INTR_CFG,
    #[doc = "0x44 - Port configuration register"]
    pub cfg: CFG,
    #[doc = "0x48 - Port input buffer configuration register"]
    pub cfg_in: CFG_IN,
    #[doc = "0x4c - Port output buffer configuration register"]
    pub cfg_out: CFG_OUT,
    #[doc = "0x50 - Port SIO configuration register"]
    pub cfg_sio: CFG_SIO,
    _reserved14: [u8; 0x04],
    #[doc = "0x58 - Port input buffer AUTOLVL configuration register"]
    pub cfg_in_autolvl: CFG_IN_AUTOLVL,
}
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Port output data register"]
pub mod out;
#[doc = "OUT_CLR (rw) register accessor: an alias for `Reg<OUT_CLR_SPEC>`"]
pub type OUT_CLR = crate::Reg<out_clr::OUT_CLR_SPEC>;
#[doc = "Port output data clear register"]
pub mod out_clr;
#[doc = "OUT_SET (rw) register accessor: an alias for `Reg<OUT_SET_SPEC>`"]
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
#[doc = "Port output data set register"]
pub mod out_set;
#[doc = "OUT_INV (rw) register accessor: an alias for `Reg<OUT_INV_SPEC>`"]
pub type OUT_INV = crate::Reg<out_inv::OUT_INV_SPEC>;
#[doc = "Port output data invert register"]
pub mod out_inv;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Port input state register"]
pub mod in_;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Port interrupt status register"]
pub mod intr;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Port interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Port interrupt masked status register"]
pub mod intr_masked;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Port interrupt set register"]
pub mod intr_set;
#[doc = "INTR_CFG (rw) register accessor: an alias for `Reg<INTR_CFG_SPEC>`"]
pub type INTR_CFG = crate::Reg<intr_cfg::INTR_CFG_SPEC>;
#[doc = "Port interrupt configuration register"]
pub mod intr_cfg;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Port configuration register"]
pub mod cfg;
#[doc = "CFG_IN (rw) register accessor: an alias for `Reg<CFG_IN_SPEC>`"]
pub type CFG_IN = crate::Reg<cfg_in::CFG_IN_SPEC>;
#[doc = "Port input buffer configuration register"]
pub mod cfg_in;
#[doc = "CFG_OUT (rw) register accessor: an alias for `Reg<CFG_OUT_SPEC>`"]
pub type CFG_OUT = crate::Reg<cfg_out::CFG_OUT_SPEC>;
#[doc = "Port output buffer configuration register"]
pub mod cfg_out;
#[doc = "CFG_SIO (rw) register accessor: an alias for `Reg<CFG_SIO_SPEC>`"]
pub type CFG_SIO = crate::Reg<cfg_sio::CFG_SIO_SPEC>;
#[doc = "Port SIO configuration register"]
pub mod cfg_sio;
#[doc = "CFG_IN_AUTOLVL (rw) register accessor: an alias for `Reg<CFG_IN_AUTOLVL_SPEC>`"]
pub type CFG_IN_AUTOLVL = crate::Reg<cfg_in_autolvl::CFG_IN_AUTOLVL_SPEC>;
#[doc = "Port input buffer AUTOLVL configuration register"]
pub mod cfg_in_autolvl;
