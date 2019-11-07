#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TASKS_ACQUIRE { }
#[cfg(not_now)] impl super::TASKS_ACQUIRE {
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
}
#[doc = "Values that can be written to the field `TASKS_ACQUIRE`"]
pub enum TASKS_ACQUIREW {
    #[doc = "Trigger task"]
    TRIGGER,
}
impl TASKS_ACQUIREW { }
#[cfg(not_now)] impl TASKS_ACQUIREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TASKS_ACQUIREW::TRIGGER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TASKS_ACQUIREW<'a> {
    w: &'a mut W,
}
impl<'a> _TASKS_ACQUIREW<'a> { }
#[cfg(not_now)] impl<'a> _TASKS_ACQUIREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TASKS_ACQUIREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger task"]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_ACQUIREW::TRIGGER)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
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
    #[doc = "Bit 0 - Acquire SPI semaphore"]
    #[inline]
    pub fn tasks_acquire(&mut self) -> _TASKS_ACQUIREW {
        _TASKS_ACQUIREW { w: self }
    }
}
