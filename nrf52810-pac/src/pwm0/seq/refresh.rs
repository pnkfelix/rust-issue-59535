#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REFRESH { } 
#[doc = "Possible values of the field `CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR {
    #[doc = "Update every PWM period"]
    CONTINUOUS,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CNTR { } 
#[doc = "Values that can be written to the field `CNT`"]
pub enum CNTW {
    #[doc = "Update every PWM period"]
    CONTINUOUS,
}
impl CNTW { } 
#[doc = r" Proxy"]
pub struct _CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTW<'a> { } 
impl R { } 
impl W { } 
