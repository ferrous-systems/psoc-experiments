#[doc = r"Register block"]
#[repr(C)]
pub struct MPU {
    #[doc = "0x00 - Master control"]
    pub ms_ctl: MS_CTL,
    #[doc = "0x04..0x200 - Master control read mirror"]
    pub ms_ctl_read_mir: [MS_CTL_READ_MIR; 127],
    #[doc = "0x200..0x208 - MPU structure"]
    pub mpu_struct0: MPU_STRUCT,
    _reserved3: [u8; 0x18],
    #[doc = "0x220..0x228 - MPU structure"]
    pub mpu_struct1: MPU_STRUCT,
    _reserved4: [u8; 0x18],
    #[doc = "0x240..0x248 - MPU structure"]
    pub mpu_struct2: MPU_STRUCT,
    _reserved5: [u8; 0x18],
    #[doc = "0x260..0x268 - MPU structure"]
    pub mpu_struct3: MPU_STRUCT,
    _reserved6: [u8; 0x18],
    #[doc = "0x280..0x288 - MPU structure"]
    pub mpu_struct4: MPU_STRUCT,
    _reserved7: [u8; 0x18],
    #[doc = "0x2a0..0x2a8 - MPU structure"]
    pub mpu_struct5: MPU_STRUCT,
    _reserved8: [u8; 0x18],
    #[doc = "0x2c0..0x2c8 - MPU structure"]
    pub mpu_struct6: MPU_STRUCT,
    _reserved9: [u8; 0x18],
    #[doc = "0x2e0..0x2e8 - MPU structure"]
    pub mpu_struct7: MPU_STRUCT,
}
#[doc = "MS_CTL (rw) register accessor: an alias for `Reg<MS_CTL_SPEC>`"]
pub type MS_CTL = crate::Reg<ms_ctl::MS_CTL_SPEC>;
#[doc = "Master control"]
pub mod ms_ctl;
#[doc = "MS_CTL_READ_MIR (r) register accessor: an alias for `Reg<MS_CTL_READ_MIR_SPEC>`"]
pub type MS_CTL_READ_MIR = crate::Reg<ms_ctl_read_mir::MS_CTL_READ_MIR_SPEC>;
#[doc = "Master control read mirror"]
pub mod ms_ctl_read_mir;
#[doc = "MPU structure"]
pub use self::mpu_struct::MPU_STRUCT;
#[doc = r"Cluster"]
#[doc = "MPU structure"]
pub mod mpu_struct;
