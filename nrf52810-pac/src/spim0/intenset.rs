#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET { } 
#[doc = "Possible values of the field `STOPPED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl STOPPEDR { } 
#[doc = "Possible values of the field `ENDRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRXR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDRXR { } 
#[doc = "Possible values of the field `END`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDR { } 
#[doc = "Possible values of the field `ENDTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTXR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ENDTXR { } 
#[doc = "Possible values of the field `STARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl STARTEDR { } 
#[doc = "Values that can be written to the field `STOPPED`"]
pub enum STOPPEDW {
    #[doc = "Enable"]
    SET,
}
impl STOPPEDW { } 
#[doc = r" Proxy"]
pub struct _STOPPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPPEDW<'a> { } 
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
#[doc = "Values that can be written to the field `ENDTX`"]
pub enum ENDTXW {
    #[doc = "Enable"]
    SET,
}
impl ENDTXW { } 
#[doc = r" Proxy"]
pub struct _ENDTXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDTXW<'a> { } 
#[doc = "Values that can be written to the field `STARTED`"]
pub enum STARTEDW {
    #[doc = "Enable"]
    SET,
}
impl STARTEDW { } 
#[doc = r" Proxy"]
pub struct _STARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTEDW<'a> { } 
impl R { } 
impl W { } 
