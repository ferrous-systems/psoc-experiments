#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub status: STATUS,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - Active descriptor control"]
    pub act_descr_ctl: ACT_DESCR_CTL,
    #[doc = "0x24 - Active descriptor source"]
    pub act_descr_src: ACT_DESCR_SRC,
    #[doc = "0x28 - Active descriptor destination"]
    pub act_descr_dst: ACT_DESCR_DST,
    _reserved5: [u8; 0x04],
    #[doc = "0x30 - Active descriptor X loop control"]
    pub act_descr_x_ctl: ACT_DESCR_X_CTL,
    #[doc = "0x34 - Active descriptor Y loop control"]
    pub act_descr_y_ctl: ACT_DESCR_Y_CTL,
    #[doc = "0x38 - Active descriptor next pointer"]
    pub act_descr_next_ptr: ACT_DESCR_NEXT_PTR,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - Active source"]
    pub act_src: ACT_SRC,
    #[doc = "0x44 - Active destination"]
    pub act_dst: ACT_DST,
    _reserved10: [u8; 0x38],
    #[doc = "0x80 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved11: [u8; 0x7c],
    #[doc = "0x100 - CRC control"]
    pub crc_ctl: CRC_CTL,
    _reserved12: [u8; 0x0c],
    #[doc = "0x110 - CRC data control"]
    pub crc_data_ctl: CRC_DATA_CTL,
    _reserved13: [u8; 0x0c],
    #[doc = "0x120 - CRC polynomial control"]
    pub crc_pol_ctl: CRC_POL_CTL,
    _reserved14: [u8; 0x0c],
    #[doc = "0x130 - CRC LFSR control"]
    pub crc_lfsr_ctl: CRC_LFSR_CTL,
    _reserved15: [u8; 0x0c],
    #[doc = "0x140 - CRC remainder control"]
    pub crc_rem_ctl: CRC_REM_CTL,
    _reserved16: [u8; 0x04],
    #[doc = "0x148 - CRC remainder result"]
    pub crc_rem_result: CRC_REM_RESULT,
    _reserved17: [u8; 0x7eb4],
    #[doc = "0x8000..0x802c - DW channel structure"]
    pub ch_struct0: CH_STRUCT,
    _reserved18: [u8; 0x14],
    #[doc = "0x8040..0x806c - DW channel structure"]
    pub ch_struct1: CH_STRUCT,
    _reserved19: [u8; 0x14],
    #[doc = "0x8080..0x80ac - DW channel structure"]
    pub ch_struct2: CH_STRUCT,
    _reserved20: [u8; 0x14],
    #[doc = "0x80c0..0x80ec - DW channel structure"]
    pub ch_struct3: CH_STRUCT,
    _reserved21: [u8; 0x14],
    #[doc = "0x8100..0x812c - DW channel structure"]
    pub ch_struct4: CH_STRUCT,
    _reserved22: [u8; 0x14],
    #[doc = "0x8140..0x816c - DW channel structure"]
    pub ch_struct5: CH_STRUCT,
    _reserved23: [u8; 0x14],
    #[doc = "0x8180..0x81ac - DW channel structure"]
    pub ch_struct6: CH_STRUCT,
    _reserved24: [u8; 0x14],
    #[doc = "0x81c0..0x81ec - DW channel structure"]
    pub ch_struct7: CH_STRUCT,
    _reserved25: [u8; 0x14],
    #[doc = "0x8200..0x822c - DW channel structure"]
    pub ch_struct8: CH_STRUCT,
    _reserved26: [u8; 0x14],
    #[doc = "0x8240..0x826c - DW channel structure"]
    pub ch_struct9: CH_STRUCT,
    _reserved27: [u8; 0x14],
    #[doc = "0x8280..0x82ac - DW channel structure"]
    pub ch_struct10: CH_STRUCT,
    _reserved28: [u8; 0x14],
    #[doc = "0x82c0..0x82ec - DW channel structure"]
    pub ch_struct11: CH_STRUCT,
    _reserved29: [u8; 0x14],
    #[doc = "0x8300..0x832c - DW channel structure"]
    pub ch_struct12: CH_STRUCT,
    _reserved30: [u8; 0x14],
    #[doc = "0x8340..0x836c - DW channel structure"]
    pub ch_struct13: CH_STRUCT,
    _reserved31: [u8; 0x14],
    #[doc = "0x8380..0x83ac - DW channel structure"]
    pub ch_struct14: CH_STRUCT,
    _reserved32: [u8; 0x14],
    #[doc = "0x83c0..0x83ec - DW channel structure"]
    pub ch_struct15: CH_STRUCT,
    _reserved33: [u8; 0x14],
    #[doc = "0x8400..0x842c - DW channel structure"]
    pub ch_struct16: CH_STRUCT,
    _reserved34: [u8; 0x14],
    #[doc = "0x8440..0x846c - DW channel structure"]
    pub ch_struct17: CH_STRUCT,
    _reserved35: [u8; 0x14],
    #[doc = "0x8480..0x84ac - DW channel structure"]
    pub ch_struct18: CH_STRUCT,
    _reserved36: [u8; 0x14],
    #[doc = "0x84c0..0x84ec - DW channel structure"]
    pub ch_struct19: CH_STRUCT,
    _reserved37: [u8; 0x14],
    #[doc = "0x8500..0x852c - DW channel structure"]
    pub ch_struct20: CH_STRUCT,
    _reserved38: [u8; 0x14],
    #[doc = "0x8540..0x856c - DW channel structure"]
    pub ch_struct21: CH_STRUCT,
    _reserved39: [u8; 0x14],
    #[doc = "0x8580..0x85ac - DW channel structure"]
    pub ch_struct22: CH_STRUCT,
    _reserved40: [u8; 0x14],
    #[doc = "0x85c0..0x85ec - DW channel structure"]
    pub ch_struct23: CH_STRUCT,
    _reserved41: [u8; 0x14],
    #[doc = "0x8600..0x862c - DW channel structure"]
    pub ch_struct24: CH_STRUCT,
    _reserved42: [u8; 0x14],
    #[doc = "0x8640..0x866c - DW channel structure"]
    pub ch_struct25: CH_STRUCT,
    _reserved43: [u8; 0x14],
    #[doc = "0x8680..0x86ac - DW channel structure"]
    pub ch_struct26: CH_STRUCT,
    _reserved44: [u8; 0x14],
    #[doc = "0x86c0..0x86ec - DW channel structure"]
    pub ch_struct27: CH_STRUCT,
    _reserved45: [u8; 0x14],
    #[doc = "0x8700..0x872c - DW channel structure"]
    pub ch_struct28: CH_STRUCT,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ACT_DESCR_CTL (r) register accessor: an alias for `Reg<ACT_DESCR_CTL_SPEC>`"]
