#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x23e8 - SMPU"]
    pub smpu: SMPU,
    _reserved1: [u8; 0x1c18],
    #[doc = "0x4000..0x42e8 - MPU"]
    pub mpu0: MPU,
    _reserved2: [u8; 0x0118],
    #[doc = "0x4400..0x46e8 - MPU"]
    pub mpu1: MPU,
    _reserved3: [u8; 0x0118],
    #[doc = "0x4800..0x4ae8 - MPU"]
    pub mpu2: MPU,
    _reserved4: [u8; 0x0118],
    #[doc = "0x4c00..0x4ee8 - MPU"]
    pub mpu3: MPU,
    _reserved5: [u8; 0x0118],
    #[doc = "0x5000..0x52e8 - MPU"]
    pub mpu4: MPU,
    _reserved6: [u8; 0x0118],
    #[doc = "0x5400..0x56e8 - MPU"]
    pub mpu5: MPU,
    _reserved7: [u8; 0x0118],
    #[doc = "0x5800..0x5ae8 - MPU"]
    pub mpu6: MPU,
    _reserved8: [u8; 0x0118],
    #[doc = "0x5c00..0x5ee8 - MPU"]
    pub mpu7: MPU,
    _reserved9: [u8; 0x0118],
    #[doc = "0x6000..0x62e8 - MPU"]
    pub mpu8: MPU,
    _reserved10: [u8; 0x0118],
    #[doc = "0x6400..0x66e8 - MPU"]
    pub mpu9: MPU,
    _reserved11: [u8; 0x0118],
    #[doc = "0x6800..0x6ae8 - MPU"]
    pub mpu10: MPU,
    _reserved12: [u8; 0x0118],
    #[doc = "0x6c00..0x6ee8 - MPU"]
    pub mpu11: MPU,
    _reserved13: [u8; 0x0118],
    #[doc = "0x7000..0x72e8 - MPU"]
    pub mpu12: MPU,
    _reserved14: [u8; 0x0118],
    #[doc = "0x7400..0x76e8 - MPU"]
    pub mpu13: MPU,
    _reserved15: [u8; 0x0118],
    #[doc = "0x7800..0x7ae8 - MPU"]
    pub mpu14: MPU,
    _reserved16: [u8; 0x0118],
    #[doc = "0x7c00..0x7ee8 - MPU"]
    pub mpu15: MPU,
}
#[doc = "SMPU"]
pub use self::smpu::SMPU;
#[doc = r"Cluster"]
#[doc = "SMPU"]
pub mod smpu;
#[doc = "MPU"]
pub use self::mpu::MPU;
#[doc = r"Cluster"]
#[doc = "MPU"]
pub mod mpu;
