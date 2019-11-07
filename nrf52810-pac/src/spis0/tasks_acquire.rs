#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TASKS_ACQUIRE { }

#[doc = "Values that can be written to the field `TASKS_ACQUIRE`"]
pub enum TASKS_ACQUIREW {
    #[doc = "Trigger task"]
    TRIGGER,
}
impl TASKS_ACQUIREW { }

#[doc = r" Proxy"]
pub struct _TASKS_ACQUIREW<'a> {
    w: &'a mut W,
}
impl<'a> _TASKS_ACQUIREW<'a> { }

impl W { }

