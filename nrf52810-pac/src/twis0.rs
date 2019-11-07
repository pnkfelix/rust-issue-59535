
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

pub struct TASKS_STOP;

pub mod tasks_stop;

pub struct TASKS_SUSPEND;

pub mod tasks_suspend;

pub struct TASKS_RESUME;

pub mod tasks_resume;

pub struct TASKS_PREPARERX;

pub mod tasks_preparerx;

pub struct TASKS_PREPARETX;

pub mod tasks_preparetx;

pub struct EVENTS_STOPPED;

pub mod events_stopped;

pub struct EVENTS_ERROR;

pub mod events_error;

pub struct EVENTS_RXSTARTED;

pub mod events_rxstarted;

pub struct EVENTS_TXSTARTED;

pub mod events_txstarted;

pub struct EVENTS_WRITE;

pub mod events_write;

pub struct EVENTS_READ;

pub mod events_read;

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

pub struct MATCH;

pub mod match_;

pub struct ENABLE;

pub mod enable;

pub struct ADDRESS;

pub mod address;

pub struct CONFIG;

pub mod config;

pub struct ORC;

pub mod orc;
