#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN { }

#[doc = "Possible values of the field `STOPPED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPEDR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl STOPPEDR { }

#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl ERRORR { }

#[doc = "Possible values of the field `SUSPENDED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPENDEDR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SUSPENDEDR { }

#[doc = "Possible values of the field `RXSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTEDR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RXSTARTEDR { }

#[doc = "Possible values of the field `TXSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTEDR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl TXSTARTEDR { }

#[doc = "Possible values of the field `LASTRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRXR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl LASTRXR { }

#[doc = "Possible values of the field `LASTTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTXR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl LASTTXR { }

#[doc = "Values that can be written to the field `STOPPED`"]
pub enum STOPPEDW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl STOPPEDW { }

#[doc = r" Proxy"]
pub struct _STOPPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPPEDW<'a> { }

#[doc = "Values that can be written to the field `ERROR`"]
pub enum ERRORW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl ERRORW { }

#[doc = r" Proxy"]
pub struct _ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRORW<'a> { }

#[doc = "Values that can be written to the field `SUSPENDED`"]
pub enum SUSPENDEDW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SUSPENDEDW { }

#[doc = r" Proxy"]
pub struct _SUSPENDEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPENDEDW<'a> { }

#[doc = "Values that can be written to the field `RXSTARTED`"]
pub enum RXSTARTEDW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RXSTARTEDW { }

#[doc = r" Proxy"]
pub struct _RXSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSTARTEDW<'a> { }

#[doc = "Values that can be written to the field `TXSTARTED`"]
pub enum TXSTARTEDW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl TXSTARTEDW { }

#[doc = r" Proxy"]
pub struct _TXSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSTARTEDW<'a> { }

#[doc = "Values that can be written to the field `LASTRX`"]
pub enum LASTRXW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl LASTRXW { }

#[doc = r" Proxy"]
pub struct _LASTRXW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTRXW<'a> { }

#[doc = "Values that can be written to the field `LASTTX`"]
pub enum LASTTXW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl LASTTXW { }

#[doc = r" Proxy"]
pub struct _LASTTXW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTTXW<'a> { }

impl R { }

impl W { }

