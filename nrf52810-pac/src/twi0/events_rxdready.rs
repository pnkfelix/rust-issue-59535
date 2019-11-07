#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_RXDREADY { } 
#[doc = "Possible values of the field `EVENTS_RXDREADY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_RXDREADYR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_RXDREADYR { } 
#[doc = "Values that can be written to the field `EVENTS_RXDREADY`"]
pub enum EVENTS_RXDREADYW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_RXDREADYW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_RXDREADYW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_RXDREADYW<'a> { } 
impl R { } 
impl W { } 
