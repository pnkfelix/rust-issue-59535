#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERASEALL { } 
#[doc = "Possible values of the field `ERASEALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEALLR {
    #[doc = "No operation"]
    NOOPERATION,
    #[doc = "Start erase of chip"]
    ERASE,
}
impl ERASEALLR { } 
#[doc = "Values that can be written to the field `ERASEALL`"]
pub enum ERASEALLW {
    #[doc = "No operation"]
    NOOPERATION,
    #[doc = "Start erase of chip"]
    ERASE,
}
impl ERASEALLW { } 
#[doc = r" Proxy"]
pub struct _ERASEALLW<'a> {
    w: &'a mut W,
}
impl<'a> _ERASEALLW<'a> { } 
impl R { } 
impl W { } 
