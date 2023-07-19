#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xd0 - Fault structure"]
    pub struct0: STRUCT,
    _reserved1: [u8; 0x30],
    #[doc = "0x100..0x1d0 - Fault structure"]
    pub struct1: STRUCT,
}
#[doc = "Fault structure"]
pub use self::struct_::STRUCT;
#[doc = r"Cluster"]
#[doc = "Fault structure"]
pub mod struct_;
