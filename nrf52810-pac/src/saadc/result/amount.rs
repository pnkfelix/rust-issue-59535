#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AMOUNT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct AMOUNTR {
    bits: u16,
}
impl AMOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:14 - Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
    #[inline]
    pub fn amount(&self) -> AMOUNTR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        AMOUNTR { bits }
    }
}
