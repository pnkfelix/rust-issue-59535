
#[repr(C)]
pub struct RegisterBlock;

#[repr(C)]
pub struct SEQ;


pub mod seq;

#[repr(C)]
pub struct PSEL;


pub mod psel;

pub struct TASKS_STOP;

pub mod tasks_stop;

pub struct TASKS_SEQSTART;

pub mod tasks_seqstart;

pub struct TASKS_NEXTSTEP;

pub mod tasks_nextstep;

pub struct EVENTS_STOPPED;

pub mod events_stopped;

pub struct EVENTS_SEQSTARTED;

pub mod events_seqstarted;

pub struct EVENTS_SEQEND;

pub mod events_seqend;

pub struct EVENTS_PWMPERIODEND;

pub mod events_pwmperiodend;

pub struct EVENTS_LOOPSDONE;

pub mod events_loopsdone;

pub struct SHORTS;

pub mod shorts;

pub struct INTEN;

pub mod inten;

pub struct INTENSET;

pub mod intenset;

pub struct INTENCLR;

pub mod intenclr;

pub struct ENABLE;

pub mod enable;

pub struct MODE;

pub mod mode;

pub struct COUNTERTOP;

pub mod countertop;

pub struct PRESCALER;

pub mod prescaler;

pub struct DECODER;

pub mod decoder;

pub struct LOOP;

pub mod loop_;
