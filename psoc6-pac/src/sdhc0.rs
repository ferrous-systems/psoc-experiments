#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MMIO at SDHC wrapper level"]
    pub wrap: WRAP,
    _reserved1: [u8; 0x0ffc],
    #[doc = "0x1000..0x1538 - MMIO for Synopsys Mobile Storage Host Controller IP"]
    pub core: CORE,
}
#[doc = "MMIO at SDHC wrapper level"]
pub use self::wrap::WRAP;
#[doc = r"Cluster"]
#[doc = "MMIO at SDHC wrapper level"]
pub mod wrap;
#[doc = "MMIO for Synopsys Mobile Storage Host Controller IP"]
pub use self::core::CORE;
#[doc = r"Cluster"]
#[doc = "MMIO for Synopsys Mobile Storage Host Controller IP"]
pub mod core;
