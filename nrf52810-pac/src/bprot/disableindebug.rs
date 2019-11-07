#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DISABLEINDEBUG { } 
#[doc = "Possible values of the field `DISABLEINDEBUG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLEINDEBUGR {
    #[doc = "Disabled in debug"]
    DISABLED,
    #[doc = "Enabled in debug"]
    ENABLED,
}
impl DISABLEINDEBUGR { } 
#[doc = "Values that can be written to the field `DISABLEINDEBUG`"]
pub enum DISABLEINDEBUGW {
    #[doc = "Disabled in debug"]
    DISABLED,
    #[doc = "Enabled in debug"]
    ENABLED,
}
impl DISABLEINDEBUGW { } 
#[doc = r" Proxy"]
pub struct _DISABLEINDEBUGW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLEINDEBUGW<'a> { } 
impl R { } 
impl W { } 
