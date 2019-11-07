#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_LASTTX { } 
#[doc = "Possible values of the field `EVENTS_LASTTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_LASTTXR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_LASTTXR { } 
#[doc = "Values that can be written to the field `EVENTS_LASTTX`"]
pub enum EVENTS_LASTTXW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_LASTTXW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_LASTTXW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_LASTTXW<'a> { } 
impl R { } 
impl W { } 
