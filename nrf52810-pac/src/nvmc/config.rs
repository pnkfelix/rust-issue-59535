#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG { } 
#[doc = "Possible values of the field `WEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WENR {
    #[doc = "Read only access"]
    REN,
    #[doc = "Write enabled"]
    WEN,
    #[doc = "Erase enabled"]
    EEN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WENR { } 
#[doc = "Values that can be written to the field `WEN`"]
pub enum WENW {
    #[doc = "Read only access"]
    REN,
    #[doc = "Write enabled"]
    WEN,
    #[doc = "Erase enabled"]
    EEN,
}
impl WENW { } 
#[doc = r" Proxy"]
pub struct _WENW<'a> {
    w: &'a mut W,
}
impl<'a> _WENW<'a> { } 
impl R { } 
impl W { } 
