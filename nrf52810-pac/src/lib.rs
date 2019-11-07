#![allow(non_camel_case_types, dead_code)]
#![no_std]

extern crate cortex_m_rt;
extern "C" { fn POWER_CLOCK(); }
pub struct Vector { _handler: unsafe extern "C" fn(), }

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 1] = [ Vector { _handler: POWER_CLOCK } ];

pub struct Interrupt;
unsafe impl ::bare_metal::Nr for Interrupt { fn nr(&self) -> u8 { loop { } } }

pub struct FICR { }   pub mod ficr;
pub struct BPROT { }  pub mod bprot;
pub struct TWI0 { }   pub mod twi0;
pub struct TWIM0 { }  pub mod twim0;
pub struct TWIS0 { }  pub mod twis0;
pub struct SPI0 { }   pub mod spi0;
pub struct SPIM0 { }  pub mod spim0;
pub struct SPIS0 { }  pub mod spis0;
pub struct TIMER0 { } pub mod timer0;
pub struct TEMP { } pub mod temp;
pub struct RNG { }  pub mod rng;
pub struct EGU0 { } pub mod egu0;
pub struct PWM0 { } pub mod pwm0;
pub struct NVMC { } pub mod nvmc;

pub mod ppi {
    #[repr(C)]
    pub struct RegisterBlock {  }
    pub struct CHG;
    pub mod chg {
        pub struct R { bits: u32, }
        pub struct W { bits: u32, }
        impl super::CHG { }

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct CH0R;
        impl CH0R { }
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct CH1R;
        impl CH1R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH2R;
impl CH2R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH3R;
impl CH3R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH4R;
impl CH4R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH5R;
impl CH5R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH6R;
impl CH6R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH7R;
impl CH7R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH8R;
impl CH8R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH9R;
impl CH9R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH10R;
impl CH10R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH11R;
impl CH11R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH12R;
impl CH12R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH13R;
impl CH13R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH14R;
    impl CH14R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH15R;
    impl CH15R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH16R;
    impl CH16R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH17R;
        impl CH17R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH18R;
        impl CH18R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH19R;
        impl CH19R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH20R;
        impl CH20R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH21R;
        impl CH21R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH22R;
        impl CH22R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH23R;
impl CH23R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH24R;
impl CH24R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH25R;
impl CH25R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH26R;
impl CH26R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH27R;
impl CH27R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH28R;
impl CH28R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH29R;
impl CH29R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH30R;
impl CH30R { }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CH31R;
impl CH31R { }
        impl R { }
        impl W { }

    }
}
