
#[repr(C)]
pub struct RegisterBlock;

pub struct TASKS_START;

pub mod tasks_start;

pub struct TASKS_STOP;

pub mod tasks_stop;

pub struct EVENTS_VALRDY;

pub mod events_valrdy;

pub struct SHORTS;

pub mod shorts;

pub struct INTENSET;

pub mod intenset;

pub struct INTENCLR;

pub mod intenclr;

pub struct CONFIG;

pub mod config;

pub struct VALUE;

pub mod value;