pub type ACT_DESCR_CTL = crate::Reg<act_descr_ctl::ACT_DESCR_CTL_SPEC>;
#[doc = "Active descriptor control"]
pub mod act_descr_ctl;
#[doc = "ACT_DESCR_SRC (r) register accessor: an alias for `Reg<ACT_DESCR_SRC_SPEC>`"]
pub type ACT_DESCR_SRC = crate::Reg<act_descr_src::ACT_DESCR_SRC_SPEC>;
#[doc = "Active descriptor source"]
pub mod act_descr_src;
#[doc = "ACT_DESCR_DST (r) register accessor: an alias for `Reg<ACT_DESCR_DST_SPEC>`"]
pub type ACT_DESCR_DST = crate::Reg<act_descr_dst::ACT_DESCR_DST_SPEC>;
#[doc = "Active descriptor destination"]
pub mod act_descr_dst;
#[doc = "ACT_DESCR_X_CTL (r) register accessor: an alias for `Reg<ACT_DESCR_X_CTL_SPEC>`"]
pub type ACT_DESCR_X_CTL = crate::Reg<act_descr_x_ctl::ACT_DESCR_X_CTL_SPEC>;
#[doc = "Active descriptor X loop control"]
pub mod act_descr_x_ctl;
#[doc = "ACT_DESCR_Y_CTL (r) register accessor: an alias for `Reg<ACT_DESCR_Y_CTL_SPEC>`"]
pub type ACT_DESCR_Y_CTL = crate::Reg<act_descr_y_ctl::ACT_DESCR_Y_CTL_SPEC>;
#[doc = "Active descriptor Y loop control"]
pub mod act_descr_y_ctl;
#[doc = "ACT_DESCR_NEXT_PTR (r) register accessor: an alias for `Reg<ACT_DESCR_NEXT_PTR_SPEC>`"]
pub type ACT_DESCR_NEXT_PTR = crate::Reg<act_descr_next_ptr::ACT_DESCR_NEXT_PTR_SPEC>;
#[doc = "Active descriptor next pointer"]
pub mod act_descr_next_ptr;
#[doc = "ACT_SRC (r) register accessor: an alias for `Reg<ACT_SRC_SPEC>`"]
pub type ACT_SRC = crate::Reg<act_src::ACT_SRC_SPEC>;
#[doc = "Active source"]
pub mod act_src;
#[doc = "ACT_DST (r) register accessor: an alias for `Reg<ACT_DST_SPEC>`"]
pub type ACT_DST = crate::Reg<act_dst::ACT_DST_SPEC>;
#[doc = "Active destination"]
pub mod act_dst;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "CRC_CTL (rw) register accessor: an alias for `Reg<CRC_CTL_SPEC>`"]
pub type CRC_CTL = crate::Reg<crc_ctl::CRC_CTL_SPEC>;
#[doc = "CRC control"]
pub mod crc_ctl;
#[doc = "CRC_DATA_CTL (rw) register accessor: an alias for `Reg<CRC_DATA_CTL_SPEC>`"]
pub type CRC_DATA_CTL = crate::Reg<crc_data_ctl::CRC_DATA_CTL_SPEC>;
#[doc = "CRC data control"]
pub mod crc_data_ctl;
#[doc = "CRC_POL_CTL (rw) register accessor: an alias for `Reg<CRC_POL_CTL_SPEC>`"]
pub type CRC_POL_CTL = crate::Reg<crc_pol_ctl::CRC_POL_CTL_SPEC>;
#[doc = "CRC polynomial control"]
pub mod crc_pol_ctl;
#[doc = "CRC_LFSR_CTL (rw) register accessor: an alias for `Reg<CRC_LFSR_CTL_SPEC>`"]
pub type CRC_LFSR_CTL = crate::Reg<crc_lfsr_ctl::CRC_LFSR_CTL_SPEC>;
#[doc = "CRC LFSR control"]
pub mod crc_lfsr_ctl;
#[doc = "CRC_REM_CTL (rw) register accessor: an alias for `Reg<CRC_REM_CTL_SPEC>`"]
pub type CRC_REM_CTL = crate::Reg<crc_rem_ctl::CRC_REM_CTL_SPEC>;
#[doc = "CRC remainder control"]
pub mod crc_rem_ctl;
#[doc = "CRC_REM_RESULT (r) register accessor: an alias for `Reg<CRC_REM_RESULT_SPEC>`"]
pub type CRC_REM_RESULT = crate::Reg<crc_rem_result::CRC_REM_RESULT_SPEC>;
#[doc = "CRC remainder result"]
pub mod crc_rem_result;
#[doc = "DW channel structure"]
pub use self::ch_struct::CH_STRUCT;
#[doc = r"Cluster"]
#[doc = "DW channel structure"]
pub mod ch_struct;
