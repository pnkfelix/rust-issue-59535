#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_END { } 
#[doc = "Possible values of the field `EVENTS_END`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_ENDR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_ENDR { } 
#[doc = "Values that can be written to the field `EVENTS_END`"]
pub enum EVENTS_ENDW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_ENDW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_ENDW<'a> { } 
impl R { } 
impl W { } 
