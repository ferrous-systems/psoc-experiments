#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub flash_ctl: FLASH_CTL,
    #[doc = "0x04 - Flash power control"]
    pub flash_pwr_ctl: FLASH_PWR_CTL,
    #[doc = "0x08 - Command"]
    pub flash_cmd: FLASH_CMD,
    _reserved3: [u8; 0x0294],
    #[doc = "0x2a0 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved4: [u8; 0x0c],
    #[doc = "0x2b0 - eCT Flash SRAM ECC control 0"]
    pub fm_sram_ecc_ctl0: FM_SRAM_ECC_CTL0,
    #[doc = "0x2b4 - eCT Flash SRAM ECC control 1"]
    pub fm_sram_ecc_ctl1: FM_SRAM_ECC_CTL1,
    #[doc = "0x2b8 - eCT Flash SRAM ECC control 2"]
    pub fm_sram_ecc_ctl2: FM_SRAM_ECC_CTL2,
    #[doc = "0x2bc - eCT Flash SRAM ECC control 3"]
    pub fm_sram_ecc_ctl3: FM_SRAM_ECC_CTL3,
    _reserved8: [u8; 0x0140],
    #[doc = "0x400 - CM0+ cache control"]
    pub cm0_ca_ctl0: CM0_CA_CTL0,
    #[doc = "0x404 - CM0+ cache control"]
    pub cm0_ca_ctl1: CM0_CA_CTL1,
    #[doc = "0x408 - CM0+ cache control"]
    pub cm0_ca_ctl2: CM0_CA_CTL2,
    _reserved11: [u8; 0x34],
    #[doc = "0x440 - CM0+ cache status 0"]
    pub cm0_ca_status0: CM0_CA_STATUS0,
    #[doc = "0x444 - CM0+ cache status 1"]
    pub cm0_ca_status1: CM0_CA_STATUS1,
    #[doc = "0x448 - CM0+ cache status 2"]
    pub cm0_ca_status2: CM0_CA_STATUS2,
    _reserved14: [u8; 0x14],
    #[doc = "0x460 - CM0+ interface status"]
    pub cm0_status: CM0_STATUS,
    _reserved15: [u8; 0x1c],
    #[doc = "0x480 - CM4 cache control"]
    pub cm4_ca_ctl0: CM4_CA_CTL0,
    #[doc = "0x484 - CM4 cache control"]
    pub cm4_ca_ctl1: CM4_CA_CTL1,
    #[doc = "0x488 - CM4 cache control"]
    pub cm4_ca_ctl2: CM4_CA_CTL2,
    _reserved18: [u8; 0x34],
    #[doc = "0x4c0 - CM4 cache status 0"]
    pub cm4_ca_status0: CM4_CA_STATUS0,
    #[doc = "0x4c4 - CM4 cache status 1"]
    pub cm4_ca_status1: CM4_CA_STATUS1,
    #[doc = "0x4c8 - CM4 cache status 2"]
    pub cm4_ca_status2: CM4_CA_STATUS2,
    _reserved21: [u8; 0x14],
    #[doc = "0x4e0 - CM4 interface status"]
    pub cm4_status: CM4_STATUS,
    _reserved22: [u8; 0x1c],
    #[doc = "0x500 - Cryptography buffer control"]
    pub crypto_buff_ctl: CRYPTO_BUFF_CTL,
    _reserved23: [u8; 0x7c],
    #[doc = "0x580 - Datawire 0 buffer control"]
    pub dw0_buff_ctl: DW0_BUFF_CTL,
    _reserved24: [u8; 0x7c],
    #[doc = "0x600 - Datawire 1 buffer control"]
    pub dw1_buff_ctl: DW1_BUFF_CTL,
    _reserved25: [u8; 0x7c],
    #[doc = "0x680 - DMA controller buffer control"]
    pub dmac_buff_ctl: DMAC_BUFF_CTL,
    _reserved26: [u8; 0x7c],
    #[doc = "0x700 - External master 0 buffer control"]
    pub ext_ms0_buff_ctl: EXT_MS0_BUFF_CTL,
    _reserved27: [u8; 0x7c],
    #[doc = "0x780 - External master 1 buffer control"]
    pub ext_ms1_buff_ctl: EXT_MS1_BUFF_CTL,
    _reserved28: [u8; 0xe87c],
    #[doc = "0xf000..0x10000 - Flash Macro Registers"]
    pub fm_ctl: FM_CTL,
}
#[doc = "FLASH_CTL (rw) register accessor: an alias for `Reg<FLASH_CTL_SPEC>`"]
pub type FLASH_CTL = crate::Reg<flash_ctl::FLASH_CTL_SPEC>;
#[doc = "Control"]
pub mod flash_ctl;
#[doc = "FLASH_PWR_CTL (rw) register accessor: an alias for `Reg<FLASH_PWR_CTL_SPEC>`"]
pub type FLASH_PWR_CTL = crate::Reg<flash_pwr_ctl::FLASH_PWR_CTL_SPEC>;
#[doc = "Flash power control"]
pub mod flash_pwr_ctl;
#[doc = "FLASH_CMD (rw) register accessor: an alias for `Reg<FLASH_CMD_SPEC>`"]
pub type FLASH_CMD = crate::Reg<flash_cmd::FLASH_CMD_SPEC>;
#[doc = "Command"]
pub mod flash_cmd;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "FM_SRAM_ECC_CTL0 (rw) register accessor: an alias for `Reg<FM_SRAM_ECC_CTL0_SPEC>`"]
pub type FM_SRAM_ECC_CTL0 = crate::Reg<fm_sram_ecc_ctl0::FM_SRAM_ECC_CTL0_SPEC>;
#[doc = "eCT Flash SRAM ECC control 0"]
pub mod fm_sram_ecc_ctl0;
#[doc = "FM_SRAM_ECC_CTL1 (rw) register accessor: an alias for `Reg<FM_SRAM_ECC_CTL1_SPEC>`"]
pub type FM_SRAM_ECC_CTL1 = crate::Reg<fm_sram_ecc_ctl1::FM_SRAM_ECC_CTL1_SPEC>;
#[doc = "eCT Flash SRAM ECC control 1"]
pub mod fm_sram_ecc_ctl1;
#[doc = "FM_SRAM_ECC_CTL2 (r) register accessor: an alias for `Reg<FM_SRAM_ECC_CTL2_SPEC>`"]
pub type FM_SRAM_ECC_CTL2 = crate::Reg<fm_sram_ecc_ctl2::FM_SRAM_ECC_CTL2_SPEC>;
#[doc = "eCT Flash SRAM ECC control 2"]
pub mod fm_sram_ecc_ctl2;
#[doc = "FM_SRAM_ECC_CTL3 (rw) register accessor: an alias for `Reg<FM_SRAM_ECC_CTL3_SPEC>`"]
pub type FM_SRAM_ECC_CTL3 = crate::Reg<fm_sram_ecc_ctl3::FM_SRAM_ECC_CTL3_SPEC>;
#[doc = "eCT Flash SRAM ECC control 3"]
pub mod fm_sram_ecc_ctl3;
#[doc = "CM0_CA_CTL0 (rw) register accessor: an alias for `Reg<CM0_CA_CTL0_SPEC>`"]
pub type CM0_CA_CTL0 = crate::Reg<cm0_ca_ctl0::CM0_CA_CTL0_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl0;
#[doc = "CM0_CA_CTL1 (rw) register accessor: an alias for `Reg<CM0_CA_CTL1_SPEC>`"]
pub type CM0_CA_CTL1 = crate::Reg<cm0_ca_ctl1::CM0_CA_CTL1_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl1;
#[doc = "CM0_CA_CTL2 (rw) register accessor: an alias for `Reg<CM0_CA_CTL2_SPEC>`"]
pub type CM0_CA_CTL2 = crate::Reg<cm0_ca_ctl2::CM0_CA_CTL2_SPEC>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl2;
#[doc = "CM0_CA_STATUS0 (r) register accessor: an alias for `Reg<CM0_CA_STATUS0_SPEC>`"]
pub type CM0_CA_STATUS0 = crate::Reg<cm0_ca_status0::CM0_CA_STATUS0_SPEC>;
#[doc = "CM0+ cache status 0"]
pub mod cm0_ca_status0;
#[doc = "CM0_CA_STATUS1 (r) register accessor: an alias for `Reg<CM0_CA_STATUS1_SPEC>`"]
pub type CM0_CA_STATUS1 = crate::Reg<cm0_ca_status1::CM0_CA_STATUS1_SPEC>;
#[doc = "CM0+ cache status 1"]
pub mod cm0_ca_status1;
#[doc = "CM0_CA_STATUS2 (r) register accessor: an alias for `Reg<CM0_CA_STATUS2_SPEC>`"]
pub type CM0_CA_STATUS2 = crate::Reg<cm0_ca_status2::CM0_CA_STATUS2_SPEC>;
#[doc = "CM0+ cache status 2"]
pub mod cm0_ca_status2;
#[doc = "CM0_STATUS (rw) register accessor: an alias for `Reg<CM0_STATUS_SPEC>`"]
pub type CM0_STATUS = crate::Reg<cm0_status::CM0_STATUS_SPEC>;
#[doc = "CM0+ interface status"]
pub mod cm0_status;
#[doc = "CM4_CA_CTL0 (rw) register accessor: an alias for `Reg<CM4_CA_CTL0_SPEC>`"]
pub type CM4_CA_CTL0 = crate::Reg<cm4_ca_ctl0::CM4_CA_CTL0_SPEC>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl0;
#[doc = "CM4_CA_CTL1 (rw) register accessor: an alias for `Reg<CM4_CA_CTL1_SPEC>`"]
pub type CM4_CA_CTL1 = crate::Reg<cm4_ca_ctl1::CM4_CA_CTL1_SPEC>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl1;
#[doc = "CM4_CA_CTL2 (rw) register accessor: an alias for `Reg<CM4_CA_CTL2_SPEC>`"]
pub type CM4_CA_CTL2 = crate::Reg<cm4_ca_ctl2::CM4_CA_CTL2_SPEC>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl2;
#[doc = "CM4_CA_STATUS0 (r) register accessor: an alias for `Reg<CM4_CA_STATUS0_SPEC>`"]
pub type CM4_CA_STATUS0 = crate::Reg<cm4_ca_status0::CM4_CA_STATUS0_SPEC>;
#[doc = "CM4 cache status 0"]
pub mod cm4_ca_status0;
#[doc = "CM4_CA_STATUS1 (r) register accessor: an alias for `Reg<CM4_CA_STATUS1_SPEC>`"]
pub type CM4_CA_STATUS1 = crate::Reg<cm4_ca_status1::CM4_CA_STATUS1_SPEC>;
#[doc = "CM4 cache status 1"]
pub mod cm4_ca_status1;
#[doc = "CM4_CA_STATUS2 (r) register accessor: an alias for `Reg<CM4_CA_STATUS2_SPEC>`"]
pub type CM4_CA_STATUS2 = crate::Reg<cm4_ca_status2::CM4_CA_STATUS2_SPEC>;
#[doc = "CM4 cache status 2"]
pub mod cm4_ca_status2;
#[doc = "CM4_STATUS (rw) register accessor: an alias for `Reg<CM4_STATUS_SPEC>`"]
pub type CM4_STATUS = crate::Reg<cm4_status::CM4_STATUS_SPEC>;
#[doc = "CM4 interface status"]
pub mod cm4_status;
#[doc = "CRYPTO_BUFF_CTL (rw) register accessor: an alias for `Reg<CRYPTO_BUFF_CTL_SPEC>`"]
pub type CRYPTO_BUFF_CTL = crate::Reg<crypto_buff_ctl::CRYPTO_BUFF_CTL_SPEC>;
#[doc = "Cryptography buffer control"]
pub mod crypto_buff_ctl;
#[doc = "DW0_BUFF_CTL (rw) register accessor: an alias for `Reg<DW0_BUFF_CTL_SPEC>`"]
pub type DW0_BUFF_CTL = crate::Reg<dw0_buff_ctl::DW0_BUFF_CTL_SPEC>;
#[doc = "Datawire 0 buffer control"]
pub mod dw0_buff_ctl;
#[doc = "DW1_BUFF_CTL (rw) register accessor: an alias for `Reg<DW1_BUFF_CTL_SPEC>`"]
pub type DW1_BUFF_CTL = crate::Reg<dw1_buff_ctl::DW1_BUFF_CTL_SPEC>;
#[doc = "Datawire 1 buffer control"]
pub mod dw1_buff_ctl;
#[doc = "DMAC_BUFF_CTL (rw) register accessor: an alias for `Reg<DMAC_BUFF_CTL_SPEC>`"]
pub type DMAC_BUFF_CTL = crate::Reg<dmac_buff_ctl::DMAC_BUFF_CTL_SPEC>;
#[doc = "DMA controller buffer control"]
pub mod dmac_buff_ctl;
#[doc = "EXT_MS0_BUFF_CTL (rw) register accessor: an alias for `Reg<EXT_MS0_BUFF_CTL_SPEC>`"]
pub type EXT_MS0_BUFF_CTL = crate::Reg<ext_ms0_buff_ctl::EXT_MS0_BUFF_CTL_SPEC>;
#[doc = "External master 0 buffer control"]
pub mod ext_ms0_buff_ctl;
#[doc = "EXT_MS1_BUFF_CTL (rw) register accessor: an alias for `Reg<EXT_MS1_BUFF_CTL_SPEC>`"]
pub type EXT_MS1_BUFF_CTL = crate::Reg<ext_ms1_buff_ctl::EXT_MS1_BUFF_CTL_SPEC>;
#[doc = "External master 1 buffer control"]
pub mod ext_ms1_buff_ctl;
#[doc = "Flash Macro Registers"]
pub use self::fm_ctl::FM_CTL;
#[doc = r"Cluster"]
#[doc = "Flash Macro Registers"]
pub mod fm_ctl;
