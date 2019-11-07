#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RAM { } 
#[doc = "Possible values of the field `RAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMR {
    #[doc = "24 kByte RAM"]
    K24,
    #[doc = "Unspecified"]
    UNSPECIFIED,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl RAMR { } 
impl R { } 
