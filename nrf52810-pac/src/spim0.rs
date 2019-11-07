
#[repr(C)]
pub struct RegisterBlock;

#[repr(C)]
pub struct PSEL;


pub mod psel;

#[repr(C)]
pub struct RXD;


pub mod rxd;

#[repr(C)]
pub struct TXD;


pub mod txd;

pub struct TASKS_START;

pub mod tasks_start;

pub struct TASKS_STOP;

pub mod tasks_stop;

pub struct TASKS_SUSPEND;

pub mod tasks_suspend;

pub struct TASKS_RESUME;

pub mod tasks_resume;

pub struct EVENTS_STOPPED;

pub mod events_stopped;

pub struct EVENTS_ENDRX;

pub mod events_endrx;

pub struct EVENTS_END;

pub mod events_end;

pub struct EVENTS_ENDTX;

pub mod events_endtx;

pub struct EVENTS_STARTED;

pub mod events_started;

pub struct SHORTS;

pub mod shorts;

pub struct INTENSET;

pub mod intenset;

pub struct INTENCLR;

pub mod intenclr;

pub struct ENABLE;

pub mod enable;

pub struct FREQUENCY;

pub mod frequency;

pub struct CONFIG;

pub mod config;

pub struct ORC;

pub mod orc;
