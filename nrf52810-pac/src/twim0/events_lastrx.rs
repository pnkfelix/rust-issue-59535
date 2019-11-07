#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_LASTRX { } 
#[doc = "Possible values of the field `EVENTS_LASTRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_LASTRXR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_LASTRXR { } 
#[doc = "Values that can be written to the field `EVENTS_LASTRX`"]
pub enum EVENTS_LASTRXW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_LASTRXW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_LASTRXW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_LASTRXW<'a> { } 
impl R { } 
impl W { } 
