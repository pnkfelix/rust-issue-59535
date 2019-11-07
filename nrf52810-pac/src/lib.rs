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

pub struct FICR;pub mod ficr {
    #[repr(C)] pub struct INFO;
    pub mod info {
        pub struct PART;
        pub mod part {
            pub struct R;
            impl super::PART { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct PARTR;
            impl PARTR { }
            impl R { }
        }
        pub struct VARIANT;
        pub struct PACKAGE;
        pub mod package {
            pub struct R;
            impl super::PACKAGE { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct PACKAGER;
            impl PACKAGER { }
            impl R { }
        }
        pub struct FLASH;
        pub mod flash {
            pub struct R;
            impl super::FLASH { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct FLASHR;
            impl FLASHR { }
            impl R { }
        }
    }
    pub struct DEVICEADDRTYPE;
    pub mod deviceaddrtype {
        pub struct R;
        impl super::DEVICEADDRTYPE { }
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct DEVICEADDRTYPER;
        impl DEVICEADDRTYPER { }
        impl R { }
    }
}
pub struct BPROT;pub mod bprot {
    pub struct CONFIG0;
    pub mod config0;
    pub struct DISABLEINDEBUG;
    pub mod disableindebug;
}
pub struct TWI0;pub mod twi0 {
    #[repr(C)]
    pub struct PSEL;
    pub mod psel;
    pub struct EVENTS_STOPPED;
    pub mod events_stopped;
    pub struct EVENTS_TXDSENT;
    pub mod events_txdsent;
    pub struct EVENTS_BB;
    pub mod events_bb;
    pub struct SHORTS;
    pub mod shorts;
    pub struct INTENCLR;
    pub mod intenclr;
    pub struct ENABLE;
    pub mod enable;
    pub struct FREQUENCY;
    pub mod frequency;
}
pub struct TWIM0;pub mod twim0 {
    #[repr(C)]
    pub struct PSEL;
    pub mod psel;
    #[repr(C)] pub struct TXD;
    pub mod txd;
    pub struct TASKS_SUSPEND;
    pub mod tasks_suspend;
    pub struct EVENTS_ERROR;
    pub mod events_error;
    pub struct EVENTS_RXSTARTED;
    pub mod events_rxstarted;
    pub struct EVENTS_LASTRX;
    pub mod events_lastrx;
    pub struct SHORTS;
    pub mod shorts;
    pub struct INTENSET;
    pub mod intenset;
    pub struct ERRORSRC;
    pub mod errorsrc;
    pub struct FREQUENCY;
    pub mod frequency;
}
pub struct TWIS0;pub mod twis0 {
    #[repr(C)]
    pub struct PSEL;
    pub mod psel;
    #[repr(C)]
    pub struct TXD;
    pub mod txd;
    pub struct EVENTS_STOPPED;
    pub mod events_stopped;
    pub struct EVENTS_RXSTARTED;
    pub mod events_rxstarted;
    pub struct EVENTS_WRITE;
    pub mod events_write;
    pub struct SHORTS;
    pub mod shorts;
    pub struct INTENSET;
    pub mod intenset;
    pub struct ERRORSRC;
    pub mod errorsrc;
    pub struct ENABLE;
    pub mod enable;
    pub struct CONFIG;
    pub mod config;
}
pub struct SPI0;pub mod spi0 {
    #[repr(C)]
    pub struct PSEL;
    pub mod psel;
    pub struct INTENSET;
    pub mod intenset;
    pub struct ENABLE;
    pub mod enable;
    pub struct CONFIG;
    pub mod config;
}
pub struct SPIM0;pub mod spim0 {
    #[repr(C)]
    pub struct PSEL;
    pub mod psel;
    #[repr(C)]
    pub struct TXD;
    pub mod txd;
    pub struct TASKS_RESUME;
    pub mod tasks_resume;
    pub struct EVENTS_STOPPED;
    pub mod events_stopped;
    pub struct EVENTS_END;
    pub mod events_end;
    pub struct EVENTS_STARTED;
    pub mod events_started;
    pub struct INTENSET;
    pub mod intenset;
    pub struct INTENCLR;
    pub mod intenclr;
    pub struct FREQUENCY;
    pub mod frequency;
}
pub struct SPIS0;pub mod spis0 {
    #[repr(C)]
    pub struct PSEL;
    pub mod psel;
    #[repr(C)]
    pub struct TXD;
    pub mod txd;
    pub struct EVENTS_ENDRX;
    pub mod events_endrx;
    pub struct SHORTS;
    pub mod shorts;
    pub struct INTENCLR;
    pub mod intenclr;
    pub struct STATUS;
    pub mod status;
    pub struct CONFIG;
    pub mod config;
}
pub struct TIMER0;pub mod timer0 {
    pub struct SHORTS;
    pub mod shorts;
    pub struct INTENCLR;
    pub mod intenclr;
    pub struct BITMODE;
    pub mod bitmode;
}
pub struct TEMP;pub mod temp {
    pub struct EVENTS_DATARDY;
    pub mod events_datardy;
    pub struct INTENSET;
    pub mod intenset;
}
pub struct RNG; pub mod rng {
    pub struct EVENTS_VALRDY;
    pub mod events_valrdy;
    pub struct INTENSET;
    pub mod intenset;
    pub struct CONFIG;
    pub mod config;
}
pub struct EGU0;pub mod egu0 {
    pub struct INTEN;
    pub mod inten;
    pub struct INTENCLR;
    pub mod intenclr;
}
pub struct PWM0;pub mod pwm0 {
    #[repr(C)]
    pub struct PSEL;
    pub mod psel;
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
    pub struct EVENTS_LOOPSDONE;
    pub mod events_loopsdone;
    pub struct INTEN;
    pub mod inten;
    pub struct INTENCLR;
    pub mod intenclr;
    pub struct MODE;
    pub mod mode;
    pub struct PRESCALER;
    pub mod prescaler;
    pub struct LOOP;
    pub mod loop_;
}
pub struct NVMC;pub mod nvmc {
    pub struct READY;
    pub mod ready;
    pub struct ERASEUICR;
    pub mod eraseuicr;
}
pub mod ppi {
    #[repr(C)]
    pub struct RegisterBlock;
    pub struct CHG;
    pub mod chg {
        pub struct R;
        pub struct W;
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
