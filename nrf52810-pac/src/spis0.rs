
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

pub struct TASKS_ACQUIRE;

pub mod tasks_acquire;

pub struct TASKS_RELEASE;

pub mod tasks_release;

pub struct EVENTS_END;

pub mod events_end;

pub struct EVENTS_ENDRX;

pub mod events_endrx;

pub struct EVENTS_ACQUIRED;

pub mod events_acquired;

pub struct SHORTS;

pub mod shorts;

pub struct INTENSET;

pub mod intenset;

pub struct INTENCLR;

pub mod intenclr;

pub struct SEMSTAT;

pub mod semstat;

pub struct STATUS;

pub mod status;

pub struct ENABLE;

pub mod enable;

pub struct CONFIG;

pub mod config;

pub struct DEF;

pub mod def;

pub struct ORC;

pub mod orc;
