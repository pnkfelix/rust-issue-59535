pub struct R {
    bits: u32,
}
pub struct W {
    bits: u32,
}
impl super::CHG {
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
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0R {
    EXCLUDED,
    INCLUDED,
}
impl CH0R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH0R::EXCLUDED => false,
            CH0R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[inline]
    pub fn _from(value: bool) -> CH0R {
        match value {
            false => CH0R::EXCLUDED,
            true => CH0R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH0R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH0R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1R {
    EXCLUDED,
    INCLUDED,
}
impl CH1R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH1R::EXCLUDED => false,
            CH1R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH1R {
        match value {
            false => CH1R::EXCLUDED,
            true => CH1R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH1R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH1R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2R {
    EXCLUDED,
    INCLUDED,
}
impl CH2R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH2R::EXCLUDED => false,
            CH2R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH2R {
        match value {
            false => CH2R::EXCLUDED,
            true => CH2R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH2R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH2R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3R {
    EXCLUDED,
    INCLUDED,
}
impl CH3R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH3R::EXCLUDED => false,
            CH3R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH3R {
        match value {
            false => CH3R::EXCLUDED,
            true => CH3R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH3R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH3R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4R {
    EXCLUDED,
    INCLUDED,
}
impl CH4R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH4R::EXCLUDED => false,
            CH4R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH4R {
        match value {
            false => CH4R::EXCLUDED,
            true => CH4R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH4R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH4R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5R {
    EXCLUDED,
    INCLUDED,
}
impl CH5R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH5R::EXCLUDED => false,
            CH5R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH5R {
        match value {
            false => CH5R::EXCLUDED,
            true => CH5R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH5R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH5R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6R {
    EXCLUDED,
    INCLUDED,
}
impl CH6R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH6R::EXCLUDED => false,
            CH6R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH6R {
        match value {
            false => CH6R::EXCLUDED,
            true => CH6R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH6R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH6R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7R {
    EXCLUDED,
    INCLUDED,
}
impl CH7R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH7R::EXCLUDED => false,
            CH7R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH7R {
        match value {
            false => CH7R::EXCLUDED,
            true => CH7R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH7R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH7R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8R {
    EXCLUDED,
    INCLUDED,
}
impl CH8R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH8R::EXCLUDED => false,
            CH8R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH8R {
        match value {
            false => CH8R::EXCLUDED,
            true => CH8R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH8R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH8R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9R {
    EXCLUDED,
    INCLUDED,
}
impl CH9R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH9R::EXCLUDED => false,
            CH9R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH9R {
        match value {
            false => CH9R::EXCLUDED,
            true => CH9R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH9R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH9R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10R {
    EXCLUDED,
    INCLUDED,
}
impl CH10R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH10R::EXCLUDED => false,
            CH10R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH10R {
        match value {
            false => CH10R::EXCLUDED,
            true => CH10R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH10R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH10R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11R {
    EXCLUDED,
    INCLUDED,
}
impl CH11R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH11R::EXCLUDED => false,
            CH11R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH11R {
        match value {
            false => CH11R::EXCLUDED,
            true => CH11R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH11R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH11R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12R {
    EXCLUDED,
    INCLUDED,
}
impl CH12R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH12R::EXCLUDED => false,
            CH12R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH12R {
        match value {
            false => CH12R::EXCLUDED,
            true => CH12R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH12R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH12R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13R {
    EXCLUDED,
    INCLUDED,
}
impl CH13R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH13R::EXCLUDED => false,
            CH13R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH13R {
        match value {
            false => CH13R::EXCLUDED,
            true => CH13R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH13R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH13R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14R {
    EXCLUDED,
    INCLUDED,
}
impl CH14R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH14R::EXCLUDED => false,
            CH14R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH14R {
        match value {
            false => CH14R::EXCLUDED,
            true => CH14R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH14R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH14R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15R {
    EXCLUDED,
    INCLUDED,
}
impl CH15R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH15R::EXCLUDED => false,
            CH15R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH15R {
        match value {
            false => CH15R::EXCLUDED,
            true => CH15R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH15R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH15R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH16R {
    EXCLUDED,
    INCLUDED,
}
impl CH16R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH16R::EXCLUDED => false,
            CH16R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH16R {
        match value {
            false => CH16R::EXCLUDED,
            true => CH16R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH16R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH16R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH17R {
    EXCLUDED,
    INCLUDED,
}
impl CH17R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH17R::EXCLUDED => false,
            CH17R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH17R {
        match value {
            false => CH17R::EXCLUDED,
            true => CH17R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH17R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH17R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH18R {
    EXCLUDED,
    INCLUDED,
}
impl CH18R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH18R::EXCLUDED => false,
            CH18R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH18R {
        match value {
            false => CH18R::EXCLUDED,
            true => CH18R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH18R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH18R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH19R {
    EXCLUDED,
    INCLUDED,
}
impl CH19R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH19R::EXCLUDED => false,
            CH19R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH19R {
        match value {
            false => CH19R::EXCLUDED,
            true => CH19R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH19R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH19R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH20R {
    EXCLUDED,
    INCLUDED,
}
impl CH20R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH20R::EXCLUDED => false,
            CH20R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH20R {
        match value {
            false => CH20R::EXCLUDED,
            true => CH20R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH20R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH20R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH21R {
    EXCLUDED,
    INCLUDED,
}
impl CH21R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH21R::EXCLUDED => false,
            CH21R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH21R {
        match value {
            false => CH21R::EXCLUDED,
            true => CH21R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH21R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH21R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH22R {
    EXCLUDED,
    INCLUDED,
}
impl CH22R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH22R::EXCLUDED => false,
            CH22R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH22R {
        match value {
            false => CH22R::EXCLUDED,
            true => CH22R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH22R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH22R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH23R {
    EXCLUDED,
    INCLUDED,
}
impl CH23R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH23R::EXCLUDED => false,
            CH23R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH23R {
        match value {
            false => CH23R::EXCLUDED,
            true => CH23R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH23R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH23R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH24R {
    EXCLUDED,
    INCLUDED,
}
impl CH24R {
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH24R::EXCLUDED => false,
            CH24R::INCLUDED => true,
        }
    }
    #[inline]
    pub fn _from(value: bool) -> CH24R {
        match value {
            false => CH24R::EXCLUDED,
            true => CH24R::INCLUDED,
        }
    }
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH24R::EXCLUDED
    }
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH24R::INCLUDED
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH25R {
    EXCLUDED,
    
    INCLUDED,
}
impl CH25R {
    
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH25R::EXCLUDED => false,
            CH25R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _from(value: bool) -> CH25R {
        match value {
            false => CH25R::EXCLUDED,
            true => CH25R::INCLUDED,
        }
    }
    
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH25R::EXCLUDED
    }
    
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH25R::INCLUDED
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH26R {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH26R {
    
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH26R::EXCLUDED => false,
            CH26R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _from(value: bool) -> CH26R {
        match value {
            false => CH26R::EXCLUDED,
            true => CH26R::INCLUDED,
        }
    }
    
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH26R::EXCLUDED
    }
    
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH26R::INCLUDED
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH27R {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH27R {
    
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH27R::EXCLUDED => false,
            CH27R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _from(value: bool) -> CH27R {
        match value {
            false => CH27R::EXCLUDED,
            true => CH27R::INCLUDED,
        }
    }
    
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH27R::EXCLUDED
    }
    
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH27R::INCLUDED
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH28R {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH28R {
    
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH28R::EXCLUDED => false,
            CH28R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _from(value: bool) -> CH28R {
        match value {
            false => CH28R::EXCLUDED,
            true => CH28R::INCLUDED,
        }
    }
    
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH28R::EXCLUDED
    }
    
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH28R::INCLUDED
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH29R {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH29R {
    
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH29R::EXCLUDED => false,
            CH29R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _from(value: bool) -> CH29R {
        match value {
            false => CH29R::EXCLUDED,
            true => CH29R::INCLUDED,
        }
    }
    
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH29R::EXCLUDED
    }
    
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH29R::INCLUDED
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH30R {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH30R {
    
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH30R::EXCLUDED => false,
            CH30R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _from(value: bool) -> CH30R {
        match value {
            false => CH30R::EXCLUDED,
            true => CH30R::INCLUDED,
        }
    }
    
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH30R::EXCLUDED
    }
    
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH30R::INCLUDED
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH31R {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH31R {
    
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CH31R::EXCLUDED => false,
            CH31R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _from(value: bool) -> CH31R {
        match value {
            false => CH31R::EXCLUDED,
            true => CH31R::INCLUDED,
        }
    }
    
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH31R::EXCLUDED
    }
    
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH31R::INCLUDED
    }
}

pub enum CH0W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH0W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0W::EXCLUDED => false,
            CH0W::INCLUDED => true,
        }
    }
}

pub struct _CH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CH0W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH0W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH0W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH1W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH1W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1W::EXCLUDED => false,
            CH1W::INCLUDED => true,
        }
    }
}

pub struct _CH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CH1W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH1W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH1W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH2W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH2W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2W::EXCLUDED => false,
            CH2W::INCLUDED => true,
        }
    }
}

pub struct _CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _CH2W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH2W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH2W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH3W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH3W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3W::EXCLUDED => false,
            CH3W::INCLUDED => true,
        }
    }
}

pub struct _CH3W<'a> {
    w: &'a mut W,
}
impl<'a> _CH3W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH3W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH3W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH4W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH4W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4W::EXCLUDED => false,
            CH4W::INCLUDED => true,
        }
    }
}

pub struct _CH4W<'a> {
    w: &'a mut W,
}
impl<'a> _CH4W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH4W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH4W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH5W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH5W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5W::EXCLUDED => false,
            CH5W::INCLUDED => true,
        }
    }
}

pub struct _CH5W<'a> {
    w: &'a mut W,
}
impl<'a> _CH5W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH5W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH5W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH6W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH6W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6W::EXCLUDED => false,
            CH6W::INCLUDED => true,
        }
    }
}

pub struct _CH6W<'a> {
    w: &'a mut W,
}
impl<'a> _CH6W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH6W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH6W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH7W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH7W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7W::EXCLUDED => false,
            CH7W::INCLUDED => true,
        }
    }
}

pub struct _CH7W<'a> {
    w: &'a mut W,
}
impl<'a> _CH7W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH7W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH7W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH8W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH8W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8W::EXCLUDED => false,
            CH8W::INCLUDED => true,
        }
    }
}

pub struct _CH8W<'a> {
    w: &'a mut W,
}
impl<'a> _CH8W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH8W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH8W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH9W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH9W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH9W::EXCLUDED => false,
            CH9W::INCLUDED => true,
        }
    }
}

pub struct _CH9W<'a> {
    w: &'a mut W,
}
impl<'a> _CH9W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH9W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH9W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH10W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH10W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH10W::EXCLUDED => false,
            CH10W::INCLUDED => true,
        }
    }
}

pub struct _CH10W<'a> {
    w: &'a mut W,
}
impl<'a> _CH10W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH10W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH10W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH11W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH11W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH11W::EXCLUDED => false,
            CH11W::INCLUDED => true,
        }
    }
}

pub struct _CH11W<'a> {
    w: &'a mut W,
}
impl<'a> _CH11W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH11W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH11W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH12W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH12W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH12W::EXCLUDED => false,
            CH12W::INCLUDED => true,
        }
    }
}

pub struct _CH12W<'a> {
    w: &'a mut W,
}
impl<'a> _CH12W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH12W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH12W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH13W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH13W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH13W::EXCLUDED => false,
            CH13W::INCLUDED => true,
        }
    }
}

