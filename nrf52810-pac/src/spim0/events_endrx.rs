#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_ENDRX { } 
#[doc = "Possible values of the field `EVENTS_ENDRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_ENDRXR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_ENDRXR { } 
#[doc = "Values that can be written to the field `EVENTS_ENDRX`"]
pub enum EVENTS_ENDRXW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_ENDRXW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_ENDRXW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_ENDRXW<'a> { } 
impl R { } 
impl W { } 
