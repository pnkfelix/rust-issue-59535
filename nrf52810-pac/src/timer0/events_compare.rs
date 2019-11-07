#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_COMPARE { } 
#[doc = "Possible values of the field `EVENTS_COMPARE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_COMPARER {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_COMPARER { } 
#[doc = "Values that can be written to the field `EVENTS_COMPARE`"]
pub enum EVENTS_COMPAREW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_COMPAREW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_COMPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_COMPAREW<'a> { } 
impl R { } 
impl W { } 