pub struct _CH13W<'a> {
    w: &'a mut W,
}
impl<'a> _CH13W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH13W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH13W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH14W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH14W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH14W::EXCLUDED => false,
            CH14W::INCLUDED => true,
        }
    }
}

pub struct _CH14W<'a> {
    w: &'a mut W,
}
impl<'a> _CH14W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH14W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH14W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH15W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH15W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH15W::EXCLUDED => false,
            CH15W::INCLUDED => true,
        }
    }
}

pub struct _CH15W<'a> {
    w: &'a mut W,
}
impl<'a> _CH15W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH15W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH15W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH16W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH16W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH16W::EXCLUDED => false,
            CH16W::INCLUDED => true,
        }
    }
}

pub struct _CH16W<'a> {
    w: &'a mut W,
}
impl<'a> _CH16W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH16W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH16W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH17W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH17W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH17W::EXCLUDED => false,
            CH17W::INCLUDED => true,
        }
    }
}

pub struct _CH17W<'a> {
    w: &'a mut W,
}
impl<'a> _CH17W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH17W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH17W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH18W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH18W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH18W::EXCLUDED => false,
            CH18W::INCLUDED => true,
        }
    }
}

pub struct _CH18W<'a> {
    w: &'a mut W,
}
impl<'a> _CH18W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH18W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH18W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH19W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH19W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH19W::EXCLUDED => false,
            CH19W::INCLUDED => true,
        }
    }
}

