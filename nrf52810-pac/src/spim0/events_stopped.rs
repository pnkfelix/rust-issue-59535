#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_STOPPED { } 
#[doc = "Possible values of the field `EVENTS_STOPPED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_STOPPEDR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_STOPPEDR { } 
#[doc = "Values that can be written to the field `EVENTS_STOPPED`"]
pub enum EVENTS_STOPPEDW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_STOPPEDW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_STOPPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_STOPPEDW<'a> { } 
impl R { } 
impl W { } 
