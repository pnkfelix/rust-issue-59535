#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS { }

#[doc = "Possible values of the field `LASTTX_STARTRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTX_STARTRXR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTTX_STARTRXR { }

#[doc = "Possible values of the field `LASTTX_SUSPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTX_SUSPENDR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTTX_SUSPENDR { }

#[doc = "Possible values of the field `LASTTX_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTTX_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTTX_STOPR { }

#[doc = "Possible values of the field `LASTRX_STARTTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRX_STARTTXR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTRX_STARTTXR { }

#[doc = "Possible values of the field `LASTRX_SUSPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRX_SUSPENDR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTRX_SUSPENDR { }

#[doc = "Possible values of the field `LASTRX_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTRX_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTRX_STOPR { }

#[doc = "Values that can be written to the field `LASTTX_STARTRX`"]
pub enum LASTTX_STARTRXW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTTX_STARTRXW { }

#[doc = r" Proxy"]
pub struct _LASTTX_STARTRXW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTTX_STARTRXW<'a> { }

#[doc = "Values that can be written to the field `LASTTX_SUSPEND`"]
pub enum LASTTX_SUSPENDW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTTX_SUSPENDW { }

#[doc = r" Proxy"]
pub struct _LASTTX_SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTTX_SUSPENDW<'a> { }

#[doc = "Values that can be written to the field `LASTTX_STOP`"]
pub enum LASTTX_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTTX_STOPW { }

#[doc = r" Proxy"]
pub struct _LASTTX_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTTX_STOPW<'a> { }

#[doc = "Values that can be written to the field `LASTRX_STARTTX`"]
pub enum LASTRX_STARTTXW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTRX_STARTTXW { }

#[doc = r" Proxy"]
pub struct _LASTRX_STARTTXW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTRX_STARTTXW<'a> { }

#[doc = "Values that can be written to the field `LASTRX_SUSPEND`"]
pub enum LASTRX_SUSPENDW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTRX_SUSPENDW { }

#[doc = r" Proxy"]
pub struct _LASTRX_SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTRX_SUSPENDW<'a> { }

#[doc = "Values that can be written to the field `LASTRX_STOP`"]
pub enum LASTRX_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LASTRX_STOPW { }

#[doc = r" Proxy"]
pub struct _LASTRX_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LASTRX_STOPW<'a> { }

impl R { }

impl W { }

