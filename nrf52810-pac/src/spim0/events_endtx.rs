#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_ENDTX { } 
#[doc = "Possible values of the field `EVENTS_ENDTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_ENDTXR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_ENDTXR { } 
#[doc = "Values that can be written to the field `EVENTS_ENDTX`"]
pub enum EVENTS_ENDTXW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_ENDTXW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_ENDTXW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_ENDTXW<'a> { } 
impl R { } 
impl W { } 
