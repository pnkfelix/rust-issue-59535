#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENCLR { } 
#[doc = "Possible values of the field `STOPPED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl STOPPEDR { } 
#[doc = "Possible values of the field `RXDREADY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDREADYR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RXDREADYR { } 
#[doc = "Possible values of the field `TXDSENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDSENTR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TXDSENTR { } 
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ERRORR { } 
#[doc = "Possible values of the field `BB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl BBR { } 
#[doc = "Possible values of the field `SUSPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPENDEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SUSPENDEDR { } 
#[doc = "Values that can be written to the field `STOPPED`"]
pub enum STOPPEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl STOPPEDW { } 
#[doc = r" Proxy"]
pub struct _STOPPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPPEDW<'a> { } 
#[doc = "Values that can be written to the field `RXDREADY`"]
pub enum RXDREADYW {
    #[doc = "Disable"]
    CLEAR,
}
impl RXDREADYW { } 
#[doc = r" Proxy"]
pub struct _RXDREADYW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDREADYW<'a> { } 
#[doc = "Values that can be written to the field `TXDSENT`"]
pub enum TXDSENTW {
    #[doc = "Disable"]
    CLEAR,
}
impl TXDSENTW { } 
#[doc = r" Proxy"]
pub struct _TXDSENTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDSENTW<'a> { } 
#[doc = "Values that can be written to the field `ERROR`"]
pub enum ERRORW {
    #[doc = "Disable"]
    CLEAR,
}
impl ERRORW { } 
#[doc = r" Proxy"]
pub struct _ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRORW<'a> { } 
#[doc = "Values that can be written to the field `BB`"]
pub enum BBW {
    #[doc = "Disable"]
    CLEAR,
}
impl BBW { } 
#[doc = r" Proxy"]
pub struct _BBW<'a> {
    w: &'a mut W,
}
impl<'a> _BBW<'a> { } 
#[doc = "Values that can be written to the field `SUSPENDED`"]
pub enum SUSPENDEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl SUSPENDEDW { } 
#[doc = r" Proxy"]
pub struct _SUSPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPENDEDW<'a> { } 
impl R { } 
impl W { } 
