#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_READY { } 
#[doc = "Possible values of the field `EVENTS_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_READYR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_READYR { } 
#[doc = "Values that can be written to the field `EVENTS_READY`"]
pub enum EVENTS_READYW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_READYW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_READYW<'a> { } 
impl R { } 
impl W { } 
