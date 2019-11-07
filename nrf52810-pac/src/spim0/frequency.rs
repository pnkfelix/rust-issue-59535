#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FREQUENCY { } 
#[doc = "Possible values of the field `FREQUENCY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQUENCYR {
    #[doc = "125 kbps"]
    K125,
    #[doc = "250 kbps"]
    K250,
    #[doc = "500 kbps"]
    K500,
    #[doc = "1 Mbps"]
    M1,
    #[doc = "2 Mbps"]
    M2,
    #[doc = "4 Mbps"]
    M4,
    #[doc = "8 Mbps"]
    M8,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl FREQUENCYR { } 
#[doc = "Values that can be written to the field `FREQUENCY`"]
pub enum FREQUENCYW {
    #[doc = "125 kbps"]
    K125,
    #[doc = "250 kbps"]
    K250,
    #[doc = "500 kbps"]
    K500,
    #[doc = "1 Mbps"]
    M1,
    #[doc = "2 Mbps"]
    M2,
    #[doc = "4 Mbps"]
    M4,
    #[doc = "8 Mbps"]
    M8,
}
impl FREQUENCYW { } 
#[doc = r" Proxy"]
pub struct _FREQUENCYW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQUENCYW<'a> { } 
impl R { } 
impl W { } 
