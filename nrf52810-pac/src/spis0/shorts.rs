#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS { }

#[doc = "Possible values of the field `END_ACQUIRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_ACQUIRER {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl END_ACQUIRER { }

#[doc = "Values that can be written to the field `END_ACQUIRE`"]
pub enum END_ACQUIREW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl END_ACQUIREW { }

#[doc = r" Proxy"]
pub struct _END_ACQUIREW<'a> {
    w: &'a mut W,
}
impl<'a> _END_ACQUIREW<'a> { }

impl R { }

impl W { }

