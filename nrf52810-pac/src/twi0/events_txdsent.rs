#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_TXDSENT { } 
#[doc = "Possible values of the field `EVENTS_TXDSENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_TXDSENTR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_TXDSENTR { } 
#[doc = "Values that can be written to the field `EVENTS_TXDSENT`"]
pub enum EVENTS_TXDSENTW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_TXDSENTW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_TXDSENTW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_TXDSENTW<'a> { } 
impl R { } 
impl W { } 
