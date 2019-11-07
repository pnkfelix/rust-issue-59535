#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESCALER { } 
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERR {
    #[doc = "Divide by 1 (16 MHz)"]
    DIV_1,
    #[doc = "Divide by 2 (8 MHz)"]
    DIV_2,
    #[doc = "Divide by 4 (4 MHz)"]
    DIV_4,
    #[doc = "Divide by 8 (2 MHz)"]
    DIV_8,
    #[doc = "Divide by 16 (1 MHz)"]
    DIV_16,
    #[doc = "Divide by 32 (500 kHz)"]
    DIV_32,
    #[doc = "Divide by 64 (250 kHz)"]
    DIV_64,
    #[doc = "Divide by 128 (125 kHz)"]
    DIV_128,
}
impl PRESCALERR { } 
#[doc = "Values that can be written to the field `PRESCALER`"]
pub enum PRESCALERW {
    #[doc = "Divide by 1 (16 MHz)"]
    DIV_1,
    #[doc = "Divide by 2 (8 MHz)"]
    DIV_2,
    #[doc = "Divide by 4 (4 MHz)"]
    DIV_4,
    #[doc = "Divide by 8 (2 MHz)"]
    DIV_8,
    #[doc = "Divide by 16 (1 MHz)"]
    DIV_16,
    #[doc = "Divide by 32 (500 kHz)"]
    DIV_32,
    #[doc = "Divide by 64 (250 kHz)"]
    DIV_64,
    #[doc = "Divide by 128 (125 kHz)"]
    DIV_128,
}
impl PRESCALERW { } 
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> { } 
impl R { } 
impl W { } 
