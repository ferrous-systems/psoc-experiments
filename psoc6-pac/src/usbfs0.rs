#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x13e4 - USB Device"]
    pub usbdev: USBDEV,
    _reserved1: [u8; 0x0c1c],
    #[doc = "0x2000..0x2074 - USB Device LPM and PHY Test"]
    pub usblpm: USBLPM,
    _reserved2: [u8; 0x1f8c],
    #[doc = "0x4000..0x4b34 - USB Host Controller"]
    pub usbhost: USBHOST,
}
#[doc = "USB Device"]
pub use self::usbdev::USBDEV;
#[doc = r"Cluster"]
#[doc = "USB Device"]
pub mod usbdev;
#[doc = "USB Device LPM and PHY Test"]
pub use self::usblpm::USBLPM;
#[doc = r"Cluster"]
#[doc = "USB Device LPM and PHY Test"]
pub mod usblpm;
#[doc = "USB Host Controller"]
pub use self::usbhost::USBHOST;
#[doc = r"Cluster"]
#[doc = "USB Host Controller"]
pub mod usbhost;
