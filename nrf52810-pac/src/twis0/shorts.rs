#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS { } 
#[doc = "Possible values of the field `WRITE_SUSPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_SUSPENDR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl WRITE_SUSPENDR { } 
#[doc = "Possible values of the field `READ_SUSPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_SUSPENDR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl READ_SUSPENDR { } 
#[doc = "Values that can be written to the field `WRITE_SUSPEND`"]
pub enum WRITE_SUSPENDW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl WRITE_SUSPENDW { } 
#[doc = r" Proxy"]
pub struct _WRITE_SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_SUSPENDW<'a> { } 
#[doc = "Values that can be written to the field `READ_SUSPEND`"]
pub enum READ_SUSPENDW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl READ_SUSPENDW { } 
#[doc = r" Proxy"]
pub struct _READ_SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_SUSPENDW<'a> { } 
impl R { } 
impl W { } 
