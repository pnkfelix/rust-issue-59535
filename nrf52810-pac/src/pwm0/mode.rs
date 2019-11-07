#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE { } 
#[doc = "Possible values of the field `UPDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDOWNR {
    #[doc = "Up counter, edge-aligned PWM duty cycle"]
    UP,
    #[doc = "Up and down counter, center-aligned PWM duty cycle"]
    UPANDDOWN,
}
impl UPDOWNR { } 
#[doc = "Values that can be written to the field `UPDOWN`"]
pub enum UPDOWNW {
    #[doc = "Up counter, edge-aligned PWM duty cycle"]
    UP,
    #[doc = "Up and down counter, center-aligned PWM duty cycle"]
    UPANDDOWN,
}
impl UPDOWNW { } 
#[doc = r" Proxy"]
pub struct _UPDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDOWNW<'a> { } 
impl R { } 
impl W { } 
