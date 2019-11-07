#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENCLR { } 
#[doc = "Possible values of the field `DATARDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATARDYR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl DATARDYR { } 
#[doc = "Values that can be written to the field `DATARDY`"]
pub enum DATARDYW {
    #[doc = "Disable"]
    CLEAR,
}
impl DATARDYW { } 
#[doc = r" Proxy"]
pub struct _DATARDYW<'a> {
    w: &'a mut W,
}
impl<'a> _DATARDYW<'a> { } 
impl R { } 
impl W { } 
