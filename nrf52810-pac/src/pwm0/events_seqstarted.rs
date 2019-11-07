#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_SEQSTARTED { } 
#[doc = "Possible values of the field `EVENTS_SEQSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_SEQSTARTEDR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_SEQSTARTEDR { } 
#[doc = "Values that can be written to the field `EVENTS_SEQSTARTED`"]
pub enum EVENTS_SEQSTARTEDW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_SEQSTARTEDW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_SEQSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_SEQSTARTEDW<'a> { } 
impl R { } 
impl W { } 
