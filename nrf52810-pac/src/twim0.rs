
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

pub struct TASKS_STARTRX;

pub mod tasks_startrx;

pub struct TASKS_STARTTX;

pub mod tasks_starttx;

pub struct TASKS_STOP;

pub mod tasks_stop;

pub struct TASKS_SUSPEND;

pub mod tasks_suspend;

pub struct TASKS_RESUME;

pub mod tasks_resume;

pub struct EVENTS_STOPPED;

pub mod events_stopped;

pub struct EVENTS_ERROR;

pub mod events_error;

pub struct EVENTS_SUSPENDED;

pub mod events_suspended;

pub struct EVENTS_RXSTARTED;

pub mod events_rxstarted;

pub struct EVENTS_TXSTARTED;

pub mod events_txstarted;

pub struct EVENTS_LASTRX;

pub mod events_lastrx;

pub struct EVENTS_LASTTX;

pub mod events_lasttx;

pub struct SHORTS;

pub mod shorts;

pub struct INTEN;

pub mod inten;

pub struct INTENSET;

pub mod intenset;

pub struct INTENCLR;

pub mod intenclr;

pub struct ERRORSRC;

pub mod errorsrc;

pub struct ENABLE;

pub mod enable;

pub struct FREQUENCY;

pub mod frequency;

pub struct ADDRESS;

pub mod address;
