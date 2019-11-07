#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_VALRDY { } 
#[doc = "Possible values of the field `EVENTS_VALRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_VALRDYR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_VALRDYR { } 
#[doc = "Values that can be written to the field `EVENTS_VALRDY`"]
pub enum EVENTS_VALRDYW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_VALRDYW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_VALRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_VALRDYW<'a> { } 
impl R { } 
impl W { } 
