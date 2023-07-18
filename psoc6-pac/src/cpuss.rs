#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Identity"]
    pub identity: IDENTITY,
    #[doc = "0x04 - CM4 status"]
    pub cm4_status: CM4_STATUS,
    #[doc = "0x08 - CM4 clock control"]
    pub cm4_clock_ctl: CM4_CLOCK_CTL,
    #[doc = "0x0c - CM4 control"]
    pub cm4_ctl: CM4_CTL,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - CM4 interrupt 0 status"]
    pub cm4_int0_status: CM4_INT0_STATUS,
    #[doc = "0x104 - CM4 interrupt 1 status"]
    pub cm4_int1_status: CM4_INT1_STATUS,
    #[doc = "0x108 - CM4 interrupt 2 status"]
    pub cm4_int2_status: CM4_INT2_STATUS,
    #[doc = "0x10c - CM4 interrupt 3 status"]
    pub cm4_int3_status: CM4_INT3_STATUS,
    #[doc = "0x110 - CM4 interrupt 4 status"]
    pub cm4_int4_status: CM4_INT4_STATUS,
    #[doc = "0x114 - CM4 interrupt 5 status"]
    pub cm4_int5_status: CM4_INT5_STATUS,
    #[doc = "0x118 - CM4 interrupt 6 status"]
    pub cm4_int6_status: CM4_INT6_STATUS,
    #[doc = "0x11c - CM4 interrupt 7 status"]
    pub cm4_int7_status: CM4_INT7_STATUS,
    _reserved12: [u8; 0xe0],
    #[doc = "0x200 - CM4 vector table base"]
    pub cm4_vector_table_base: CM4_VECTOR_TABLE_BASE,
    _reserved13: [u8; 0x3c],
    #[doc = "0x240..0x250 - CM4 NMI control"]
    pub cm4_nmi_ctl: [CM4_NMI_CTL; 4],
    _reserved14: [u8; 0xb0],
    #[doc = "0x300 - UDB power control"]
    pub udb_pwr_ctl: UDB_PWR_CTL,
    #[doc = "0x304 - UDB power control"]
    pub udb_pwr_delay_ctl: UDB_PWR_DELAY_CTL,
    _reserved16: [u8; 0x0cf8],
    #[doc = "0x1000 - CM0+ control"]
    pub cm0_ctl: CM0_CTL,
    #[doc = "0x1004 - CM0+ status"]
    pub cm0_status: CM0_STATUS,
    #[doc = "0x1008 - CM0+ clock control"]
    pub cm0_clock_ctl: CM0_CLOCK_CTL,
    _reserved19: [u8; 0xf4],
    #[doc = "0x1100 - CM0+ interrupt 0 status"]
    pub cm0_int0_status: CM0_INT0_STATUS,
    #[doc = "0x1104 - CM0+ interrupt 1 status"]
    pub cm0_int1_status: CM0_INT1_STATUS,
    #[doc = "0x1108 - CM0+ interrupt 2 status"]
    pub cm0_int2_status: CM0_INT2_STATUS,
    #[doc = "0x110c - CM0+ interrupt 3 status"]
    pub cm0_int3_status: CM0_INT3_STATUS,
    #[doc = "0x1110 - CM0+ interrupt 4 status"]
    pub cm0_int4_status: CM0_INT4_STATUS,
    #[doc = "0x1114 - CM0+ interrupt 5 status"]
    pub cm0_int5_status: CM0_INT5_STATUS,
    #[doc = "0x1118 - CM0+ interrupt 6 status"]
    pub cm0_int6_status: CM0_INT6_STATUS,
    #[doc = "0x111c - CM0+ interrupt 7 status"]
    pub cm0_int7_status: CM0_INT7_STATUS,
    #[doc = "0x1120 - CM0+ vector table base"]
    pub cm0_vector_table_base: CM0_VECTOR_TABLE_BASE,
    _reserved28: [u8; 0x1c],
    #[doc = "0x1140..0x1150 - CM0+ NMI control"]
    pub cm0_nmi_ctl: [CM0_NMI_CTL; 4],
    _reserved29: [u8; 0xb0],
    #[doc = "0x1200 - CM4 power control"]
    pub cm4_pwr_ctl: CM4_PWR_CTL,
    #[doc = "0x1204 - CM4 power control"]
    pub cm4_pwr_delay_ctl: CM4_PWR_DELAY_CTL,
    _reserved31: [u8; 0xf8],
    #[doc = "0x1300 - RAM 0 control"]
    pub ram0_ctl0: RAM0_CTL0,
    #[doc = "0x1304 - RAM 0 status"]
    pub ram0_status: RAM0_STATUS,
    _reserved33: [u8; 0x38],
    #[doc = "0x1340..0x1380 - RAM 0 power control"]
    pub ram0_pwr_macro_ctl: [RAM0_PWR_MACRO_CTL; 16],
    #[doc = "0x1380 - RAM 1 control"]
    pub ram1_ctl0: RAM1_CTL0,
    #[doc = "0x1384 - RAM 1 status"]
    pub ram1_status: RAM1_STATUS,
    #[doc = "0x1388 - RAM 1 power control"]
    pub ram1_pwr_ctl: RAM1_PWR_CTL,
    _reserved37: [u8; 0x14],
    #[doc = "0x13a0 - RAM 2 control"]
    pub ram2_ctl0: RAM2_CTL0,
    #[doc = "0x13a4 - RAM 2 status"]
    pub ram2_status: RAM2_STATUS,
    #[doc = "0x13a8 - RAM 2 power control"]
    pub ram2_pwr_ctl: RAM2_PWR_CTL,
    _reserved40: [u8; 0x14],
    #[doc = "0x13c0 - Power up delay used for all SRAM power domains"]
    pub ram_pwr_delay_ctl: RAM_PWR_DELAY_CTL,
    #[doc = "0x13c4 - ROM control"]
    pub rom_ctl: ROM_CTL,
    #[doc = "0x13c8 - ECC control"]
    pub ecc_ctl: ECC_CTL,
    _reserved43: [u8; 0x34],
    #[doc = "0x1400 - Product identifier and version (same as CoreSight RomTables)"]
    pub product_id: PRODUCT_ID,
    _reserved44: [u8; 0x0c],
    #[doc = "0x1410 - Debug port status"]
    pub dp_status: DP_STATUS,
    #[doc = "0x1414 - Access port control"]
    pub ap_ctl: AP_CTL,
    _reserved46: [u8; 0xe8],
    #[doc = "0x1500 - Buffer control"]
    pub buff_ctl: BUFF_CTL,
    _reserved47: [u8; 0xfc],
    #[doc = "0x1600 - SysTick timer control"]
    pub systick_ctl: SYSTICK_CTL,
    _reserved48: [u8; 0x0100],
    #[doc = "0x1704 - Memory BIST status"]
    pub mbist_stat: MBIST_STAT,
    _reserved49: [u8; 0xf8],
    #[doc = "0x1800 - Calibration support set and read"]
    pub cal_sup_set: CAL_SUP_SET,
    #[doc = "0x1804 - Calibration support clear and reset"]
    pub cal_sup_clr: CAL_SUP_CLR,
    _reserved51: [u8; 0x07f8],
    #[doc = "0x2000 - CM0+ protection context control"]
    pub cm0_pc_ctl: CM0_PC_CTL,
    _reserved52: [u8; 0x3c],
    #[doc = "0x2040 - CM0+ protection context 0 handler"]
    pub cm0_pc0_handler: CM0_PC0_HANDLER,
    #[doc = "0x2044 - CM0+ protection context 1 handler"]
    pub cm0_pc1_handler: CM0_PC1_HANDLER,
    #[doc = "0x2048 - CM0+ protection context 2 handler"]
    pub cm0_pc2_handler: CM0_PC2_HANDLER,
    #[doc = "0x204c - CM0+ protection context 3 handler"]
    pub cm0_pc3_handler: CM0_PC3_HANDLER,
    _reserved56: [u8; 0x74],
    #[doc = "0x20c4 - Protection status"]
    pub protection: PROTECTION,
    _reserved57: [u8; 0x38],
    #[doc = "0x2100 - ROM trim control"]
    pub trim_rom_ctl: TRIM_ROM_CTL,
    #[doc = "0x2104 - RAM trim control"]
    pub trim_ram_ctl: TRIM_RAM_CTL,
    _reserved59: [u8; 0x5ef8],
    #[doc = "0x8000..0x8ffc - CM0+ system interrupt control"]
    pub cm0_system_int_ctl: [CM0_SYSTEM_INT_CTL; 1023],
    _reserved60: [u8; 0x1004],
    #[doc = "0xa000..0xaffc - CM4 system interrupt control"]
    pub cm4_system_int_ctl: [CM4_SYSTEM_INT_CTL; 1023],
}
#[doc = "IDENTITY (r) register accessor: an alias for `Reg<IDENTITY_SPEC>`"]
pub type IDENTITY = crate::Reg<identity::IDENTITY_SPEC>;
#[doc = "Identity"]
pub mod identity;
#[doc = "CM4_STATUS (r) register accessor: an alias for `Reg<CM4_STATUS_SPEC>`"]
pub type CM4_STATUS = crate::Reg<cm4_status::CM4_STATUS_SPEC>;
#[doc = "CM4 status"]
pub mod cm4_status;
#[doc = "CM4_CLOCK_CTL (rw) register accessor: an alias for `Reg<CM4_CLOCK_CTL_SPEC>`"]
pub type CM4_CLOCK_CTL = crate::Reg<cm4_clock_ctl::CM4_CLOCK_CTL_SPEC>;
#[doc = "CM4 clock control"]
pub mod cm4_clock_ctl;
#[doc = "CM4_CTL (rw) register accessor: an alias for `Reg<CM4_CTL_SPEC>`"]
pub type CM4_CTL = crate::Reg<cm4_ctl::CM4_CTL_SPEC>;
#[doc = "CM4 control"]
pub mod cm4_ctl;
#[doc = "CM4_INT0_STATUS (r) register accessor: an alias for `Reg<CM4_INT0_STATUS_SPEC>`"]
pub type CM4_INT0_STATUS = crate::Reg<cm4_int0_status::CM4_INT0_STATUS_SPEC>;
#[doc = "CM4 interrupt 0 status"]
pub mod cm4_int0_status;
#[doc = "CM4_INT1_STATUS (r) register accessor: an alias for `Reg<CM4_INT1_STATUS_SPEC>`"]
pub type CM4_INT1_STATUS = crate::Reg<cm4_int1_status::CM4_INT1_STATUS_SPEC>;
#[doc = "CM4 interrupt 1 status"]
pub mod cm4_int1_status;
#[doc = "CM4_INT2_STATUS (r) register accessor: an alias for `Reg<CM4_INT2_STATUS_SPEC>`"]
pub type CM4_INT2_STATUS = crate::Reg<cm4_int2_status::CM4_INT2_STATUS_SPEC>;
#[doc = "CM4 interrupt 2 status"]
pub mod cm4_int2_status;
#[doc = "CM4_INT3_STATUS (r) register accessor: an alias for `Reg<CM4_INT3_STATUS_SPEC>`"]
pub type CM4_INT3_STATUS = crate::Reg<cm4_int3_status::CM4_INT3_STATUS_SPEC>;
#[doc = "CM4 interrupt 3 status"]
pub mod cm4_int3_status;
#[doc = "CM4_INT4_STATUS (r) register accessor: an alias for `Reg<CM4_INT4_STATUS_SPEC>`"]
pub type CM4_INT4_STATUS = crate::Reg<cm4_int4_status::CM4_INT4_STATUS_SPEC>;
#[doc = "CM4 interrupt 4 status"]
pub mod cm4_int4_status;
#[doc = "CM4_INT5_STATUS (r) register accessor: an alias for `Reg<CM4_INT5_STATUS_SPEC>`"]
pub type CM4_INT5_STATUS = crate::Reg<cm4_int5_status::CM4_INT5_STATUS_SPEC>;
#[doc = "CM4 interrupt 5 status"]
pub mod cm4_int5_status;
#[doc = "CM4_INT6_STATUS (r) register accessor: an alias for `Reg<CM4_INT6_STATUS_SPEC>`"]
pub type CM4_INT6_STATUS = crate::Reg<cm4_int6_status::CM4_INT6_STATUS_SPEC>;
#[doc = "CM4 interrupt 6 status"]
pub mod cm4_int6_status;
#[doc = "CM4_INT7_STATUS (r) register accessor: an alias for `Reg<CM4_INT7_STATUS_SPEC>`"]
pub type CM4_INT7_STATUS = crate::Reg<cm4_int7_status::CM4_INT7_STATUS_SPEC>;
#[doc = "CM4 interrupt 7 status"]
pub mod cm4_int7_status;
#[doc = "CM4_VECTOR_TABLE_BASE (rw) register accessor: an alias for `Reg<CM4_VECTOR_TABLE_BASE_SPEC>`"]
pub type CM4_VECTOR_TABLE_BASE = crate::Reg<cm4_vector_table_base::CM4_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM4 vector table base"]
pub mod cm4_vector_table_base;
#[doc = "CM4_NMI_CTL (rw) register accessor: an alias for `Reg<CM4_NMI_CTL_SPEC>`"]
pub type CM4_NMI_CTL = crate::Reg<cm4_nmi_ctl::CM4_NMI_CTL_SPEC>;
#[doc = "CM4 NMI control"]
pub mod cm4_nmi_ctl;
#[doc = "UDB_PWR_CTL (rw) register accessor: an alias for `Reg<UDB_PWR_CTL_SPEC>`"]
pub type UDB_PWR_CTL = crate::Reg<udb_pwr_ctl::UDB_PWR_CTL_SPEC>;
#[doc = "UDB power control"]
pub mod udb_pwr_ctl;
#[doc = "UDB_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<UDB_PWR_DELAY_CTL_SPEC>`"]
pub type UDB_PWR_DELAY_CTL = crate::Reg<udb_pwr_delay_ctl::UDB_PWR_DELAY_CTL_SPEC>;
#[doc = "UDB power control"]
pub mod udb_pwr_delay_ctl;
#[doc = "CM0_CTL (rw) register accessor: an alias for `Reg<CM0_CTL_SPEC>`"]
pub type CM0_CTL = crate::Reg<cm0_ctl::CM0_CTL_SPEC>;
#[doc = "CM0+ control"]
pub mod cm0_ctl;
#[doc = "CM0_STATUS (r) register accessor: an alias for `Reg<CM0_STATUS_SPEC>`"]
pub type CM0_STATUS = crate::Reg<cm0_status::CM0_STATUS_SPEC>;
#[doc = "CM0+ status"]
pub mod cm0_status;
#[doc = "CM0_CLOCK_CTL (rw) register accessor: an alias for `Reg<CM0_CLOCK_CTL_SPEC>`"]
pub type CM0_CLOCK_CTL = crate::Reg<cm0_clock_ctl::CM0_CLOCK_CTL_SPEC>;
#[doc = "CM0+ clock control"]
pub mod cm0_clock_ctl;
#[doc = "CM0_INT0_STATUS (r) register accessor: an alias for `Reg<CM0_INT0_STATUS_SPEC>`"]
pub type CM0_INT0_STATUS = crate::Reg<cm0_int0_status::CM0_INT0_STATUS_SPEC>;
#[doc = "CM0+ interrupt 0 status"]
pub mod cm0_int0_status;
#[doc = "CM0_INT1_STATUS (r) register accessor: an alias for `Reg<CM0_INT1_STATUS_SPEC>`"]
pub type CM0_INT1_STATUS = crate::Reg<cm0_int1_status::CM0_INT1_STATUS_SPEC>;
#[doc = "CM0+ interrupt 1 status"]
pub mod cm0_int1_status;
#[doc = "CM0_INT2_STATUS (r) register accessor: an alias for `Reg<CM0_INT2_STATUS_SPEC>`"]
pub type CM0_INT2_STATUS = crate::Reg<cm0_int2_status::CM0_INT2_STATUS_SPEC>;
#[doc = "CM0+ interrupt 2 status"]
pub mod cm0_int2_status;
#[doc = "CM0_INT3_STATUS (r) register accessor: an alias for `Reg<CM0_INT3_STATUS_SPEC>`"]
pub type CM0_INT3_STATUS = crate::Reg<cm0_int3_status::CM0_INT3_STATUS_SPEC>;
#[doc = "CM0+ interrupt 3 status"]
pub mod cm0_int3_status;
#[doc = "CM0_INT4_STATUS (r) register accessor: an alias for `Reg<CM0_INT4_STATUS_SPEC>`"]
pub type CM0_INT4_STATUS = crate::Reg<cm0_int4_status::CM0_INT4_STATUS_SPEC>;
#[doc = "CM0+ interrupt 4 status"]
pub mod cm0_int4_status;
#[doc = "CM0_INT5_STATUS (r) register accessor: an alias for `Reg<CM0_INT5_STATUS_SPEC>`"]
pub type CM0_INT5_STATUS = crate::Reg<cm0_int5_status::CM0_INT5_STATUS_SPEC>;
#[doc = "CM0+ interrupt 5 status"]
pub mod cm0_int5_status;
#[doc = "CM0_INT6_STATUS (r) register accessor: an alias for `Reg<CM0_INT6_STATUS_SPEC>`"]
pub type CM0_INT6_STATUS = crate::Reg<cm0_int6_status::CM0_INT6_STATUS_SPEC>;
#[doc = "CM0+ interrupt 6 status"]
pub mod cm0_int6_status;
#[doc = "CM0_INT7_STATUS (r) register accessor: an alias for `Reg<CM0_INT7_STATUS_SPEC>`"]
pub type CM0_INT7_STATUS = crate::Reg<cm0_int7_status::CM0_INT7_STATUS_SPEC>;
#[doc = "CM0+ interrupt 7 status"]
pub mod cm0_int7_status;
#[doc = "CM0_VECTOR_TABLE_BASE (rw) register accessor: an alias for `Reg<CM0_VECTOR_TABLE_BASE_SPEC>`"]
pub type CM0_VECTOR_TABLE_BASE = crate::Reg<cm0_vector_table_base::CM0_VECTOR_TABLE_BASE_SPEC>;
#[doc = "CM0+ vector table base"]
pub mod cm0_vector_table_base;
#[doc = "CM0_NMI_CTL (rw) register accessor: an alias for `Reg<CM0_NMI_CTL_SPEC>`"]
pub type CM0_NMI_CTL = crate::Reg<cm0_nmi_ctl::CM0_NMI_CTL_SPEC>;
#[doc = "CM0+ NMI control"]
pub mod cm0_nmi_ctl;
#[doc = "CM4_PWR_CTL (rw) register accessor: an alias for `Reg<CM4_PWR_CTL_SPEC>`"]
pub type CM4_PWR_CTL = crate::Reg<cm4_pwr_ctl::CM4_PWR_CTL_SPEC>;
#[doc = "CM4 power control"]
pub mod cm4_pwr_ctl;
#[doc = "CM4_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<CM4_PWR_DELAY_CTL_SPEC>`"]
pub type CM4_PWR_DELAY_CTL = crate::Reg<cm4_pwr_delay_ctl::CM4_PWR_DELAY_CTL_SPEC>;
#[doc = "CM4 power control"]
pub mod cm4_pwr_delay_ctl;
#[doc = "RAM0_CTL0 (rw) register accessor: an alias for `Reg<RAM0_CTL0_SPEC>`"]
pub type RAM0_CTL0 = crate::Reg<ram0_ctl0::RAM0_CTL0_SPEC>;
#[doc = "RAM 0 control"]
pub mod ram0_ctl0;
#[doc = "RAM0_STATUS (r) register accessor: an alias for `Reg<RAM0_STATUS_SPEC>`"]
pub type RAM0_STATUS = crate::Reg<ram0_status::RAM0_STATUS_SPEC>;
#[doc = "RAM 0 status"]
pub mod ram0_status;
#[doc = "RAM0_PWR_MACRO_CTL (rw) register accessor: an alias for `Reg<RAM0_PWR_MACRO_CTL_SPEC>`"]
pub type RAM0_PWR_MACRO_CTL = crate::Reg<ram0_pwr_macro_ctl::RAM0_PWR_MACRO_CTL_SPEC>;
#[doc = "RAM 0 power control"]
pub mod ram0_pwr_macro_ctl;
#[doc = "RAM1_CTL0 (rw) register accessor: an alias for `Reg<RAM1_CTL0_SPEC>`"]
pub type RAM1_CTL0 = crate::Reg<ram1_ctl0::RAM1_CTL0_SPEC>;
#[doc = "RAM 1 control"]
pub mod ram1_ctl0;
#[doc = "RAM1_STATUS (r) register accessor: an alias for `Reg<RAM1_STATUS_SPEC>`"]
pub type RAM1_STATUS = crate::Reg<ram1_status::RAM1_STATUS_SPEC>;
#[doc = "RAM 1 status"]
pub mod ram1_status;
#[doc = "RAM1_PWR_CTL (rw) register accessor: an alias for `Reg<RAM1_PWR_CTL_SPEC>`"]
pub type RAM1_PWR_CTL = crate::Reg<ram1_pwr_ctl::RAM1_PWR_CTL_SPEC>;
#[doc = "RAM 1 power control"]
pub mod ram1_pwr_ctl;
#[doc = "RAM2_CTL0 (rw) register accessor: an alias for `Reg<RAM2_CTL0_SPEC>`"]
pub type RAM2_CTL0 = crate::Reg<ram2_ctl0::RAM2_CTL0_SPEC>;
#[doc = "RAM 2 control"]
pub mod ram2_ctl0;
#[doc = "RAM2_STATUS (r) register accessor: an alias for `Reg<RAM2_STATUS_SPEC>`"]
pub type RAM2_STATUS = crate::Reg<ram2_status::RAM2_STATUS_SPEC>;
#[doc = "RAM 2 status"]
pub mod ram2_status;
#[doc = "RAM2_PWR_CTL (rw) register accessor: an alias for `Reg<RAM2_PWR_CTL_SPEC>`"]
pub type RAM2_PWR_CTL = crate::Reg<ram2_pwr_ctl::RAM2_PWR_CTL_SPEC>;
#[doc = "RAM 2 power control"]
pub mod ram2_pwr_ctl;
#[doc = "RAM_PWR_DELAY_CTL (rw) register accessor: an alias for `Reg<RAM_PWR_DELAY_CTL_SPEC>`"]
pub type RAM_PWR_DELAY_CTL = crate::Reg<ram_pwr_delay_ctl::RAM_PWR_DELAY_CTL_SPEC>;
#[doc = "Power up delay used for all SRAM power domains"]
pub mod ram_pwr_delay_ctl;
#[doc = "ROM_CTL (rw) register accessor: an alias for `Reg<ROM_CTL_SPEC>`"]
pub type ROM_CTL = crate::Reg<rom_ctl::ROM_CTL_SPEC>;
#[doc = "ROM control"]
pub mod rom_ctl;
#[doc = "ECC_CTL (rw) register accessor: an alias for `Reg<ECC_CTL_SPEC>`"]
pub type ECC_CTL = crate::Reg<ecc_ctl::ECC_CTL_SPEC>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "PRODUCT_ID (r) register accessor: an alias for `Reg<PRODUCT_ID_SPEC>`"]
pub type PRODUCT_ID = crate::Reg<product_id::PRODUCT_ID_SPEC>;
#[doc = "Product identifier and version (same as CoreSight RomTables)"]
pub mod product_id;
#[doc = "DP_STATUS (r) register accessor: an alias for `Reg<DP_STATUS_SPEC>`"]
pub type DP_STATUS = crate::Reg<dp_status::DP_STATUS_SPEC>;
#[doc = "Debug port status"]
pub mod dp_status;
#[doc = "AP_CTL (rw) register accessor: an alias for `Reg<AP_CTL_SPEC>`"]
pub type AP_CTL = crate::Reg<ap_ctl::AP_CTL_SPEC>;
#[doc = "Access port control"]
pub mod ap_ctl;
#[doc = "BUFF_CTL (rw) register accessor: an alias for `Reg<BUFF_CTL_SPEC>`"]
pub type BUFF_CTL = crate::Reg<buff_ctl::BUFF_CTL_SPEC>;
#[doc = "Buffer control"]
pub mod buff_ctl;
#[doc = "SYSTICK_CTL (rw) register accessor: an alias for `Reg<SYSTICK_CTL_SPEC>`"]
pub type SYSTICK_CTL = crate::Reg<systick_ctl::SYSTICK_CTL_SPEC>;
#[doc = "SysTick timer control"]
pub mod systick_ctl;
#[doc = "MBIST_STAT (r) register accessor: an alias for `Reg<MBIST_STAT_SPEC>`"]
pub type MBIST_STAT = crate::Reg<mbist_stat::MBIST_STAT_SPEC>;
#[doc = "Memory BIST status"]
pub mod mbist_stat;
#[doc = "CAL_SUP_SET (rw) register accessor: an alias for `Reg<CAL_SUP_SET_SPEC>`"]
pub type CAL_SUP_SET = crate::Reg<cal_sup_set::CAL_SUP_SET_SPEC>;
#[doc = "Calibration support set and read"]
pub mod cal_sup_set;
#[doc = "CAL_SUP_CLR (rw) register accessor: an alias for `Reg<CAL_SUP_CLR_SPEC>`"]
pub type CAL_SUP_CLR = crate::Reg<cal_sup_clr::CAL_SUP_CLR_SPEC>;
#[doc = "Calibration support clear and reset"]
pub mod cal_sup_clr;
#[doc = "CM0_PC_CTL (rw) register accessor: an alias for `Reg<CM0_PC_CTL_SPEC>`"]
pub type CM0_PC_CTL = crate::Reg<cm0_pc_ctl::CM0_PC_CTL_SPEC>;
#[doc = "CM0+ protection context control"]
pub mod cm0_pc_ctl;
#[doc = "CM0_PC0_HANDLER (rw) register accessor: an alias for `Reg<CM0_PC0_HANDLER_SPEC>`"]
pub type CM0_PC0_HANDLER = crate::Reg<cm0_pc0_handler::CM0_PC0_HANDLER_SPEC>;
#[doc = "CM0+ protection context 0 handler"]
pub mod cm0_pc0_handler;
#[doc = "CM0_PC1_HANDLER (rw) register accessor: an alias for `Reg<CM0_PC1_HANDLER_SPEC>`"]
pub type CM0_PC1_HANDLER = crate::Reg<cm0_pc1_handler::CM0_PC1_HANDLER_SPEC>;
#[doc = "CM0+ protection context 1 handler"]
pub mod cm0_pc1_handler;
#[doc = "CM0_PC2_HANDLER (rw) register accessor: an alias for `Reg<CM0_PC2_HANDLER_SPEC>`"]
pub type CM0_PC2_HANDLER = crate::Reg<cm0_pc2_handler::CM0_PC2_HANDLER_SPEC>;
#[doc = "CM0+ protection context 2 handler"]
pub mod cm0_pc2_handler;
#[doc = "CM0_PC3_HANDLER (rw) register accessor: an alias for `Reg<CM0_PC3_HANDLER_SPEC>`"]
pub type CM0_PC3_HANDLER = crate::Reg<cm0_pc3_handler::CM0_PC3_HANDLER_SPEC>;
#[doc = "CM0+ protection context 3 handler"]
pub mod cm0_pc3_handler;
#[doc = "PROTECTION (rw) register accessor: an alias for `Reg<PROTECTION_SPEC>`"]
pub type PROTECTION = crate::Reg<protection::PROTECTION_SPEC>;
#[doc = "Protection status"]
pub mod protection;
#[doc = "TRIM_ROM_CTL (rw) register accessor: an alias for `Reg<TRIM_ROM_CTL_SPEC>`"]
pub type TRIM_ROM_CTL = crate::Reg<trim_rom_ctl::TRIM_ROM_CTL_SPEC>;
#[doc = "ROM trim control"]
pub mod trim_rom_ctl;
#[doc = "TRIM_RAM_CTL (rw) register accessor: an alias for `Reg<TRIM_RAM_CTL_SPEC>`"]
pub type TRIM_RAM_CTL = crate::Reg<trim_ram_ctl::TRIM_RAM_CTL_SPEC>;
#[doc = "RAM trim control"]
pub mod trim_ram_ctl;
#[doc = "CM0_SYSTEM_INT_CTL (rw) register accessor: an alias for `Reg<CM0_SYSTEM_INT_CTL_SPEC>`"]
pub type CM0_SYSTEM_INT_CTL = crate::Reg<cm0_system_int_ctl::CM0_SYSTEM_INT_CTL_SPEC>;
#[doc = "CM0+ system interrupt control"]
pub mod cm0_system_int_ctl;
#[doc = "CM4_SYSTEM_INT_CTL (rw) register accessor: an alias for `Reg<CM4_SYSTEM_INT_CTL_SPEC>`"]
pub type CM4_SYSTEM_INT_CTL = crate::Reg<cm4_system_int_ctl::CM4_SYSTEM_INT_CTL_SPEC>;
#[doc = "CM4 system interrupt control"]
pub mod cm4_system_int_ctl;
