
#[repr(C)]
pub struct RegisterBlock;

pub struct READY;

pub mod ready;

pub struct CONFIG;

pub mod config;

pub struct ERASEPAGE;

pub mod erasepage;

pub struct ERASEPCR1;

pub mod erasepcr1;

pub struct ERASEALL;

pub mod eraseall;

pub struct ERASEPCR0;

pub mod erasepcr0;

pub struct ERASEUICR;

pub mod eraseuicr;

pub struct ERASEPAGEPARTIAL;

pub mod erasepagepartial;

pub struct ERASEPAGEPARTIALCFG;

pub mod erasepagepartialcfg;
