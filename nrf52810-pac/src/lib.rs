// #![deny(missing_docs)]
// #![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;

pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn POWER_CLOCK();
    fn RADIO();
    fn UARTE0_UART0();
    fn TWIM0_TWIS0_TWI0();
    fn SPIM0_SPIS0_SPI0();
    fn GPIOTE();
    fn SAADC();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn TEMP();
    fn RNG();
    fn ECB();
    fn CCM_AAR();
    fn WDT();
    fn RTC1();
    fn QDEC();
    fn COMP();
    fn SWI0_EGU0();
    fn SWI1_EGU1();
    fn SWI2();
    fn SWI3();
    fn SWI4();
    fn SWI5();
    fn PWM0();
    fn PDM();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 30] = [
    Vector {
        _handler: POWER_CLOCK,
    },
    Vector { _handler: RADIO },
    Vector {
        _handler: UARTE0_UART0,
    },
    Vector {
        _handler: TWIM0_TWIS0_TWI0,
    },
    Vector {
        _handler: SPIM0_SPIS0_SPI0,
    },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE },
    Vector { _handler: SAADC },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: RTC0 },
    Vector { _handler: TEMP },
    Vector { _handler: RNG },
    Vector { _handler: ECB },
    Vector { _handler: CCM_AAR },
    Vector { _handler: WDT },
    Vector { _handler: RTC1 },
    Vector { _handler: QDEC },
    Vector { _handler: COMP },
    Vector {
        _handler: SWI0_EGU0,
    },
    Vector {
        _handler: SWI1_EGU1,
    },
    Vector { _handler: SWI2 },
    Vector { _handler: SWI3 },
    Vector { _handler: SWI4 },
    Vector { _handler: SWI5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PWM0 },
    Vector { _handler: PDM },
];
pub enum Interrupt {
    POWER_CLOCK,
    RADIO,
    UARTE0_UART0,
    TWIM0_TWIS0_TWI0,
    SPIM0_SPIS0_SPI0,
    GPIOTE,
    SAADC,
    TIMER0,
    TIMER1,
    TIMER2,
    RTC0,
    TEMP,
    RNG,
    ECB,
    CCM_AAR,
    WDT,
    RTC1,
    QDEC,
    COMP,
    SWI0_EGU0,
    SWI1_EGU1,
    SWI2,
    SWI3,
    SWI4,
    SWI5,
    PWM0,
    PDM,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    fn nr(&self) -> u8 { loop { } }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub struct FICR { }   pub mod ficr;
pub struct BPROT { }  pub mod bprot;

pub struct POWER { }  pub mod power;
pub struct P0 { }     pub mod p0;
pub struct RADIO { }  pub mod radio;
pub struct UART0 { }  pub mod uart0;
pub struct UARTE0 { } pub mod uarte0;
pub struct TWI0 { }   pub mod twi0;
pub struct TWIM0 { }  pub mod twim0;
pub struct TWIS0 { }  pub mod twis0;
pub struct SPI0 { }   pub mod spi0;
pub struct SPIM0 { }  pub mod spim0;
pub struct SPIS0 { }  pub mod spis0;
pub struct GPIOTE { } pub mod gpiote;
pub struct SAADC { }  pub mod saadc;
pub struct TIMER0 { } pub mod timer0;
pub struct TIMER1 { }
pub struct TIMER2 { }
pub struct RTC0 { } pub mod rtc0;
pub struct TEMP { } pub mod temp;
pub struct RNG { }  pub mod rng;
pub struct ECB { }  pub mod ecb;
pub struct AAR { }  pub mod aar;
pub struct CCM { }  pub mod ccm;
pub struct WDT { }  pub mod wdt;
pub struct RTC1 { }
pub struct QDEC { } pub mod qdec;
pub struct COMP { } pub mod comp;
pub struct EGU0 { } pub mod egu0;
pub struct SWI0 { } pub mod swi0;
pub struct EGU1 { }
pub struct SWI1 { }
pub struct SWI2 { }
pub struct SWI3 { }
pub struct SWI4 { }
pub struct SWI5 { }
pub struct PWM0 { } pub mod pwm0;
pub struct PDM { }  pub mod pdm;
pub struct NVMC { } pub mod nvmc;

pub struct PPI { }

pub mod ppi {
    #[repr(C)]
    pub struct RegisterBlock { pub tasks_chg: [TASKS_CHG; 6], }
    #[repr(C)]
    pub struct TASKS_CHG { }
    pub struct CHG { register: ::vcell::VolatileCell<u32>, }
    pub mod chg;
}
