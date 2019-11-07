#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CNT { } 
#[doc = "Possible values of the field `CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR {
    #[doc = "Sequence is disabled, and shall not be started as it is empty"]
    DISABLED,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CNTR { } 
#[doc = "Values that can be written to the field `CNT`"]
pub enum CNTW {
    #[doc = "Sequence is disabled, and shall not be started as it is empty"]
    DISABLED,
}
impl CNTW { } 
#[doc = r" Proxy"]
pub struct _CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTW<'a> { } 
impl R { } 
impl W { } 