pub struct _CH19W<'a> {
    w: &'a mut W,
}
impl<'a> _CH19W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH19W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH19W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH20W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH20W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH20W::EXCLUDED => false,
            CH20W::INCLUDED => true,
        }
    }
}

pub struct _CH20W<'a> {
    w: &'a mut W,
}
impl<'a> _CH20W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH20W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH20W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH21W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH21W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH21W::EXCLUDED => false,
            CH21W::INCLUDED => true,
        }
    }
}

pub struct _CH21W<'a> {
    w: &'a mut W,
}
impl<'a> _CH21W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH21W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH21W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH22W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH22W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH22W::EXCLUDED => false,
            CH22W::INCLUDED => true,
        }
    }
}

pub struct _CH22W<'a> {
    w: &'a mut W,
}
impl<'a> _CH22W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH22W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH22W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH23W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH23W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH23W::EXCLUDED => false,
            CH23W::INCLUDED => true,
        }
    }
}

pub struct _CH23W<'a> {
    w: &'a mut W,
}
impl<'a> _CH23W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH23W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH23W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH24W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH24W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH24W::EXCLUDED => false,
            CH24W::INCLUDED => true,
        }
    }
}

pub struct _CH24W<'a> {
    w: &'a mut W,
}
impl<'a> _CH24W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH24W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH24W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH25W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH25W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH25W::EXCLUDED => false,
            CH25W::INCLUDED => true,
        }
    }
}

