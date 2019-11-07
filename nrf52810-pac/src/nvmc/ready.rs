#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::READY { } 
#[doc = "Possible values of the field `READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READYR {
    #[doc = "NVMC is busy (ongoing write or erase operation)"]
    BUSY,
    #[doc = "NVMC is ready"]
    READY,
}
impl READYR { } 
impl R { } 
