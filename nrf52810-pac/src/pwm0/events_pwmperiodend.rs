#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVENTS_PWMPERIODEND { } 
#[doc = "Possible values of the field `EVENTS_PWMPERIODEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_PWMPERIODENDR {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_PWMPERIODENDR { } 
#[doc = "Values that can be written to the field `EVENTS_PWMPERIODEND`"]
pub enum EVENTS_PWMPERIODENDW {
    #[doc = "Event not generated"]
    NOTGENERATED,
    #[doc = "Event generated"]
    GENERATED,
}
impl EVENTS_PWMPERIODENDW { } 
#[doc = r" Proxy"]
pub struct _EVENTS_PWMPERIODENDW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTS_PWMPERIODENDW<'a> { } 
impl R { } 
impl W { } 
