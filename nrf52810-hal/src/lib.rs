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
            struct _REGION0W;
            impl _REGION0W { }
            struct REGION1W;
            impl REGION1W { }
            struct _REGION1W;
            impl _REGION1W { }
            struct REGION2W;
            impl REGION2W { }
            struct _REGION2W;
            impl _REGION2W { }
            struct REGION3W;
            impl REGION3W { }
            struct _REGION3W;
            impl _REGION3W { }
            struct REGION4W;
            impl REGION4W { }
            struct _REGION4W;
            impl _REGION4W { }
            struct REGION5W;
            impl REGION5W { }
            struct _REGION5W;
            impl _REGION5W { }
            struct REGION6W;
            impl REGION6W { }
            struct _REGION6W;
            impl _REGION6W { }
            struct REGION7W;
            impl REGION7W { }
            struct _REGION7W;
            impl _REGION7W { }
            struct REGION8W;
            impl REGION8W { }
            struct _REGION8W;
            impl _REGION8W { }
            struct REGION9W;
            impl REGION9W { }
            struct _REGION9W;
            impl _REGION9W { }
            struct REGION10W;
            impl REGION10W { }
            struct _REGION10W;
            impl _REGION10W { }
            struct REGION11W;
            impl REGION11W { }
            struct _REGION11W;
            impl _REGION11W { }
            struct REGION12W;
            impl REGION12W { }
            struct _REGION12W;
            impl _REGION12W { }
            struct REGION13W;
            impl REGION13W { }
            struct _REGION13W;
            impl _REGION13W { }
            struct REGION14W;
            impl REGION14W { }
            struct _REGION14W;
            impl _REGION14W { }
            struct REGION15W;
            impl REGION15W { }
            struct _REGION15W;
            impl _REGION15W { }
            struct REGION16W;
            impl REGION16W { }
            struct _REGION16W;
            impl _REGION16W { }
            struct REGION17W;
            impl REGION17W { }
            struct _REGION17W;
            impl _REGION17W { }
            struct REGION18W;
            impl REGION18W { }
            struct _REGION18W;
            impl _REGION18W { }
            struct REGION19W;
            impl REGION19W { }
            struct _REGION19W;
            impl _REGION19W { }
            struct REGION20W;
            impl REGION20W { }
            struct _REGION20W;
            impl _REGION20W { }
            struct REGION21W;
            impl REGION21W { }
            struct _REGION21W;
            impl _REGION21W { }
            struct REGION22W;
            impl REGION22W { }
            struct _REGION22W;
            impl _REGION22W { }
            struct REGION23W;
            impl REGION23W { }
            struct _REGION23W;
            impl _REGION23W { }
            struct REGION24W;
            impl REGION24W { }
            struct _REGION24W;
            impl _REGION24W { }
            struct REGION25W;
            impl REGION25W { }
            struct _REGION25W;
            impl _REGION25W { }
            struct REGION26W;
            impl REGION26W { }
            struct _REGION26W;
            impl _REGION26W { }
            struct REGION27W;
            impl REGION27W { }
            struct _REGION27W;
            impl _REGION27W { }
            struct REGION28W;
            impl REGION28W { }
            struct _REGION28W;
            impl _REGION28W { }
            struct REGION29W;
            impl REGION29W { }
            struct _REGION29W;
            impl _REGION29W { }
            struct REGION30W;
            impl REGION30W { }
            struct _REGION30W;
            impl _REGION30W { }
            struct REGION31W;
            impl REGION31W { }
            struct _REGION31W;
            impl _REGION31W { }
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
            struct _DISABLEINDEBUGW;
            impl _DISABLEINDEBUGW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
            struct _EVENTS_STOPPEDW;
            impl _EVENTS_STOPPEDW { }
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
            struct _EVENTS_TXDSENTW;
            impl _EVENTS_TXDSENTW { }
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
            struct _EVENTS_BBW;
            impl _EVENTS_BBW { }
            impl R { }
            impl R { }
        }
        mod shorts {
            struct R;
            impl R { }
            #[derive(Debug, PartialEq)]
            struct BB_SUSPENDR;
            impl BB_SUSPENDR { }
            #[derive(Debug, PartialEq)]
            struct BB_STOPR;
            impl BB_STOPR { }
            struct BB_SUSPENDW;
            impl BB_SUSPENDW { }
            struct _BB_SUSPENDW;
            impl _BB_SUSPENDW { }
            struct BB_STOPW;
            impl BB_STOPW { }
            struct _BB_STOPW;
            impl _BB_STOPW { }
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
            impl BBR { }
            #[derive(Debug, PartialEq)]
            struct SUSPENDEDR;
            impl SUSPENDEDR { }
            struct STOPPEDW;
            impl STOPPEDW { }
            struct _STOPPEDW;
            impl _STOPPEDW { }
            struct RXDREADYW;
            impl RXDREADYW { }
            struct _RXDREADYW;
            impl _RXDREADYW { }
            struct TXDSENTW;
            impl TXDSENTW { }
            struct _TXDSENTW;
            impl _TXDSENTW { }
            struct ERRORW;
            impl ERRORW { }
            struct _ERRORW;
            impl _ERRORW { }
            struct BBW;
            impl BBW { }
            struct _BBW;
            impl _BBW { }
            struct SUSPENDEDW;
            impl SUSPENDEDW { }
            struct _SUSPENDEDW;
            impl _SUSPENDEDW { }
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
            struct _ENABLEW;
            impl _ENABLEW { }
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
            struct _FREQUENCYW;
            impl _FREQUENCYW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PTRW;
                impl _PTRW { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                
                impl R { }
                struct MAXCNTR;
                impl MAXCNTR { }
                struct _MAXCNTW;
                impl _MAXCNTW { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                struct AMOUNTR;
                impl AMOUNTR { }
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
                struct _LISTW;
                impl _LISTW { }
                impl R { }
                impl R { }
            }
        }
        mod tasks_suspend {
            struct R;
            impl R { }
            struct TASKS_SUSPENDW;
            impl TASKS_SUSPENDW { }
            struct _TASKS_SUSPENDW;
            impl _TASKS_SUSPENDW { }
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
            struct _EVENTS_ERRORW;
            impl _EVENTS_ERRORW { }
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
            struct _EVENTS_RXSTARTEDW;
            impl _EVENTS_RXSTARTEDW { }
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
            struct _EVENTS_LASTRXW;
            impl _EVENTS_LASTRXW { }
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
            struct _LASTTX_STARTRXW;
            impl _LASTTX_STARTRXW { }
            struct LASTTX_SUSPENDW;
            impl LASTTX_SUSPENDW { }
            struct _LASTTX_SUSPENDW;
            impl _LASTTX_SUSPENDW { }
            struct LASTTX_STOPW;
            impl LASTTX_STOPW { }
            struct _LASTTX_STOPW;
            impl _LASTTX_STOPW { }
            struct LASTRX_STARTTXW;
            impl LASTRX_STARTTXW { }
            struct _LASTRX_STARTTXW;
            impl _LASTRX_STARTTXW { }
            struct LASTRX_SUSPENDW;
            impl LASTRX_SUSPENDW { }
            struct _LASTRX_SUSPENDW;
            impl _LASTRX_SUSPENDW { }
            struct LASTRX_STOPW;
            impl LASTRX_STOPW { }
            struct _LASTRX_STOPW;
            impl _LASTRX_STOPW { }
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
            struct _STOPPEDW;
            impl _STOPPEDW { }
            struct ERRORW;
            impl ERRORW { }
            struct _ERRORW;
            impl _ERRORW { }
            struct SUSPENDEDW;
            impl SUSPENDEDW { }
            struct _SUSPENDEDW;
            impl _SUSPENDEDW { }
            struct RXSTARTEDW;
            impl RXSTARTEDW { }
            struct _RXSTARTEDW;
            impl _RXSTARTEDW { }
            struct TXSTARTEDW;
            impl TXSTARTEDW { }
            struct _TXSTARTEDW;
            impl _TXSTARTEDW { }
            struct LASTRXW;
            impl LASTRXW { }
            struct _LASTRXW;
            impl _LASTRXW { }
            struct LASTTXW;
            impl LASTTXW { }
            struct _LASTTXW;
            impl _LASTTXW { }
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
            impl ANACKR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct DNACKR;
            impl DNACKR { }
            struct OVERRUNW;
            impl OVERRUNW { }
            struct _OVERRUNW;
            impl _OVERRUNW { }
            struct ANACKW;
            impl ANACKW { }
            struct _ANACKW;
            impl _ANACKW { }
            struct DNACKW;
            impl DNACKW { }
            struct _DNACKW;
            impl _DNACKW { }
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
            struct _FREQUENCYW;
            impl _FREQUENCYW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PTRW;
                impl _PTRW { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                
                impl R { }
                struct MAXCNTR;
                impl MAXCNTR { }
                struct _MAXCNTW;
                impl _MAXCNTW { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                struct AMOUNTR;
                impl AMOUNTR { }
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
                struct _LISTW;
                impl _LISTW { }
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
            struct _EVENTS_STOPPEDW;
            impl _EVENTS_STOPPEDW { }
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
            struct _EVENTS_RXSTARTEDW;
            impl _EVENTS_RXSTARTEDW { }
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
            struct _EVENTS_WRITEW;
            impl _EVENTS_WRITEW { }
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
            struct _WRITE_SUSPENDW;
            impl _WRITE_SUSPENDW { }
            struct READ_SUSPENDW;
            impl READ_SUSPENDW { }
            struct _READ_SUSPENDW;
            impl _READ_SUSPENDW { }
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
            struct _STOPPEDW;
            impl _STOPPEDW { }
            struct ERRORW;
            impl ERRORW { }
            struct _ERRORW;
            impl _ERRORW { }
            struct RXSTARTEDW;
            impl RXSTARTEDW { }
            struct _RXSTARTEDW;
            impl _RXSTARTEDW { }
            struct TXSTARTEDW;
            impl TXSTARTEDW { }
            struct _TXSTARTEDW;
            impl _TXSTARTEDW { }
            struct WRITEW;
            impl WRITEW { }
            struct _WRITEW;
            impl _WRITEW { }
            struct READW;
            impl READW { }
            struct _READW;
            impl _READW { }
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
            struct _OVERFLOWW;
            impl _OVERFLOWW { }
            struct DNACKW;
            impl DNACKW { }
            struct _DNACKW;
            impl _DNACKW { }
            struct OVERREADW;
            impl OVERREADW { }
            struct _OVERREADW;
            impl _OVERREADW { }
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
            struct _ENABLEW;
            impl _ENABLEW { }
            impl R { }
            impl R { }
        }
        mod config {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ADDRESS0R;
            impl ADDRESS0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct ADDRESS1R;
            impl ADDRESS1R { }
            struct ADDRESS0W;
            impl ADDRESS0W { }
            struct _ADDRESS0W;
            impl _ADDRESS0W { }
            struct ADDRESS1W;
            impl ADDRESS1W { }
            struct _ADDRESS1W;
            impl _ADDRESS1W { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
            struct _READYW;
            impl _READYW { }
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
            struct _ENABLEW;
            impl _ENABLEW { }
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
            struct _ORDERW;
            impl _ORDERW { }
            struct CPHAW;
            impl CPHAW { }
            struct _CPHAW;
            impl _CPHAW { }
            struct CPOLW;
            impl CPOLW { }
            struct _CPOLW;
            impl _CPOLW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PTRW;
                impl _PTRW { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                
                impl R { }
                struct MAXCNTR;
                impl MAXCNTR { }
                struct _MAXCNTW;
                impl _MAXCNTW { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                struct AMOUNTR;
                impl AMOUNTR { }
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
                struct _LISTW;
                impl _LISTW { }
                impl R { }
                impl R { }
            }
        }
        mod tasks_resume {
            struct R;
            impl R { }
            struct TASKS_RESUMEW;
            impl TASKS_RESUMEW { }
            struct _TASKS_RESUMEW;
            impl _TASKS_RESUMEW { }
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
            struct _EVENTS_STOPPEDW;
            impl _EVENTS_STOPPEDW { }
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
            struct _EVENTS_ENDW;
            impl _EVENTS_ENDW { }
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
            struct _EVENTS_STARTEDW;
            impl _EVENTS_STARTEDW { }
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
            struct _STOPPEDW;
            impl _STOPPEDW { }
            struct ENDRXW;
            impl ENDRXW { }
            struct _ENDRXW;
            impl _ENDRXW { }
            struct ENDW;
            impl ENDW { }
            struct _ENDW;
            impl _ENDW { }
            struct ENDTXW;
            impl ENDTXW { }
            struct _ENDTXW;
            impl _ENDTXW { }
            struct STARTEDW;
            impl STARTEDW { }
            struct _STARTEDW;
            impl _STARTEDW { }
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
            struct _STOPPEDW;
            impl _STOPPEDW { }
            struct ENDRXW;
            impl ENDRXW { }
            struct _ENDRXW;
            impl _ENDRXW { }
            struct ENDW;
            impl ENDW { }
            struct _ENDW;
            impl _ENDW { }
            struct ENDTXW;
            impl ENDTXW { }
            struct _ENDTXW;
            impl _ENDTXW { }
            struct STARTEDW;
            impl STARTEDW { }
            struct _STARTEDW;
            impl _STARTEDW { }
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
            struct _FREQUENCYW;
            impl _FREQUENCYW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
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
                struct _PTRW;
                impl _PTRW { }
                impl R { }
                impl R { }
            }
            mod maxcnt {
                struct R;
                
                impl R { }
                struct MAXCNTR;
                impl MAXCNTR { }
                struct _MAXCNTW;
                impl _MAXCNTW { }
                impl R { }
                impl R { }
            }
            mod amount {
                struct R;
                impl R { }
                struct AMOUNTR;
                impl AMOUNTR { }
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
                struct _LISTW;
                impl _LISTW { }
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
            struct _EVENTS_ENDRXW;
            impl _EVENTS_ENDRXW { }
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
            struct _END_ACQUIREW;
            impl _END_ACQUIREW { }
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
            impl ACQUIREDR { }
            struct ENDW;
            impl ENDW { }
            struct _ENDW;
            impl _ENDW { }
            struct ENDRXW;
            impl ENDRXW { }
            struct _ENDRXW;
            impl _ENDRXW { }
            struct ACQUIREDW;
            impl ACQUIREDW { }
            struct _ACQUIREDW;
            impl _ACQUIREDW { }
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
            struct _OVERREADW;
            impl _OVERREADW { }
            struct OVERFLOWW;
            impl OVERFLOWW { }
            struct _OVERFLOWW;
            impl _OVERFLOWW { }
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
            struct _ORDERW;
            impl _ORDERW { }
            struct CPHAW;
            impl CPHAW { }
            struct _CPHAW;
            impl _CPHAW { }
            struct CPOLW;
            impl CPOLW { }
            struct _CPOLW;
            impl _CPOLW { }
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
            struct _COMPARE0_CLEARW;
            impl _COMPARE0_CLEARW { }
            struct COMPARE1_CLEARW;
            impl COMPARE1_CLEARW { }
            struct _COMPARE1_CLEARW;
            impl _COMPARE1_CLEARW { }
            struct COMPARE2_CLEARW;
            impl COMPARE2_CLEARW { }
            struct _COMPARE2_CLEARW;
            impl _COMPARE2_CLEARW { }
            struct COMPARE3_CLEARW;
            impl COMPARE3_CLEARW { }
            struct _COMPARE3_CLEARW;
            impl _COMPARE3_CLEARW { }
            struct COMPARE4_CLEARW;
            impl COMPARE4_CLEARW { }
            struct _COMPARE4_CLEARW;
            impl _COMPARE4_CLEARW { }
            struct COMPARE5_CLEARW;
            impl COMPARE5_CLEARW { }
            struct _COMPARE5_CLEARW;
            impl _COMPARE5_CLEARW { }
            struct COMPARE0_STOPW;
            impl COMPARE0_STOPW { }
            struct _COMPARE0_STOPW;
            impl _COMPARE0_STOPW { }
            struct COMPARE1_STOPW;
            impl COMPARE1_STOPW { }
            struct _COMPARE1_STOPW;
            impl _COMPARE1_STOPW { }
            struct COMPARE2_STOPW;
            impl COMPARE2_STOPW { }
            struct _COMPARE2_STOPW;
            impl _COMPARE2_STOPW { }
            struct COMPARE3_STOPW;
            impl COMPARE3_STOPW { }
            struct _COMPARE3_STOPW;
            impl _COMPARE3_STOPW { }
            struct COMPARE4_STOPW;
            impl COMPARE4_STOPW { }
            struct _COMPARE4_STOPW;
            impl _COMPARE4_STOPW { }
            struct COMPARE5_STOPW;
            impl COMPARE5_STOPW { }
            struct _COMPARE5_STOPW;
            impl _COMPARE5_STOPW { }
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
            struct _COMPARE0W;
            impl _COMPARE0W { }
            struct COMPARE1W;
            impl COMPARE1W { }
            struct _COMPARE1W;
            impl _COMPARE1W { }
            struct COMPARE2W;
            impl COMPARE2W { }
            struct _COMPARE2W;
            impl _COMPARE2W { }
            struct COMPARE3W;
            impl COMPARE3W { }
            struct _COMPARE3W;
            impl _COMPARE3W { }
            struct COMPARE4W;
            impl COMPARE4W { }
            struct _COMPARE4W;
            impl _COMPARE4W { }
            struct COMPARE5W;
            impl COMPARE5W { }
            struct _COMPARE5W;
            impl _COMPARE5W { }
            impl R { }
            impl R { }
        }
        mod bitmode {
            struct R;
            impl R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            struct BITMODER;
            impl BITMODER { }
            struct BITMODEW;
            impl BITMODEW { }
            struct _BITMODEW;
            impl _BITMODEW { }
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
            struct _EVENTS_DATARDYW;
            impl _EVENTS_DATARDYW { }
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
            struct _DATARDYW;
            impl _DATARDYW { }
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
            struct _EVENTS_VALRDYW;
            impl _EVENTS_VALRDYW { }
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
            struct _VALRDYW;
            impl _VALRDYW { }
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
            struct _DERCENW;
            impl _DERCENW { }
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
            struct _TRIGGERED0W;
            impl _TRIGGERED0W { }
            struct TRIGGERED1W;
            impl TRIGGERED1W { }
            struct _TRIGGERED1W;
            impl _TRIGGERED1W { }
            struct TRIGGERED2W;
            impl TRIGGERED2W { }
            struct _TRIGGERED2W;
            impl _TRIGGERED2W { }
            struct TRIGGERED3W;
            impl TRIGGERED3W { }
            struct _TRIGGERED3W;
            impl _TRIGGERED3W { }
            struct TRIGGERED4W;
            impl TRIGGERED4W { }
            struct _TRIGGERED4W;
            impl _TRIGGERED4W { }
            struct TRIGGERED5W;
            impl TRIGGERED5W { }
            struct _TRIGGERED5W;
            impl _TRIGGERED5W { }
            struct TRIGGERED6W;
            impl TRIGGERED6W { }
            struct _TRIGGERED6W;
            impl _TRIGGERED6W { }
            struct TRIGGERED7W;
            impl TRIGGERED7W { }
            struct _TRIGGERED7W;
            impl _TRIGGERED7W { }
            struct TRIGGERED8W;
            impl TRIGGERED8W { }
            struct _TRIGGERED8W;
            impl _TRIGGERED8W { }
            struct TRIGGERED9W;
            impl TRIGGERED9W { }
            struct _TRIGGERED9W;
            impl _TRIGGERED9W { }
            struct TRIGGERED10W;
            impl TRIGGERED10W { }
            struct _TRIGGERED10W;
            impl _TRIGGERED10W { }
            struct TRIGGERED11W;
            impl TRIGGERED11W { }
            struct _TRIGGERED11W;
            impl _TRIGGERED11W { }
            struct TRIGGERED12W;
            impl TRIGGERED12W { }
            struct _TRIGGERED12W;
            impl _TRIGGERED12W { }
            struct TRIGGERED13W;
            impl TRIGGERED13W { }
            struct _TRIGGERED13W;
            impl _TRIGGERED13W { }
            struct TRIGGERED14W;
            impl TRIGGERED14W { }
            struct _TRIGGERED14W;
            impl _TRIGGERED14W { }
            struct TRIGGERED15W;
            impl TRIGGERED15W { }
            struct _TRIGGERED15W;
            impl _TRIGGERED15W { }
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
            struct _TRIGGERED0W;
            impl _TRIGGERED0W { }
            struct TRIGGERED1W;
            impl TRIGGERED1W { }
            struct _TRIGGERED1W;
            impl _TRIGGERED1W { }
            struct TRIGGERED2W;
            impl TRIGGERED2W { }
            struct _TRIGGERED2W;
            impl _TRIGGERED2W { }
            struct TRIGGERED3W;
            impl TRIGGERED3W { }
            struct _TRIGGERED3W;
            impl _TRIGGERED3W { }
            struct TRIGGERED4W;
            impl TRIGGERED4W { }
            struct _TRIGGERED4W;
            impl _TRIGGERED4W { }
            struct TRIGGERED5W;
            impl TRIGGERED5W { }
            struct _TRIGGERED5W;
            impl _TRIGGERED5W { }
            struct TRIGGERED6W;
            impl TRIGGERED6W { }
            struct _TRIGGERED6W;
            impl _TRIGGERED6W { }
            struct TRIGGERED7W;
            impl TRIGGERED7W { }
            struct _TRIGGERED7W;
            impl _TRIGGERED7W { }
            struct TRIGGERED8W;
            impl TRIGGERED8W { }
            struct _TRIGGERED8W;
            impl _TRIGGERED8W { }
            struct TRIGGERED9W;
            impl TRIGGERED9W { }
            struct _TRIGGERED9W;
            impl _TRIGGERED9W { }
            struct TRIGGERED10W;
            impl TRIGGERED10W { }
            struct _TRIGGERED10W;
            impl _TRIGGERED10W { }
            struct TRIGGERED11W;
            impl TRIGGERED11W { }
            struct _TRIGGERED11W;
            impl _TRIGGERED11W { }
            struct TRIGGERED12W;
            impl TRIGGERED12W { }
            struct _TRIGGERED12W;
            impl _TRIGGERED12W { }
            struct TRIGGERED13W;
            impl TRIGGERED13W { }
            struct _TRIGGERED13W;
            impl _TRIGGERED13W { }
            struct TRIGGERED14W;
            impl TRIGGERED14W { }
            struct _TRIGGERED14W;
            impl _TRIGGERED14W { }
            struct TRIGGERED15W;
            impl TRIGGERED15W { }
            struct _TRIGGERED15W;
            impl _TRIGGERED15W { }
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
                struct _PINW;
                impl _PINW { }
                struct CONNECTW;
                impl CONNECTW { }
                struct _CONNECTW;
                impl _CONNECTW { }
                impl R { }
                impl R { }

            }
        }
        mod tasks_seqstart {
            struct R;
            impl R { }
            struct TASKS_SEQSTARTW;
            impl TASKS_SEQSTARTW { }
            struct _TASKS_SEQSTARTW;
            impl _TASKS_SEQSTARTW { }
            impl R { }
        }
        mod tasks_nextstep {
            struct R;
            impl R { }
            struct TASKS_NEXTSTEPW;
            impl TASKS_NEXTSTEPW { }
            struct _TASKS_NEXTSTEPW;
            impl _TASKS_NEXTSTEPW { }
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
            struct _EVENTS_STOPPEDW;
            impl _EVENTS_STOPPEDW { }
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
            struct _EVENTS_SEQSTARTEDW;
            impl _EVENTS_SEQSTARTEDW { }
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
            struct _EVENTS_SEQENDW;
            impl _EVENTS_SEQENDW { }
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
            struct _EVENTS_LOOPSDONEW;
            impl _EVENTS_LOOPSDONEW { }
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
            struct _STOPPEDW;
            impl _STOPPEDW { }
            struct SEQSTARTED0W;
            impl SEQSTARTED0W { }
            struct _SEQSTARTED0W;
            impl _SEQSTARTED0W { }
            struct SEQSTARTED1W;
            impl SEQSTARTED1W { }
            struct _SEQSTARTED1W;
            impl _SEQSTARTED1W { }
            struct SEQEND0W;
            impl SEQEND0W { }
            struct _SEQEND0W;
            impl _SEQEND0W { }
            struct SEQEND1W;
            impl SEQEND1W { }
            struct _SEQEND1W;
            impl _SEQEND1W { }
            struct PWMPERIODENDW;
            impl PWMPERIODENDW { }
            struct _PWMPERIODENDW;
            impl _PWMPERIODENDW { }
            struct LOOPSDONEW;
            impl LOOPSDONEW { }
            struct _LOOPSDONEW;
            impl _LOOPSDONEW { }
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
            struct _STOPPEDW;
            impl _STOPPEDW { }
            struct SEQSTARTED0W;
            impl SEQSTARTED0W { }
            struct _SEQSTARTED0W;
            impl _SEQSTARTED0W { }
            struct SEQSTARTED1W;
            impl SEQSTARTED1W { }
            struct _SEQSTARTED1W;
            impl _SEQSTARTED1W { }
            struct SEQEND0W;
            impl SEQEND0W { }
            struct _SEQEND0W;
            impl _SEQEND0W { }
            struct SEQEND1W;
            impl SEQEND1W { }
            struct _SEQEND1W;
            impl _SEQEND1W { }
            struct PWMPERIODENDW;
            impl PWMPERIODENDW { }
            struct _PWMPERIODENDW;
            impl _PWMPERIODENDW { }
            struct LOOPSDONEW;
            impl LOOPSDONEW { }
            struct _LOOPSDONEW;
            impl _LOOPSDONEW { }
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
            struct _UPDOWNW;
            impl _UPDOWNW { }
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
            struct _PRESCALERW;
            impl _PRESCALERW { }
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
            struct _CNTW;
            impl _CNTW { }
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
            struct _ERASEUICRW;
            impl _ERASEUICRW { }
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
