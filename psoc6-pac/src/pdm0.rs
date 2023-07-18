#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Clock control"]
    pub clock_ctl: CLOCK_CTL,
    #[doc = "0x14 - Mode control"]
    pub mode_ctl: MODE_CTL,
    #[doc = "0x18 - Data control"]
    pub data_ctl: DATA_CTL,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - Command"]
    pub cmd: CMD,
    _reserved5: [u8; 0x1c],
    #[doc = "0x40 - Trigger control"]
    pub tr_ctl: TR_CTL,
    _reserved6: [u8; 0x02bc],
    #[doc = "0x300 - RX FIFO control"]
    pub rx_fifo_ctl: RX_FIFO_CTL,
    #[doc = "0x304 - RX FIFO status"]
    pub rx_fifo_status: RX_FIFO_STATUS,
    #[doc = "0x308 - RX FIFO read"]
    pub rx_fifo_rd: RX_FIFO_RD,
    #[doc = "0x30c - RX FIFO silent read"]
    pub rx_fifo_rd_silent: RX_FIFO_RD_SILENT,
    _reserved10: [u8; 0x0bf0],
    #[doc = "0xf00 - Interrupt register"]
    pub intr: INTR,
    #[doc = "0xf04 - Interrupt set register"]
    pub intr_set: INTR_SET,
    #[doc = "0xf08 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0xf0c - Interrupt masked register"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CLOCK_CTL (rw) register accessor: an alias for `Reg<CLOCK_CTL_SPEC>`"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "MODE_CTL (rw) register accessor: an alias for `Reg<MODE_CTL_SPEC>`"]
pub type MODE_CTL = crate::Reg<mode_ctl::MODE_CTL_SPEC>;
#[doc = "Mode control"]
pub mod mode_ctl;
#[doc = "DATA_CTL (rw) register accessor: an alias for `Reg<DATA_CTL_SPEC>`"]
pub type DATA_CTL = crate::Reg<data_ctl::DATA_CTL_SPEC>;
#[doc = "Data control"]
pub mod data_ctl;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command"]
pub mod cmd;
#[doc = "TR_CTL (rw) register accessor: an alias for `Reg<TR_CTL_SPEC>`"]
pub type TR_CTL = crate::Reg<tr_ctl::TR_CTL_SPEC>;
#[doc = "Trigger control"]
pub mod tr_ctl;
#[doc = "RX_FIFO_CTL (rw) register accessor: an alias for `Reg<RX_FIFO_CTL_SPEC>`"]
pub type RX_FIFO_CTL = crate::Reg<rx_fifo_ctl::RX_FIFO_CTL_SPEC>;
#[doc = "RX FIFO control"]
pub mod rx_fifo_ctl;
#[doc = "RX_FIFO_STATUS (r) register accessor: an alias for `Reg<RX_FIFO_STATUS_SPEC>`"]
pub type RX_FIFO_STATUS = crate::Reg<rx_fifo_status::RX_FIFO_STATUS_SPEC>;
#[doc = "RX FIFO status"]
pub mod rx_fifo_status;
#[doc = "RX_FIFO_RD (r) register accessor: an alias for `Reg<RX_FIFO_RD_SPEC>`"]
pub type RX_FIFO_RD = crate::Reg<rx_fifo_rd::RX_FIFO_RD_SPEC>;
#[doc = "RX FIFO read"]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT (r) register accessor: an alias for `Reg<RX_FIFO_RD_SILENT_SPEC>`"]
pub type RX_FIFO_RD_SILENT = crate::Reg<rx_fifo_rd_silent::RX_FIFO_RD_SILENT_SPEC>;
#[doc = "RX FIFO silent read"]
pub mod rx_fifo_rd_silent;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked register"]
pub mod intr_masked;
