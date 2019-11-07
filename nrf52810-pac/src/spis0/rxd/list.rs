#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LIST { }

#[doc = "Possible values of the field `LIST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LISTR {
    #[doc = "Disable EasyDMA list"]
    DISABLED,
    #[doc = "Use array list"]
    ARRAYLIST,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LISTR { }

#[doc = "Values that can be written to the field `LIST`"]
pub enum LISTW {
    #[doc = "Disable EasyDMA list"]
    DISABLED,
    #[doc = "Use array list"]
    ARRAYLIST,
}
impl LISTW { }

#[doc = r" Proxy"]
pub struct _LISTW<'a> {
    w: &'a mut W,
}
impl<'a> _LISTW<'a> { }

impl R { }

impl W { }

