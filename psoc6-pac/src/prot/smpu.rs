#[doc = r"Register block"]
#[repr(C)]
pub struct SMPU {
    #[doc = "0x00 - Master 0 protection context control"]
    pub ms0_ctl: MS0_CTL,
    #[doc = "0x04 - Master 1 protection context control"]
    pub ms1_ctl: MS1_CTL,
    #[doc = "0x08 - Master 2 protection context control"]
    pub ms2_ctl: MS2_CTL,
    #[doc = "0x0c - Master 3 protection context control"]
    pub ms3_ctl: MS3_CTL,
    #[doc = "0x10 - Master 4 protection context control"]
    pub ms4_ctl: MS4_CTL,
    #[doc = "0x14 - Master 5 protection context control"]
    pub ms5_ctl: MS5_CTL,
    #[doc = "0x18 - Master 6 protection context control"]
    pub ms6_ctl: MS6_CTL,
    #[doc = "0x1c - Master 7 protection context control"]
    pub ms7_ctl: MS7_CTL,
    #[doc = "0x20 - Master 8 protection context control"]
    pub ms8_ctl: MS8_CTL,
    #[doc = "0x24 - Master 9 protection context control"]
    pub ms9_ctl: MS9_CTL,
    #[doc = "0x28 - Master 10 protection context control"]
    pub ms10_ctl: MS10_CTL,
    #[doc = "0x2c - Master 11 protection context control"]
    pub ms11_ctl: MS11_CTL,
    #[doc = "0x30 - Master 12 protection context control"]
    pub ms12_ctl: MS12_CTL,
    #[doc = "0x34 - Master 13 protection context control"]
    pub ms13_ctl: MS13_CTL,
    #[doc = "0x38 - Master 14 protection context control"]
    pub ms14_ctl: MS14_CTL,
    #[doc = "0x3c - Master 15 protection context control"]
    pub ms15_ctl: MS15_CTL,
    _reserved16: [u8; 0x1fc0],
    #[doc = "0x2000..0x2028 - SMPU structure"]
    pub smpu_struct0: SMPU_STRUCT,
    _reserved17: [u8; 0x18],
    #[doc = "0x2040..0x2068 - SMPU structure"]
    pub smpu_struct1: SMPU_STRUCT,
    _reserved18: [u8; 0x18],
    #[doc = "0x2080..0x20a8 - SMPU structure"]
    pub smpu_struct2: SMPU_STRUCT,
    _reserved19: [u8; 0x18],
    #[doc = "0x20c0..0x20e8 - SMPU structure"]
    pub smpu_struct3: SMPU_STRUCT,
    _reserved20: [u8; 0x18],
    #[doc = "0x2100..0x2128 - SMPU structure"]
    pub smpu_struct4: SMPU_STRUCT,
    _reserved21: [u8; 0x18],
    #[doc = "0x2140..0x2168 - SMPU structure"]
    pub smpu_struct5: SMPU_STRUCT,
    _reserved22: [u8; 0x18],
    #[doc = "0x2180..0x21a8 - SMPU structure"]
    pub smpu_struct6: SMPU_STRUCT,
    _reserved23: [u8; 0x18],
    #[doc = "0x21c0..0x21e8 - SMPU structure"]
    pub smpu_struct7: SMPU_STRUCT,
    _reserved24: [u8; 0x18],
    #[doc = "0x2200..0x2228 - SMPU structure"]
    pub smpu_struct8: SMPU_STRUCT,
    _reserved25: [u8; 0x18],
    #[doc = "0x2240..0x2268 - SMPU structure"]
    pub smpu_struct9: SMPU_STRUCT,
    _reserved26: [u8; 0x18],
    #[doc = "0x2280..0x22a8 - SMPU structure"]
    pub smpu_struct10: SMPU_STRUCT,
    _reserved27: [u8; 0x18],
    #[doc = "0x22c0..0x22e8 - SMPU structure"]
    pub smpu_struct11: SMPU_STRUCT,
    _reserved28: [u8; 0x18],
    #[doc = "0x2300..0x2328 - SMPU structure"]
    pub smpu_struct12: SMPU_STRUCT,
    _reserved29: [u8; 0x18],
    #[doc = "0x2340..0x2368 - SMPU structure"]
    pub smpu_struct13: SMPU_STRUCT,
    _reserved30: [u8; 0x18],
    #[doc = "0x2380..0x23a8 - SMPU structure"]
    pub smpu_struct14: SMPU_STRUCT,
    _reserved31: [u8; 0x18],
    #[doc = "0x23c0..0x23e8 - SMPU structure"]
    pub smpu_struct15: SMPU_STRUCT,
}
#[doc = "MS0_CTL (rw) register accessor: an alias for `Reg<MS0_CTL_SPEC>`"]
pub type MS0_CTL = crate::Reg<ms0_ctl::MS0_CTL_SPEC>;
#[doc = "Master 0 protection context control"]
pub mod ms0_ctl;
#[doc = "MS1_CTL (rw) register accessor: an alias for `Reg<MS1_CTL_SPEC>`"]
pub type MS1_CTL = crate::Reg<ms1_ctl::MS1_CTL_SPEC>;
#[doc = "Master 1 protection context control"]
pub mod ms1_ctl;
#[doc = "MS2_CTL (rw) register accessor: an alias for `Reg<MS2_CTL_SPEC>`"]
pub type MS2_CTL = crate::Reg<ms2_ctl::MS2_CTL_SPEC>;
#[doc = "Master 2 protection context control"]
pub mod ms2_ctl;
#[doc = "MS3_CTL (rw) register accessor: an alias for `Reg<MS3_CTL_SPEC>`"]
pub type MS3_CTL = crate::Reg<ms3_ctl::MS3_CTL_SPEC>;
#[doc = "Master 3 protection context control"]
pub mod ms3_ctl;
#[doc = "MS4_CTL (rw) register accessor: an alias for `Reg<MS4_CTL_SPEC>`"]
pub type MS4_CTL = crate::Reg<ms4_ctl::MS4_CTL_SPEC>;
#[doc = "Master 4 protection context control"]
pub mod ms4_ctl;
#[doc = "MS5_CTL (rw) register accessor: an alias for `Reg<MS5_CTL_SPEC>`"]
pub type MS5_CTL = crate::Reg<ms5_ctl::MS5_CTL_SPEC>;
#[doc = "Master 5 protection context control"]
pub mod ms5_ctl;
#[doc = "MS6_CTL (rw) register accessor: an alias for `Reg<MS6_CTL_SPEC>`"]
pub type MS6_CTL = crate::Reg<ms6_ctl::MS6_CTL_SPEC>;
#[doc = "Master 6 protection context control"]
pub mod ms6_ctl;
#[doc = "MS7_CTL (rw) register accessor: an alias for `Reg<MS7_CTL_SPEC>`"]
pub type MS7_CTL = crate::Reg<ms7_ctl::MS7_CTL_SPEC>;
#[doc = "Master 7 protection context control"]
pub mod ms7_ctl;
#[doc = "MS8_CTL (rw) register accessor: an alias for `Reg<MS8_CTL_SPEC>`"]
pub type MS8_CTL = crate::Reg<ms8_ctl::MS8_CTL_SPEC>;
#[doc = "Master 8 protection context control"]
pub mod ms8_ctl;
#[doc = "MS9_CTL (rw) register accessor: an alias for `Reg<MS9_CTL_SPEC>`"]
pub type MS9_CTL = crate::Reg<ms9_ctl::MS9_CTL_SPEC>;
#[doc = "Master 9 protection context control"]
pub mod ms9_ctl;
#[doc = "MS10_CTL (rw) register accessor: an alias for `Reg<MS10_CTL_SPEC>`"]
pub type MS10_CTL = crate::Reg<ms10_ctl::MS10_CTL_SPEC>;
#[doc = "Master 10 protection context control"]
pub mod ms10_ctl;
#[doc = "MS11_CTL (rw) register accessor: an alias for `Reg<MS11_CTL_SPEC>`"]
pub type MS11_CTL = crate::Reg<ms11_ctl::MS11_CTL_SPEC>;
#[doc = "Master 11 protection context control"]
pub mod ms11_ctl;
#[doc = "MS12_CTL (rw) register accessor: an alias for `Reg<MS12_CTL_SPEC>`"]
pub type MS12_CTL = crate::Reg<ms12_ctl::MS12_CTL_SPEC>;
#[doc = "Master 12 protection context control"]
pub mod ms12_ctl;
#[doc = "MS13_CTL (rw) register accessor: an alias for `Reg<MS13_CTL_SPEC>`"]
pub type MS13_CTL = crate::Reg<ms13_ctl::MS13_CTL_SPEC>;
#[doc = "Master 13 protection context control"]
pub mod ms13_ctl;
#[doc = "MS14_CTL (rw) register accessor: an alias for `Reg<MS14_CTL_SPEC>`"]
pub type MS14_CTL = crate::Reg<ms14_ctl::MS14_CTL_SPEC>;
#[doc = "Master 14 protection context control"]
pub mod ms14_ctl;
#[doc = "MS15_CTL (rw) register accessor: an alias for `Reg<MS15_CTL_SPEC>`"]
pub type MS15_CTL = crate::Reg<ms15_ctl::MS15_CTL_SPEC>;
#[doc = "Master 15 protection context control"]
pub mod ms15_ctl;
#[doc = "SMPU structure"]
pub use self::smpu_struct::SMPU_STRUCT;
#[doc = r"Cluster"]
#[doc = "SMPU structure"]
pub mod smpu_struct;
