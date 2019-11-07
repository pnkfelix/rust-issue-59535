#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TASKS_RELEASE { }

#[doc = "Values that can be written to the field `TASKS_RELEASE`"]
pub enum TASKS_RELEASEW {
    #[doc = "Trigger task"]
    TRIGGER,
}
impl TASKS_RELEASEW { }

#[doc = r" Proxy"]
pub struct _TASKS_RELEASEW<'a> {
    w: &'a mut W,
}
impl<'a> _TASKS_RELEASEW<'a> { }

impl W { }

