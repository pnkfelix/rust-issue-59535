#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS { } 
#[doc = "Possible values of the field `COMPARE0_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_CLEARR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE0_CLEARR { } 
#[doc = "Possible values of the field `COMPARE1_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_CLEARR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE1_CLEARR { } 
#[doc = "Possible values of the field `COMPARE2_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_CLEARR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE2_CLEARR { } 
#[doc = "Possible values of the field `COMPARE3_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_CLEARR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE3_CLEARR { } 
#[doc = "Possible values of the field `COMPARE4_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE4_CLEARR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE4_CLEARR { } 
#[doc = "Possible values of the field `COMPARE5_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE5_CLEARR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE5_CLEARR { } 
#[doc = "Possible values of the field `COMPARE0_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE0_STOPR { } 
#[doc = "Possible values of the field `COMPARE1_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE1_STOPR { } 
#[doc = "Possible values of the field `COMPARE2_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE2_STOPR { } 
#[doc = "Possible values of the field `COMPARE3_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE3_STOPR { } 
#[doc = "Possible values of the field `COMPARE4_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE4_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE4_STOPR { } 
#[doc = "Possible values of the field `COMPARE5_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE5_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE5_STOPR { } 
#[doc = "Values that can be written to the field `COMPARE0_CLEAR`"]
pub enum COMPARE0_CLEARW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE0_CLEARW { } 
#[doc = r" Proxy"]
pub struct _COMPARE0_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE0_CLEARW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE1_CLEAR`"]
pub enum COMPARE1_CLEARW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE1_CLEARW { } 
#[doc = r" Proxy"]
pub struct _COMPARE1_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE1_CLEARW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE2_CLEAR`"]
pub enum COMPARE2_CLEARW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE2_CLEARW { } 
#[doc = r" Proxy"]
pub struct _COMPARE2_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE2_CLEARW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE3_CLEAR`"]
pub enum COMPARE3_CLEARW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE3_CLEARW { } 
#[doc = r" Proxy"]
pub struct _COMPARE3_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE3_CLEARW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE4_CLEAR`"]
pub enum COMPARE4_CLEARW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE4_CLEARW { } 
#[doc = r" Proxy"]
pub struct _COMPARE4_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE4_CLEARW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE5_CLEAR`"]
pub enum COMPARE5_CLEARW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE5_CLEARW { } 
#[doc = r" Proxy"]
pub struct _COMPARE5_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE5_CLEARW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE0_STOP`"]
pub enum COMPARE0_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE0_STOPW { } 
#[doc = r" Proxy"]
pub struct _COMPARE0_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE0_STOPW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE1_STOP`"]
pub enum COMPARE1_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE1_STOPW { } 
#[doc = r" Proxy"]
pub struct _COMPARE1_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE1_STOPW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE2_STOP`"]
pub enum COMPARE2_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE2_STOPW { } 
#[doc = r" Proxy"]
pub struct _COMPARE2_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE2_STOPW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE3_STOP`"]
pub enum COMPARE3_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE3_STOPW { } 
#[doc = r" Proxy"]
pub struct _COMPARE3_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE3_STOPW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE4_STOP`"]
pub enum COMPARE4_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE4_STOPW { } 
#[doc = r" Proxy"]
pub struct _COMPARE4_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE4_STOPW<'a> { } 
#[doc = "Values that can be written to the field `COMPARE5_STOP`"]
pub enum COMPARE5_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl COMPARE5_STOPW { } 
#[doc = r" Proxy"]
pub struct _COMPARE5_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPARE5_STOPW<'a> { } 
impl R { } 
impl W { } 
