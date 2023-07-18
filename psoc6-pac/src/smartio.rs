#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xf4 - Programmable IO port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 0x0c],
    #[doc = "0x100..0x1f4 - Programmable IO port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 0x0c],
    #[doc = "0x200..0x2f4 - Programmable IO port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 0x0c],
    #[doc = "0x300..0x3f4 - Programmable IO port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 0x0c],
    #[doc = "0x400..0x4f4 - Programmable IO port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 0x0c],
    #[doc = "0x500..0x5f4 - Programmable IO port registers"]
    pub prt5: PRT,
    _reserved6: [u8; 0x0c],
    #[doc = "0x600..0x6f4 - Programmable IO port registers"]
    pub prt6: PRT,
    _reserved7: [u8; 0x0c],
    #[doc = "0x700..0x7f4 - Programmable IO port registers"]
    pub prt7: PRT,
    _reserved8: [u8; 0x0c],
    #[doc = "0x800..0x8f4 - Programmable IO port registers"]
    pub prt8: PRT,
    _reserved9: [u8; 0x0c],
    #[doc = "0x900..0x9f4 - Programmable IO port registers"]
    pub prt9: PRT,
}
#[doc = "Programmable IO port registers"]
pub use self::prt::PRT;
#[doc = r"Cluster"]
#[doc = "Programmable IO port registers"]
pub mod prt;
