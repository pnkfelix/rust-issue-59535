#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock;
#[doc = "Start Timer"]
pub struct TASKS_START;
#[doc = "Start Timer"]
pub mod tasks_start;
#[doc = "Stop Timer"]
pub struct TASKS_STOP;
#[doc = "Stop Timer"]
pub mod tasks_stop;
#[doc = "Increment Timer (Counter mode only)"]
pub struct TASKS_COUNT;
#[doc = "Increment Timer (Counter mode only)"]
pub mod tasks_count;
#[doc = "Clear time"]
pub struct TASKS_CLEAR;
#[doc = "Clear time"]
pub mod tasks_clear;
#[doc = "Deprecated register - Shut down timer"]
pub struct TASKS_SHUTDOWN;
#[doc = "Deprecated register - Shut down timer"]
pub mod tasks_shutdown;
#[doc = "Description collection: Capture Timer value to CC\\[n\\] register"]
pub struct TASKS_CAPTURE;
#[doc = "Description collection: Capture Timer value to CC\\[n\\] register"]
pub mod tasks_capture;
#[doc = "Description collection: Compare event on CC\\[n\\] match"]
pub struct EVENTS_COMPARE;
#[doc = "Description collection: Compare event on CC\\[n\\] match"]
pub mod events_compare;
#[doc = "Shortcuts between local events and tasks"]
pub struct SHORTS;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable interrupt"]
pub struct INTENSET;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Timer mode selection"]
pub struct MODE;
#[doc = "Timer mode selection"]
pub mod mode;
#[doc = "Configure the number of bits used by the TIMER"]
pub struct BITMODE;
#[doc = "Configure the number of bits used by the TIMER"]
pub mod bitmode;
#[doc = "Timer prescaler register"]
pub struct PRESCALER;
#[doc = "Timer prescaler register"]
pub mod prescaler;
#[doc = "Description collection: Capture/Compare register n"]
pub struct CC;
#[doc = "Description collection: Capture/Compare register n"]
pub mod cc;