pub struct _CH25W<'a> {
    w: &'a mut W,
}
impl<'a> _CH25W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH25W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH25W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH26W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH26W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH26W::EXCLUDED => false,
            CH26W::INCLUDED => true,
        }
    }
}

pub struct _CH26W<'a> {
    w: &'a mut W,
}
impl<'a> _CH26W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH26W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH26W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH27W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH27W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH27W::EXCLUDED => false,
            CH27W::INCLUDED => true,
        }
    }
}

pub struct _CH27W<'a> {
    w: &'a mut W,
}
impl<'a> _CH27W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH27W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH27W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH28W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH28W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH28W::EXCLUDED => false,
            CH28W::INCLUDED => true,
        }
    }
}

pub struct _CH28W<'a> {
    w: &'a mut W,
}
impl<'a> _CH28W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH28W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH28W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH29W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH29W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH29W::EXCLUDED => false,
            CH29W::INCLUDED => true,
        }
    }
}

pub struct _CH29W<'a> {
    w: &'a mut W,
}
impl<'a> _CH29W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH29W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH29W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH30W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH30W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH30W::EXCLUDED => false,
            CH30W::INCLUDED => true,
        }
    }
}

pub struct _CH30W<'a> {
    w: &'a mut W,
}
impl<'a> _CH30W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH30W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH30W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

pub enum CH31W {
    
    EXCLUDED,
    
    INCLUDED,
}
impl CH31W {
    #[allow(missing_docs)]
    
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH31W::EXCLUDED => false,
            CH31W::INCLUDED => true,
        }
    }
}

pub struct _CH31W<'a> {
    w: &'a mut W,
}
impl<'a> _CH31W<'a> {
    
    #[inline]
    pub fn variant(self, variant: CH31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH31W::EXCLUDED)
    }
    
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH31W::INCLUDED)
    }
    
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}

impl R {
    
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    
    #[inline]
    pub fn ch0(&self) -> CH0R {
        CH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch1(&self) -> CH1R {
        CH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch2(&self) -> CH2R {
        CH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch3(&self) -> CH3R {
        CH3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch4(&self) -> CH4R {
        CH4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch5(&self) -> CH5R {
        CH5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch6(&self) -> CH6R {
        CH6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch7(&self) -> CH7R {
        CH7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch8(&self) -> CH8R {
        CH8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch9(&self) -> CH9R {
        CH9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch10(&self) -> CH10R {
        CH10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch11(&self) -> CH11R {
        CH11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch12(&self) -> CH12R {
        CH12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch13(&self) -> CH13R {
        CH13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch14(&self) -> CH14R {
        CH14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch15(&self) -> CH15R {
        CH15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch16(&self) -> CH16R {
        CH16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch17(&self) -> CH17R {
        CH17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch18(&self) -> CH18R {
        CH18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch19(&self) -> CH19R {
        CH19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch20(&self) -> CH20R {
        CH20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch21(&self) -> CH21R {
        CH21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch22(&self) -> CH22R {
        CH22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch23(&self) -> CH23R {
        CH23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch24(&self) -> CH24R {
        CH24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch25(&self) -> CH25R {
        CH25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch26(&self) -> CH26R {
        CH26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch27(&self) -> CH27R {
        CH27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch28(&self) -> CH28R {
        CH28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch29(&self) -> CH29R {
        CH29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch30(&self) -> CH30R {
        CH30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    
    #[inline]
    pub fn ch31(&self) -> CH31R {
        CH31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
}
