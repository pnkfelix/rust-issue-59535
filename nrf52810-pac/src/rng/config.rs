#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG { } 
#[doc = "Possible values of the field `DERCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DERCENR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DERCENR { } 
#[doc = "Values that can be written to the field `DERCEN`"]
pub enum DERCENW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DERCENW { } 
#[doc = r" Proxy"]
pub struct _DERCENW<'a> {
    w: &'a mut W,
}
impl<'a> _DERCENW<'a> { } 
impl R { } 
impl W { } 
