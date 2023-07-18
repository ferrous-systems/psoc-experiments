#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    #[doc = "0x00 - IPC acquire"]
    pub acquire: ACQUIRE,
    #[doc = "0x04 - IPC release"]
    pub release: RELEASE,
    #[doc = "0x08 - IPC notification"]
    pub notify: NOTIFY,
    #[doc = "0x0c - IPC data 0"]
    pub data0: DATA0,
    #[doc = "0x10 - IPC data 1"]
    pub data1: DATA1,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - IPC lock status"]
    pub lock_status: LOCK_STATUS,
}
#[doc = "ACQUIRE (r) register accessor: an alias for `Reg<ACQUIRE_SPEC>`"]
pub type ACQUIRE = crate::Reg<acquire::ACQUIRE_SPEC>;
#[doc = "IPC acquire"]
pub mod acquire;
#[doc = "RELEASE (w) register accessor: an alias for `Reg<RELEASE_SPEC>`"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "IPC release"]
pub mod release;
#[doc = "NOTIFY (w) register accessor: an alias for `Reg<NOTIFY_SPEC>`"]
pub type NOTIFY = crate::Reg<notify::NOTIFY_SPEC>;
#[doc = "IPC notification"]
pub mod notify;
#[doc = "DATA0 (rw) register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "IPC data 0"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "IPC data 1"]
pub mod data1;
#[doc = "LOCK_STATUS (r) register accessor: an alias for `Reg<LOCK_STATUS_SPEC>`"]
pub type LOCK_STATUS = crate::Reg<lock_status::LOCK_STATUS_SPEC>;
#[doc = "IPC lock status"]
pub mod lock_status;
