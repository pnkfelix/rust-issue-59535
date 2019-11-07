#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG { } 
#[doc = "Possible values of the field `ADDRESS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS0R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADDRESS0R { } 
#[doc = "Possible values of the field `ADDRESS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS1R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADDRESS1R { } 
#[doc = "Values that can be written to the field `ADDRESS0`"]
pub enum ADDRESS0W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADDRESS0W { } 
#[doc = r" Proxy"]
pub struct _ADDRESS0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESS0W<'a> { } 
#[doc = "Values that can be written to the field `ADDRESS1`"]
pub enum ADDRESS1W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADDRESS1W { } 
#[doc = r" Proxy"]
pub struct _ADDRESS1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESS1W<'a> { } 
impl R { } 
impl W { } 
