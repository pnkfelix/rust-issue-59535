#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1536usize],
    pub config0: CONFIG0,
    pub config1: CONFIG1,
    pub disableindebug: DISABLEINDEBUG,
}
pub struct CONFIG0 {
    register: ::vcell::VolatileCell<u32>,
}
pub mod config0;
pub struct CONFIG1 {
    register: ::vcell::VolatileCell<u32>,
}
pub mod config1;
pub struct DISABLEINDEBUG {
    register: ::vcell::VolatileCell<u32>,
}
pub mod disableindebug;
