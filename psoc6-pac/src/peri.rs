#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    #[doc = "0x200 - Timeout control"]
    pub timeout_ctl: TIMEOUT_CTL,
    _reserved1: [u8; 0x1c],
    #[doc = "0x220 - Trigger command"]
    pub tr_cmd: TR_CMD,
    _reserved2: [u8; 0x01dc],
    #[doc = "0x400 - Divider command"]
    pub div_cmd: DIV_CMD,
    _reserved3: [u8; 0x07fc],
    #[doc = "0xc00..0x1000 - Clock control"]
    pub clock_ctl: [CLOCK_CTL; 256],
    #[doc = "0x1000..0x1400 - Divider control (for 8.0 divider)"]
    pub div_8_ctl: [DIV_8_CTL; 256],
    #[doc = "0x1400..0x1800 - Divider control (for 16.0 divider)"]
    pub div_16_ctl: [DIV_16_CTL; 256],
    #[doc = "0x1800..0x1c00 - Divider control (for 16.5 divider)"]
    pub div_16_5_ctl: [DIV_16_5_CTL; 256],
    #[doc = "0x1c00..0x1ffc - Divider control (for 24.5 divider)"]
    pub div_24_5_ctl: [DIV_24_5_CTL; 255],
    _reserved8: [u8; 0x04],
    #[doc = "0x2000 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved9: [u8; 0x1ffc],
    #[doc = "0x4000..0x4014 - Peripheral group structure"]
    pub gr0: GR,
    _reserved10: [u8; 0x0c],
    #[doc = "0x4020..0x4034 - Peripheral group structure"]
    pub gr1: GR,
    _reserved11: [u8; 0x0c],
    #[doc = "0x4040..0x4054 - Peripheral group structure"]
    pub gr2: GR,
    _reserved12: [u8; 0x0c],
    #[doc = "0x4060..0x4074 - Peripheral group structure"]
    pub gr3: GR,
    _reserved13: [u8; 0x0c],
    #[doc = "0x4080..0x4094 - Peripheral group structure"]
    pub gr4: GR,
    _reserved14: [u8; 0x0c],
    #[doc = "0x40a0..0x40b4 - Peripheral group structure"]
    pub gr5: GR,
    _reserved15: [u8; 0x0c],
    #[doc = "0x40c0..0x40d4 - Peripheral group structure"]
    pub gr6: GR,
    _reserved16: [u8; 0x0c],
    #[doc = "0x40e0..0x40f4 - Peripheral group structure"]
    pub gr7: GR,
    _reserved17: [u8; 0x0c],
    #[doc = "0x4100..0x4114 - Peripheral group structure"]
    pub gr8: GR,
    _reserved18: [u8; 0x0c],
    #[doc = "0x4120..0x4134 - Peripheral group structure"]
    pub gr9: GR,
    _reserved19: [u8; 0x0c],
    #[doc = "0x4140..0x4154 - Peripheral group structure"]
    pub gr10: GR,
    _reserved20: [u8; 0x3eac],
    #[doc = "0x8000..0xa800 - Trigger group"]
    pub tr_gr: [TR_GR; 10],
    _reserved21: [u8; 0x1800],
    #[doc = "0xc000..0xdc00 - Trigger 1-to-1 group"]
    pub tr_1to1_gr: [TR_1TO1_GR; 7],
}
#[doc = "TIMEOUT_CTL (rw) register accessor: an alias for `Reg<TIMEOUT_CTL_SPEC>`"]
pub type TIMEOUT_CTL = crate::Reg<timeout_ctl::TIMEOUT_CTL_SPEC>;
#[doc = "Timeout control"]
pub mod timeout_ctl;
#[doc = "TR_CMD (rw) register accessor: an alias for `Reg<TR_CMD_SPEC>`"]
pub type TR_CMD = crate::Reg<tr_cmd::TR_CMD_SPEC>;
#[doc = "Trigger command"]
pub mod tr_cmd;
#[doc = "DIV_CMD (rw) register accessor: an alias for `Reg<DIV_CMD_SPEC>`"]
pub type DIV_CMD = crate::Reg<div_cmd::DIV_CMD_SPEC>;
#[doc = "Divider command"]
pub mod div_cmd;
#[doc = "CLOCK_CTL (rw) register accessor: an alias for `Reg<CLOCK_CTL_SPEC>`"]
pub type CLOCK_CTL = crate::Reg<clock_ctl::CLOCK_CTL_SPEC>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "DIV_8_CTL (rw) register accessor: an alias for `Reg<DIV_8_CTL_SPEC>`"]
pub type DIV_8_CTL = crate::Reg<div_8_ctl::DIV_8_CTL_SPEC>;
#[doc = "Divider control (for 8.0 divider)"]
pub mod div_8_ctl;
#[doc = "DIV_16_CTL (rw) register accessor: an alias for `Reg<DIV_16_CTL_SPEC>`"]
pub type DIV_16_CTL = crate::Reg<div_16_ctl::DIV_16_CTL_SPEC>;
#[doc = "Divider control (for 16.0 divider)"]
pub mod div_16_ctl;
#[doc = "DIV_16_5_CTL (rw) register accessor: an alias for `Reg<DIV_16_5_CTL_SPEC>`"]
pub type DIV_16_5_CTL = crate::Reg<div_16_5_ctl::DIV_16_5_CTL_SPEC>;
#[doc = "Divider control (for 16.5 divider)"]
pub mod div_16_5_ctl;
#[doc = "DIV_24_5_CTL (rw) register accessor: an alias for `Reg<DIV_24_5_CTL_SPEC>`"]
pub type DIV_24_5_CTL = crate::Reg<div_24_5_ctl::DIV_24_5_CTL_SPEC>;
#[doc = "Divider control (for 24.5 divider)"]
pub mod div_24_5_ctl;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "Peripheral group structure"]
pub use self::gr::GR;
#[doc = r"Cluster"]
#[doc = "Peripheral group structure"]
pub mod gr;
#[doc = "Trigger group"]
pub use self::tr_gr::TR_GR;
#[doc = r"Cluster"]
#[doc = "Trigger group"]
pub mod tr_gr;
#[doc = "Trigger 1-to-1 group"]
pub use self::tr_1to1_gr::TR_1TO1_GR;
#[doc = r"Cluster"]
#[doc = "Trigger 1-to-1 group"]
pub mod tr_1to1_gr;
