#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x08 - HSIOM port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 0x08],
    #[doc = "0x10..0x18 - HSIOM port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 0x08],
    #[doc = "0x20..0x28 - HSIOM port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 0x08],
    #[doc = "0x30..0x38 - HSIOM port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 0x08],
    #[doc = "0x40..0x48 - HSIOM port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 0x08],
    #[doc = "0x50..0x58 - HSIOM port registers"]
    pub prt5: PRT,
    _reserved6: [u8; 0x08],
    #[doc = "0x60..0x68 - HSIOM port registers"]
    pub prt6: PRT,
    _reserved7: [u8; 0x08],
    #[doc = "0x70..0x78 - HSIOM port registers"]
    pub prt7: PRT,
    _reserved8: [u8; 0x08],
    #[doc = "0x80..0x88 - HSIOM port registers"]
    pub prt8: PRT,
    _reserved9: [u8; 0x08],
    #[doc = "0x90..0x98 - HSIOM port registers"]
    pub prt9: PRT,
    _reserved10: [u8; 0x08],
    #[doc = "0xa0..0xa8 - HSIOM port registers"]
    pub prt10: PRT,
    _reserved11: [u8; 0x08],
    #[doc = "0xb0..0xb8 - HSIOM port registers"]
    pub prt11: PRT,
    _reserved12: [u8; 0x08],
    #[doc = "0xc0..0xc8 - HSIOM port registers"]
    pub prt12: PRT,
    _reserved13: [u8; 0x08],
    #[doc = "0xd0..0xd8 - HSIOM port registers"]
    pub prt13: PRT,
    _reserved14: [u8; 0x08],
    #[doc = "0xe0..0xe8 - HSIOM port registers"]
    pub prt14: PRT,
    _reserved15: [u8; 0x1f18],
    #[doc = "0x2000..0x2100 - AMUX splitter cell control"]
    pub amux_split_ctl: [AMUX_SPLIT_CTL; 64],
    _reserved16: [u8; 0x0100],
    #[doc = "0x2200 - Power/Ground Monitor cell control 0"]
    pub monitor_ctl_0: MONITOR_CTL_0,
    #[doc = "0x2204 - Power/Ground Monitor cell control 1"]
    pub monitor_ctl_1: MONITOR_CTL_1,
    #[doc = "0x2208 - Power/Ground Monitor cell control 2"]
    pub monitor_ctl_2: MONITOR_CTL_2,
    #[doc = "0x220c - Power/Ground Monitor cell control 3"]
    pub monitor_ctl_3: MONITOR_CTL_3,
    _reserved20: [u8; 0x30],
    #[doc = "0x2240 - Alternate JTAG IF selection register"]
    pub alt_jtag_en: ALT_JTAG_EN,
}
#[doc = "HSIOM port registers"]
pub use self::prt::PRT;
#[doc = r"Cluster"]
#[doc = "HSIOM port registers"]
pub mod prt;
#[doc = "AMUX_SPLIT_CTL (rw) register accessor: an alias for `Reg<AMUX_SPLIT_CTL_SPEC>`"]
pub type AMUX_SPLIT_CTL = crate::Reg<amux_split_ctl::AMUX_SPLIT_CTL_SPEC>;
#[doc = "AMUX splitter cell control"]
pub mod amux_split_ctl;
#[doc = "MONITOR_CTL_0 (rw) register accessor: an alias for `Reg<MONITOR_CTL_0_SPEC>`"]
pub type MONITOR_CTL_0 = crate::Reg<monitor_ctl_0::MONITOR_CTL_0_SPEC>;
#[doc = "Power/Ground Monitor cell control 0"]
pub mod monitor_ctl_0;
#[doc = "MONITOR_CTL_1 (rw) register accessor: an alias for `Reg<MONITOR_CTL_1_SPEC>`"]
pub type MONITOR_CTL_1 = crate::Reg<monitor_ctl_1::MONITOR_CTL_1_SPEC>;
#[doc = "Power/Ground Monitor cell control 1"]
pub mod monitor_ctl_1;
#[doc = "MONITOR_CTL_2 (rw) register accessor: an alias for `Reg<MONITOR_CTL_2_SPEC>`"]
pub type MONITOR_CTL_2 = crate::Reg<monitor_ctl_2::MONITOR_CTL_2_SPEC>;
#[doc = "Power/Ground Monitor cell control 2"]
pub mod monitor_ctl_2;
#[doc = "MONITOR_CTL_3 (rw) register accessor: an alias for `Reg<MONITOR_CTL_3_SPEC>`"]
pub type MONITOR_CTL_3 = crate::Reg<monitor_ctl_3::MONITOR_CTL_3_SPEC>;
#[doc = "Power/Ground Monitor cell control 3"]
pub mod monitor_ctl_3;
#[doc = "ALT_JTAG_EN (rw) register accessor: an alias for `Reg<ALT_JTAG_EN_SPEC>`"]
pub type ALT_JTAG_EN = crate::Reg<alt_jtag_en::ALT_JTAG_EN_SPEC>;
#[doc = "Alternate JTAG IF selection register"]
pub mod alt_jtag_en;
