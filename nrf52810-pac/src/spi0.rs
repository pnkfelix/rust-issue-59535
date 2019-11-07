
#[repr(C)]
pub struct RegisterBlock;

#[repr(C)]
pub struct PSEL;


pub mod psel;

pub struct EVENTS_READY;

pub mod events_ready;

pub struct INTENSET;

pub mod intenset;

pub struct INTENCLR;

pub mod intenclr;

pub struct ENABLE;

pub mod enable;

pub struct RXD;

pub mod rxd;

pub struct TXD;

pub mod txd;

pub struct FREQUENCY;

pub mod frequency;

pub struct CONFIG;

pub mod config;
