#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERRORSRC { } 
#[doc = "Possible values of the field `OVERRUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUNR {
    #[doc = "Read: no overrun occured"]
    NOTPRESENT,
    #[doc = "Read: overrun occured"]
    PRESENT,
}
impl OVERRUNR { } 
#[doc = "Possible values of the field `ANACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACKR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl ANACKR { } 
#[doc = "Possible values of the field `DNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNACKR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl DNACKR { } 
#[doc = "Values that can be written to the field `OVERRUN`"]
pub enum OVERRUNW {
    #[doc = "Read: no overrun occured"]
    NOTPRESENT,
    #[doc = "Read: overrun occured"]
    PRESENT,
}
impl OVERRUNW { } 
#[doc = r" Proxy"]
pub struct _OVERRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERRUNW<'a> { } 
#[doc = "Values that can be written to the field `ANACK`"]
pub enum ANACKW {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl ANACKW { } 
#[doc = r" Proxy"]
pub struct _ANACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ANACKW<'a> { } 
#[doc = "Values that can be written to the field `DNACK`"]
pub enum DNACKW {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl DNACKW { } 
#[doc = r" Proxy"]
pub struct _DNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DNACKW<'a> { } 
impl R { } 
impl W { } 
