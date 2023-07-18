#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - RTC Read Write register"]
    pub rtc_rw: RTC_RW,
    #[doc = "0x0c - Oscillator calibration for absolute frequency"]
    pub cal_ctl: CAL_CTL,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    #[doc = "0x14 - Calendar Seconds, Minutes, Hours, Day of Week"]
    pub rtc_time: RTC_TIME,
    #[doc = "0x18 - Calendar Day of Month, Month, Year"]
    pub rtc_date: RTC_DATE,
    #[doc = "0x1c - Alarm 1 Seconds, Minute, Hours, Day of Week"]
    pub alm1_time: ALM1_TIME,
    #[doc = "0x20 - Alarm 1 Day of Month, Month"]
    pub alm1_date: ALM1_DATE,
    #[doc = "0x24 - Alarm 2 Seconds, Minute, Hours, Day of Week"]
    pub alm2_time: ALM2_TIME,
    #[doc = "0x28 - Alarm 2 Day of Month, Month"]
    pub alm2_date: ALM2_DATE,
    #[doc = "0x2c - Interrupt request register"]
    pub intr: INTR,
    #[doc = "0x30 - Interrupt set request register"]
    pub intr_set: INTR_SET,
    #[doc = "0x34 - Interrupt mask register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x38 - Interrupt masked request register"]
    pub intr_masked: INTR_MASKED,
    #[doc = "0x3c - 32kHz oscillator counter"]
    pub osccnt: OSCCNT,
    #[doc = "0x40 - 128Hz tick counter"]
    pub ticks: TICKS,
    #[doc = "0x44 - PMIC control register"]
    pub pmic_ctl: PMIC_CTL,
    #[doc = "0x48 - Backup reset register"]
    pub reset: RESET,
    _reserved18: [u8; 0x0fb4],
    #[doc = "0x1000..0x1100 - Backup register region"]
    pub breg: [BREG; 64],
    _reserved19: [u8; 0xee00],
    #[doc = "0xff00 - Trim Register"]
    pub trim: TRIM,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "RTC_RW (rw) register accessor: an alias for `Reg<RTC_RW_SPEC>`"]
pub type RTC_RW = crate::Reg<rtc_rw::RTC_RW_SPEC>;
#[doc = "RTC Read Write register"]
pub mod rtc_rw;
#[doc = "CAL_CTL (rw) register accessor: an alias for `Reg<CAL_CTL_SPEC>`"]
pub type CAL_CTL = crate::Reg<cal_ctl::CAL_CTL_SPEC>;
#[doc = "Oscillator calibration for absolute frequency"]
pub mod cal_ctl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "RTC_TIME (rw) register accessor: an alias for `Reg<RTC_TIME_SPEC>`"]
pub type RTC_TIME = crate::Reg<rtc_time::RTC_TIME_SPEC>;
#[doc = "Calendar Seconds, Minutes, Hours, Day of Week"]
pub mod rtc_time;
#[doc = "RTC_DATE (rw) register accessor: an alias for `Reg<RTC_DATE_SPEC>`"]
pub type RTC_DATE = crate::Reg<rtc_date::RTC_DATE_SPEC>;
#[doc = "Calendar Day of Month, Month, Year"]
pub mod rtc_date;
#[doc = "ALM1_TIME (rw) register accessor: an alias for `Reg<ALM1_TIME_SPEC>`"]
pub type ALM1_TIME = crate::Reg<alm1_time::ALM1_TIME_SPEC>;
#[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week"]
pub mod alm1_time;
#[doc = "ALM1_DATE (rw) register accessor: an alias for `Reg<ALM1_DATE_SPEC>`"]
pub type ALM1_DATE = crate::Reg<alm1_date::ALM1_DATE_SPEC>;
#[doc = "Alarm 1 Day of Month, Month"]
pub mod alm1_date;
#[doc = "ALM2_TIME (rw) register accessor: an alias for `Reg<ALM2_TIME_SPEC>`"]
pub type ALM2_TIME = crate::Reg<alm2_time::ALM2_TIME_SPEC>;
#[doc = "Alarm 2 Seconds, Minute, Hours, Day of Week"]
pub mod alm2_time;
#[doc = "ALM2_DATE (rw) register accessor: an alias for `Reg<ALM2_DATE_SPEC>`"]
pub type ALM2_DATE = crate::Reg<alm2_date::ALM2_DATE_SPEC>;
#[doc = "Alarm 2 Day of Month, Month"]
pub mod alm2_date;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
#[doc = "OSCCNT (r) register accessor: an alias for `Reg<OSCCNT_SPEC>`"]
pub type OSCCNT = crate::Reg<osccnt::OSCCNT_SPEC>;
#[doc = "32kHz oscillator counter"]
pub mod osccnt;
#[doc = "TICKS (r) register accessor: an alias for `Reg<TICKS_SPEC>`"]
pub type TICKS = crate::Reg<ticks::TICKS_SPEC>;
#[doc = "128Hz tick counter"]
pub mod ticks;
#[doc = "PMIC_CTL (rw) register accessor: an alias for `Reg<PMIC_CTL_SPEC>`"]
pub type PMIC_CTL = crate::Reg<pmic_ctl::PMIC_CTL_SPEC>;
#[doc = "PMIC control register"]
pub mod pmic_ctl;
#[doc = "RESET (rw) register accessor: an alias for `Reg<RESET_SPEC>`"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "Backup reset register"]
pub mod reset;
#[doc = "BREG (rw) register accessor: an alias for `Reg<BREG_SPEC>`"]
pub type BREG = crate::Reg<breg::BREG_SPEC>;
#[doc = "Backup register region"]
pub mod breg;
#[doc = "TRIM (rw) register accessor: an alias for `Reg<TRIM_SPEC>`"]
pub type TRIM = crate::Reg<trim::TRIM_SPEC>;
#[doc = "Trim Register"]
pub mod trim;
