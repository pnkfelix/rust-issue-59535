#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_SEQEND { } 
#[doc = "Possible values of the field `EVENTS_SEQEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_SEQENDR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_SEQENDR { } 
#[doc = "Values that can be written to the field `EVENTS_SEQEND`"]
pub enum EVENTS_SEQENDW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_SEQENDW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_SEQENDW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_SEQENDW<'a> { } 
impl R { } 
impl W { } 
