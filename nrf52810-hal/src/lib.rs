#![allow(non_camel_case_types, dead_code)]
#![no_std]

pub mod nrf52810_pac {
    extern crate cortex_m_rt;
    pub struct Vector { _handler: unsafe extern "C" fn(), }
    extern "C" fn power_clock_2() { }

    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 1] = [ Vector { _handler: power_clock_2 } ];

    mod ficr {
        mod info {
            mod part {
                struct R;
                #[derive(Debug, PartialEq)]struct PARTR;
                impl R { }
                impl R { }
                impl R { }
            }
            mod package {
                struct R;
                #[derive(Debug, PartialEq)]struct PACKAGER;
                impl R { }
                impl R { }
                impl R { }
            }
            mod flash {
                struct R;
                #[derive(Debug, PartialEq)]struct FLASHR;
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod deviceaddrtype {
            struct R;
            #[derive(Debug, PartialEq)]struct DEVICEADDRTYPER;
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod bprot {
        mod config0 {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct REGION0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION1R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION2R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION3R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION4R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION5R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION6R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION7R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION8R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION9R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION10R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION11R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION12R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION13R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION14R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION15R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION16R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION17R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION18R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION19R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION20R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION21R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION22R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION23R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION24R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION25R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION26R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION27R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION28R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION29R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION30R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct REGION31R;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod disableindebug {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct DISABLEINDEBUGR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod twi0 {
        mod psel {
            mod scl {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod sda {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod events_stopped {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct EVENTS_STOPPEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_txdsent {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct EVENTS_TXDSENTR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_bb {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct EVENTS_BBR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod shorts {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct BB_SUSPENDR;
            impl R { }
            #[derive(Debug, PartialEq)]struct BB_STOPR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct STOPPEDR;
            impl R { }
            #[derive(Debug, PartialEq)]struct RXDREADYR;
            impl R { }
            #[derive(Debug, PartialEq)]struct TXDSENTR;
            impl R { }
            #[derive(Debug, PartialEq)]struct ERRORR;
            impl R { }
            #[derive(Debug, PartialEq)]struct BBR;
            impl R { }
            #[derive(Debug, PartialEq)]struct SUSPENDEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod enable {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct ENABLER;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod frequency {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct FREQUENCYR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod twim0 {
        mod psel {
            mod scl {
                struct R;
                impl R { }
                impl R { }
                #[derive(Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod sda {
                struct R;
                impl R { }
                impl R { }
                #[derive(Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod txd {
            mod ptr {
                struct R;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                impl R { }
                impl R { }
            }
            mod list {
                struct R;
                impl R { }
                #[derive(Debug, PartialEq)]struct LISTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod tasks_suspend {
            struct R;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_error {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct EVENTS_ERRORR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_rxstarted {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct EVENTS_RXSTARTEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_lastrx {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct EVENTS_LASTRXR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod shorts {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct LASTTX_STARTRXR;
            impl R { }
            #[derive(Debug, PartialEq)]struct LASTTX_SUSPENDR;
            impl R { }
            #[derive(Debug, PartialEq)]struct LASTTX_STOPR;
            impl R { }
            #[derive(Debug, PartialEq)]struct LASTRX_STARTTXR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct LASTRX_SUSPENDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct LASTRX_STOPR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct STOPPEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ERRORR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct SUSPENDEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct RXSTARTEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TXSTARTEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct LASTRXR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct LASTTXR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod errorsrc {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct OVERRUNR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ANACKR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct DNACKR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod frequency {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct FREQUENCYR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod twis0 {
        mod psel {
            mod scl {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod sda {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod txd {
            mod ptr{
                struct R;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                impl R { }
                impl R { }
            }
            mod list {
                struct R;
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct LISTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod events_stopped {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_STOPPEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_rxstarted {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_RXSTARTEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_write {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_WRITER;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod shorts {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct WRITE_SUSPENDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct READ_SUSPENDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct STOPPEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ERRORR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct RXSTARTEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TXSTARTEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct WRITER;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct READR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod errorsrc {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct OVERFLOWR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct DNACKR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct OVERREADR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod enable {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ENABLER;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod config {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ADDRESS0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ADDRESS1R;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod spi0 {
        mod psel {
            mod sck {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }

            }
            mod mosi {
                struct R;
                impl R { }
                impl R { }
                #[derive(Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod miso {
                struct R;
                impl R { }
                impl R { }
                #[derive(Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct READYR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod enable {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]struct ENABLER;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod config {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ORDERR;
            impl R { }
            #[derive(Debug, PartialEq)]struct CPHAR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CPOLR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod spim0 {
        mod psel {
            mod sck {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod mosi {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod miso {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod txd {
            mod ptr {
                struct R;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                impl R { }
                impl R { }
            }
            mod list {
                struct R;
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct LISTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod tasks_resume {
            struct R;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_stopped {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_STOPPEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_end {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_ENDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_started {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_STARTEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct STOPPEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ENDRXR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ENDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ENDTXR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct STARTEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct STOPPEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ENDRXR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ENDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ENDTXR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct STARTEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod frequency {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct FREQUENCYR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod spis0 {
        mod psel {
            mod sck {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod miso {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod mosi {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod csn {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod txd {
            mod ptr {
                struct R;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                impl R { }
                impl R { }
            }
            mod list {
                struct R;
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct LISTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod events_endrx {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_ENDRXR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod shorts {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct END_ACQUIRER;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ENDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ENDRXR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ACQUIREDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod status {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct OVERREADR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct OVERFLOWR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod config {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ORDERR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CPHAR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CPOLR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    pub struct TIMER0;
    mod timer0 {
        mod shorts {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE0_CLEARR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE1_CLEARR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE2_CLEARR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE3_CLEARR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE4_CLEARR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE5_CLEARR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE0_STOPR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE1_STOPR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE2_STOPR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE3_STOPR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE4_STOPR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE5_STOPR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE1R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE2R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE3R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE4R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct COMPARE5R;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod bitmode {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct BITMODER;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod temp {
        mod events_datardy {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_DATARDYR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct DATARDYR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod rng {
        mod events_valrdy {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_VALRDYR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct VALRDYR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod config {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct DERCENR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod egu0 {
        mod inten {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED1R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED2R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED3R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED4R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED5R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED6R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED7R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED8R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED9R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED10R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED11R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED12R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED13R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED14R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED15R;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED1R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED2R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED3R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED4R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED5R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED6R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED7R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED8R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED9R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED10R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED11R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED12R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED13R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED14R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct TRIGGERED15R;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod pwm0 {
        mod psel {
            mod out {
                struct R;
                impl R { }
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]struct CONNECTR;
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod tasks_seqstart {
            struct R;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod tasks_nextstep {
            struct R;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_stopped {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_STOPPEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_seqstarted {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_SEQSTARTEDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_seqend {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_SEQENDR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_loopsdone {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct EVENTS_LOOPSDONER;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod inten {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct STOPPEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct SEQSTARTED0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct SEQSTARTED1R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct SEQEND0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct SEQEND1R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct PWMPERIODENDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct LOOPSDONER;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct STOPPEDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct SEQSTARTED0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct SEQSTARTED1R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct SEQEND0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct SEQEND1R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct PWMPERIODENDR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct LOOPSDONER;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod mode {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct UPDOWNR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod prescaler {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct PRESCALERR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod loop_ {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CNTR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod nvmc {
        mod ready {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct READYR;
            impl R { }
            impl R { }
        }
        mod eraseuicr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct ERASEUICRR;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod ppi {
        mod chg {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH1R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH2R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH3R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH4R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH5R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH6R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH7R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH8R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH9R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH10R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH11R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH12R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH13R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH14R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH15R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH16R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH17R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH18R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH19R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH20R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH21R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH22R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH23R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH24R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH25R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH26R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH27R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH28R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH29R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH30R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]struct CH31R;
            impl R { }
            impl R { }
            impl R { }
        }
    }
}
