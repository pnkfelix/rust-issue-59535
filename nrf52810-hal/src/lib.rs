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
                impl R { }
                #[derive(Debug, PartialEq)]
                struct PARTR;
                impl PARTR { }
                impl R { }
            }
            mod package {
                struct R;
                impl R { }
                #[derive(Debug, PartialEq)]
                struct PACKAGER;
                impl PACKAGER { }
                impl R { }
            }
            mod flash {
                struct R;
                impl R { }
                #[derive(Debug, PartialEq)]
                struct FLASHR;
                impl FLASHR { }
                impl R { }
            }
        }
        mod deviceaddrtype {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct DEVICEADDRTYPER;
            impl DEVICEADDRTYPER { }
            impl R { }
        }
    }
    mod bprot {
        mod config0 {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct REGION0R;
            impl REGION0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION1R;
            impl REGION1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION2R;
            impl REGION2R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION3R;
            impl REGION3R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION4R;
            impl REGION4R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION5R;
            impl REGION5R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION6R;
            impl REGION6R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION7R;
            impl REGION7R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION8R;
            impl REGION8R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION9R;
            impl REGION9R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION10R;
            impl REGION10R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION11R;
            impl REGION11R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION12R;
            impl REGION12R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION13R;
            impl REGION13R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION14R;
            impl REGION14R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION15R;
            impl REGION15R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION16R;
            impl REGION16R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION17R;
            impl REGION17R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION18R;
            impl REGION18R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION19R;
            impl REGION19R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION20R;
            impl REGION20R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION21R;
            impl REGION21R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION22R;
            impl REGION22R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION23R;
            impl REGION23R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION24R;
            impl REGION24R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION25R;
            impl REGION25R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION26R;
            impl REGION26R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION27R;
            impl REGION27R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION28R;
            impl REGION28R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION29R;
            impl REGION29R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION30R;
            impl REGION30R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct REGION31R;
            impl REGION31R { }
            struct REGION0W;
            impl REGION0W { }
            impl R { }
            struct REGION1W;
            impl REGION1W { }
            impl R { }
            struct REGION2W;
            impl REGION2W { }
            impl R { }
            struct REGION3W;
            impl REGION3W { }
            impl R { }
            struct REGION4W;
            impl REGION4W { }
            impl R { }
            struct REGION5W;
            impl REGION5W { }
            impl R { }
            struct REGION6W;
            impl REGION6W { }
            impl R { }
            struct REGION7W;
            impl REGION7W { }
            impl R { }
            struct REGION8W;
            impl REGION8W { }
            impl R { }
            struct REGION9W;
            impl REGION9W { }
            impl R { }
            struct REGION10W;
            impl REGION10W { }
            impl R { }
            struct REGION11W;
            impl REGION11W { }
            impl R { }
            struct REGION12W;
            impl REGION12W { }
            impl R { }
            struct REGION13W;
            impl REGION13W { }
            impl R { }
            struct REGION14W;
            impl REGION14W { }
            impl R { }
            struct REGION15W;
            impl REGION15W { }
            impl R { }
            struct REGION16W;
            impl REGION16W { }
            impl R { }
            struct REGION17W;
            impl REGION17W { }
            impl R { }
            struct REGION18W;
            impl REGION18W { }
            impl R { }
            struct REGION19W;
            impl REGION19W { }
            impl R { }
            struct REGION20W;
            impl REGION20W { }
            impl R { }
            struct REGION21W;
            impl REGION21W { }
            impl R { }
            struct REGION22W;
            impl REGION22W { }
            impl R { }
            struct REGION23W;
            impl REGION23W { }
            impl R { }
            struct REGION24W;
            impl REGION24W { }
            impl R { }
            struct REGION25W;
            impl REGION25W { }
            impl R { }
            struct REGION26W;
            impl REGION26W { }
            impl R { }
            struct REGION27W;
            impl REGION27W { }
            impl R { }
            struct REGION28W;
            impl REGION28W { }
            impl R { }
            struct REGION29W;
            impl REGION29W { }
            impl R { }
            struct REGION30W;
            impl REGION30W { }
            impl R { }
            struct REGION31W;
            impl REGION31W { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod disableindebug {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct DISABLEINDEBUGR;
            impl DISABLEINDEBUGR { }
            struct DISABLEINDEBUGW;
            impl DISABLEINDEBUGW { }
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
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod sda {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod events_stopped {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct EVENTS_STOPPEDR;
            impl EVENTS_STOPPEDR { }
            struct EVENTS_STOPPEDW;
            impl EVENTS_STOPPEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_txdsent {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct EVENTS_TXDSENTR;
            impl EVENTS_TXDSENTR { }
            struct EVENTS_TXDSENTW;
            impl EVENTS_TXDSENTW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_bb {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct EVENTS_BBR;
            impl EVENTS_BBR { }
            struct EVENTS_BBW;
            impl EVENTS_BBW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod shorts {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct BB_SUSPENDR;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct BB_STOPR;
            impl R { }
            struct BB_SUSPENDW;
            impl R { }
            impl R { }
            struct BB_STOPW;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Debug, PartialEq)]
            struct RXDREADYR;
            impl RXDREADYR { }
            #[derive(Debug, PartialEq)]
            struct TXDSENTR;
            impl TXDSENTR { }
            #[derive(Debug, PartialEq)]
            struct ERRORR;
            impl ERRORR { }
            #[derive(Debug, PartialEq)]
            struct BBR;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct SUSPENDEDR;
            impl SUSPENDEDR { }
            struct STOPPEDW;
            impl STOPPEDW { }
            impl R { }
            struct RXDREADYW;
            impl RXDREADYW { }
            impl R { }
            struct TXDSENTW;
            impl TXDSENTW { }
            impl R { }
            struct ERRORW;
            impl ERRORW { }
            impl R { }
            struct BBW;
            impl R { }
            impl R { }
            struct SUSPENDEDW;
            impl SUSPENDEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod enable {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct ENABLER;
            impl ENABLER { }
            struct ENABLEW;
            impl ENABLEW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod frequency {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct FREQUENCYR;
            impl FREQUENCYR { }
            struct FREQUENCYW;
            impl FREQUENCYW { }
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
                struct PINR;
                impl PINR { }
                #[derive(Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod sda {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod txd {
            mod ptr {
                struct R;
                impl R { }
                struct PTRR;
                impl PTRR { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                impl R { }
                struct MAXCNTR;
                impl MAXCNTR { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                struct AMOUNTR;
                impl R { }
                impl R { }
            }
            mod list {
                struct R;
                impl R { }
                #[derive(Debug, PartialEq)]
                struct LISTR;
                impl LISTR { }
                struct LISTW;
                impl LISTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod tasks_suspend {
            struct R;
            impl R { }
            struct TASKS_SUSPENDW;
            impl TASKS_SUSPENDW { }
            impl R { }
            impl R { }
        }
        mod events_error {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct EVENTS_ERRORR;
            impl EVENTS_ERRORR { }
            struct EVENTS_ERRORW;
            impl EVENTS_ERRORW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_rxstarted {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct EVENTS_RXSTARTEDR;
            impl EVENTS_RXSTARTEDR { }
            struct EVENTS_RXSTARTEDW;
            impl EVENTS_RXSTARTEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_lastrx {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct EVENTS_LASTRXR;
            impl EVENTS_LASTRXR { }
            struct EVENTS_LASTRXW;
            impl EVENTS_LASTRXW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod shorts {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct LASTTX_STARTRXR;
            impl LASTTX_STARTRXR { }
            #[derive(Debug, PartialEq)]
            struct LASTTX_SUSPENDR;
            impl LASTTX_SUSPENDR { }
            #[derive(Debug, PartialEq)]
            struct LASTTX_STOPR;
            impl LASTTX_STOPR { }
            #[derive(Debug, PartialEq)]
            struct LASTRX_STARTTXR;
            impl LASTRX_STARTTXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct LASTRX_SUSPENDR;
            impl LASTRX_SUSPENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct LASTRX_STOPR;
            impl LASTRX_STOPR { }
            struct LASTTX_STARTRXW;
            impl LASTTX_STARTRXW { }
            impl R { }
            struct LASTTX_SUSPENDW;
            impl LASTTX_SUSPENDW { }
            impl R { }
            struct LASTTX_STOPW;
            impl LASTTX_STOPW { }
            impl R { }
            struct LASTRX_STARTTXW;
            impl LASTRX_STARTTXW { }
            impl R { }
            struct LASTRX_SUSPENDW;
            impl LASTRX_SUSPENDW { }
            impl R { }
            struct LASTRX_STOPW;
            impl LASTRX_STOPW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ERRORR;
            impl ERRORR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct SUSPENDEDR;
            impl SUSPENDEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct RXSTARTEDR;
            impl RXSTARTEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TXSTARTEDR;
            impl TXSTARTEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct LASTRXR;
            impl LASTRXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct LASTTXR;
            impl LASTTXR { }
            struct STOPPEDW;
            impl STOPPEDW { }
            impl R { }
            struct ERRORW;
            impl ERRORW { }
            impl R { }
            struct SUSPENDEDW;
            impl SUSPENDEDW { }
            impl R { }
            struct RXSTARTEDW;
            impl RXSTARTEDW { }
            impl R { }
            struct TXSTARTEDW;
            impl TXSTARTEDW { }
            impl R { }
            struct LASTRXW;
            impl LASTRXW { }
            impl R { }
            struct LASTTXW;
            impl LASTTXW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod errorsrc {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct OVERRUNR;
            impl OVERRUNR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ANACKR;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct DNACKR;
            impl DNACKR { }
            struct OVERRUNW;
            impl OVERRUNW { }
            impl R { }
            struct ANACKW;
            impl R { }
            impl R { }
            struct DNACKW;
            impl DNACKW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod frequency {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct FREQUENCYR;
            impl FREQUENCYR { }
            struct FREQUENCYW;
            impl FREQUENCYW { }
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
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod sda {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod txd {
            mod ptr{
                struct R;
                impl R { }
                struct PTRR;
                impl PTRR { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                impl R { }
                struct MAXCNTR;
                impl MAXCNTR { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                struct AMOUNTR;
                impl R { }
                impl R { }
            }
            mod list {
                struct R;
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct LISTR;
                impl LISTR { }
                struct LISTW;
                impl LISTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod events_stopped {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_STOPPEDR;
            impl EVENTS_STOPPEDR { }
            struct EVENTS_STOPPEDW;
            impl EVENTS_STOPPEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_rxstarted {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_RXSTARTEDR;
            impl EVENTS_RXSTARTEDR { }
            struct EVENTS_RXSTARTEDW;
            impl EVENTS_RXSTARTEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_write {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_WRITER;
            impl EVENTS_WRITER { }
            struct EVENTS_WRITEW;
            impl EVENTS_WRITEW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod shorts {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct WRITE_SUSPENDR;
            impl WRITE_SUSPENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct READ_SUSPENDR;
            impl READ_SUSPENDR { }
            struct WRITE_SUSPENDW;
            impl WRITE_SUSPENDW { }
            impl R { }
            struct READ_SUSPENDW;
            impl READ_SUSPENDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ERRORR;
            impl ERRORR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct RXSTARTEDR;
            impl RXSTARTEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TXSTARTEDR;
            impl TXSTARTEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct WRITER;
            impl WRITER { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct READR;
            impl READR { }
            struct STOPPEDW;
            impl STOPPEDW { }
            impl R { }
            struct ERRORW;
            impl ERRORW { }
            impl R { }
            struct RXSTARTEDW;
            impl RXSTARTEDW { }
            impl R { }
            struct TXSTARTEDW;
            impl TXSTARTEDW { }
            impl R { }
            struct WRITEW;
            impl WRITEW { }
            impl R { }
            struct READW;
            impl READW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod errorsrc {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct OVERFLOWR;
            impl OVERFLOWR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct DNACKR;
            impl DNACKR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct OVERREADR;
            impl OVERREADR { }
            struct OVERFLOWW;
            impl OVERFLOWW { }
            impl R { }
            struct DNACKW;
            impl DNACKW { }
            impl R { }
            struct OVERREADW;
            impl OVERREADW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod enable {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ENABLER;
            impl ENABLER { }
            struct ENABLEW;
            impl ENABLEW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod config {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ADDRESS0R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ADDRESS1R;
            impl R { }
            struct ADDRESS0W;
            impl R { }
            impl R { }
            struct ADDRESS1W;
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
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }

            }
            mod mosi {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod miso {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct READYR;
            impl READYR { }
            struct READYW;
            impl READYW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod enable {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct ENABLER;
            impl ENABLER { }
            struct ENABLEW;
            impl ENABLEW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod config {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ORDERR;
            impl ORDERR { }
            #[derive(Debug, PartialEq)]
            struct CPHAR;
            impl CPHAR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CPOLR;
            impl CPOLR { }
            struct ORDERW;
            impl ORDERW { }
            impl R { }
            struct CPHAW;
            impl CPHAW { }
            impl R { }
            struct CPOLW;
            impl CPOLW { }
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
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod mosi {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod miso {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod txd {
            mod ptr {
                struct R;
                impl R { }
                struct PTRR;
                impl PTRR { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                impl R { }
                struct MAXCNTR;
                impl MAXCNTR { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                struct AMOUNTR;
                impl R { }
                impl R { }
            }
            mod list {
                struct R;
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct LISTR;
                impl LISTR { }
                struct LISTW;
                impl LISTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod tasks_resume {
            struct R;
            impl R { }
            struct TASKS_RESUMEW;
            impl TASKS_RESUMEW { }
            impl R { }
            impl R { }
        }
        mod events_stopped {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_STOPPEDR;
            impl EVENTS_STOPPEDR { }
            struct EVENTS_STOPPEDW;
            impl EVENTS_STOPPEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_end {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_ENDR;
            impl EVENTS_ENDR { }
            struct EVENTS_ENDW;
            impl EVENTS_ENDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_started {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_STARTEDR;
            impl EVENTS_STARTEDR { }
            struct EVENTS_STARTEDW;
            impl EVENTS_STARTEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ENDRXR;
            impl ENDRXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ENDR;
            impl ENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ENDTXR;
            impl ENDTXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct STARTEDR;
            impl STARTEDR { }
            struct STOPPEDW;
            impl STOPPEDW { }
            impl R { }
            struct ENDRXW;
            impl ENDRXW { }
            impl R { }
            struct ENDW;
            impl ENDW { }
            impl R { }
            struct ENDTXW;
            impl ENDTXW { }
            impl R { }
            struct STARTEDW;
            impl STARTEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ENDRXR;
            impl ENDRXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ENDR;
            impl ENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ENDTXR;
            impl ENDTXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct STARTEDR;
            impl STARTEDR { }
            struct STOPPEDW;
            impl STOPPEDW { }
            impl R { }
            struct ENDRXW;
            impl ENDRXW { }
            impl R { }
            struct ENDW;
            impl ENDW { }
            impl R { }
            struct ENDTXW;
            impl ENDTXW { }
            impl R { }
            struct STARTEDW;
            impl STARTEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod frequency {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct FREQUENCYR;
            impl FREQUENCYR { }
            struct FREQUENCYW;
            impl FREQUENCYW { }
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
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod miso {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod mosi {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod csn {
                struct R;
                impl R { }
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod txd {
            mod ptr {
                struct R;
                impl R { }
                struct PTRR;
                impl PTRR { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                impl R { }
                struct MAXCNTR;
                impl MAXCNTR { }
                impl R { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                struct AMOUNTR;
                impl R { }
                impl R { }
            }
            mod list {
                struct R;
                impl R { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct LISTR;
                impl LISTR { }
                struct LISTW;
                impl LISTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod events_endrx {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_ENDRXR;
            impl EVENTS_ENDRXR { }
            struct EVENTS_ENDRXW;
            impl EVENTS_ENDRXW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod shorts {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct END_ACQUIRER;
            impl END_ACQUIRER { }
            struct END_ACQUIREW;
            impl END_ACQUIREW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ENDR;
            impl ENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ENDRXR;
            impl ENDRXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ACQUIREDR;
            impl R { }
            struct ENDW;
            impl ENDW { }
            impl R { }
            struct ENDRXW;
            impl ENDRXW { }
            impl R { }
            struct ACQUIREDW;
            impl R { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod status {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct OVERREADR;
            impl OVERREADR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct OVERFLOWR;
            impl OVERFLOWR { }
            struct OVERREADW;
            impl OVERREADW { }
            impl R { }
            struct OVERFLOWW;
            impl OVERFLOWW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod config {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ORDERR;
            impl ORDERR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CPHAR;
            impl CPHAR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CPOLR;
            impl CPOLR { }
            struct ORDERW;
            impl ORDERW { }
            impl R { }
            struct CPHAW;
            impl CPHAW { }
            impl R { }
            struct CPOLW;
            impl CPOLW { }
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
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE0_CLEARR;
            impl COMPARE0_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE1_CLEARR;
            impl COMPARE1_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE2_CLEARR;
            impl COMPARE2_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE3_CLEARR;
            impl COMPARE3_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE4_CLEARR;
            impl COMPARE4_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE5_CLEARR;
            impl COMPARE5_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE0_STOPR;
            impl COMPARE0_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE1_STOPR;
            impl COMPARE1_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE2_STOPR;
            impl COMPARE2_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE3_STOPR;
            impl COMPARE3_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE4_STOPR;
            impl COMPARE4_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE5_STOPR;
            impl COMPARE5_STOPR { }
            struct COMPARE0_CLEARW;
            impl COMPARE0_CLEARW { }
            impl R { }
            struct COMPARE1_CLEARW;
            impl COMPARE1_CLEARW { }
            impl R { }
            struct COMPARE2_CLEARW;
            impl COMPARE2_CLEARW { }
            impl R { }
            struct COMPARE3_CLEARW;
            impl COMPARE3_CLEARW { }
            impl R { }
            struct COMPARE4_CLEARW;
            impl COMPARE4_CLEARW { }
            impl R { }
            struct COMPARE5_CLEARW;
            impl COMPARE5_CLEARW { }
            impl R { }
            struct COMPARE0_STOPW;
            impl COMPARE0_STOPW { }
            impl R { }
            struct COMPARE1_STOPW;
            impl COMPARE1_STOPW { }
            impl R { }
            struct COMPARE2_STOPW;
            impl COMPARE2_STOPW { }
            impl R { }
            struct COMPARE3_STOPW;
            impl COMPARE3_STOPW { }
            impl R { }
            struct COMPARE4_STOPW;
            impl COMPARE4_STOPW { }
            impl R { }
            struct COMPARE5_STOPW;
            impl COMPARE5_STOPW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE0R;
            impl COMPARE0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE1R;
            impl COMPARE1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE2R;
            impl COMPARE2R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE3R;
            impl COMPARE3R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE4R;
            impl COMPARE4R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct COMPARE5R;
            impl COMPARE5R { }
            struct COMPARE0W;
            impl COMPARE0W { }
            impl R { }
            struct COMPARE1W;
            impl COMPARE1W { }
            impl R { }
            struct COMPARE2W;
            impl COMPARE2W { }
            impl R { }
            struct COMPARE3W;
            impl COMPARE3W { }
            impl R { }
            struct COMPARE4W;
            impl COMPARE4W { }
            impl R { }
            struct COMPARE5W;
            impl COMPARE5W { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod bitmode {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct BITMODER;
            impl R { }
            struct BITMODEW;
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
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_DATARDYR;
            impl EVENTS_DATARDYR { }
            struct EVENTS_DATARDYW;
            impl EVENTS_DATARDYW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct DATARDYR;
            impl DATARDYR { }
            struct DATARDYW;
            impl DATARDYW { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod rng {
        mod events_valrdy {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_VALRDYR;
            impl EVENTS_VALRDYR { }
            struct EVENTS_VALRDYW;
            impl EVENTS_VALRDYW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenset {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct VALRDYR;
            impl VALRDYR { }
            struct VALRDYW;
            impl VALRDYW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod config {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct DERCENR;
            impl DERCENR { }
            struct DERCENW;
            impl DERCENW { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod egu0 {
        mod inten {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED0R;
            impl TRIGGERED0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED1R;
            impl TRIGGERED1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED2R;
            impl TRIGGERED2R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED3R;
            impl TRIGGERED3R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED4R;
            impl TRIGGERED4R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED5R;
            impl TRIGGERED5R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED6R;
            impl TRIGGERED6R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED7R;
            impl TRIGGERED7R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED8R;
            impl TRIGGERED8R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED9R;
            impl TRIGGERED9R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED10R;
            impl TRIGGERED10R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED11R;
            impl TRIGGERED11R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED12R;
            impl TRIGGERED12R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED13R;
            impl TRIGGERED13R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED14R;
            impl TRIGGERED14R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED15R;
            impl TRIGGERED15R { }
            struct TRIGGERED0W;
            impl TRIGGERED0W { }
            impl R { }
            struct TRIGGERED1W;
            impl TRIGGERED1W { }
            impl R { }
            struct TRIGGERED2W;
            impl TRIGGERED2W { }
            impl R { }
            struct TRIGGERED3W;
            impl TRIGGERED3W { }
            impl R { }
            struct TRIGGERED4W;
            impl TRIGGERED4W { }
            impl R { }
            struct TRIGGERED5W;
            impl TRIGGERED5W { }
            impl R { }
            struct TRIGGERED6W;
            impl TRIGGERED6W { }
            impl R { }
            struct TRIGGERED7W;
            impl TRIGGERED7W { }
            impl R { }
            struct TRIGGERED8W;
            impl TRIGGERED8W { }
            impl R { }
            struct TRIGGERED9W;
            impl TRIGGERED9W { }
            impl R { }
            struct TRIGGERED10W;
            impl TRIGGERED10W { }
            impl R { }
            struct TRIGGERED11W;
            impl TRIGGERED11W { }
            impl R { }
            struct TRIGGERED12W;
            impl TRIGGERED12W { }
            impl R { }
            struct TRIGGERED13W;
            impl TRIGGERED13W { }
            impl R { }
            struct TRIGGERED14W;
            impl TRIGGERED14W { }
            impl R { }
            struct TRIGGERED15W;
            impl TRIGGERED15W { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED0R;
            impl TRIGGERED0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED1R;
            impl TRIGGERED1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED2R;
            impl TRIGGERED2R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED3R;
            impl TRIGGERED3R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED4R;
            impl TRIGGERED4R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED5R;
            impl TRIGGERED5R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED6R;
            impl TRIGGERED6R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED7R;
            impl TRIGGERED7R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED8R;
            impl TRIGGERED8R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED9R;
            impl TRIGGERED9R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED10R;
            impl TRIGGERED10R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED11R;
            impl TRIGGERED11R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED12R;
            impl TRIGGERED12R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED13R;
            impl TRIGGERED13R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED14R;
            impl TRIGGERED14R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct TRIGGERED15R;
            impl TRIGGERED15R { }
            struct TRIGGERED0W;
            impl TRIGGERED0W { }
            impl R { }
            struct TRIGGERED1W;
            impl TRIGGERED1W { }
            impl R { }
            struct TRIGGERED2W;
            impl TRIGGERED2W { }
            impl R { }
            struct TRIGGERED3W;
            impl TRIGGERED3W { }
            impl R { }
            struct TRIGGERED4W;
            impl TRIGGERED4W { }
            impl R { }
            struct TRIGGERED5W;
            impl TRIGGERED5W { }
            impl R { }
            struct TRIGGERED6W;
            impl TRIGGERED6W { }
            impl R { }
            struct TRIGGERED7W;
            impl TRIGGERED7W { }
            impl R { }
            struct TRIGGERED8W;
            impl TRIGGERED8W { }
            impl R { }
            struct TRIGGERED9W;
            impl TRIGGERED9W { }
            impl R { }
            struct TRIGGERED10W;
            impl TRIGGERED10W { }
            impl R { }
            struct TRIGGERED11W;
            impl TRIGGERED11W { }
            impl R { }
            struct TRIGGERED12W;
            impl TRIGGERED12W { }
            impl R { }
            struct TRIGGERED13W;
            impl TRIGGERED13W { }
            impl R { }
            struct TRIGGERED14W;
            impl TRIGGERED14W { }
            impl R { }
            struct TRIGGERED15W;
            impl TRIGGERED15W { }
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
                struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                struct CONNECTR;
                impl CONNECTR { }
                impl R { }
                struct CONNECTW;
                impl CONNECTW { }
                impl R { }
                impl R { }
                impl R { }
            }
        }
        mod tasks_seqstart {
            struct R;
            impl R { }
            struct TASKS_SEQSTARTW;
            impl TASKS_SEQSTARTW { }
            impl R { }
            impl R { }
        }
        mod tasks_nextstep {
            struct R;
            impl R { }
            struct TASKS_NEXTSTEPW;
            impl TASKS_NEXTSTEPW { }
            impl R { }
            impl R { }
        }
        mod events_stopped {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_STOPPEDR;
            impl EVENTS_STOPPEDR { }
            struct EVENTS_STOPPEDW;
            impl EVENTS_STOPPEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_seqstarted {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_SEQSTARTEDR;
            impl EVENTS_SEQSTARTEDR { }
            struct EVENTS_SEQSTARTEDW;
            impl EVENTS_SEQSTARTEDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_seqend {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_SEQENDR;
            impl EVENTS_SEQENDR { }
            struct EVENTS_SEQENDW;
            impl EVENTS_SEQENDW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod events_loopsdone {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct EVENTS_LOOPSDONER;
            impl EVENTS_LOOPSDONER { }
            struct EVENTS_LOOPSDONEW;
            impl EVENTS_LOOPSDONEW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod inten {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct SEQSTARTED0R;
            impl SEQSTARTED0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct SEQSTARTED1R;
            impl SEQSTARTED1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct SEQEND0R;
            impl SEQEND0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct SEQEND1R;
            impl SEQEND1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct PWMPERIODENDR;
            impl PWMPERIODENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct LOOPSDONER;
            impl LOOPSDONER { }
            struct STOPPEDW;
            impl STOPPEDW { }
            impl R { }
            struct SEQSTARTED0W;
            impl SEQSTARTED0W { }
            impl R { }
            struct SEQSTARTED1W;
            impl SEQSTARTED1W { }
            impl R { }
            struct SEQEND0W;
            impl SEQEND0W { }
            impl R { }
            struct SEQEND1W;
            impl SEQEND1W { }
            impl R { }
            struct PWMPERIODENDW;
            impl PWMPERIODENDW { }
            impl R { }
            struct LOOPSDONEW;
            impl LOOPSDONEW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod intenclr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct SEQSTARTED0R;
            impl SEQSTARTED0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct SEQSTARTED1R;
            impl SEQSTARTED1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct SEQEND0R;
            impl SEQEND0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct SEQEND1R;
            impl SEQEND1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct PWMPERIODENDR;
            impl PWMPERIODENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct LOOPSDONER;
            impl LOOPSDONER { }
            struct STOPPEDW;
            impl STOPPEDW { }
            impl R { }
            struct SEQSTARTED0W;
            impl SEQSTARTED0W { }
            impl R { }
            struct SEQSTARTED1W;
            impl SEQSTARTED1W { }
            impl R { }
            struct SEQEND0W;
            impl SEQEND0W { }
            impl R { }
            struct SEQEND1W;
            impl SEQEND1W { }
            impl R { }
            struct PWMPERIODENDW;
            impl PWMPERIODENDW { }
            impl R { }
            struct LOOPSDONEW;
            impl LOOPSDONEW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod mode {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct UPDOWNR;
            impl UPDOWNR { }
            struct UPDOWNW;
            impl UPDOWNW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod prescaler {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct PRESCALERR;
            impl PRESCALERR { }
            struct PRESCALERW;
            impl PRESCALERW { }
            impl R { }
            impl R { }
            impl R { }
        }
        mod loop_ {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CNTR;
            impl CNTR { }
            struct CNTW;
            impl CNTW { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod nvmc {
        mod ready {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct READYR;
            impl READYR { }
            impl R { }
        }
        mod eraseuicr {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ERASEUICRR;
            impl ERASEUICRR { }
            struct ERASEUICRW;
            impl ERASEUICRW { }
            impl R { }
            impl R { }
            impl R { }
        }
    }
    mod ppi {
        mod chg {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH0R;
            impl CH0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH1R;
            impl CH1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH2R;
            impl CH2R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH3R;
            impl CH3R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH4R;
            impl CH4R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH5R;
            impl CH5R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH6R;
            impl CH6R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH7R;
            impl CH7R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH8R;
            impl CH8R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH9R;
            impl CH9R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH10R;
            impl CH10R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH11R;
            impl CH11R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH12R;
            impl CH12R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH13R;
            impl CH13R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH14R;
            impl CH14R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH15R;
            impl CH15R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH16R;
            impl CH16R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH17R;
            impl CH17R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH18R;
            impl CH18R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH19R;
            impl CH19R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH20R;
            impl CH20R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH21R;
            impl CH21R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH22R;
            impl CH22R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH23R;
            impl CH23R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH24R;
            impl CH24R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH25R;
            impl CH25R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH26R;
            impl CH26R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH27R;
            impl CH27R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH28R;
            impl CH28R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH29R;
            impl CH29R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH30R;
            impl CH30R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct CH31R;
            impl CH31R { }
            impl R { }
            impl R { }
        }
    }
}
