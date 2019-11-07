#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE { } 
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Select Timer mode"]
    TIMER,
    #[doc = "Deprecated enumerator -  Select Counter mode"]
    COUNTER,
    #[doc = "Select Low Power Counter mode"]
    LOWPOWERCOUNTER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER { } 
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Select Timer mode"]
    TIMER,
    #[doc = "Deprecated enumerator -  Select Counter mode"]
    COUNTER,
    #[doc = "Select Low Power Counter mode"]
    LOWPOWERCOUNTER,
}
impl MODEW { } 
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> { } 
impl R { } 
impl W { } 
