#[doc = r"Register block"]
#[repr(C)]
pub struct USBDEV {
    #[doc = "0x00..0x20 - Control End point EP0 Data Register"]
    pub ep0_dr: [EP0_DR; 8],
    #[doc = "0x20 - USB control 0 Register"]
    pub cr0: CR0,
    #[doc = "0x24 - USB control 1 Register"]
    pub cr1: CR1,
    #[doc = "0x28 - USB SIE Data Endpoints Interrupt Enable Register"]
    pub sie_ep_int_en: SIE_EP_INT_EN,
    #[doc = "0x2c - USB SIE Data Endpoint Interrupt Status"]
    pub sie_ep_int_sr: SIE_EP_INT_SR,
    #[doc = "0x30 - Non-control endpoint count register"]
    pub sie_ep1_cnt0: SIE_EP1_CNT0,
    #[doc = "0x34 - Non-control endpoint count register"]
    pub sie_ep1_cnt1: SIE_EP1_CNT1,
    #[doc = "0x38 - Non-control endpoint's control Register"]
    pub sie_ep1_cr0: SIE_EP1_CR0,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - USBIO Control 0 Register"]
    pub usbio_cr0: USBIO_CR0,
    #[doc = "0x44 - USBIO control 2 Register"]
    pub usbio_cr2: USBIO_CR2,
    #[doc = "0x48 - USBIO control 1 Register"]
    pub usbio_cr1: USBIO_CR1,
    _reserved11: [u8; 0x04],
    #[doc = "0x50 - USB Dynamic reconfiguration register"]
    pub dyn_reconfig: DYN_RECONFIG,
    _reserved12: [u8; 0x0c],
    #[doc = "0x60 - Start Of Frame Register"]
    pub sof0: SOF0,
    #[doc = "0x64 - Start Of Frame Register"]
    pub sof1: SOF1,
    _reserved14: [u8; 0x08],
    #[doc = "0x70 - Non-control endpoint count register"]
    pub sie_ep2_cnt0: SIE_EP2_CNT0,
    #[doc = "0x74 - Non-control endpoint count register"]
    pub sie_ep2_cnt1: SIE_EP2_CNT1,
    #[doc = "0x78 - Non-control endpoint's control Register"]
    pub sie_ep2_cr0: SIE_EP2_CR0,
    _reserved17: [u8; 0x04],
    #[doc = "0x80 - Oscillator lock data register 0"]
    pub osclk_dr0: OSCLK_DR0,
    #[doc = "0x84 - Oscillator lock data register 1"]
    pub osclk_dr1: OSCLK_DR1,
    _reserved19: [u8; 0x18],
    #[doc = "0xa0 - Endpoint0 control Register"]
    pub ep0_cr: EP0_CR,
    #[doc = "0xa4 - Endpoint0 count Register"]
    pub ep0_cnt: EP0_CNT,
    _reserved21: [u8; 0x08],
    #[doc = "0xb0 - Non-control endpoint count register"]
    pub sie_ep3_cnt0: SIE_EP3_CNT0,
    #[doc = "0xb4 - Non-control endpoint count register"]
    pub sie_ep3_cnt1: SIE_EP3_CNT1,
    #[doc = "0xb8 - Non-control endpoint's control Register"]
    pub sie_ep3_cr0: SIE_EP3_CR0,
    _reserved24: [u8; 0x34],
    #[doc = "0xf0 - Non-control endpoint count register"]
    pub sie_ep4_cnt0: SIE_EP4_CNT0,
    #[doc = "0xf4 - Non-control endpoint count register"]
    pub sie_ep4_cnt1: SIE_EP4_CNT1,
    #[doc = "0xf8 - Non-control endpoint's control Register"]
    pub sie_ep4_cr0: SIE_EP4_CR0,
    _reserved27: [u8; 0x34],
    #[doc = "0x130 - Non-control endpoint count register"]
    pub sie_ep5_cnt0: SIE_EP5_CNT0,
    #[doc = "0x134 - Non-control endpoint count register"]
    pub sie_ep5_cnt1: SIE_EP5_CNT1,
    #[doc = "0x138 - Non-control endpoint's control Register"]
    pub sie_ep5_cr0: SIE_EP5_CR0,
    _reserved30: [u8; 0x34],
    #[doc = "0x170 - Non-control endpoint count register"]
    pub sie_ep6_cnt0: SIE_EP6_CNT0,
    #[doc = "0x174 - Non-control endpoint count register"]
    pub sie_ep6_cnt1: SIE_EP6_CNT1,
    #[doc = "0x178 - Non-control endpoint's control Register"]
    pub sie_ep6_cr0: SIE_EP6_CR0,
    _reserved33: [u8; 0x34],
    #[doc = "0x1b0 - Non-control endpoint count register"]
    pub sie_ep7_cnt0: SIE_EP7_CNT0,
    #[doc = "0x1b4 - Non-control endpoint count register"]
    pub sie_ep7_cnt1: SIE_EP7_CNT1,
    #[doc = "0x1b8 - Non-control endpoint's control Register"]
    pub sie_ep7_cr0: SIE_EP7_CR0,
    _reserved36: [u8; 0x34],
    #[doc = "0x1f0 - Non-control endpoint count register"]
    pub sie_ep8_cnt0: SIE_EP8_CNT0,
    #[doc = "0x1f4 - Non-control endpoint count register"]
    pub sie_ep8_cnt1: SIE_EP8_CNT1,
    #[doc = "0x1f8 - Non-control endpoint's control Register"]
    pub sie_ep8_cr0: SIE_EP8_CR0,
    _reserved39: [u8; 0x04],
    #[doc = "0x200 - Endpoint Configuration Register *1"]
    pub arb_ep1_cfg: ARB_EP1_CFG,
    #[doc = "0x204 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep1_int_en: ARB_EP1_INT_EN,
    #[doc = "0x208 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep1_sr: ARB_EP1_SR,
    _reserved42: [u8; 0x04],
    #[doc = "0x210 - Endpoint Write Address value *1, *2"]
    pub arb_rw1_wa: ARB_RW1_WA,
    #[doc = "0x214 - Endpoint Write Address value *1, *2"]
    pub arb_rw1_wa_msb: ARB_RW1_WA_MSB,
    #[doc = "0x218 - Endpoint Read Address value *1, *2"]
    pub arb_rw1_ra: ARB_RW1_RA,
    #[doc = "0x21c - Endpoint Read Address value *1, *2"]
    pub arb_rw1_ra_msb: ARB_RW1_RA_MSB,
    #[doc = "0x220 - Endpoint Data Register"]
    pub arb_rw1_dr: ARB_RW1_DR,
    _reserved47: [u8; 0x0c],
    #[doc = "0x230 - Dedicated Endpoint Buffer Size Register *1"]
    pub buf_size: BUF_SIZE,
    _reserved48: [u8; 0x04],
    #[doc = "0x238 - Endpoint Active Indication Register *1"]
    pub ep_active: EP_ACTIVE,
    #[doc = "0x23c - Endpoint Type (IN/OUT) Indication *1"]
    pub ep_type: EP_TYPE,
    #[doc = "0x240 - Endpoint Configuration Register *1"]
    pub arb_ep2_cfg: ARB_EP2_CFG,
    #[doc = "0x244 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep2_int_en: ARB_EP2_INT_EN,
    #[doc = "0x248 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep2_sr: ARB_EP2_SR,
    _reserved53: [u8; 0x04],
    #[doc = "0x250 - Endpoint Write Address value *1, *2"]
    pub arb_rw2_wa: ARB_RW2_WA,
    #[doc = "0x254 - Endpoint Write Address value *1, *2"]
    pub arb_rw2_wa_msb: ARB_RW2_WA_MSB,
    #[doc = "0x258 - Endpoint Read Address value *1, *2"]
    pub arb_rw2_ra: ARB_RW2_RA,
    #[doc = "0x25c - Endpoint Read Address value *1, *2"]
    pub arb_rw2_ra_msb: ARB_RW2_RA_MSB,
    #[doc = "0x260 - Endpoint Data Register"]
    pub arb_rw2_dr: ARB_RW2_DR,
    _reserved58: [u8; 0x0c],
    #[doc = "0x270 - Arbiter Configuration Register *1"]
    pub arb_cfg: ARB_CFG,
    #[doc = "0x274 - USB Block Clock Enable Register"]
    pub usb_clk_en: USB_CLK_EN,
    #[doc = "0x278 - Arbiter Interrupt Enable *1"]
    pub arb_int_en: ARB_INT_EN,
    #[doc = "0x27c - Arbiter Interrupt Status *1"]
    pub arb_int_sr: ARB_INT_SR,
    #[doc = "0x280 - Endpoint Configuration Register *1"]
    pub arb_ep3_cfg: ARB_EP3_CFG,
    #[doc = "0x284 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep3_int_en: ARB_EP3_INT_EN,
    #[doc = "0x288 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep3_sr: ARB_EP3_SR,
    _reserved65: [u8; 0x04],
    #[doc = "0x290 - Endpoint Write Address value *1, *2"]
    pub arb_rw3_wa: ARB_RW3_WA,
    #[doc = "0x294 - Endpoint Write Address value *1, *2"]
    pub arb_rw3_wa_msb: ARB_RW3_WA_MSB,
    #[doc = "0x298 - Endpoint Read Address value *1, *2"]
    pub arb_rw3_ra: ARB_RW3_RA,
    #[doc = "0x29c - Endpoint Read Address value *1, *2"]
    pub arb_rw3_ra_msb: ARB_RW3_RA_MSB,
    #[doc = "0x2a0 - Endpoint Data Register"]
    pub arb_rw3_dr: ARB_RW3_DR,
    _reserved70: [u8; 0x0c],
    #[doc = "0x2b0 - Common Area Write Address *1"]
    pub cwa: CWA,
    #[doc = "0x2b4 - Endpoint Read Address value *1"]
    pub cwa_msb: CWA_MSB,
    _reserved72: [u8; 0x08],
    #[doc = "0x2c0 - Endpoint Configuration Register *1"]
    pub arb_ep4_cfg: ARB_EP4_CFG,
    #[doc = "0x2c4 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep4_int_en: ARB_EP4_INT_EN,
    #[doc = "0x2c8 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep4_sr: ARB_EP4_SR,
    _reserved75: [u8; 0x04],
    #[doc = "0x2d0 - Endpoint Write Address value *1, *2"]
    pub arb_rw4_wa: ARB_RW4_WA,
    #[doc = "0x2d4 - Endpoint Write Address value *1, *2"]
    pub arb_rw4_wa_msb: ARB_RW4_WA_MSB,
    #[doc = "0x2d8 - Endpoint Read Address value *1, *2"]
    pub arb_rw4_ra: ARB_RW4_RA,
    #[doc = "0x2dc - Endpoint Read Address value *1, *2"]
    pub arb_rw4_ra_msb: ARB_RW4_RA_MSB,
    #[doc = "0x2e0 - Endpoint Data Register"]
    pub arb_rw4_dr: ARB_RW4_DR,
    _reserved80: [u8; 0x0c],
    #[doc = "0x2f0 - DMA Burst / Threshold Configuration"]
    pub dma_thres: DMA_THRES,
    #[doc = "0x2f4 - DMA Burst / Threshold Configuration"]
    pub dma_thres_msb: DMA_THRES_MSB,
    _reserved82: [u8; 0x08],
    #[doc = "0x300 - Endpoint Configuration Register *1"]
    pub arb_ep5_cfg: ARB_EP5_CFG,
    #[doc = "0x304 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep5_int_en: ARB_EP5_INT_EN,
    #[doc = "0x308 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep5_sr: ARB_EP5_SR,
    _reserved85: [u8; 0x04],
    #[doc = "0x310 - Endpoint Write Address value *1, *2"]
    pub arb_rw5_wa: ARB_RW5_WA,
    #[doc = "0x314 - Endpoint Write Address value *1, *2"]
    pub arb_rw5_wa_msb: ARB_RW5_WA_MSB,
    #[doc = "0x318 - Endpoint Read Address value *1, *2"]
    pub arb_rw5_ra: ARB_RW5_RA,
    #[doc = "0x31c - Endpoint Read Address value *1, *2"]
    pub arb_rw5_ra_msb: ARB_RW5_RA_MSB,
    #[doc = "0x320 - Endpoint Data Register"]
    pub arb_rw5_dr: ARB_RW5_DR,
    _reserved90: [u8; 0x0c],
    #[doc = "0x330 - Bus Reset Count Register"]
    pub bus_rst_cnt: BUS_RST_CNT,
    _reserved91: [u8; 0x0c],
    #[doc = "0x340 - Endpoint Configuration Register *1"]
    pub arb_ep6_cfg: ARB_EP6_CFG,
    #[doc = "0x344 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep6_int_en: ARB_EP6_INT_EN,
    #[doc = "0x348 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep6_sr: ARB_EP6_SR,
    _reserved94: [u8; 0x04],
    #[doc = "0x350 - Endpoint Write Address value *1, *2"]
    pub arb_rw6_wa: ARB_RW6_WA,
    #[doc = "0x354 - Endpoint Write Address value *1, *2"]
    pub arb_rw6_wa_msb: ARB_RW6_WA_MSB,
    #[doc = "0x358 - Endpoint Read Address value *1, *2"]
    pub arb_rw6_ra: ARB_RW6_RA,
    #[doc = "0x35c - Endpoint Read Address value *1, *2"]
    pub arb_rw6_ra_msb: ARB_RW6_RA_MSB,
    #[doc = "0x360 - Endpoint Data Register"]
    pub arb_rw6_dr: ARB_RW6_DR,
    _reserved99: [u8; 0x1c],
    #[doc = "0x380 - Endpoint Configuration Register *1"]
    pub arb_ep7_cfg: ARB_EP7_CFG,
    #[doc = "0x384 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep7_int_en: ARB_EP7_INT_EN,
    #[doc = "0x388 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep7_sr: ARB_EP7_SR,
    _reserved102: [u8; 0x04],
    #[doc = "0x390 - Endpoint Write Address value *1, *2"]
    pub arb_rw7_wa: ARB_RW7_WA,
    #[doc = "0x394 - Endpoint Write Address value *1, *2"]
    pub arb_rw7_wa_msb: ARB_RW7_WA_MSB,
    #[doc = "0x398 - Endpoint Read Address value *1, *2"]
    pub arb_rw7_ra: ARB_RW7_RA,
    #[doc = "0x39c - Endpoint Read Address value *1, *2"]
    pub arb_rw7_ra_msb: ARB_RW7_RA_MSB,
    #[doc = "0x3a0 - Endpoint Data Register"]
    pub arb_rw7_dr: ARB_RW7_DR,
    _reserved107: [u8; 0x1c],
    #[doc = "0x3c0 - Endpoint Configuration Register *1"]
    pub arb_ep8_cfg: ARB_EP8_CFG,
    #[doc = "0x3c4 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep8_int_en: ARB_EP8_INT_EN,
    #[doc = "0x3c8 - Endpoint Interrupt Enable Register *1"]
    pub arb_ep8_sr: ARB_EP8_SR,
    _reserved110: [u8; 0x04],
    #[doc = "0x3d0 - Endpoint Write Address value *1, *2"]
    pub arb_rw8_wa: ARB_RW8_WA,
    #[doc = "0x3d4 - Endpoint Write Address value *1, *2"]
    pub arb_rw8_wa_msb: ARB_RW8_WA_MSB,
    #[doc = "0x3d8 - Endpoint Read Address value *1, *2"]
    pub arb_rw8_ra: ARB_RW8_RA,
    #[doc = "0x3dc - Endpoint Read Address value *1, *2"]
    pub arb_rw8_ra_msb: ARB_RW8_RA_MSB,
    #[doc = "0x3e0 - Endpoint Data Register"]
    pub arb_rw8_dr: ARB_RW8_DR,
    _reserved115: [u8; 0x1c],
    #[doc = "0x400..0xc00 - DATA"]
    pub mem_data: [MEM_DATA; 512],
    _reserved116: [u8; 0x0460],
    #[doc = "0x1060 - Start Of Frame Register"]
    pub sof16: SOF16,
    _reserved117: [u8; 0x1c],
    #[doc = "0x1080 - Oscillator lock data register"]
    pub osclk_dr16: OSCLK_DR16,
    _reserved118: [u8; 0x018c],
    #[doc = "0x1210 - Endpoint Write Address value *3"]
    pub arb_rw1_wa16: ARB_RW1_WA16,
    _reserved119: [u8; 0x04],
    #[doc = "0x1218 - Endpoint Read Address value *3"]
    pub arb_rw1_ra16: ARB_RW1_RA16,
    _reserved120: [u8; 0x04],
    #[doc = "0x1220 - Endpoint Data Register"]
    pub arb_rw1_dr16: ARB_RW1_DR16,
    _reserved121: [u8; 0x2c],
    #[doc = "0x1250 - Endpoint Write Address value *3"]
    pub arb_rw2_wa16: ARB_RW2_WA16,
    _reserved122: [u8; 0x04],
    #[doc = "0x1258 - Endpoint Read Address value *3"]
    pub arb_rw2_ra16: ARB_RW2_RA16,
    _reserved123: [u8; 0x04],
    #[doc = "0x1260 - Endpoint Data Register"]
    pub arb_rw2_dr16: ARB_RW2_DR16,
    _reserved124: [u8; 0x2c],
    #[doc = "0x1290 - Endpoint Write Address value *3"]
    pub arb_rw3_wa16: ARB_RW3_WA16,
    _reserved125: [u8; 0x04],
    #[doc = "0x1298 - Endpoint Read Address value *3"]
    pub arb_rw3_ra16: ARB_RW3_RA16,
    _reserved126: [u8; 0x04],
    #[doc = "0x12a0 - Endpoint Data Register"]
    pub arb_rw3_dr16: ARB_RW3_DR16,
    _reserved127: [u8; 0x0c],
    #[doc = "0x12b0 - Common Area Write Address"]
    pub cwa16: CWA16,
    _reserved128: [u8; 0x1c],
    #[doc = "0x12d0 - Endpoint Write Address value *3"]
    pub arb_rw4_wa16: ARB_RW4_WA16,
    _reserved129: [u8; 0x04],
    #[doc = "0x12d8 - Endpoint Read Address value *3"]
    pub arb_rw4_ra16: ARB_RW4_RA16,
    _reserved130: [u8; 0x04],
    #[doc = "0x12e0 - Endpoint Data Register"]
    pub arb_rw4_dr16: ARB_RW4_DR16,
    _reserved131: [u8; 0x0c],
    #[doc = "0x12f0 - DMA Burst / Threshold Configuration"]
    pub dma_thres16: DMA_THRES16,
    _reserved132: [u8; 0x1c],
    #[doc = "0x1310 - Endpoint Write Address value *3"]
    pub arb_rw5_wa16: ARB_RW5_WA16,
    _reserved133: [u8; 0x04],
    #[doc = "0x1318 - Endpoint Read Address value *3"]
    pub arb_rw5_ra16: ARB_RW5_RA16,
    _reserved134: [u8; 0x04],
    #[doc = "0x1320 - Endpoint Data Register"]
    pub arb_rw5_dr16: ARB_RW5_DR16,
    _reserved135: [u8; 0x2c],
    #[doc = "0x1350 - Endpoint Write Address value *3"]
    pub arb_rw6_wa16: ARB_RW6_WA16,
    _reserved136: [u8; 0x04],
    #[doc = "0x1358 - Endpoint Read Address value *3"]
    pub arb_rw6_ra16: ARB_RW6_RA16,
    _reserved137: [u8; 0x04],
    #[doc = "0x1360 - Endpoint Data Register"]
    pub arb_rw6_dr16: ARB_RW6_DR16,
    _reserved138: [u8; 0x2c],
    #[doc = "0x1390 - Endpoint Write Address value *3"]
    pub arb_rw7_wa16: ARB_RW7_WA16,
    _reserved139: [u8; 0x04],
    #[doc = "0x1398 - Endpoint Read Address value *3"]
    pub arb_rw7_ra16: ARB_RW7_RA16,
    _reserved140: [u8; 0x04],
    #[doc = "0x13a0 - Endpoint Data Register"]
    pub arb_rw7_dr16: ARB_RW7_DR16,
    _reserved141: [u8; 0x2c],
    #[doc = "0x13d0 - Endpoint Write Address value *3"]
    pub arb_rw8_wa16: ARB_RW8_WA16,
    _reserved142: [u8; 0x04],
    #[doc = "0x13d8 - Endpoint Read Address value *3"]
    pub arb_rw8_ra16: ARB_RW8_RA16,
    _reserved143: [u8; 0x04],
    #[doc = "0x13e0 - Endpoint Data Register"]
    pub arb_rw8_dr16: ARB_RW8_DR16,
}
#[doc = "EP0_DR (rw) register accessor: an alias for `Reg<EP0_DR_SPEC>`"]
pub type EP0_DR = crate::Reg<ep0_dr::EP0_DR_SPEC>;
#[doc = "Control End point EP0 Data Register"]
pub mod ep0_dr;
#[doc = "CR0 (rw) register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "USB control 0 Register"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "USB control 1 Register"]
pub mod cr1;
#[doc = "SIE_EP_INT_EN (rw) register accessor: an alias for `Reg<SIE_EP_INT_EN_SPEC>`"]
pub type SIE_EP_INT_EN = crate::Reg<sie_ep_int_en::SIE_EP_INT_EN_SPEC>;
#[doc = "USB SIE Data Endpoints Interrupt Enable Register"]
pub mod sie_ep_int_en;
#[doc = "SIE_EP_INT_SR (rw) register accessor: an alias for `Reg<SIE_EP_INT_SR_SPEC>`"]
pub type SIE_EP_INT_SR = crate::Reg<sie_ep_int_sr::SIE_EP_INT_SR_SPEC>;
#[doc = "USB SIE Data Endpoint Interrupt Status"]
pub mod sie_ep_int_sr;
#[doc = "SIE_EP1_CNT0 (rw) register accessor: an alias for `Reg<SIE_EP1_CNT0_SPEC>`"]
pub type SIE_EP1_CNT0 = crate::Reg<sie_ep1_cnt0::SIE_EP1_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt0;
#[doc = "SIE_EP1_CNT1 (rw) register accessor: an alias for `Reg<SIE_EP1_CNT1_SPEC>`"]
pub type SIE_EP1_CNT1 = crate::Reg<sie_ep1_cnt1::SIE_EP1_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt1;
#[doc = "SIE_EP1_CR0 (rw) register accessor: an alias for `Reg<SIE_EP1_CR0_SPEC>`"]
pub type SIE_EP1_CR0 = crate::Reg<sie_ep1_cr0::SIE_EP1_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep1_cr0;
#[doc = "USBIO_CR0 (rw) register accessor: an alias for `Reg<USBIO_CR0_SPEC>`"]
pub type USBIO_CR0 = crate::Reg<usbio_cr0::USBIO_CR0_SPEC>;
#[doc = "USBIO Control 0 Register"]
pub mod usbio_cr0;
#[doc = "USBIO_CR2 (rw) register accessor: an alias for `Reg<USBIO_CR2_SPEC>`"]
pub type USBIO_CR2 = crate::Reg<usbio_cr2::USBIO_CR2_SPEC>;
#[doc = "USBIO control 2 Register"]
pub mod usbio_cr2;
#[doc = "USBIO_CR1 (rw) register accessor: an alias for `Reg<USBIO_CR1_SPEC>`"]
pub type USBIO_CR1 = crate::Reg<usbio_cr1::USBIO_CR1_SPEC>;
#[doc = "USBIO control 1 Register"]
pub mod usbio_cr1;
#[doc = "DYN_RECONFIG (rw) register accessor: an alias for `Reg<DYN_RECONFIG_SPEC>`"]
pub type DYN_RECONFIG = crate::Reg<dyn_reconfig::DYN_RECONFIG_SPEC>;
#[doc = "USB Dynamic reconfiguration register"]
pub mod dyn_reconfig;
#[doc = "SOF0 (r) register accessor: an alias for `Reg<SOF0_SPEC>`"]
pub type SOF0 = crate::Reg<sof0::SOF0_SPEC>;
#[doc = "Start Of Frame Register"]
pub mod sof0;
#[doc = "SOF1 (r) register accessor: an alias for `Reg<SOF1_SPEC>`"]
pub type SOF1 = crate::Reg<sof1::SOF1_SPEC>;
#[doc = "Start Of Frame Register"]
pub mod sof1;
#[doc = "SIE_EP2_CNT0 (rw) register accessor: an alias for `Reg<SIE_EP2_CNT0_SPEC>`"]
pub type SIE_EP2_CNT0 = crate::Reg<sie_ep2_cnt0::SIE_EP2_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt0;
#[doc = "SIE_EP2_CNT1 (rw) register accessor: an alias for `Reg<SIE_EP2_CNT1_SPEC>`"]
pub type SIE_EP2_CNT1 = crate::Reg<sie_ep2_cnt1::SIE_EP2_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt1;
#[doc = "SIE_EP2_CR0 (rw) register accessor: an alias for `Reg<SIE_EP2_CR0_SPEC>`"]
pub type SIE_EP2_CR0 = crate::Reg<sie_ep2_cr0::SIE_EP2_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep2_cr0;
#[doc = "OSCLK_DR0 (r) register accessor: an alias for `Reg<OSCLK_DR0_SPEC>`"]
pub type OSCLK_DR0 = crate::Reg<osclk_dr0::OSCLK_DR0_SPEC>;
#[doc = "Oscillator lock data register 0"]
pub mod osclk_dr0;
#[doc = "OSCLK_DR1 (r) register accessor: an alias for `Reg<OSCLK_DR1_SPEC>`"]
pub type OSCLK_DR1 = crate::Reg<osclk_dr1::OSCLK_DR1_SPEC>;
#[doc = "Oscillator lock data register 1"]
pub mod osclk_dr1;
#[doc = "EP0_CR (rw) register accessor: an alias for `Reg<EP0_CR_SPEC>`"]
pub type EP0_CR = crate::Reg<ep0_cr::EP0_CR_SPEC>;
#[doc = "Endpoint0 control Register"]
pub mod ep0_cr;
#[doc = "EP0_CNT (rw) register accessor: an alias for `Reg<EP0_CNT_SPEC>`"]
pub type EP0_CNT = crate::Reg<ep0_cnt::EP0_CNT_SPEC>;
#[doc = "Endpoint0 count Register"]
pub mod ep0_cnt;
#[doc = "SIE_EP3_CNT0 (rw) register accessor: an alias for `Reg<SIE_EP3_CNT0_SPEC>`"]
pub type SIE_EP3_CNT0 = crate::Reg<sie_ep3_cnt0::SIE_EP3_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt0;
#[doc = "SIE_EP3_CNT1 (rw) register accessor: an alias for `Reg<SIE_EP3_CNT1_SPEC>`"]
pub type SIE_EP3_CNT1 = crate::Reg<sie_ep3_cnt1::SIE_EP3_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt1;
#[doc = "SIE_EP3_CR0 (rw) register accessor: an alias for `Reg<SIE_EP3_CR0_SPEC>`"]
pub type SIE_EP3_CR0 = crate::Reg<sie_ep3_cr0::SIE_EP3_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep3_cr0;
#[doc = "SIE_EP4_CNT0 (rw) register accessor: an alias for `Reg<SIE_EP4_CNT0_SPEC>`"]
pub type SIE_EP4_CNT0 = crate::Reg<sie_ep4_cnt0::SIE_EP4_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt0;
#[doc = "SIE_EP4_CNT1 (rw) register accessor: an alias for `Reg<SIE_EP4_CNT1_SPEC>`"]
pub type SIE_EP4_CNT1 = crate::Reg<sie_ep4_cnt1::SIE_EP4_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt1;
#[doc = "SIE_EP4_CR0 (rw) register accessor: an alias for `Reg<SIE_EP4_CR0_SPEC>`"]
pub type SIE_EP4_CR0 = crate::Reg<sie_ep4_cr0::SIE_EP4_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep4_cr0;
#[doc = "SIE_EP5_CNT0 (rw) register accessor: an alias for `Reg<SIE_EP5_CNT0_SPEC>`"]
pub type SIE_EP5_CNT0 = crate::Reg<sie_ep5_cnt0::SIE_EP5_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt0;
#[doc = "SIE_EP5_CNT1 (rw) register accessor: an alias for `Reg<SIE_EP5_CNT1_SPEC>`"]
pub type SIE_EP5_CNT1 = crate::Reg<sie_ep5_cnt1::SIE_EP5_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt1;
#[doc = "SIE_EP5_CR0 (rw) register accessor: an alias for `Reg<SIE_EP5_CR0_SPEC>`"]
pub type SIE_EP5_CR0 = crate::Reg<sie_ep5_cr0::SIE_EP5_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep5_cr0;
#[doc = "SIE_EP6_CNT0 (rw) register accessor: an alias for `Reg<SIE_EP6_CNT0_SPEC>`"]
pub type SIE_EP6_CNT0 = crate::Reg<sie_ep6_cnt0::SIE_EP6_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt0;
#[doc = "SIE_EP6_CNT1 (rw) register accessor: an alias for `Reg<SIE_EP6_CNT1_SPEC>`"]
pub type SIE_EP6_CNT1 = crate::Reg<sie_ep6_cnt1::SIE_EP6_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt1;
#[doc = "SIE_EP6_CR0 (rw) register accessor: an alias for `Reg<SIE_EP6_CR0_SPEC>`"]
pub type SIE_EP6_CR0 = crate::Reg<sie_ep6_cr0::SIE_EP6_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep6_cr0;
#[doc = "SIE_EP7_CNT0 (rw) register accessor: an alias for `Reg<SIE_EP7_CNT0_SPEC>`"]
pub type SIE_EP7_CNT0 = crate::Reg<sie_ep7_cnt0::SIE_EP7_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt0;
#[doc = "SIE_EP7_CNT1 (rw) register accessor: an alias for `Reg<SIE_EP7_CNT1_SPEC>`"]
pub type SIE_EP7_CNT1 = crate::Reg<sie_ep7_cnt1::SIE_EP7_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt1;
#[doc = "SIE_EP7_CR0 (rw) register accessor: an alias for `Reg<SIE_EP7_CR0_SPEC>`"]
pub type SIE_EP7_CR0 = crate::Reg<sie_ep7_cr0::SIE_EP7_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep7_cr0;
#[doc = "SIE_EP8_CNT0 (rw) register accessor: an alias for `Reg<SIE_EP8_CNT0_SPEC>`"]
pub type SIE_EP8_CNT0 = crate::Reg<sie_ep8_cnt0::SIE_EP8_CNT0_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt0;
#[doc = "SIE_EP8_CNT1 (rw) register accessor: an alias for `Reg<SIE_EP8_CNT1_SPEC>`"]
pub type SIE_EP8_CNT1 = crate::Reg<sie_ep8_cnt1::SIE_EP8_CNT1_SPEC>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt1;
#[doc = "SIE_EP8_CR0 (rw) register accessor: an alias for `Reg<SIE_EP8_CR0_SPEC>`"]
pub type SIE_EP8_CR0 = crate::Reg<sie_ep8_cr0::SIE_EP8_CR0_SPEC>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep8_cr0;
#[doc = "ARB_EP1_CFG (rw) register accessor: an alias for `Reg<ARB_EP1_CFG_SPEC>`"]
pub type ARB_EP1_CFG = crate::Reg<arb_ep1_cfg::ARB_EP1_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep1_cfg;
#[doc = "ARB_EP1_INT_EN (rw) register accessor: an alias for `Reg<ARB_EP1_INT_EN_SPEC>`"]
pub type ARB_EP1_INT_EN = crate::Reg<arb_ep1_int_en::ARB_EP1_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_int_en;
#[doc = "ARB_EP1_SR (rw) register accessor: an alias for `Reg<ARB_EP1_SR_SPEC>`"]
pub type ARB_EP1_SR = crate::Reg<arb_ep1_sr::ARB_EP1_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_sr;
#[doc = "ARB_RW1_WA (rw) register accessor: an alias for `Reg<ARB_RW1_WA_SPEC>`"]
pub type ARB_RW1_WA = crate::Reg<arb_rw1_wa::ARB_RW1_WA_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw1_wa;
#[doc = "ARB_RW1_WA_MSB (rw) register accessor: an alias for `Reg<ARB_RW1_WA_MSB_SPEC>`"]
pub type ARB_RW1_WA_MSB = crate::Reg<arb_rw1_wa_msb::ARB_RW1_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw1_wa_msb;
#[doc = "ARB_RW1_RA (rw) register accessor: an alias for `Reg<ARB_RW1_RA_SPEC>`"]
pub type ARB_RW1_RA = crate::Reg<arb_rw1_ra::ARB_RW1_RA_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw1_ra;
#[doc = "ARB_RW1_RA_MSB (rw) register accessor: an alias for `Reg<ARB_RW1_RA_MSB_SPEC>`"]
pub type ARB_RW1_RA_MSB = crate::Reg<arb_rw1_ra_msb::ARB_RW1_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw1_ra_msb;
#[doc = "ARB_RW1_DR (rw) register accessor: an alias for `Reg<ARB_RW1_DR_SPEC>`"]
pub type ARB_RW1_DR = crate::Reg<arb_rw1_dr::ARB_RW1_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr;
#[doc = "BUF_SIZE (rw) register accessor: an alias for `Reg<BUF_SIZE_SPEC>`"]
pub type BUF_SIZE = crate::Reg<buf_size::BUF_SIZE_SPEC>;
#[doc = "Dedicated Endpoint Buffer Size Register *1"]
pub mod buf_size;
#[doc = "EP_ACTIVE (rw) register accessor: an alias for `Reg<EP_ACTIVE_SPEC>`"]
pub type EP_ACTIVE = crate::Reg<ep_active::EP_ACTIVE_SPEC>;
#[doc = "Endpoint Active Indication Register *1"]
pub mod ep_active;
#[doc = "EP_TYPE (rw) register accessor: an alias for `Reg<EP_TYPE_SPEC>`"]
pub type EP_TYPE = crate::Reg<ep_type::EP_TYPE_SPEC>;
#[doc = "Endpoint Type (IN/OUT) Indication *1"]
pub mod ep_type;
#[doc = "ARB_EP2_CFG (rw) register accessor: an alias for `Reg<ARB_EP2_CFG_SPEC>`"]
pub type ARB_EP2_CFG = crate::Reg<arb_ep2_cfg::ARB_EP2_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep2_cfg;
#[doc = "ARB_EP2_INT_EN (rw) register accessor: an alias for `Reg<ARB_EP2_INT_EN_SPEC>`"]
pub type ARB_EP2_INT_EN = crate::Reg<arb_ep2_int_en::ARB_EP2_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_int_en;
#[doc = "ARB_EP2_SR (rw) register accessor: an alias for `Reg<ARB_EP2_SR_SPEC>`"]
pub type ARB_EP2_SR = crate::Reg<arb_ep2_sr::ARB_EP2_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_sr;
#[doc = "ARB_RW2_WA (rw) register accessor: an alias for `Reg<ARB_RW2_WA_SPEC>`"]
pub type ARB_RW2_WA = crate::Reg<arb_rw2_wa::ARB_RW2_WA_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw2_wa;
#[doc = "ARB_RW2_WA_MSB (rw) register accessor: an alias for `Reg<ARB_RW2_WA_MSB_SPEC>`"]
pub type ARB_RW2_WA_MSB = crate::Reg<arb_rw2_wa_msb::ARB_RW2_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw2_wa_msb;
#[doc = "ARB_RW2_RA (rw) register accessor: an alias for `Reg<ARB_RW2_RA_SPEC>`"]
pub type ARB_RW2_RA = crate::Reg<arb_rw2_ra::ARB_RW2_RA_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw2_ra;
#[doc = "ARB_RW2_RA_MSB (rw) register accessor: an alias for `Reg<ARB_RW2_RA_MSB_SPEC>`"]
pub type ARB_RW2_RA_MSB = crate::Reg<arb_rw2_ra_msb::ARB_RW2_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw2_ra_msb;
#[doc = "ARB_RW2_DR (rw) register accessor: an alias for `Reg<ARB_RW2_DR_SPEC>`"]
pub type ARB_RW2_DR = crate::Reg<arb_rw2_dr::ARB_RW2_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr;
#[doc = "ARB_CFG (rw) register accessor: an alias for `Reg<ARB_CFG_SPEC>`"]
pub type ARB_CFG = crate::Reg<arb_cfg::ARB_CFG_SPEC>;
#[doc = "Arbiter Configuration Register *1"]
pub mod arb_cfg;
#[doc = "USB_CLK_EN (rw) register accessor: an alias for `Reg<USB_CLK_EN_SPEC>`"]
pub type USB_CLK_EN = crate::Reg<usb_clk_en::USB_CLK_EN_SPEC>;
#[doc = "USB Block Clock Enable Register"]
pub mod usb_clk_en;
#[doc = "ARB_INT_EN (rw) register accessor: an alias for `Reg<ARB_INT_EN_SPEC>`"]
pub type ARB_INT_EN = crate::Reg<arb_int_en::ARB_INT_EN_SPEC>;
#[doc = "Arbiter Interrupt Enable *1"]
pub mod arb_int_en;
#[doc = "ARB_INT_SR (r) register accessor: an alias for `Reg<ARB_INT_SR_SPEC>`"]
pub type ARB_INT_SR = crate::Reg<arb_int_sr::ARB_INT_SR_SPEC>;
#[doc = "Arbiter Interrupt Status *1"]
pub mod arb_int_sr;
#[doc = "ARB_EP3_CFG (rw) register accessor: an alias for `Reg<ARB_EP3_CFG_SPEC>`"]
pub type ARB_EP3_CFG = crate::Reg<arb_ep3_cfg::ARB_EP3_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep3_cfg;
#[doc = "ARB_EP3_INT_EN (rw) register accessor: an alias for `Reg<ARB_EP3_INT_EN_SPEC>`"]
pub type ARB_EP3_INT_EN = crate::Reg<arb_ep3_int_en::ARB_EP3_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_int_en;
#[doc = "ARB_EP3_SR (rw) register accessor: an alias for `Reg<ARB_EP3_SR_SPEC>`"]
pub type ARB_EP3_SR = crate::Reg<arb_ep3_sr::ARB_EP3_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_sr;
#[doc = "ARB_RW3_WA (rw) register accessor: an alias for `Reg<ARB_RW3_WA_SPEC>`"]
pub type ARB_RW3_WA = crate::Reg<arb_rw3_wa::ARB_RW3_WA_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw3_wa;
#[doc = "ARB_RW3_WA_MSB (rw) register accessor: an alias for `Reg<ARB_RW3_WA_MSB_SPEC>`"]
pub type ARB_RW3_WA_MSB = crate::Reg<arb_rw3_wa_msb::ARB_RW3_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw3_wa_msb;
#[doc = "ARB_RW3_RA (rw) register accessor: an alias for `Reg<ARB_RW3_RA_SPEC>`"]
pub type ARB_RW3_RA = crate::Reg<arb_rw3_ra::ARB_RW3_RA_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw3_ra;
#[doc = "ARB_RW3_RA_MSB (rw) register accessor: an alias for `Reg<ARB_RW3_RA_MSB_SPEC>`"]
pub type ARB_RW3_RA_MSB = crate::Reg<arb_rw3_ra_msb::ARB_RW3_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw3_ra_msb;
#[doc = "ARB_RW3_DR (rw) register accessor: an alias for `Reg<ARB_RW3_DR_SPEC>`"]
pub type ARB_RW3_DR = crate::Reg<arb_rw3_dr::ARB_RW3_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr;
#[doc = "CWA (rw) register accessor: an alias for `Reg<CWA_SPEC>`"]
pub type CWA = crate::Reg<cwa::CWA_SPEC>;
#[doc = "Common Area Write Address *1"]
pub mod cwa;
#[doc = "CWA_MSB (rw) register accessor: an alias for `Reg<CWA_MSB_SPEC>`"]
pub type CWA_MSB = crate::Reg<cwa_msb::CWA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1"]
pub mod cwa_msb;
#[doc = "ARB_EP4_CFG (rw) register accessor: an alias for `Reg<ARB_EP4_CFG_SPEC>`"]
pub type ARB_EP4_CFG = crate::Reg<arb_ep4_cfg::ARB_EP4_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep4_cfg;
#[doc = "ARB_EP4_INT_EN (rw) register accessor: an alias for `Reg<ARB_EP4_INT_EN_SPEC>`"]
pub type ARB_EP4_INT_EN = crate::Reg<arb_ep4_int_en::ARB_EP4_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_int_en;
#[doc = "ARB_EP4_SR (rw) register accessor: an alias for `Reg<ARB_EP4_SR_SPEC>`"]
pub type ARB_EP4_SR = crate::Reg<arb_ep4_sr::ARB_EP4_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_sr;
#[doc = "ARB_RW4_WA (rw) register accessor: an alias for `Reg<ARB_RW4_WA_SPEC>`"]
pub type ARB_RW4_WA = crate::Reg<arb_rw4_wa::ARB_RW4_WA_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw4_wa;
#[doc = "ARB_RW4_WA_MSB (rw) register accessor: an alias for `Reg<ARB_RW4_WA_MSB_SPEC>`"]
pub type ARB_RW4_WA_MSB = crate::Reg<arb_rw4_wa_msb::ARB_RW4_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw4_wa_msb;
#[doc = "ARB_RW4_RA (rw) register accessor: an alias for `Reg<ARB_RW4_RA_SPEC>`"]
pub type ARB_RW4_RA = crate::Reg<arb_rw4_ra::ARB_RW4_RA_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw4_ra;
#[doc = "ARB_RW4_RA_MSB (rw) register accessor: an alias for `Reg<ARB_RW4_RA_MSB_SPEC>`"]
pub type ARB_RW4_RA_MSB = crate::Reg<arb_rw4_ra_msb::ARB_RW4_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw4_ra_msb;
#[doc = "ARB_RW4_DR (rw) register accessor: an alias for `Reg<ARB_RW4_DR_SPEC>`"]
pub type ARB_RW4_DR = crate::Reg<arb_rw4_dr::ARB_RW4_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr;
#[doc = "DMA_THRES (rw) register accessor: an alias for `Reg<DMA_THRES_SPEC>`"]
pub type DMA_THRES = crate::Reg<dma_thres::DMA_THRES_SPEC>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres;
#[doc = "DMA_THRES_MSB (rw) register accessor: an alias for `Reg<DMA_THRES_MSB_SPEC>`"]
pub type DMA_THRES_MSB = crate::Reg<dma_thres_msb::DMA_THRES_MSB_SPEC>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres_msb;
#[doc = "ARB_EP5_CFG (rw) register accessor: an alias for `Reg<ARB_EP5_CFG_SPEC>`"]
pub type ARB_EP5_CFG = crate::Reg<arb_ep5_cfg::ARB_EP5_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep5_cfg;
#[doc = "ARB_EP5_INT_EN (rw) register accessor: an alias for `Reg<ARB_EP5_INT_EN_SPEC>`"]
pub type ARB_EP5_INT_EN = crate::Reg<arb_ep5_int_en::ARB_EP5_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_int_en;
#[doc = "ARB_EP5_SR (rw) register accessor: an alias for `Reg<ARB_EP5_SR_SPEC>`"]
pub type ARB_EP5_SR = crate::Reg<arb_ep5_sr::ARB_EP5_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_sr;
#[doc = "ARB_RW5_WA (rw) register accessor: an alias for `Reg<ARB_RW5_WA_SPEC>`"]
pub type ARB_RW5_WA = crate::Reg<arb_rw5_wa::ARB_RW5_WA_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw5_wa;
#[doc = "ARB_RW5_WA_MSB (rw) register accessor: an alias for `Reg<ARB_RW5_WA_MSB_SPEC>`"]
pub type ARB_RW5_WA_MSB = crate::Reg<arb_rw5_wa_msb::ARB_RW5_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw5_wa_msb;
#[doc = "ARB_RW5_RA (rw) register accessor: an alias for `Reg<ARB_RW5_RA_SPEC>`"]
pub type ARB_RW5_RA = crate::Reg<arb_rw5_ra::ARB_RW5_RA_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw5_ra;
#[doc = "ARB_RW5_RA_MSB (rw) register accessor: an alias for `Reg<ARB_RW5_RA_MSB_SPEC>`"]
pub type ARB_RW5_RA_MSB = crate::Reg<arb_rw5_ra_msb::ARB_RW5_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw5_ra_msb;
#[doc = "ARB_RW5_DR (rw) register accessor: an alias for `Reg<ARB_RW5_DR_SPEC>`"]
pub type ARB_RW5_DR = crate::Reg<arb_rw5_dr::ARB_RW5_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr;
#[doc = "BUS_RST_CNT (rw) register accessor: an alias for `Reg<BUS_RST_CNT_SPEC>`"]
pub type BUS_RST_CNT = crate::Reg<bus_rst_cnt::BUS_RST_CNT_SPEC>;
#[doc = "Bus Reset Count Register"]
pub mod bus_rst_cnt;
#[doc = "ARB_EP6_CFG (rw) register accessor: an alias for `Reg<ARB_EP6_CFG_SPEC>`"]
pub type ARB_EP6_CFG = crate::Reg<arb_ep6_cfg::ARB_EP6_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep6_cfg;
#[doc = "ARB_EP6_INT_EN (rw) register accessor: an alias for `Reg<ARB_EP6_INT_EN_SPEC>`"]
pub type ARB_EP6_INT_EN = crate::Reg<arb_ep6_int_en::ARB_EP6_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_int_en;
#[doc = "ARB_EP6_SR (rw) register accessor: an alias for `Reg<ARB_EP6_SR_SPEC>`"]
pub type ARB_EP6_SR = crate::Reg<arb_ep6_sr::ARB_EP6_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_sr;
#[doc = "ARB_RW6_WA (rw) register accessor: an alias for `Reg<ARB_RW6_WA_SPEC>`"]
pub type ARB_RW6_WA = crate::Reg<arb_rw6_wa::ARB_RW6_WA_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw6_wa;
#[doc = "ARB_RW6_WA_MSB (rw) register accessor: an alias for `Reg<ARB_RW6_WA_MSB_SPEC>`"]
pub type ARB_RW6_WA_MSB = crate::Reg<arb_rw6_wa_msb::ARB_RW6_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw6_wa_msb;
#[doc = "ARB_RW6_RA (rw) register accessor: an alias for `Reg<ARB_RW6_RA_SPEC>`"]
pub type ARB_RW6_RA = crate::Reg<arb_rw6_ra::ARB_RW6_RA_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw6_ra;
#[doc = "ARB_RW6_RA_MSB (rw) register accessor: an alias for `Reg<ARB_RW6_RA_MSB_SPEC>`"]
pub type ARB_RW6_RA_MSB = crate::Reg<arb_rw6_ra_msb::ARB_RW6_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw6_ra_msb;
#[doc = "ARB_RW6_DR (rw) register accessor: an alias for `Reg<ARB_RW6_DR_SPEC>`"]
pub type ARB_RW6_DR = crate::Reg<arb_rw6_dr::ARB_RW6_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr;
#[doc = "ARB_EP7_CFG (rw) register accessor: an alias for `Reg<ARB_EP7_CFG_SPEC>`"]
pub type ARB_EP7_CFG = crate::Reg<arb_ep7_cfg::ARB_EP7_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep7_cfg;
#[doc = "ARB_EP7_INT_EN (rw) register accessor: an alias for `Reg<ARB_EP7_INT_EN_SPEC>`"]
pub type ARB_EP7_INT_EN = crate::Reg<arb_ep7_int_en::ARB_EP7_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_int_en;
#[doc = "ARB_EP7_SR (rw) register accessor: an alias for `Reg<ARB_EP7_SR_SPEC>`"]
pub type ARB_EP7_SR = crate::Reg<arb_ep7_sr::ARB_EP7_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_sr;
#[doc = "ARB_RW7_WA (rw) register accessor: an alias for `Reg<ARB_RW7_WA_SPEC>`"]
pub type ARB_RW7_WA = crate::Reg<arb_rw7_wa::ARB_RW7_WA_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw7_wa;
#[doc = "ARB_RW7_WA_MSB (rw) register accessor: an alias for `Reg<ARB_RW7_WA_MSB_SPEC>`"]
pub type ARB_RW7_WA_MSB = crate::Reg<arb_rw7_wa_msb::ARB_RW7_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw7_wa_msb;
#[doc = "ARB_RW7_RA (rw) register accessor: an alias for `Reg<ARB_RW7_RA_SPEC>`"]
pub type ARB_RW7_RA = crate::Reg<arb_rw7_ra::ARB_RW7_RA_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw7_ra;
#[doc = "ARB_RW7_RA_MSB (rw) register accessor: an alias for `Reg<ARB_RW7_RA_MSB_SPEC>`"]
pub type ARB_RW7_RA_MSB = crate::Reg<arb_rw7_ra_msb::ARB_RW7_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw7_ra_msb;
#[doc = "ARB_RW7_DR (rw) register accessor: an alias for `Reg<ARB_RW7_DR_SPEC>`"]
pub type ARB_RW7_DR = crate::Reg<arb_rw7_dr::ARB_RW7_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr;
#[doc = "ARB_EP8_CFG (rw) register accessor: an alias for `Reg<ARB_EP8_CFG_SPEC>`"]
pub type ARB_EP8_CFG = crate::Reg<arb_ep8_cfg::ARB_EP8_CFG_SPEC>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep8_cfg;
#[doc = "ARB_EP8_INT_EN (rw) register accessor: an alias for `Reg<ARB_EP8_INT_EN_SPEC>`"]
pub type ARB_EP8_INT_EN = crate::Reg<arb_ep8_int_en::ARB_EP8_INT_EN_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_int_en;
#[doc = "ARB_EP8_SR (rw) register accessor: an alias for `Reg<ARB_EP8_SR_SPEC>`"]
pub type ARB_EP8_SR = crate::Reg<arb_ep8_sr::ARB_EP8_SR_SPEC>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_sr;
#[doc = "ARB_RW8_WA (rw) register accessor: an alias for `Reg<ARB_RW8_WA_SPEC>`"]
pub type ARB_RW8_WA = crate::Reg<arb_rw8_wa::ARB_RW8_WA_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw8_wa;
#[doc = "ARB_RW8_WA_MSB (rw) register accessor: an alias for `Reg<ARB_RW8_WA_MSB_SPEC>`"]
pub type ARB_RW8_WA_MSB = crate::Reg<arb_rw8_wa_msb::ARB_RW8_WA_MSB_SPEC>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw8_wa_msb;
#[doc = "ARB_RW8_RA (rw) register accessor: an alias for `Reg<ARB_RW8_RA_SPEC>`"]
pub type ARB_RW8_RA = crate::Reg<arb_rw8_ra::ARB_RW8_RA_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw8_ra;
#[doc = "ARB_RW8_RA_MSB (rw) register accessor: an alias for `Reg<ARB_RW8_RA_MSB_SPEC>`"]
pub type ARB_RW8_RA_MSB = crate::Reg<arb_rw8_ra_msb::ARB_RW8_RA_MSB_SPEC>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw8_ra_msb;
#[doc = "ARB_RW8_DR (rw) register accessor: an alias for `Reg<ARB_RW8_DR_SPEC>`"]
pub type ARB_RW8_DR = crate::Reg<arb_rw8_dr::ARB_RW8_DR_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr;
#[doc = "MEM_DATA (rw) register accessor: an alias for `Reg<MEM_DATA_SPEC>`"]
pub type MEM_DATA = crate::Reg<mem_data::MEM_DATA_SPEC>;
#[doc = "DATA"]
pub mod mem_data;
#[doc = "SOF16 (r) register accessor: an alias for `Reg<SOF16_SPEC>`"]
pub type SOF16 = crate::Reg<sof16::SOF16_SPEC>;
#[doc = "Start Of Frame Register"]
pub mod sof16;
#[doc = "OSCLK_DR16 (r) register accessor: an alias for `Reg<OSCLK_DR16_SPEC>`"]
pub type OSCLK_DR16 = crate::Reg<osclk_dr16::OSCLK_DR16_SPEC>;
#[doc = "Oscillator lock data register"]
pub mod osclk_dr16;
#[doc = "ARB_RW1_WA16 (rw) register accessor: an alias for `Reg<ARB_RW1_WA16_SPEC>`"]
pub type ARB_RW1_WA16 = crate::Reg<arb_rw1_wa16::ARB_RW1_WA16_SPEC>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw1_wa16;
#[doc = "ARB_RW1_RA16 (rw) register accessor: an alias for `Reg<ARB_RW1_RA16_SPEC>`"]
pub type ARB_RW1_RA16 = crate::Reg<arb_rw1_ra16::ARB_RW1_RA16_SPEC>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw1_ra16;
#[doc = "ARB_RW1_DR16 (rw) register accessor: an alias for `Reg<ARB_RW1_DR16_SPEC>`"]
pub type ARB_RW1_DR16 = crate::Reg<arb_rw1_dr16::ARB_RW1_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr16;
#[doc = "ARB_RW2_WA16 (rw) register accessor: an alias for `Reg<ARB_RW2_WA16_SPEC>`"]
pub type ARB_RW2_WA16 = crate::Reg<arb_rw2_wa16::ARB_RW2_WA16_SPEC>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw2_wa16;
#[doc = "ARB_RW2_RA16 (rw) register accessor: an alias for `Reg<ARB_RW2_RA16_SPEC>`"]
pub type ARB_RW2_RA16 = crate::Reg<arb_rw2_ra16::ARB_RW2_RA16_SPEC>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw2_ra16;
#[doc = "ARB_RW2_DR16 (rw) register accessor: an alias for `Reg<ARB_RW2_DR16_SPEC>`"]
pub type ARB_RW2_DR16 = crate::Reg<arb_rw2_dr16::ARB_RW2_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr16;
#[doc = "ARB_RW3_WA16 (rw) register accessor: an alias for `Reg<ARB_RW3_WA16_SPEC>`"]
pub type ARB_RW3_WA16 = crate::Reg<arb_rw3_wa16::ARB_RW3_WA16_SPEC>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw3_wa16;
#[doc = "ARB_RW3_RA16 (rw) register accessor: an alias for `Reg<ARB_RW3_RA16_SPEC>`"]
pub type ARB_RW3_RA16 = crate::Reg<arb_rw3_ra16::ARB_RW3_RA16_SPEC>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw3_ra16;
#[doc = "ARB_RW3_DR16 (rw) register accessor: an alias for `Reg<ARB_RW3_DR16_SPEC>`"]
pub type ARB_RW3_DR16 = crate::Reg<arb_rw3_dr16::ARB_RW3_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr16;
#[doc = "CWA16 (rw) register accessor: an alias for `Reg<CWA16_SPEC>`"]
pub type CWA16 = crate::Reg<cwa16::CWA16_SPEC>;
#[doc = "Common Area Write Address"]
pub mod cwa16;
#[doc = "ARB_RW4_WA16 (rw) register accessor: an alias for `Reg<ARB_RW4_WA16_SPEC>`"]
pub type ARB_RW4_WA16 = crate::Reg<arb_rw4_wa16::ARB_RW4_WA16_SPEC>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw4_wa16;
#[doc = "ARB_RW4_RA16 (rw) register accessor: an alias for `Reg<ARB_RW4_RA16_SPEC>`"]
pub type ARB_RW4_RA16 = crate::Reg<arb_rw4_ra16::ARB_RW4_RA16_SPEC>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw4_ra16;
#[doc = "ARB_RW4_DR16 (rw) register accessor: an alias for `Reg<ARB_RW4_DR16_SPEC>`"]
pub type ARB_RW4_DR16 = crate::Reg<arb_rw4_dr16::ARB_RW4_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr16;
#[doc = "DMA_THRES16 (rw) register accessor: an alias for `Reg<DMA_THRES16_SPEC>`"]
pub type DMA_THRES16 = crate::Reg<dma_thres16::DMA_THRES16_SPEC>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres16;
#[doc = "ARB_RW5_WA16 (rw) register accessor: an alias for `Reg<ARB_RW5_WA16_SPEC>`"]
pub type ARB_RW5_WA16 = crate::Reg<arb_rw5_wa16::ARB_RW5_WA16_SPEC>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw5_wa16;
#[doc = "ARB_RW5_RA16 (rw) register accessor: an alias for `Reg<ARB_RW5_RA16_SPEC>`"]
pub type ARB_RW5_RA16 = crate::Reg<arb_rw5_ra16::ARB_RW5_RA16_SPEC>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw5_ra16;
#[doc = "ARB_RW5_DR16 (rw) register accessor: an alias for `Reg<ARB_RW5_DR16_SPEC>`"]
pub type ARB_RW5_DR16 = crate::Reg<arb_rw5_dr16::ARB_RW5_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr16;
#[doc = "ARB_RW6_WA16 (rw) register accessor: an alias for `Reg<ARB_RW6_WA16_SPEC>`"]
pub type ARB_RW6_WA16 = crate::Reg<arb_rw6_wa16::ARB_RW6_WA16_SPEC>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw6_wa16;
#[doc = "ARB_RW6_RA16 (rw) register accessor: an alias for `Reg<ARB_RW6_RA16_SPEC>`"]
pub type ARB_RW6_RA16 = crate::Reg<arb_rw6_ra16::ARB_RW6_RA16_SPEC>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw6_ra16;
#[doc = "ARB_RW6_DR16 (rw) register accessor: an alias for `Reg<ARB_RW6_DR16_SPEC>`"]
pub type ARB_RW6_DR16 = crate::Reg<arb_rw6_dr16::ARB_RW6_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr16;
#[doc = "ARB_RW7_WA16 (rw) register accessor: an alias for `Reg<ARB_RW7_WA16_SPEC>`"]
pub type ARB_RW7_WA16 = crate::Reg<arb_rw7_wa16::ARB_RW7_WA16_SPEC>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw7_wa16;
#[doc = "ARB_RW7_RA16 (rw) register accessor: an alias for `Reg<ARB_RW7_RA16_SPEC>`"]
pub type ARB_RW7_RA16 = crate::Reg<arb_rw7_ra16::ARB_RW7_RA16_SPEC>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw7_ra16;
#[doc = "ARB_RW7_DR16 (rw) register accessor: an alias for `Reg<ARB_RW7_DR16_SPEC>`"]
pub type ARB_RW7_DR16 = crate::Reg<arb_rw7_dr16::ARB_RW7_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr16;
#[doc = "ARB_RW8_WA16 (rw) register accessor: an alias for `Reg<ARB_RW8_WA16_SPEC>`"]
pub type ARB_RW8_WA16 = crate::Reg<arb_rw8_wa16::ARB_RW8_WA16_SPEC>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw8_wa16;
#[doc = "ARB_RW8_RA16 (rw) register accessor: an alias for `Reg<ARB_RW8_RA16_SPEC>`"]
pub type ARB_RW8_RA16 = crate::Reg<arb_rw8_ra16::ARB_RW8_RA16_SPEC>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw8_ra16;
#[doc = "ARB_RW8_DR16 (rw) register accessor: an alias for `Reg<ARB_RW8_DR16_SPEC>`"]
pub type ARB_RW8_DR16 = crate::Reg<arb_rw8_dr16::ARB_RW8_DR16_SPEC>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr16;
