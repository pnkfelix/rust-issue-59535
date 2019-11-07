#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET { }

#[doc = "Possible values of the field `END`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDR { }

#[doc = "Possible values of the field `ENDRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRXR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDRXR { }

#[doc = "Possible values of the field `ACQUIRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACQUIREDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ACQUIREDR { }

#[doc = "Values that can be written to the field `END`"]
pub enum ENDW {
    #[doc = "Enable"]
    SET,
}
impl ENDW { }

#[doc = r" Proxy"]
pub struct _ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDW<'a> { }

#[doc = "Values that can be written to the field `ENDRX`"]
pub enum ENDRXW {
    #[doc = "Enable"]
    SET,
}
impl ENDRXW { }

#[doc = r" Proxy"]
pub struct _ENDRXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDRXW<'a> { }

#[doc = "Values that can be written to the field `ACQUIRED`"]
pub enum ACQUIREDW {
    #[doc = "Enable"]
    SET,
}
impl ACQUIREDW { }

#[doc = r" Proxy"]
pub struct _ACQUIREDW<'a> {
    w: &'a mut W,
}
impl<'a> _ACQUIREDW<'a> { }

impl R { }

impl W { }

