#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_RXSTARTED { } 
#[doc = "Possible values of the field `EVENTS_RXSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_RXSTARTEDR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_RXSTARTEDR { } 
#[doc = "Values that can be written to the field `EVENTS_RXSTARTED`"]
pub enum EVENTS_RXSTARTEDW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_RXSTARTEDW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_RXSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_RXSTARTEDW<'a> { } 
impl R { } 
impl W { } 
