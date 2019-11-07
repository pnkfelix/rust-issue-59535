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
#[doc = "Possible values of the field `SEQSTARTED0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED0R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SEQSTARTED0R { } 
#[doc = "Possible values of the field `SEQSTARTED1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED1R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SEQSTARTED1R { } 
#[doc = "Possible values of the field `SEQEND0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND0R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SEQEND0R { } 
#[doc = "Possible values of the field `SEQEND1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND1R {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SEQEND1R { } 
#[doc = "Possible values of the field `PWMPERIODEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMPERIODENDR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl PWMPERIODENDR { } 
#[doc = "Possible values of the field `LOOPSDONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONER {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl LOOPSDONER { } 
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
#[doc = "Values that can be written to the field `SEQSTARTED0`"]
pub enum SEQSTARTED0W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SEQSTARTED0W { } 
#[doc = r" Proxy"]
pub struct _SEQSTARTED0W<'a> {
    w: &'a mut W,
}
impl<'a> _SEQSTARTED0W<'a> { } 
#[doc = "Values that can be written to the field `SEQSTARTED1`"]
pub enum SEQSTARTED1W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SEQSTARTED1W { } 
#[doc = r" Proxy"]
pub struct _SEQSTARTED1W<'a> {
    w: &'a mut W,
}
impl<'a> _SEQSTARTED1W<'a> { } 
#[doc = "Values that can be written to the field `SEQEND0`"]
pub enum SEQEND0W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SEQEND0W { } 
#[doc = r" Proxy"]
pub struct _SEQEND0W<'a> {
    w: &'a mut W,
}
impl<'a> _SEQEND0W<'a> { } 
#[doc = "Values that can be written to the field `SEQEND1`"]
pub enum SEQEND1W {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl SEQEND1W { } 
#[doc = r" Proxy"]
pub struct _SEQEND1W<'a> {
    w: &'a mut W,
}
impl<'a> _SEQEND1W<'a> { } 
#[doc = "Values that can be written to the field `PWMPERIODEND`"]
pub enum PWMPERIODENDW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl PWMPERIODENDW { } 
#[doc = r" Proxy"]
pub struct _PWMPERIODENDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMPERIODENDW<'a> { } 
#[doc = "Values that can be written to the field `LOOPSDONE`"]
pub enum LOOPSDONEW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl LOOPSDONEW { } 
#[doc = r" Proxy"]
pub struct _LOOPSDONEW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSDONEW<'a> { } 
impl R { } 
impl W { } 
