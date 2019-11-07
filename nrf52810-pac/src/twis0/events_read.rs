#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_READ { } 
#[doc = "Possible values of the field `EVENTS_READ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_READR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_READR { } 
#[doc = "Values that can be written to the field `EVENTS_READ`"]
pub enum EVENTS_READW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_READW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_READW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_READW<'a> { } 
impl R { } 
impl W { } 
