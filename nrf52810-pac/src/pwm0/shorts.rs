#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS { } 
#[doc = "Possible values of the field `SEQEND0_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND0_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl SEQEND0_STOPR { } 
#[doc = "Possible values of the field `SEQEND1_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND1_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl SEQEND1_STOPR { } 
#[doc = "Possible values of the field `LOOPSDONE_SEQSTART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_SEQSTART0R {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_SEQSTART0R { } 
#[doc = "Possible values of the field `LOOPSDONE_SEQSTART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_SEQSTART1R {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_SEQSTART1R { } 
#[doc = "Possible values of the field `LOOPSDONE_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_STOPR { } 
#[doc = "Values that can be written to the field `SEQEND0_STOP`"]
pub enum SEQEND0_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl SEQEND0_STOPW { } 
#[doc = r" Proxy"]
pub struct _SEQEND0_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQEND0_STOPW<'a> { } 
#[doc = "Values that can be written to the field `SEQEND1_STOP`"]
pub enum SEQEND1_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl SEQEND1_STOPW { } 
#[doc = r" Proxy"]
pub struct _SEQEND1_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQEND1_STOPW<'a> { } 
#[doc = "Values that can be written to the field `LOOPSDONE_SEQSTART0`"]
pub enum LOOPSDONE_SEQSTART0W {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_SEQSTART0W { } 
#[doc = r" Proxy"]
pub struct _LOOPSDONE_SEQSTART0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSDONE_SEQSTART0W<'a> { } 
#[doc = "Values that can be written to the field `LOOPSDONE_SEQSTART1`"]
pub enum LOOPSDONE_SEQSTART1W {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_SEQSTART1W { } 
#[doc = r" Proxy"]
pub struct _LOOPSDONE_SEQSTART1W<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSDONE_SEQSTART1W<'a> { } 
#[doc = "Values that can be written to the field `LOOPSDONE_STOP`"]
pub enum LOOPSDONE_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_STOPW { } 
#[doc = r" Proxy"]
pub struct _LOOPSDONE_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSDONE_STOPW<'a> { } 
impl R { } 
impl W { } 
