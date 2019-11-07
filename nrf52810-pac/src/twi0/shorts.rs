#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS { } 
#[doc = "Possible values of the field `BB_SUSPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BB_SUSPENDR ;
impl BB_SUSPENDR { } 
#[doc = "Possible values of the field `BB_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BB_STOPR ;
impl BB_STOPR { } 
#[doc = "Values that can be written to the field `BB_SUSPEND`"]
pub struct BB_SUSPENDW ;
impl BB_SUSPENDW { } 
#[doc = r" Proxy"]
pub struct _BB_SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_SUSPENDW<'a> { } 
#[doc = "Values that can be written to the field `BB_STOP`"]
pub struct BB_STOPW ;
impl BB_STOPW { } 
#[doc = r" Proxy"]
pub struct _BB_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_STOPW<'a> { } 
impl R { } 
impl W { } 
