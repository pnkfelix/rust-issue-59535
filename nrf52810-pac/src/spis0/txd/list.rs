#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LIST { }
#[cfg(not_now)] impl super::LIST {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `LIST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LISTR {
    #[doc = "Disable EasyDMA list"]
    DISABLED,
    #[doc = "Use array list"]
    ARRAYLIST,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LISTR { }
#[cfg(not_now)] impl LISTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LISTR::DISABLED => 0,
            LISTR::ARRAYLIST => 1,
            LISTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LISTR {
        match value {
            0 => LISTR::DISABLED,
            1 => LISTR::ARRAYLIST,
            i => LISTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LISTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ARRAYLIST`"]
    #[inline]
    pub fn is_array_list(&self) -> bool {
        *self == LISTR::ARRAYLIST
    }
}
#[doc = "Values that can be written to the field `LIST`"]
pub enum LISTW {
    #[doc = "Disable EasyDMA list"]
    DISABLED,
    #[doc = "Use array list"]
    ARRAYLIST,
}
impl LISTW { }
#[cfg(not_now)] impl LISTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LISTW::DISABLED => 0,
            LISTW::ARRAYLIST => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LISTW<'a> {
    w: &'a mut W,
}
impl<'a> _LISTW<'a> { }
#[cfg(not_now)] impl<'a> _LISTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LISTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable EasyDMA list"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LISTW::DISABLED)
    }
    #[doc = "Use array list"]
    #[inline]
    pub fn array_list(self) -> &'a mut W {
        self.variant(LISTW::ARRAYLIST)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R { }
#[cfg(not_now)] impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - List type"]
    #[inline]
    pub fn list(&self) -> LISTR {
        LISTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W { }
#[cfg(not_now)] impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - List type"]
    #[inline]
    pub fn list(&mut self) -> _LISTW {
        _LISTW { w: self }
    }
}
