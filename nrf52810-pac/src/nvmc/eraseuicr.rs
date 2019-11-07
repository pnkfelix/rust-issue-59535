#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERASEUICR { } 
#[doc = "Possible values of the field `ERASEUICR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEUICRR {
    #[doc = "No operation"]
    NOOPERATION,
    #[doc = "Start erase of UICR"]
    ERASE,
}
impl ERASEUICRR { } 
#[doc = "Values that can be written to the field `ERASEUICR`"]
pub enum ERASEUICRW {
    #[doc = "No operation"]
    NOOPERATION,
    #[doc = "Start erase of UICR"]
    ERASE,
}
impl ERASEUICRW { } 
#[doc = r" Proxy"]
pub struct _ERASEUICRW<'a> {
    w: &'a mut W,
}
impl<'a> _ERASEUICRW<'a> { } 
impl R { } 
impl W { } 
