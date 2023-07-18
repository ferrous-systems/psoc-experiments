#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Device region base address"]
    pub addr: ADDR,
    #[doc = "0x0c - Device region mask"]
    pub mask: MASK,
    _reserved3: [u8; 0x10],
    #[doc = "0x20 - Address control"]
    pub addr_ctl: ADDR_CTL,
    _reserved4: [u8; 0x1c],
    #[doc = "0x40 - Read command control"]
    pub rd_cmd_ctl: RD_CMD_CTL,
    #[doc = "0x44 - Read address control"]
    pub rd_addr_ctl: RD_ADDR_CTL,
    #[doc = "0x48 - Read mode control"]
    pub rd_mode_ctl: RD_MODE_CTL,
    #[doc = "0x4c - Read dummy control"]
    pub rd_dummy_ctl: RD_DUMMY_CTL,
    #[doc = "0x50 - Read data control"]
    pub rd_data_ctl: RD_DATA_CTL,
    _reserved9: [u8; 0x0c],
    #[doc = "0x60 - Write command control"]
    pub wr_cmd_ctl: WR_CMD_CTL,
    #[doc = "0x64 - Write address control"]
    pub wr_addr_ctl: WR_ADDR_CTL,
    #[doc = "0x68 - Write mode control"]
    pub wr_mode_ctl: WR_MODE_CTL,
    #[doc = "0x6c - Write dummy control"]
    pub wr_dummy_ctl: WR_DUMMY_CTL,
    #[doc = "0x70 - Write data control"]
    pub wr_data_ctl: WR_DATA_CTL,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Device region base address"]
pub mod addr;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Device region mask"]
pub mod mask;
#[doc = "ADDR_CTL (rw) register accessor: an alias for `Reg<ADDR_CTL_SPEC>`"]
pub type ADDR_CTL = crate::Reg<addr_ctl::ADDR_CTL_SPEC>;
#[doc = "Address control"]
pub mod addr_ctl;
#[doc = "RD_CMD_CTL (rw) register accessor: an alias for `Reg<RD_CMD_CTL_SPEC>`"]
pub type RD_CMD_CTL = crate::Reg<rd_cmd_ctl::RD_CMD_CTL_SPEC>;
#[doc = "Read command control"]
pub mod rd_cmd_ctl;
#[doc = "RD_ADDR_CTL (rw) register accessor: an alias for `Reg<RD_ADDR_CTL_SPEC>`"]
pub type RD_ADDR_CTL = crate::Reg<rd_addr_ctl::RD_ADDR_CTL_SPEC>;
#[doc = "Read address control"]
pub mod rd_addr_ctl;
#[doc = "RD_MODE_CTL (rw) register accessor: an alias for `Reg<RD_MODE_CTL_SPEC>`"]
pub type RD_MODE_CTL = crate::Reg<rd_mode_ctl::RD_MODE_CTL_SPEC>;
#[doc = "Read mode control"]
pub mod rd_mode_ctl;
#[doc = "RD_DUMMY_CTL (rw) register accessor: an alias for `Reg<RD_DUMMY_CTL_SPEC>`"]
pub type RD_DUMMY_CTL = crate::Reg<rd_dummy_ctl::RD_DUMMY_CTL_SPEC>;
#[doc = "Read dummy control"]
pub mod rd_dummy_ctl;
#[doc = "RD_DATA_CTL (rw) register accessor: an alias for `Reg<RD_DATA_CTL_SPEC>`"]
pub type RD_DATA_CTL = crate::Reg<rd_data_ctl::RD_DATA_CTL_SPEC>;
#[doc = "Read data control"]
pub mod rd_data_ctl;
#[doc = "WR_CMD_CTL (rw) register accessor: an alias for `Reg<WR_CMD_CTL_SPEC>`"]
pub type WR_CMD_CTL = crate::Reg<wr_cmd_ctl::WR_CMD_CTL_SPEC>;
#[doc = "Write command control"]
pub mod wr_cmd_ctl;
#[doc = "WR_ADDR_CTL (rw) register accessor: an alias for `Reg<WR_ADDR_CTL_SPEC>`"]
pub type WR_ADDR_CTL = crate::Reg<wr_addr_ctl::WR_ADDR_CTL_SPEC>;
#[doc = "Write address control"]
pub mod wr_addr_ctl;
#[doc = "WR_MODE_CTL (rw) register accessor: an alias for `Reg<WR_MODE_CTL_SPEC>`"]
pub type WR_MODE_CTL = crate::Reg<wr_mode_ctl::WR_MODE_CTL_SPEC>;
#[doc = "Write mode control"]
pub mod wr_mode_ctl;
#[doc = "WR_DUMMY_CTL (rw) register accessor: an alias for `Reg<WR_DUMMY_CTL_SPEC>`"]
pub type WR_DUMMY_CTL = crate::Reg<wr_dummy_ctl::WR_DUMMY_CTL_SPEC>;
#[doc = "Write dummy control"]
pub mod wr_dummy_ctl;
#[doc = "WR_DATA_CTL (rw) register accessor: an alias for `Reg<WR_DATA_CTL_SPEC>`"]
pub type WR_DATA_CTL = crate::Reg<wr_data_ctl::WR_DATA_CTL_SPEC>;
#[doc = "Write data control"]
pub mod wr_data_ctl;
