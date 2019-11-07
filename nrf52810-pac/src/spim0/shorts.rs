#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS { } 
#[doc = "Possible values of the field `END_START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_STARTR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl END_STARTR { } 
#[doc = "Values that can be written to the field `END_START`"]
pub enum END_STARTW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl END_STARTW { } 
#[doc = r" Proxy"]
pub struct _END_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _END_STARTW<'a> { } 
impl R { } 
impl W { } 
