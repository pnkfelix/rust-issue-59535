#[doc = r" Value read from the register"]
pub struct R;
#[doc = r" Value to write to the register"]
pub struct W;
impl super::STATUS { }

#[doc = "Possible values of the field `OVERREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OVERREADR ;
impl OVERREADR { }

#[doc = "Possible values of the field `OVERFLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OVERFLOWR ;
impl OVERFLOWR { }

#[doc = "Values that can be written to the field `OVERREAD`"]
pub struct OVERREADW ;
impl OVERREADW { }

#[doc = r" Proxy"]
pub struct _OVERREADW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERREADW<'a> { }

#[doc = "Values that can be written to the field `OVERFLOW`"]
pub struct OVERFLOWW ;
impl OVERFLOWW { }

#[doc = r" Proxy"]
pub struct _OVERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFLOWW<'a> { }

impl R { }

impl W { }

