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
pub enum ORDERR {
    #[doc = "Most significant bit shifted out first"]
    MSBFIRST,
    #[doc = "Least significant bit shifted out first"]
    LSBFIRST,
}
impl ORDERR { }

#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    LEADING,
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    TRAILING,
}
impl CPHAR { }

#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "Active high"]
    ACTIVEHIGH,
    #[doc = "Active low"]
    ACTIVELOW,
}
impl CPOLR { }

#[doc = "Values that can be written to the field `ORDER`"]
pub enum ORDERW {
    #[doc = "Most significant bit shifted out first"]
    MSBFIRST,
    #[doc = "Least significant bit shifted out first"]
    LSBFIRST,
}
impl ORDERW { }

#[doc = r" Proxy"]
pub struct _ORDERW<'a> {
    w: &'a mut W,
}
impl<'a> _ORDERW<'a> { }

#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    LEADING,
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    TRAILING,
}
impl CPHAW { }

#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> { }

#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "Active high"]
    ACTIVEHIGH,
    #[doc = "Active low"]
    ACTIVELOW,
}
impl CPOLW { }

#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> { }

impl R { }

impl W { }

