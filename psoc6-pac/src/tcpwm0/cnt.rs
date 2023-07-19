#[doc = r"Register block"]
#[repr(C)]
pub struct CNT {
    #[doc = "0x00 - Counter control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Counter status register"]
    pub status: STATUS,
    #[doc = "0x08 - Counter count register"]
    pub counter: COUNTER,
    #[doc = "0x0c - Counter compare/capture register"]
    pub cc: CC,
    #[doc = "0x10 - Counter buffered compare/capture register"]
    pub cc_buff: CC_BUFF,
    #[doc = "0x14 - Counter period register"]
    pub period: PERIOD,
    #[doc = "0x18 - Counter buffered period register"]
    pub period_buff: PERIOD_BUFF,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Counter trigger control register 0"]
    pub tr_ctrl0: TR_CTRL0,
    #[doc = "0x24 - Counter trigger control register 1"]
    pub tr_ctrl1: TR_CTRL1,
    #[doc = "0x28 - Counter trigger control register 2"]
    pub tr_ctrl2: TR_CTRL2,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Interrupt request register"]
    pub intr: INTR,
    #[doc = "0x34 - Interrupt set request register"]
    pub intr_set: INTR_SET,
    #[doc = "0x38 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x3c - Interrupt masked request register"]
    pub intr_masked: INTR_MASKED,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Counter control register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Counter status register"]
pub mod status;
#[doc = "COUNTER (rw) register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Counter count register"]
pub mod counter;
#[doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Counter compare/capture register"]
pub mod cc;
#[doc = "CC_BUFF (rw) register accessor: an alias for `Reg<CC_BUFF_SPEC>`"]
pub type CC_BUFF = crate::Reg<cc_buff::CC_BUFF_SPEC>;
#[doc = "Counter buffered compare/capture register"]
pub mod cc_buff;
#[doc = "PERIOD (rw) register accessor: an alias for `Reg<PERIOD_SPEC>`"]
pub type PERIOD = crate::Reg<period::PERIOD_SPEC>;
#[doc = "Counter period register"]
pub mod period;
#[doc = "PERIOD_BUFF (rw) register accessor: an alias for `Reg<PERIOD_BUFF_SPEC>`"]
pub type PERIOD_BUFF = crate::Reg<period_buff::PERIOD_BUFF_SPEC>;
#[doc = "Counter buffered period register"]
pub mod period_buff;
#[doc = "TR_CTRL0 (rw) register accessor: an alias for `Reg<TR_CTRL0_SPEC>`"]
pub type TR_CTRL0 = crate::Reg<tr_ctrl0::TR_CTRL0_SPEC>;
#[doc = "Counter trigger control register 0"]
pub mod tr_ctrl0;
#[doc = "TR_CTRL1 (rw) register accessor: an alias for `Reg<TR_CTRL1_SPEC>`"]
pub type TR_CTRL1 = crate::Reg<tr_ctrl1::TR_CTRL1_SPEC>;
#[doc = "Counter trigger control register 1"]
pub mod tr_ctrl1;
#[doc = "TR_CTRL2 (rw) register accessor: an alias for `Reg<TR_CTRL2_SPEC>`"]
pub type TR_CTRL2 = crate::Reg<tr_ctrl2::TR_CTRL2_SPEC>;
#[doc = "Counter trigger control register 2"]
pub mod tr_ctrl2;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
