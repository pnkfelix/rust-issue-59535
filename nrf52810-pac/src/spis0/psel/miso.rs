#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISO { }

#[doc = r" Value of the field"]
pub struct PINR {
    bits: u8,
}
impl PINR { }

#[doc = "Possible values of the field `CONNECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONNECTR {
    #[doc = "Disconnect"]
    DISCONNECTED,
    #[doc = "Connect"]
    CONNECTED,
}
impl CONNECTR { }

#[doc = r" Proxy"]
pub struct _PINW<'a> {
    w: &'a mut W,
}
impl<'a> _PINW<'a> { }

#[doc = "Values that can be written to the field `CONNECT`"]
pub enum CONNECTW {
    #[doc = "Disconnect"]
    DISCONNECTED,
    #[doc = "Connect"]
    CONNECTED,
}
impl CONNECTW { }

#[doc = r" Proxy"]
pub struct _CONNECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONNECTW<'a> { }

impl R { }

impl W { }

