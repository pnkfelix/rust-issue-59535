#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_DATARDY { } 
#[doc = "Possible values of the field `EVENTS_DATARDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_DATARDYR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_DATARDYR { } 
#[doc = "Values that can be written to the field `EVENTS_DATARDY`"]
pub enum EVENTS_DATARDYW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_DATARDYW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_DATARDYW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_DATARDYW<'a> { } 
impl R { } 
impl W { } 
