#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERRORSRC { } 
#[doc = "Possible values of the field `OVERFLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOWR {
    #[doc = "Error did not occur"]
    NOTDETECTED,
    #[doc = "Error occurred"]
    DETECTED,
}
impl OVERFLOWR { } 
#[doc = "Possible values of the field `DNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNACKR {
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
}
impl DNACKR { } 
#[doc = "Possible values of the field `OVERREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREADR {
    #[doc = "Error did not occur"]
    NOTDETECTED,
    #[doc = "Error occurred"]
    DETECTED,
}
impl OVERREADR { } 
#[doc = "Values that can be written to the field `OVERFLOW`"]
pub enum OVERFLOWW {
    #[doc = "Error did not occur"]
    NOTDETECTED,
    #[doc = "Error occurred"]
    DETECTED,
}
impl OVERFLOWW { } 
#[doc = r" Proxy"]
pub struct _OVERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFLOWW<'a> { } 
#[doc = "Values that can be written to the field `DNACK`"]
pub enum DNACKW {
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
}
impl DNACKW { } 
#[doc = r" Proxy"]
pub struct _DNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DNACKW<'a> { } 
#[doc = "Values that can be written to the field `OVERREAD`"]
pub enum OVERREADW {
    #[doc = "Error did not occur"]
    NOTDETECTED,
    #[doc = "Error occurred"]
    DETECTED,
}
impl OVERREADW { } 
#[doc = r" Proxy"]
pub struct _OVERREADW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERREADW<'a> { } 
impl R { } 
impl W { } 
