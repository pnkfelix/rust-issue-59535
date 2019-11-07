#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS { }

#[doc = "Possible values of the field `OVERREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREADR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl OVERREADR { }

#[doc = "Possible values of the field `OVERFLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOWR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl OVERFLOWR { }

#[doc = "Values that can be written to the field `OVERREAD`"]
pub enum OVERREADW {
    #[doc = "Write: clear error on writing '1'"]
    CLEAR,
}
impl OVERREADW { }

#[doc = r" Proxy"]
pub struct _OVERREADW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERREADW<'a> { }

#[doc = "Values that can be written to the field `OVERFLOW`"]
pub enum OVERFLOWW {
    #[doc = "Write: clear error on writing '1'"]
    CLEAR,
}
impl OVERFLOWW { }

#[doc = r" Proxy"]
pub struct _OVERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFLOWW<'a> { }

impl R { }

impl W { }

