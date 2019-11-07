#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG { }

#[doc = "Possible values of the field `ORDER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ORDERR ;
impl ORDERR { }

#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPHAR ;
impl CPHAR { }

#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CPOLR ;
impl CPOLR { }

#[doc = "Values that can be written to the field `ORDER`"]
pub struct ORDERW ;
impl ORDERW { }

#[doc = r" Proxy"]
pub struct _ORDERW<'a> {
    w: &'a mut W,
}
impl<'a> _ORDERW<'a> { }

#[doc = "Values that can be written to the field `CPHA`"]
pub struct CPHAW ;
impl CPHAW { }

#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> { }

#[doc = "Values that can be written to the field `CPOL`"]
pub struct CPOLW ;
impl CPOLW { }

#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> { }

impl R { }

impl W { }

