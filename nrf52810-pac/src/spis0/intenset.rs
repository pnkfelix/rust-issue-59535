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
pub struct ENDR ;
impl ENDR { }

#[doc = "Possible values of the field `ENDRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENDRXR ;
impl ENDRXR { }

#[doc = "Possible values of the field `ACQUIRED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACQUIREDR ;
impl ACQUIREDR { }

#[doc = "Values that can be written to the field `END`"]
pub struct ENDW ;
impl ENDW { }

#[doc = r" Proxy"]
pub struct _ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDW<'a> { }

#[doc = "Values that can be written to the field `ENDRX`"]
pub struct ENDRXW ;
impl ENDRXW { }

#[doc = r" Proxy"]
pub struct _ENDRXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDRXW<'a> { }

#[doc = "Values that can be written to the field `ACQUIRED`"]
pub struct ACQUIREDW ;
impl ACQUIREDW { }

#[doc = r" Proxy"]
pub struct _ACQUIREDW<'a> {
    w: &'a mut W,
}
impl<'a> _ACQUIREDW<'a> { }

impl R { }

impl W { }

