
#[repr(C)]
pub struct RegisterBlock;

pub struct TASKS_START;

pub mod tasks_start;

pub struct TASKS_STOP;

pub mod tasks_stop;

pub struct TASKS_COUNT;

pub mod tasks_count;

pub struct TASKS_CLEAR;

pub mod tasks_clear;

pub struct TASKS_SHUTDOWN;

pub mod tasks_shutdown;

pub struct TASKS_CAPTURE;

pub mod tasks_capture;

pub struct EVENTS_COMPARE;

pub mod events_compare;

pub struct SHORTS;

pub mod shorts;

pub struct INTENSET;

pub mod intenset;

pub struct INTENCLR;

pub mod intenclr;

pub struct MODE;

pub mod mode;

pub struct BITMODE;

pub mod bitmode;

pub struct PRESCALER;

pub mod prescaler;

pub struct CC;

pub mod cc;
