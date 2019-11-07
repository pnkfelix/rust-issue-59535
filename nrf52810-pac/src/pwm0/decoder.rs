#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DECODER { } 
#[doc = "Possible values of the field `LOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOADR {
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    COMMON,
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    GROUPED,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    INDIVIDUAL,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    WAVEFORM,
}
impl LOADR { } 
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    REFRESHCOUNT,
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    NEXTSTEP,
}
impl MODER { } 
#[doc = "Values that can be written to the field `LOAD`"]
pub enum LOADW {
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    COMMON,
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    GROUPED,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    INDIVIDUAL,
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    WAVEFORM,
}
impl LOADW { } 
#[doc = r" Proxy"]
pub struct _LOADW<'a> {
    w: &'a mut W,
}
impl<'a> _LOADW<'a> { } 
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    REFRESHCOUNT,
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    NEXTSTEP,
}
impl MODEW { } 
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> { } 
impl R { } 
impl W { } 
