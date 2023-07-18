#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TCPWM control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - TCPWM control clear register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x08 - TCPWM control set register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x0c - TCPWM capture command register"]
    pub cmd_capture: CMD_CAPTURE,
    #[doc = "0x10 - TCPWM reload command register"]
    pub cmd_reload: CMD_RELOAD,
    #[doc = "0x14 - TCPWM stop command register"]
    pub cmd_stop: CMD_STOP,
    #[doc = "0x18 - TCPWM start command register"]
    pub cmd_start: CMD_START,
    #[doc = "0x1c - TCPWM Counter interrupt cause register"]
    pub intr_cause: INTR_CAUSE,
    _reserved8: [u8; 0xe0],
    #[doc = "0x100..0x700 - Timer/Counter/PWM Counter Module"]
    pub cnt: [CNT; 24],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "TCPWM control register"]
pub mod ctrl;
#[doc = "CTRL_CLR (rw) register accessor: an alias for `Reg<CTRL_CLR_SPEC>`"]
pub type CTRL_CLR = crate::Reg<ctrl_clr::CTRL_CLR_SPEC>;
#[doc = "TCPWM control clear register"]
pub mod ctrl_clr;
#[doc = "CTRL_SET (rw) register accessor: an alias for `Reg<CTRL_SET_SPEC>`"]
pub type CTRL_SET = crate::Reg<ctrl_set::CTRL_SET_SPEC>;
#[doc = "TCPWM control set register"]
pub mod ctrl_set;
#[doc = "CMD_CAPTURE (rw) register accessor: an alias for `Reg<CMD_CAPTURE_SPEC>`"]
pub type CMD_CAPTURE = crate::Reg<cmd_capture::CMD_CAPTURE_SPEC>;
#[doc = "TCPWM capture command register"]
pub mod cmd_capture;
#[doc = "CMD_RELOAD (rw) register accessor: an alias for `Reg<CMD_RELOAD_SPEC>`"]
pub type CMD_RELOAD = crate::Reg<cmd_reload::CMD_RELOAD_SPEC>;
#[doc = "TCPWM reload command register"]
pub mod cmd_reload;
#[doc = "CMD_STOP (rw) register accessor: an alias for `Reg<CMD_STOP_SPEC>`"]
pub type CMD_STOP = crate::Reg<cmd_stop::CMD_STOP_SPEC>;
#[doc = "TCPWM stop command register"]
pub mod cmd_stop;
#[doc = "CMD_START (rw) register accessor: an alias for `Reg<CMD_START_SPEC>`"]
pub type CMD_START = crate::Reg<cmd_start::CMD_START_SPEC>;
#[doc = "TCPWM start command register"]
pub mod cmd_start;
#[doc = "INTR_CAUSE (r) register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "TCPWM Counter interrupt cause register"]
pub mod intr_cause;
#[doc = "Timer/Counter/PWM Counter Module"]
pub use self::cnt::CNT;
#[doc = r"Cluster"]
#[doc = "Timer/Counter/PWM Counter Module"]
pub mod cnt;
