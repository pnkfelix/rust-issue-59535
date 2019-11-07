#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SEMSTAT { }

#[doc = "Possible values of the field `SEMSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMSTATR {
    #[doc = "Semaphore is free"]
    FREE,
    #[doc = "Semaphore is assigned to CPU"]
    CPU,
    #[doc = "Semaphore is assigned to SPI slave"]
    SPIS,
    #[doc = "Semaphore is assigned to SPI but a handover to the CPU is pending"]
    CPUPENDING,
}
impl SEMSTATR { }

impl R { }

