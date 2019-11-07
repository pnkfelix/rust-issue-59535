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
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl ERRORR { } 
#[doc = "Possible values of the field `SUSPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPENDEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SUSPENDEDR { } 
#[doc = "Possible values of the field `RXSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RXSTARTEDR { } 
#[doc = "Possible values of the field `TXSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl TXSTARTEDR { } 
#[doc = "Possible values of the field `LASTRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRXR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl LASTRXR { } 
#[doc = "Possible values of the field `LASTTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTXR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl LASTTXR { } 
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
#[doc = "Values that can be written to the field `ERROR`"]
pub enum ERRORW {
    #[doc = "Enable"]
    SET,
}
impl ERRORW { } 
#[doc = r" Proxy"]
pub struct _ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRORW<'a> { } 
#[doc = "Values that can be written to the field `SUSPENDED`"]
pub enum SUSPENDEDW {
    #[doc = "Enable"]
    SET,
}
impl SUSPENDEDW { } 
#[doc = r" Proxy"]
pub struct _SUSPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPENDEDW<'a> { } 
#[doc = "Values that can be written to the field `RXSTARTED`"]
pub enum RXSTARTEDW {
    #[doc = "Enable"]
    SET,
}
impl RXSTARTEDW { } 
#[doc = r" Proxy"]
pub struct _RXSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSTARTEDW<'a> { } 
#[doc = "Values that can be written to the field `TXSTARTED`"]
pub enum TXSTARTEDW {
    #[doc = "Enable"]
    SET,
}
impl TXSTARTEDW { } 
#[doc = r" Proxy"]
pub struct _TXSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSTARTEDW<'a> { } 
#[doc = "Values that can be written to the field `LASTRX`"]
pub enum LASTRXW {
    #[doc = "Enable"]
    SET,
}
impl LASTRXW { } 
#[doc = r" Proxy"]
pub struct _LASTRXW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTRXW<'a> { } 
#[doc = "Values that can be written to the field `LASTTX`"]
pub enum LASTTXW {
    #[doc = "Enable"]
    SET,
}
impl LASTTXW { } 
#[doc = r" Proxy"]
pub struct _LASTTXW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTTXW<'a> { } 
impl R { } 
impl W { } 
