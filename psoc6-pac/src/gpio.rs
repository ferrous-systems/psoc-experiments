#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x5c - GPIO port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 0x24],
    #[doc = "0x80..0xdc - GPIO port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 0x24],
    #[doc = "0x100..0x15c - GPIO port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 0x24],
    #[doc = "0x180..0x1dc - GPIO port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 0x24],
    #[doc = "0x200..0x25c - GPIO port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 0x24],
    #[doc = "0x280..0x2dc - GPIO port registers"]
    pub prt5: PRT,
    _reserved6: [u8; 0x24],
    #[doc = "0x300..0x35c - GPIO port registers"]
    pub prt6: PRT,
    _reserved7: [u8; 0x24],
    #[doc = "0x380..0x3dc - GPIO port registers"]
    pub prt7: PRT,
    _reserved8: [u8; 0x24],
    #[doc = "0x400..0x45c - GPIO port registers"]
    pub prt8: PRT,
    _reserved9: [u8; 0x24],
    #[doc = "0x480..0x4dc - GPIO port registers"]
    pub prt9: PRT,
    _reserved10: [u8; 0x24],
    #[doc = "0x500..0x55c - GPIO port registers"]
    pub prt10: PRT,
    _reserved11: [u8; 0x24],
    #[doc = "0x580..0x5dc - GPIO port registers"]
    pub prt11: PRT,
    _reserved12: [u8; 0x24],
    #[doc = "0x600..0x65c - GPIO port registers"]
    pub prt12: PRT,
    _reserved13: [u8; 0x24],
    #[doc = "0x680..0x6dc - GPIO port registers"]
    pub prt13: PRT,
    _reserved14: [u8; 0x24],
    #[doc = "0x700..0x75c - GPIO port registers"]
    pub prt14: PRT,
    _reserved15: [u8; 0x38a4],
    #[doc = "0x4000 - Interrupt port cause register 0"]
    pub intr_cause0: INTR_CAUSE0,
    #[doc = "0x4004 - Interrupt port cause register 1"]
    pub intr_cause1: INTR_CAUSE1,
    #[doc = "0x4008 - Interrupt port cause register 2"]
    pub intr_cause2: INTR_CAUSE2,
    #[doc = "0x400c - Interrupt port cause register 3"]
    pub intr_cause3: INTR_CAUSE3,
    #[doc = "0x4010 - Extern power supply detection register"]
    pub vdd_active: VDD_ACTIVE,
    #[doc = "0x4014 - Supply detection interrupt register"]
    pub vdd_intr: VDD_INTR,
    #[doc = "0x4018 - Supply detection interrupt mask register"]
    pub vdd_intr_mask: VDD_INTR_MASK,
    #[doc = "0x401c - Supply detection interrupt masked register"]
    pub vdd_intr_masked: VDD_INTR_MASKED,
    #[doc = "0x4020 - Supply detection interrupt set register"]
    pub vdd_intr_set: VDD_INTR_SET,
}
#[doc = "GPIO port registers"]
pub use self::prt::PRT;
#[doc = r"Cluster"]
#[doc = "GPIO port registers"]
pub mod prt;
#[doc = "INTR_CAUSE0 (r) register accessor: an alias for `Reg<INTR_CAUSE0_SPEC>`"]
pub type INTR_CAUSE0 = crate::Reg<intr_cause0::INTR_CAUSE0_SPEC>;
#[doc = "Interrupt port cause register 0"]
pub mod intr_cause0;
#[doc = "INTR_CAUSE1 (r) register accessor: an alias for `Reg<INTR_CAUSE1_SPEC>`"]
pub type INTR_CAUSE1 = crate::Reg<intr_cause1::INTR_CAUSE1_SPEC>;
#[doc = "Interrupt port cause register 1"]
pub mod intr_cause1;
#[doc = "INTR_CAUSE2 (r) register accessor: an alias for `Reg<INTR_CAUSE2_SPEC>`"]
pub type INTR_CAUSE2 = crate::Reg<intr_cause2::INTR_CAUSE2_SPEC>;
#[doc = "Interrupt port cause register 2"]
pub mod intr_cause2;
#[doc = "INTR_CAUSE3 (r) register accessor: an alias for `Reg<INTR_CAUSE3_SPEC>`"]
pub type INTR_CAUSE3 = crate::Reg<intr_cause3::INTR_CAUSE3_SPEC>;
#[doc = "Interrupt port cause register 3"]
pub mod intr_cause3;
#[doc = "VDD_ACTIVE (r) register accessor: an alias for `Reg<VDD_ACTIVE_SPEC>`"]
pub type VDD_ACTIVE = crate::Reg<vdd_active::VDD_ACTIVE_SPEC>;
#[doc = "Extern power supply detection register"]
pub mod vdd_active;
#[doc = "VDD_INTR (rw) register accessor: an alias for `Reg<VDD_INTR_SPEC>`"]
pub type VDD_INTR = crate::Reg<vdd_intr::VDD_INTR_SPEC>;
#[doc = "Supply detection interrupt register"]
pub mod vdd_intr;
#[doc = "VDD_INTR_MASK (rw) register accessor: an alias for `Reg<VDD_INTR_MASK_SPEC>`"]
pub type VDD_INTR_MASK = crate::Reg<vdd_intr_mask::VDD_INTR_MASK_SPEC>;
#[doc = "Supply detection interrupt mask register"]
pub mod vdd_intr_mask;
#[doc = "VDD_INTR_MASKED (r) register accessor: an alias for `Reg<VDD_INTR_MASKED_SPEC>`"]
pub type VDD_INTR_MASKED = crate::Reg<vdd_intr_masked::VDD_INTR_MASKED_SPEC>;
#[doc = "Supply detection interrupt masked register"]
pub mod vdd_intr_masked;
#[doc = "VDD_INTR_SET (rw) register accessor: an alias for `Reg<VDD_INTR_SET_SPEC>`"]
pub type VDD_INTR_SET = crate::Reg<vdd_intr_set::VDD_INTR_SET_SPEC>;
#[doc = "Supply detection interrupt set register"]
pub mod vdd_intr_set;
