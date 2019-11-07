#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_LOOPSDONE { } 
#[doc = "Possible values of the field `EVENTS_LOOPSDONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_LOOPSDONER {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_LOOPSDONER { } 
#[doc = "Values that can be written to the field `EVENTS_LOOPSDONE`"]
pub enum EVENTS_LOOPSDONEW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_LOOPSDONEW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_LOOPSDONEW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_LOOPSDONEW<'a> { } 
impl R { } 
impl W { } 
