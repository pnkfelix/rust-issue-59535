#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_WRITE { } 
#[doc = "Possible values of the field `EVENTS_WRITE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_WRITER {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_WRITER { } 
#[doc = "Values that can be written to the field `EVENTS_WRITE`"]
pub enum EVENTS_WRITEW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_WRITEW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_WRITEW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_WRITEW<'a> { } 
impl R { } 
impl W { } 
