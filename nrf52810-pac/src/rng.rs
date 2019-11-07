#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock;
#[doc = "Task starting the random number generator"]
pub struct TASKS_START;
#[doc = "Task starting the random number generator"]
pub mod tasks_start;
#[doc = "Task stopping the random number generator"]
pub struct TASKS_STOP;
#[doc = "Task stopping the random number generator"]
pub mod tasks_stop;
#[doc = "Event being generated for every new random number written to the VALUE register"]
pub struct EVENTS_VALRDY;
#[doc = "Event being generated for every new random number written to the VALUE register"]
pub mod events_valrdy;
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
#[doc = "Configuration register"]
pub struct CONFIG;
#[doc = "Configuration register"]
pub mod config;
#[doc = "Output random number"]
pub struct VALUE;
#[doc = "Output random number"]
pub mod value;
