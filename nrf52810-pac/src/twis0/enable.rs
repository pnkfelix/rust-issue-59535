#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENABLE { } 
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Disable TWIS"]
    DISABLED,
    #[doc = "Enable TWIS"]
    ENABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENABLER { } 
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "Disable TWIS"]
    DISABLED,
    #[doc = "Enable TWIS"]
    ENABLED,
}
impl ENABLEW { } 
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> { } 
impl R { } 
impl W { } 
