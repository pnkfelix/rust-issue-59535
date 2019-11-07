#![allow(non_camel_case_types, dead_code)]
#![no_std]

pub mod nrf52810_pac {
    extern crate cortex_m_rt;
    pub struct Vector { _handler: unsafe extern "C" fn(), }
    extern "C" fn power_clock_2() { }

    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 1] = [ Vector { _handler: power_clock_2 } ];

    pub struct FICR;pub mod ficr {
        pub mod info {
            pub struct PART;
            pub mod part {
                pub struct R;
                impl super::PART { }
                #[derive(Debug, PartialEq)]
                pub struct PARTR;
                impl PARTR { }
                impl R { }
            }
            pub struct VARIANT;
            pub struct PACKAGE;
            pub mod package {
                pub struct R;
                impl super::PACKAGE { }
                #[derive(Debug, PartialEq)]
                pub struct PACKAGER;
                impl PACKAGER { }
                impl R { }
            }
            pub struct FLASH;
            pub mod flash {
                pub struct R;
                impl super::FLASH { }
                #[derive(Debug, PartialEq)]
                pub struct FLASHR;
                impl FLASHR { }
                impl R { }
            }
        }
        pub struct DEVICEADDRTYPE;
        pub mod deviceaddrtype {
            pub struct R;
            impl super::DEVICEADDRTYPE { }
            #[derive(Debug, PartialEq)]
            pub struct DEVICEADDRTYPER;
            impl DEVICEADDRTYPER { }
            impl R { }
        }
    }
    pub struct BPROT;pub mod bprot {
        pub struct CONFIG0;
        pub mod config0 {
            pub struct R;
            pub struct W;
            impl super::CONFIG0 { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION0R;
            impl REGION0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION1R;
            impl REGION1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION2R;
            impl REGION2R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION3R;
            impl REGION3R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION4R;
            impl REGION4R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION5R;
            impl REGION5R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION6R;
            impl REGION6R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION7R;
            impl REGION7R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION8R;
            impl REGION8R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION9R;
            impl REGION9R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION10R;
            impl REGION10R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION11R;
            impl REGION11R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION12R;
            impl REGION12R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION13R;
            impl REGION13R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION14R;
            impl REGION14R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION15R;
            impl REGION15R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION16R;
            impl REGION16R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION17R;
            impl REGION17R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION18R;
            impl REGION18R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION19R;
            impl REGION19R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION20R;
            impl REGION20R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION21R;
            impl REGION21R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION22R;
            impl REGION22R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION23R;
            impl REGION23R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION24R;
            impl REGION24R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION25R;
            impl REGION25R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION26R;
            impl REGION26R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION27R;
            impl REGION27R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION28R;
            impl REGION28R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION29R;
            impl REGION29R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION30R;
            impl REGION30R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct REGION31R;
            impl REGION31R { }
            pub struct REGION0W;
            impl REGION0W { }
            pub struct _REGION0W { w: &'static mut W, }
            impl _REGION0W { }
            pub struct REGION1W;
            impl REGION1W { }
            pub struct _REGION1W { w: &'static mut W, }
            impl _REGION1W { }
            pub struct REGION2W;
            impl REGION2W { }
            pub struct _REGION2W { w: &'static mut W, }
            impl _REGION2W { }
            pub struct REGION3W;
            impl REGION3W { }
            pub struct _REGION3W { w: &'static mut W, }
            impl _REGION3W { }
            pub struct REGION4W;
            impl REGION4W { }
            pub struct _REGION4W { w: &'static mut W, }
            impl _REGION4W { }
            pub struct REGION5W;
            impl REGION5W { }
            pub struct _REGION5W { w: &'static mut W, }
            impl _REGION5W { }
            pub struct REGION6W;
            impl REGION6W { }
            pub struct _REGION6W { w: &'static mut W, }
            impl _REGION6W { }
            pub struct REGION7W;
            impl REGION7W { }
            pub struct _REGION7W { w: &'static mut W, }
            impl _REGION7W { }
            pub struct REGION8W;
            impl REGION8W { }
            pub struct _REGION8W { w: &'static mut W, }
            impl _REGION8W { }
            pub struct REGION9W;
            impl REGION9W { }
            pub struct _REGION9W { w: &'static mut W, }
            impl _REGION9W { }
            pub struct REGION10W;
            impl REGION10W { }
            pub struct _REGION10W { w: &'static mut W, }
            impl _REGION10W { }
            pub struct REGION11W;
            impl REGION11W { }
            pub struct _REGION11W { w: &'static mut W, }
            impl _REGION11W { }
            pub struct REGION12W;
            impl REGION12W { }
            pub struct _REGION12W { w: &'static mut W, }
            impl _REGION12W { }
            pub struct REGION13W;
            impl REGION13W { }
            pub struct _REGION13W { w: &'static mut W, }
            impl _REGION13W { }
            pub struct REGION14W;
            impl REGION14W { }
            pub struct _REGION14W { w: &'static mut W, }
            impl _REGION14W { }
            pub struct REGION15W;
            impl REGION15W { }
            pub struct _REGION15W { w: &'static mut W, }
            impl _REGION15W { }
            pub struct REGION16W;
            impl REGION16W { }
            pub struct _REGION16W { w: &'static mut W, }
            impl _REGION16W { }
            pub struct REGION17W;
            impl REGION17W { }
            pub struct _REGION17W { w: &'static mut W, }
            impl _REGION17W { }
            pub struct REGION18W;
            impl REGION18W { }
            pub struct _REGION18W { w: &'static mut W, }
            impl _REGION18W { }
            pub struct REGION19W;
            impl REGION19W { }
            pub struct _REGION19W { w: &'static mut W, }
            impl _REGION19W { }
            pub struct REGION20W;
            impl REGION20W { }
            pub struct _REGION20W { w: &'static mut W, }
            impl _REGION20W { }
            pub struct REGION21W;
            impl REGION21W { }
            pub struct _REGION21W { w: &'static mut W, }
            impl _REGION21W { }
            pub struct REGION22W;
            impl REGION22W { }
            pub struct _REGION22W { w: &'static mut W, }
            impl _REGION22W { }
            pub struct REGION23W;
            impl REGION23W { }
            pub struct _REGION23W { w: &'static mut W, }
            impl _REGION23W { }
            pub struct REGION24W;
            impl REGION24W { }
            pub struct _REGION24W { w: &'static mut W, }
            impl _REGION24W { }
            pub struct REGION25W;
            impl REGION25W { }
            pub struct _REGION25W { w: &'static mut W, }
            impl _REGION25W { }
            pub struct REGION26W;
            impl REGION26W { }
            pub struct _REGION26W { w: &'static mut W, }
            impl _REGION26W { }
            pub struct REGION27W;
            impl REGION27W { }
            pub struct _REGION27W { w: &'static mut W, }
            impl _REGION27W { }
            pub struct REGION28W;
            impl REGION28W { }
            pub struct _REGION28W { w: &'static mut W, }
            impl _REGION28W { }
            pub struct REGION29W;
            impl REGION29W { }
            pub struct _REGION29W { w: &'static mut W, }
            impl _REGION29W { }
            pub struct REGION30W;
            impl REGION30W { }
            pub struct _REGION30W { w: &'static mut W, }
            impl _REGION30W { }
            pub struct REGION31W;
            impl REGION31W { }
            pub struct _REGION31W { w: &'static mut W, }
            impl _REGION31W { }
            impl R { }
            impl W { }
        }
        pub struct DISABLEINDEBUG;
        pub mod disableindebug {
            pub struct R;
            pub struct W;
            impl super::DISABLEINDEBUG { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct DISABLEINDEBUGR ;
            impl DISABLEINDEBUGR { }
            pub struct DISABLEINDEBUGW ;
            impl DISABLEINDEBUGW { }
            pub struct _DISABLEINDEBUGW {
                w: &'static mut W,
            }
            impl _DISABLEINDEBUGW { }
            impl R { }
            impl W { }
        }
    }
    pub struct TWI0;pub mod twi0 {
        #[repr(C)]
        pub struct PSEL;
        pub mod psel {
            pub struct SCL;
            pub mod scl {
                pub struct R;
                pub struct W;
                impl super::SCL { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }
            }
            pub struct SDA;
            pub mod sda {
                pub struct R;
                pub struct W;
                impl super::SDA { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }
            }
        }
        pub struct EVENTS_STOPPED;
        pub mod events_stopped {
            pub struct R;
            pub struct W;
            impl super::EVENTS_STOPPED { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_STOPPEDR;
            impl EVENTS_STOPPEDR { }
            pub struct EVENTS_STOPPEDW;
            impl EVENTS_STOPPEDW { }
            pub struct _EVENTS_STOPPEDW { w: &'static mut W, }
            impl _EVENTS_STOPPEDW { }
            impl R { }
            impl W { }
        }
        pub struct EVENTS_TXDSENT;
        pub mod events_txdsent {
            pub struct R;
            pub struct W;
            impl super::EVENTS_TXDSENT { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_TXDSENTR;
            impl EVENTS_TXDSENTR { }
            pub struct EVENTS_TXDSENTW;
            impl EVENTS_TXDSENTW { }
            pub struct _EVENTS_TXDSENTW { w: &'static mut W, }
            impl _EVENTS_TXDSENTW { }
            impl R { }
            impl W { }
        }
        pub struct EVENTS_BB;
        pub mod events_bb {
            pub struct R;
            pub struct W;
            impl super::EVENTS_BB { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_BBR;
            impl EVENTS_BBR { }
            pub struct EVENTS_BBW ;
            impl EVENTS_BBW { }
            pub struct _EVENTS_BBW { w: &'static mut W, }
            impl _EVENTS_BBW { }
            impl R { }
            impl W { }
        }
        pub struct SHORTS;
        pub mod shorts {
            pub struct R;
            pub struct W;
            impl super::SHORTS { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct BB_SUSPENDR;
            impl BB_SUSPENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct BB_STOPR;
            impl BB_STOPR { }
            pub struct BB_SUSPENDW;
            impl BB_SUSPENDW { }
            pub struct _BB_SUSPENDW { w: &'static mut W, }
            impl _BB_SUSPENDW { }
            pub struct BB_STOPW;
            impl BB_STOPW { }
            pub struct _BB_STOPW { w: &'static mut W, }
            impl _BB_STOPW { }
            impl R { }
            impl W { }
        }
        pub struct INTENCLR;
        pub mod intenclr {
            pub struct R;
            pub struct W;
            impl super::INTENCLR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct RXDREADYR;
            impl RXDREADYR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TXDSENTR;
            impl TXDSENTR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ERRORR;
            impl ERRORR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct BBR;
            impl BBR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SUSPENDEDR;
            impl SUSPENDEDR { }
            pub struct STOPPEDW;
            impl STOPPEDW { }
            pub struct _STOPPEDW { w: &'static mut W, }
            impl _STOPPEDW { }
            pub struct RXDREADYW;
            impl RXDREADYW { }
            pub struct _RXDREADYW { w: &'static mut W, }
            impl _RXDREADYW { }
            pub struct TXDSENTW;
            impl TXDSENTW { }
            pub struct _TXDSENTW { w: &'static mut W, }
            impl _TXDSENTW { }
            pub struct ERRORW;
            impl ERRORW { }
            pub struct _ERRORW { w: &'static mut W, }
            impl _ERRORW { }
            pub struct BBW;
            impl BBW { }
            pub struct _BBW { w: &'static mut W, }
            impl _BBW { }
            pub struct SUSPENDEDW;
            impl SUSPENDEDW { }
            pub struct _SUSPENDEDW { w: &'static mut W, }
            impl _SUSPENDEDW { }
            impl R { }
            impl W { }
        }
        pub struct ENABLE;
        pub mod enable {
            pub struct R;
            pub struct W;
            impl super::ENABLE { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENABLER;
            impl ENABLER { }
            pub struct ENABLEW;
            impl ENABLEW { }
            pub struct _ENABLEW { w: &'static mut W, }
            impl _ENABLEW { }
            impl R { }
            impl W { }
        }
        pub struct FREQUENCY;
        pub mod frequency {
            pub struct R;
            pub struct W;
            impl super::FREQUENCY { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct FREQUENCYR;
            impl FREQUENCYR { }
            pub struct FREQUENCYW;
            impl FREQUENCYW { }
            pub struct _FREQUENCYW { w: &'static mut W, }
            impl _FREQUENCYW { }
            impl R { }
            impl W { }
        }
    }
    pub struct TWIM0;pub mod twim0 {
        #[repr(C)]
        pub struct PSEL;
        pub mod psel {
            pub struct SCL;
            pub mod scl {
                pub struct R;
                pub struct W;
                impl super::SCL { } 
                pub struct PINR;
                impl PINR { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { } 
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { } 
                pub struct CONNECTW ;
                impl CONNECTW { } 
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { } 
                impl R { } 
                impl W { } 
            }
            pub struct SDA;
            pub mod sda {
                pub struct R;
                pub struct W;
                impl super::SDA { } 
                pub struct PINR;
                impl PINR { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { } 
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { } 
                pub struct CONNECTW ;
                impl CONNECTW { } 
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { } 
                impl R { } 
                impl W { } 
            }
        }
        #[repr(C)] pub struct TXD;
        pub mod txd {
            pub struct PTR;
            pub mod ptr {
                pub struct R;
                pub struct W;
                impl super::PTR { } 
                pub struct PTRR;
                impl PTRR { } 
                pub struct _PTRW { w: &'static mut W, }
                impl _PTRW { } 
                impl R { } 
                impl W { } 
            }
            pub struct MAXCNT;
            pub mod maxcnt {
                pub struct R;
                pub struct W;
                impl super::MAXCNT { } 
                pub struct MAXCNTR;
                impl MAXCNTR { } 
                pub struct _MAXCNTW { w: &'static mut W, }
                impl _MAXCNTW { } 
                impl R { } 
                impl W { } 
            }
            pub struct AMOUNT;
            pub mod amount {
                pub struct R;
                impl super::AMOUNT { } 
                pub struct AMOUNTR;
                impl AMOUNTR { } 
                impl R { } 
            }
            pub struct LIST;
            pub mod list {
                pub struct R;
                pub struct W;
                impl super::LIST { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct LISTR ;
                impl LISTR { } 
                pub struct LISTW ;
                impl LISTW { } 
                pub struct _LISTW { w: &'static mut W, }
                impl _LISTW { } 
                impl R { } 
                impl W { } 
            }
        }
        pub struct TASKS_SUSPEND;
        pub mod tasks_suspend {
            pub struct W;
            impl super::TASKS_SUSPEND { } 
            pub struct TASKS_SUSPENDW ;
            impl TASKS_SUSPENDW { } 
            pub struct _TASKS_SUSPENDW { w: &'static mut W, }
            impl _TASKS_SUSPENDW { } 
            impl W { } 
        }
        pub struct EVENTS_ERROR;
        pub mod events_error {
            pub struct R;
            pub struct W;
            impl super::EVENTS_ERROR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_ERRORR ;
            impl EVENTS_ERRORR { } 
            pub struct EVENTS_ERRORW ;
            impl EVENTS_ERRORW { } 
            pub struct _EVENTS_ERRORW { w: &'static mut W, }
            impl _EVENTS_ERRORW { } 
            impl R { } 
            impl W { } 
        }
        pub struct EVENTS_RXSTARTED;
        pub mod events_rxstarted {
            pub struct R;
            pub struct W;
            impl super::EVENTS_RXSTARTED { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_RXSTARTEDR ;
            impl EVENTS_RXSTARTEDR { } 
            pub struct EVENTS_RXSTARTEDW ;
            impl EVENTS_RXSTARTEDW { } 
            pub struct _EVENTS_RXSTARTEDW { w: &'static mut W, }
            impl _EVENTS_RXSTARTEDW { } 
            impl R { } 
            impl W { } 
        }
        pub struct EVENTS_LASTRX;
        pub mod events_lastrx {
            pub struct R;
            pub struct W;
            impl super::EVENTS_LASTRX { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_LASTRXR ;
            impl EVENTS_LASTRXR { } 
            pub struct EVENTS_LASTRXW ;
            impl EVENTS_LASTRXW { } 
            pub struct _EVENTS_LASTRXW { w: &'static mut W, }
            impl _EVENTS_LASTRXW { } 
            impl R { } 
            impl W { } 
        }
        pub struct SHORTS;
        pub mod shorts {
            pub struct R;
            pub struct W;
            impl super::SHORTS { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LASTTX_STARTRXR ;
            impl LASTTX_STARTRXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LASTTX_SUSPENDR ;
            impl LASTTX_SUSPENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LASTTX_STOPR ;
            impl LASTTX_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LASTRX_STARTTXR ;
            impl LASTRX_STARTTXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LASTRX_SUSPENDR ;
            impl LASTRX_SUSPENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LASTRX_STOPR ;
            impl LASTRX_STOPR { }
            pub struct LASTTX_STARTRXW ;
            impl LASTTX_STARTRXW { }
            pub struct _LASTTX_STARTRXW { w: &'static mut W, }
            impl _LASTTX_STARTRXW { }
            pub struct LASTTX_SUSPENDW ;
            impl LASTTX_SUSPENDW { }
            pub struct _LASTTX_SUSPENDW { w: &'static mut W, }
            impl _LASTTX_SUSPENDW { }
            pub struct LASTTX_STOPW ;
            impl LASTTX_STOPW { }
            pub struct _LASTTX_STOPW { w: &'static mut W, }
            impl _LASTTX_STOPW { }
            pub struct LASTRX_STARTTXW ;
            impl LASTRX_STARTTXW { }
            pub struct _LASTRX_STARTTXW { w: &'static mut W, }
            impl _LASTRX_STARTTXW { }
            pub struct LASTRX_SUSPENDW ;
            impl LASTRX_SUSPENDW { }
            pub struct _LASTRX_SUSPENDW { w: &'static mut W, }
            impl _LASTRX_SUSPENDW { }
            pub struct LASTRX_STOPW ;
            impl LASTRX_STOPW { }
            pub struct _LASTRX_STOPW { w: &'static mut W, }
            impl _LASTRX_STOPW { }
            impl R { }
            impl W { }
        }
        pub struct INTENSET;
        pub mod intenset {
            pub struct R;
            pub struct W;
            impl super::INTENSET { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct STOPPEDR ;
            impl STOPPEDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ERRORR ;
            impl ERRORR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SUSPENDEDR ;
            impl SUSPENDEDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct RXSTARTEDR ;
            impl RXSTARTEDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TXSTARTEDR ;
            impl TXSTARTEDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LASTRXR ;
            impl LASTRXR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LASTTXR ;
            impl LASTTXR { } 
            pub struct STOPPEDW ;
            impl STOPPEDW { } 
            pub struct _STOPPEDW { w: &'static mut W, }
            impl _STOPPEDW { } 
            pub struct ERRORW ;
            impl ERRORW { } 
            pub struct _ERRORW { w: &'static mut W, }
            impl _ERRORW { } 
            pub struct SUSPENDEDW ;
            impl SUSPENDEDW { } 
            pub struct _SUSPENDEDW { w: &'static mut W, }
            impl _SUSPENDEDW { } 
            pub struct RXSTARTEDW ;
            impl RXSTARTEDW { } 
            pub struct _RXSTARTEDW { w: &'static mut W, }
            impl _RXSTARTEDW { } 
            pub struct TXSTARTEDW ;
            impl TXSTARTEDW { } 
            pub struct _TXSTARTEDW { w: &'static mut W, }
            impl _TXSTARTEDW { } 
            pub struct LASTRXW ;
            impl LASTRXW { } 
            pub struct _LASTRXW { w: &'static mut W, }
            impl _LASTRXW { } 
            pub struct LASTTXW ;
            impl LASTTXW { } 
            pub struct _LASTTXW { w: &'static mut W, }
            impl _LASTTXW { } 
            impl R { } 
            impl W { } 
        }
        pub struct ERRORSRC;
        pub mod errorsrc {
            pub struct R;
            pub struct W;
            impl super::ERRORSRC { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct OVERRUNR ;
            impl OVERRUNR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ANACKR ;
            impl ANACKR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct DNACKR ;
            impl DNACKR { } 
            pub struct OVERRUNW ;
            impl OVERRUNW { } 
            pub struct _OVERRUNW { w: &'static mut W, }
            impl _OVERRUNW { } 
            pub struct ANACKW ;
            impl ANACKW { } 
            pub struct _ANACKW { w: &'static mut W, }
            impl _ANACKW { } 
            pub struct DNACKW ;
            impl DNACKW { } 
            pub struct _DNACKW { w: &'static mut W, }
            impl _DNACKW { } 
            impl R { } 
            impl W { } 
        }
        pub struct FREQUENCY;
        pub mod frequency {
            pub struct R;
            pub struct W;
            impl super::FREQUENCY { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct FREQUENCYR ;
            impl FREQUENCYR { }
            pub struct FREQUENCYW ;
            impl FREQUENCYW { }
            pub struct _FREQUENCYW { w: &'static mut W, }
            impl _FREQUENCYW { }
            impl R { }
            impl W { }
        }
    }
    pub struct TWIS0;pub mod twis0 {
        #[repr(C)]
        pub struct PSEL;
        pub mod psel {
            pub struct SCL;
            pub mod scl {
                pub struct R;
                pub struct W;
                impl super::SCL { } 
                pub struct PINR;
                impl PINR { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { } 
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { } 
                pub struct CONNECTW ;
                impl CONNECTW { } 
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { } 
                impl R { } 
                impl W { } 
            }
            pub struct SDA;
            pub mod sda {
                pub struct R;
                pub struct W;
                impl super::SDA { } 
                pub struct PINR;
                impl PINR { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { } 
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { } 
                pub struct CONNECTW ;
                impl CONNECTW { } 
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { } 
                impl R { } 
                impl W { } 
            }
        }
        #[repr(C)]
        pub struct TXD;
        pub mod txd {
            pub struct PTR;
            pub mod ptr{
                pub struct R;
                pub struct W;
                impl super::PTR { } 
                pub struct PTRR;
                impl PTRR { } 
                pub struct _PTRW { w: &'static mut W, }
                impl _PTRW { } 
                impl R { } 
                impl W { } 
            }
            pub struct MAXCNT;
            pub mod maxcnt {
                pub struct R;
                pub struct W;
                impl super::MAXCNT { } 
                pub struct MAXCNTR;
                impl MAXCNTR { } 
                pub struct _MAXCNTW { w: &'static mut W, }
                impl _MAXCNTW { } 
                impl R { } 
                impl W { } 
            }
            pub struct AMOUNT;
            pub mod amount {
                pub struct R;
                impl super::AMOUNT { } 
                pub struct AMOUNTR;
                impl AMOUNTR { } 
                impl R { } 
            }
            pub struct LIST;
            pub mod list {
                pub struct R;
                pub struct W;
                impl super::LIST { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct LISTR ;
                impl LISTR { } 
                pub struct LISTW ;
                impl LISTW { } 
                pub struct _LISTW { w: &'static mut W, }
                impl _LISTW { } 
                impl R { } 
                impl W { } 
            }
        }
        pub struct EVENTS_STOPPED;
        pub mod events_stopped {
            pub struct R;
            pub struct W;
            impl super::EVENTS_STOPPED { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_STOPPEDR ;
            impl EVENTS_STOPPEDR { } 
            pub struct EVENTS_STOPPEDW ;
            impl EVENTS_STOPPEDW { } 
            pub struct _EVENTS_STOPPEDW { w: &'static mut W, }
            impl _EVENTS_STOPPEDW { } 
            impl R { } 
            impl W { } 
        }
        pub struct EVENTS_RXSTARTED;
        pub mod events_rxstarted {
            pub struct R;
            pub struct W;
            impl super::EVENTS_RXSTARTED { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_RXSTARTEDR ;
            impl EVENTS_RXSTARTEDR { } 
            pub struct EVENTS_RXSTARTEDW ;
            impl EVENTS_RXSTARTEDW { } 
            pub struct _EVENTS_RXSTARTEDW { w: &'static mut W, }
            impl _EVENTS_RXSTARTEDW { } 
            impl R { } 
            impl W { } 
        }
        pub struct EVENTS_WRITE;
        pub mod events_write {
            pub struct R;
            pub struct W;
            impl super::EVENTS_WRITE { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_WRITER ;
            impl EVENTS_WRITER { } 
            pub struct EVENTS_WRITEW ;
            impl EVENTS_WRITEW { } 
            pub struct _EVENTS_WRITEW { w: &'static mut W, }
            impl _EVENTS_WRITEW { } 
            impl R { } 
            impl W { } 
        }
        pub struct SHORTS;
        pub mod shorts {
            pub struct R;
            pub struct W;
            impl super::SHORTS { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct WRITE_SUSPENDR ;
            impl WRITE_SUSPENDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct READ_SUSPENDR ;
            impl READ_SUSPENDR { } 
            pub struct WRITE_SUSPENDW ;
            impl WRITE_SUSPENDW { } 
            pub struct _WRITE_SUSPENDW { w: &'static mut W, }
            impl _WRITE_SUSPENDW { } 
            pub struct READ_SUSPENDW ;
            impl READ_SUSPENDW { } 
            pub struct _READ_SUSPENDW { w: &'static mut W, }
            impl _READ_SUSPENDW { } 
            impl R { } 
            impl W { } 
        }
        pub struct INTENSET;
        pub mod intenset {
            pub struct R;
            pub struct W;
            impl super::INTENSET { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct STOPPEDR ;
            impl STOPPEDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ERRORR ;
            impl ERRORR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct RXSTARTEDR ;
            impl RXSTARTEDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TXSTARTEDR ;
            impl TXSTARTEDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct WRITER ;
            impl WRITER { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct READR ;
            impl READR { } 
            pub struct STOPPEDW ;
            impl STOPPEDW { } 
            pub struct _STOPPEDW { w: &'static mut W, }
            impl _STOPPEDW { } 
            pub struct ERRORW ;
            impl ERRORW { } 
            pub struct _ERRORW { w: &'static mut W, }
            impl _ERRORW { } 
            pub struct RXSTARTEDW ;
            impl RXSTARTEDW { } 
            pub struct _RXSTARTEDW { w: &'static mut W, }
            impl _RXSTARTEDW { } 
            pub struct TXSTARTEDW ;
            impl TXSTARTEDW { } 
            pub struct _TXSTARTEDW { w: &'static mut W, }
            impl _TXSTARTEDW { } 
            pub struct WRITEW ;
            impl WRITEW { } 
            pub struct _WRITEW { w: &'static mut W, }
            impl _WRITEW { } 
            pub struct READW ;
            impl READW { } 
            pub struct _READW { w: &'static mut W, }
            impl _READW { } 
            impl R { } 
            impl W { } 
        }
        pub struct ERRORSRC;
        pub mod errorsrc {
            pub struct R;
            pub struct W;
            impl super::ERRORSRC { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct OVERFLOWR ;
            impl OVERFLOWR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct DNACKR ;
            impl DNACKR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct OVERREADR ;
            impl OVERREADR { } 
            pub struct OVERFLOWW ;
            impl OVERFLOWW { } 
            pub struct _OVERFLOWW { w: &'static mut W, }
            impl _OVERFLOWW { } 
            pub struct DNACKW ;
            impl DNACKW { } 
            pub struct _DNACKW { w: &'static mut W, }
            impl _DNACKW { } 
            pub struct OVERREADW ;
            impl OVERREADW { } 
            pub struct _OVERREADW { w: &'static mut W, }
            impl _OVERREADW { } 
            impl R { } 
            impl W { } 
        }
        pub struct ENABLE;
        pub mod enable {
            pub struct R;
            pub struct W;
            impl super::ENABLE { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENABLER ;
            impl ENABLER { } 
            pub struct ENABLEW ;
            impl ENABLEW { } 
            pub struct _ENABLEW { w: &'static mut W, }
            impl _ENABLEW { } 
            impl R { } 
            impl W { } 
        }
        pub struct CONFIG;
        pub mod config {
            pub struct R;
            pub struct W;
            impl super::CONFIG { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ADDRESS0R ;
            impl ADDRESS0R { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ADDRESS1R ;
            impl ADDRESS1R { } 
            pub struct ADDRESS0W ;
            impl ADDRESS0W { } 
            pub struct _ADDRESS0W { w: &'static mut W, }
            impl _ADDRESS0W { } 
            pub struct ADDRESS1W ;
            impl ADDRESS1W { } 
            pub struct _ADDRESS1W { w: &'static mut W, }
            impl _ADDRESS1W { } 
            impl R { } 
            impl W { } 
        }
    }
    pub struct SPI0;pub mod spi0 {
        #[repr(C)]
        pub struct PSEL;
        pub mod psel {
            pub struct SCK;
            pub mod sck {
                pub struct R;
                pub struct W;
                impl super::SCK { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }

            }
            pub struct MOSI;
            pub mod mosi {
                pub struct R;
                pub struct W;
                impl super::MOSI { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW ;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }
            }
            pub struct MISO;
            pub mod miso {
                pub struct R;
                pub struct W;
                impl super::MISO { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }
            }
        }
        pub struct INTENSET;
        pub mod intenset {
            pub struct R;
            pub struct W;
            impl super::INTENSET { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct READYR;
            impl READYR { }
            pub struct READYW;
            impl READYW { }
            pub struct _READYW { w: &'static mut W, }
            impl _READYW { }
            impl R { }
            impl W { }
        }
        pub struct ENABLE;
        pub mod enable {
            pub struct R;
            pub struct W;
            impl super::ENABLE { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENABLER;
            impl ENABLER { }
            pub struct ENABLEW;
            impl ENABLEW { }
            pub struct _ENABLEW { w: &'static mut W, }
            impl _ENABLEW { }
            impl R { }
            impl W { }
        }
        pub struct CONFIG;
        pub mod config {
            pub struct R;
            pub struct W;
            impl super::CONFIG { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ORDERR;
            impl ORDERR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct CPHAR;
            impl CPHAR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct CPOLR;
            impl CPOLR { }
            pub struct ORDERW;
            impl ORDERW { }
            pub struct _ORDERW { w: &'static mut W, }
            impl _ORDERW { }
            pub struct CPHAW;
            impl CPHAW { }
            pub struct _CPHAW { w: &'static mut W, }
            impl _CPHAW { }
            pub struct CPOLW;
            impl CPOLW { }
            pub struct _CPOLW { w: &'static mut W, }
            impl _CPOLW { }
            impl R { }
            impl W { }
        }
    }
    pub struct SPIM0;pub mod spim0 {
        #[repr(C)]
        pub struct PSEL;
        pub mod psel {
            pub struct SCK;
            pub mod sck {
                pub struct R;
                pub struct W;
                impl super::SCK { } 
                pub struct PINR;
                impl PINR { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { } 
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { } 
                pub struct CONNECTW ;
                impl CONNECTW { } 
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { } 
                impl R { } 
                impl W { } 
            }
            pub struct MOSI;
            pub mod mosi {
                pub struct R;
                pub struct W;
                impl super::MOSI { } 
                pub struct PINR;
                impl PINR { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { } 
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { } 
                pub struct CONNECTW ;
                impl CONNECTW { } 
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { } 
                impl R { } 
                impl W { } 
            }
            pub struct MISO;
            pub mod miso {
                pub struct R;
                pub struct W;
                impl super::MISO { } 
                pub struct PINR;
                impl PINR { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { } 
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { } 
                pub struct CONNECTW ;
                impl CONNECTW { } 
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { } 
                impl R { } 
                impl W { } 
            }
        }
        #[repr(C)]
        pub struct TXD;
        pub mod txd {
            pub struct PTR;
            pub mod ptr {
                pub struct R;
                pub struct W;
                impl super::PTR { } 
                pub struct PTRR;
                impl PTRR { } 
                pub struct _PTRW { w: &'static mut W, }
                impl _PTRW { } 
                impl R { } 
                impl W { } 
            }
            pub struct MAXCNT;
            pub mod maxcnt {
                pub struct R;
                pub struct W;
                impl super::MAXCNT { } 
                pub struct MAXCNTR;
                impl MAXCNTR { } 
                pub struct _MAXCNTW { w: &'static mut W, }
                impl _MAXCNTW { } 
                impl R { } 
                impl W { } 
            }
            pub struct AMOUNT;
            pub mod amount {
                pub struct R;
                impl super::AMOUNT { } 
                pub struct AMOUNTR;
                impl AMOUNTR { } 
                impl R { } 
            }
            pub struct LIST;
            pub mod list {
                pub struct R;
                pub struct W;
                impl super::LIST { } 
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct LISTR ;
                impl LISTR { } 
                pub struct LISTW ;
                impl LISTW { } 
                pub struct _LISTW { w: &'static mut W, }
                impl _LISTW { } 
                impl R { } 
                impl W { } 
            }
        }
        pub struct TASKS_RESUME;
        pub mod tasks_resume {
            pub struct W;
            impl super::TASKS_RESUME { } 
            pub struct TASKS_RESUMEW ;
            impl TASKS_RESUMEW { } 
            pub struct _TASKS_RESUMEW { w: &'static mut W, }
            impl _TASKS_RESUMEW { } 
            impl W { } 
        }
        pub struct EVENTS_STOPPED;
        pub mod events_stopped {
            pub struct R;
            pub struct W;
            impl super::EVENTS_STOPPED { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_STOPPEDR ;
            impl EVENTS_STOPPEDR { } 
            pub struct EVENTS_STOPPEDW ;
            impl EVENTS_STOPPEDW { } 
            pub struct _EVENTS_STOPPEDW { w: &'static mut W, }
            impl _EVENTS_STOPPEDW { } 
            impl R { } 
            impl W { } 
        }
        pub struct EVENTS_END;
        pub mod events_end {
            pub struct R;
            pub struct W;
            impl super::EVENTS_END { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_ENDR ;
            impl EVENTS_ENDR { } 
            pub struct EVENTS_ENDW ;
            impl EVENTS_ENDW { } 
            pub struct _EVENTS_ENDW { w: &'static mut W, }
            impl _EVENTS_ENDW { } 
            impl R { } 
            impl W { } 
        }
        pub struct EVENTS_STARTED;
        pub mod events_started {
            pub struct R;
            pub struct W;
            impl super::EVENTS_STARTED { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_STARTEDR ;
            impl EVENTS_STARTEDR { } 
            pub struct EVENTS_STARTEDW ;
            impl EVENTS_STARTEDW { } 
            pub struct _EVENTS_STARTEDW { w: &'static mut W, }
            impl _EVENTS_STARTEDW { } 
            impl R { } 
            impl W { } 
        }
        pub struct INTENSET;
        pub mod intenset {
            pub struct R;
            pub struct W;
            impl super::INTENSET { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct STOPPEDR ;
            impl STOPPEDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENDRXR ;
            impl ENDRXR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENDR ;
            impl ENDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENDTXR ;
            impl ENDTXR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct STARTEDR ;
            impl STARTEDR { } 
            pub struct STOPPEDW ;
            impl STOPPEDW { } 
            pub struct _STOPPEDW { w: &'static mut W, }
            impl _STOPPEDW { } 
            pub struct ENDRXW ;
            impl ENDRXW { } 
            pub struct _ENDRXW { w: &'static mut W, }
            impl _ENDRXW { } 
            pub struct ENDW ;
            impl ENDW { } 
            pub struct _ENDW { w: &'static mut W, }
            impl _ENDW { } 
            pub struct ENDTXW ;
            impl ENDTXW { } 
            pub struct _ENDTXW { w: &'static mut W, }
            impl _ENDTXW { } 
            pub struct STARTEDW ;
            impl STARTEDW { } 
            pub struct _STARTEDW { w: &'static mut W, }
            impl _STARTEDW { } 
            impl R { } 
            impl W { } 
        }
        pub struct INTENCLR;
        pub mod intenclr {
            pub struct R;
            pub struct W;
            impl super::INTENCLR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct STOPPEDR ;
            impl STOPPEDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENDRXR ;
            impl ENDRXR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENDR ;
            impl ENDR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENDTXR ;
            impl ENDTXR { } 
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct STARTEDR ;
            impl STARTEDR { } 
            pub struct STOPPEDW ;
            impl STOPPEDW { } 
            pub struct _STOPPEDW { w: &'static mut W, }
            impl _STOPPEDW { } 
            pub struct ENDRXW ;
            impl ENDRXW { } 
            pub struct _ENDRXW { w: &'static mut W, }
            impl _ENDRXW { } 
            pub struct ENDW ;
            impl ENDW { } 
            pub struct _ENDW { w: &'static mut W, }
            impl _ENDW { } 
            pub struct ENDTXW ;
            impl ENDTXW { } 
            pub struct _ENDTXW { w: &'static mut W, }
            impl _ENDTXW { } 
            pub struct STARTEDW ;
            impl STARTEDW { } 
            pub struct _STARTEDW { w: &'static mut W, }
            impl _STARTEDW { } 
            impl R { } 
            impl W { } 
        }
        pub struct FREQUENCY;
        pub mod frequency {
            pub struct R;
            pub struct W;
            impl super::FREQUENCY { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct FREQUENCYR ;
            impl FREQUENCYR { }
            pub struct FREQUENCYW ;
            impl FREQUENCYW { }
            pub struct _FREQUENCYW { w: &'static mut W, }
            impl _FREQUENCYW { }
            impl R { }
            impl W { }
        }
    }
    pub struct SPIS0;pub mod spis0 {
        #[repr(C)]
        pub struct PSEL;
        pub mod psel {
            pub struct SCK;
            pub mod sck {
                pub struct R;
                pub struct W;
                impl super::SCK { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW ;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }
            }
            pub struct MISO;
            pub mod miso {
                pub struct R;
                pub struct W;
                impl super::MISO { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW ;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }
            }
            pub struct MOSI;
            pub mod mosi {
                pub struct R;
                pub struct W;
                impl super::MOSI { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW ;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }
            }
            pub struct CSN;
            pub mod csn {
                pub struct R;
                pub struct W;
                impl super::CSN { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR ;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW ;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }
            }
        }
        #[repr(C)]
        pub struct TXD;
        pub mod txd {
            pub struct PTR;
            pub mod ptr {
                pub struct R;
                pub struct W;
                impl super::PTR { }
                pub struct PTRR;
                impl PTRR { }
                pub struct _PTRW { w: &'static mut W, }
                impl _PTRW { }
                impl R { }
                impl W { }
            }
            pub struct MAXCNT;
            pub mod maxcnt {
                pub struct R;
                pub struct W;
                impl super::MAXCNT { }
                pub struct MAXCNTR;
                impl MAXCNTR { }
                pub struct _MAXCNTW { w: &'static mut W, }
                impl _MAXCNTW { }
                impl R { }
                impl W { }
            }
            pub struct AMOUNT;
            pub mod amount {
                pub struct R;
                impl super::AMOUNT { }
                pub struct AMOUNTR;
                impl AMOUNTR { }
                impl R { }
            }
            pub struct LIST;
            pub mod list {
                pub struct R;
                pub struct W;
                impl super::LIST { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct LISTR ;
                impl LISTR { }
                pub struct LISTW ;
                impl LISTW { }
                pub struct _LISTW { w: &'static mut W, }
                impl _LISTW { }
                impl R { }
                impl W { }
            }
        }
        pub struct EVENTS_ENDRX;
        pub mod events_endrx {
            pub struct R;
            pub struct W;
            impl super::EVENTS_ENDRX { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_ENDRXR ;
            impl EVENTS_ENDRXR { }
            pub struct EVENTS_ENDRXW ;
            impl EVENTS_ENDRXW { }
            pub struct _EVENTS_ENDRXW { w: &'static mut W, }
            impl _EVENTS_ENDRXW { }
            impl R { }
            impl W { }
        }
        pub struct SHORTS;
        pub mod shorts {
            pub struct R;
            pub struct W;
            impl super::SHORTS { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct END_ACQUIRER ;
            impl END_ACQUIRER { }
            pub struct END_ACQUIREW ;
            impl END_ACQUIREW { }
            pub struct _END_ACQUIREW { w: &'static mut W, }
            impl _END_ACQUIREW { }
            impl R { }
            impl W { }
        }
        pub struct INTENCLR;
        pub mod intenclr {
            pub struct R;
            pub struct W;
            impl super::INTENCLR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENDR ;
            impl ENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ENDRXR ;
            impl ENDRXR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ACQUIREDR ;
            impl ACQUIREDR { }
            pub struct ENDW ;
            impl ENDW { }
            pub struct _ENDW { w: &'static mut W, }
            impl _ENDW { }
            pub struct ENDRXW ;
            impl ENDRXW { }
            pub struct _ENDRXW { w: &'static mut W, }
            impl _ENDRXW { }
            pub struct ACQUIREDW ;
            impl ACQUIREDW { }
            pub struct _ACQUIREDW { w: &'static mut W, }
            impl _ACQUIREDW { }
            impl R { }
            impl W { }
        }
        pub struct STATUS;
        pub mod status {
            pub struct R;
            pub struct W;
            impl super::STATUS { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct OVERREADR ;
            impl OVERREADR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct OVERFLOWR ;
            impl OVERFLOWR { }
            pub struct OVERREADW ;
            impl OVERREADW { }
            pub struct _OVERREADW { w: &'static mut W, }
            impl _OVERREADW { }
            pub struct OVERFLOWW ;
            impl OVERFLOWW { }
            pub struct _OVERFLOWW { w: &'static mut W, }
            impl _OVERFLOWW { }
            impl R { }
            impl W { }
        }
        pub struct CONFIG;
        pub mod config {
            pub struct R;
            pub struct W;
            impl super::CONFIG { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ORDERR;
            impl ORDERR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct CPHAR;
            impl CPHAR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct CPOLR ;
            impl CPOLR { }
            pub struct ORDERW ;
            impl ORDERW { }
            pub struct _ORDERW { w: &'static mut W, }
            impl _ORDERW { }
            pub struct CPHAW ;
            impl CPHAW { }
            pub struct _CPHAW { w: &'static mut W, }
            impl _CPHAW { }
            pub struct CPOLW ;
            impl CPOLW { }
            pub struct _CPOLW { w: &'static mut W, }
            impl _CPOLW { }
            impl R { }
            impl W { }
        }
    }
    pub struct TIMER0;pub mod timer0 {
        pub struct SHORTS;
        pub mod shorts {
            pub struct R;
            pub struct W;
            impl super::SHORTS { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE0_CLEARR;
            impl COMPARE0_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE1_CLEARR;
            impl COMPARE1_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE2_CLEARR;
            impl COMPARE2_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE3_CLEARR;
            impl COMPARE3_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE4_CLEARR;
            impl COMPARE4_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE5_CLEARR;
            impl COMPARE5_CLEARR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE0_STOPR;
            impl COMPARE0_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE1_STOPR;
            impl COMPARE1_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE2_STOPR;
            impl COMPARE2_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE3_STOPR;
            impl COMPARE3_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE4_STOPR;
            impl COMPARE4_STOPR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE5_STOPR;
            impl COMPARE5_STOPR { }
            pub struct COMPARE0_CLEARW;
            impl COMPARE0_CLEARW { }
            pub struct _COMPARE0_CLEARW { w: &'static mut W, }
            impl _COMPARE0_CLEARW { }
            pub struct COMPARE1_CLEARW;
            impl COMPARE1_CLEARW { }
            pub struct _COMPARE1_CLEARW { w: &'static mut W, }
            impl _COMPARE1_CLEARW { }
            pub struct COMPARE2_CLEARW;
            impl COMPARE2_CLEARW { }
            pub struct _COMPARE2_CLEARW { w: &'static mut W, }
            impl _COMPARE2_CLEARW { }
            pub struct COMPARE3_CLEARW;
            impl COMPARE3_CLEARW { }
            pub struct _COMPARE3_CLEARW { w: &'static mut W, }
            impl _COMPARE3_CLEARW { }
            pub struct COMPARE4_CLEARW;
            impl COMPARE4_CLEARW { }
            pub struct _COMPARE4_CLEARW { w: &'static mut W, }
            impl _COMPARE4_CLEARW { }
            pub struct COMPARE5_CLEARW;
            impl COMPARE5_CLEARW { }
            pub struct _COMPARE5_CLEARW { w: &'static mut W, }
            impl _COMPARE5_CLEARW { }
            pub struct COMPARE0_STOPW;
            impl COMPARE0_STOPW { }
            pub struct _COMPARE0_STOPW { w: &'static mut W, }
            impl _COMPARE0_STOPW { }
            pub struct COMPARE1_STOPW;
            impl COMPARE1_STOPW { }
            pub struct _COMPARE1_STOPW { w: &'static mut W, }
            impl _COMPARE1_STOPW { }
            pub struct COMPARE2_STOPW;
            impl COMPARE2_STOPW { }
            pub struct _COMPARE2_STOPW { w: &'static mut W, }
            impl _COMPARE2_STOPW { }
            pub struct COMPARE3_STOPW ;
            impl COMPARE3_STOPW { }
            pub struct _COMPARE3_STOPW { w: &'static mut W, }
            impl _COMPARE3_STOPW { }
            pub struct COMPARE4_STOPW;
            impl COMPARE4_STOPW { }
            pub struct _COMPARE4_STOPW { w: &'static mut W, }
            impl _COMPARE4_STOPW { }
            pub struct COMPARE5_STOPW;
            impl COMPARE5_STOPW { }
            pub struct _COMPARE5_STOPW { w: &'static mut W, }
            impl _COMPARE5_STOPW { }
            impl R { }
            impl W { }
        }
        pub struct INTENCLR;
        pub mod intenclr {
            pub struct R;
            pub struct W;
            impl super::INTENCLR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE0R;
            impl COMPARE0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE1R;
            impl COMPARE1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE2R;
            impl COMPARE2R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE3R;
            impl COMPARE3R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE4R;
            impl COMPARE4R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct COMPARE5R;
            impl COMPARE5R { }
            pub struct COMPARE0W;
            impl COMPARE0W { }
            pub struct _COMPARE0W { w: &'static mut W, }
            impl _COMPARE0W { }
            pub struct COMPARE1W;
            impl COMPARE1W { }
            pub struct _COMPARE1W { w: &'static mut W, }
            impl _COMPARE1W { }
            pub struct COMPARE2W;
            impl COMPARE2W { }
            pub struct _COMPARE2W { w: &'static mut W, }
            impl _COMPARE2W { }
            pub struct COMPARE3W;
            impl COMPARE3W { }
            pub struct _COMPARE3W { w: &'static mut W, }
            impl _COMPARE3W { }
            pub struct COMPARE4W;
            impl COMPARE4W { }
            pub struct _COMPARE4W { w: &'static mut W, }
            impl _COMPARE4W { }
            pub struct COMPARE5W;
            impl COMPARE5W { }
            pub struct _COMPARE5W { w: &'static mut W, }
            impl _COMPARE5W { }
            impl R { }
            impl W { }
        }
        pub struct BITMODE;
        pub mod bitmode {
            pub struct R;
            pub struct W;
            impl super::BITMODE { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct BITMODER;
            impl BITMODER { }
            pub struct BITMODEW;
            impl BITMODEW { }
            pub struct _BITMODEW { w: &'static mut W, }
            impl _BITMODEW { }
            impl R { }
            impl W { }
        }
    }
    pub struct TEMP;pub mod temp {
        pub struct EVENTS_DATARDY;
        pub mod events_datardy {
            pub struct R;
            pub struct W;
            impl super::EVENTS_DATARDY { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_DATARDYR;
            impl EVENTS_DATARDYR { }
            pub struct EVENTS_DATARDYW;
            impl EVENTS_DATARDYW { }
            pub struct _EVENTS_DATARDYW { w: &'static mut W, }
            impl _EVENTS_DATARDYW { }
            impl R { }
            impl W { }
        }
        pub struct INTENSET;
        pub mod intenset {
            pub struct R;
            pub struct W;
            impl super::INTENSET { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct DATARDYR;
            impl DATARDYR { }
            pub struct DATARDYW;
            impl DATARDYW { }
            pub struct _DATARDYW { w: &'static mut W, }
            impl _DATARDYW { }
            impl R { }
            impl W { }
        }
    }
    pub struct RNG; pub mod rng {
        pub struct EVENTS_VALRDY;
        pub mod events_valrdy {
            pub struct R;
            pub struct W;
            impl super::EVENTS_VALRDY { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_VALRDYR;
            impl EVENTS_VALRDYR { }
            pub struct EVENTS_VALRDYW;
            impl EVENTS_VALRDYW { }
            pub struct _EVENTS_VALRDYW { w: &'static mut W, }
            impl _EVENTS_VALRDYW { }
            impl R { }
            impl W { }
        }
        pub struct INTENSET;
        pub mod intenset {
            pub struct R;
            pub struct W;
            impl super::INTENSET { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct VALRDYR;
            impl VALRDYR { }
            pub struct VALRDYW;
            impl VALRDYW { }
            pub struct _VALRDYW { w: &'static mut W, }
            impl _VALRDYW { }
            impl R { }
            impl W { }
        }
        pub struct CONFIG;
        pub mod config {
            pub struct R;
            pub struct W;
            impl super::CONFIG { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct DERCENR;
            impl DERCENR { }
            pub struct DERCENW;
            impl DERCENW { }
            pub struct _DERCENW { w: &'static mut W, }
            impl _DERCENW { }
            impl R { }
            impl W { }
        }
    }
    pub struct EGU0;pub mod egu0 {
        pub struct INTEN;
        pub mod inten {
            pub struct R;
            pub struct W;
            impl super::INTEN { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED0R;
            impl TRIGGERED0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED1R;
            impl TRIGGERED1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED2R;
            impl TRIGGERED2R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED3R;
            impl TRIGGERED3R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED4R;
            impl TRIGGERED4R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED5R;
            impl TRIGGERED5R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED6R;
            impl TRIGGERED6R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED7R;
            impl TRIGGERED7R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED8R;
            impl TRIGGERED8R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED9R;
            impl TRIGGERED9R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED10R;
            impl TRIGGERED10R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED11R;
            impl TRIGGERED11R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED12R;
            impl TRIGGERED12R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED13R;
            impl TRIGGERED13R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED14R;
            impl TRIGGERED14R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED15R;
            impl TRIGGERED15R { }
            pub struct TRIGGERED0W;
            impl TRIGGERED0W { }
            pub struct _TRIGGERED0W { w: &'static mut W, }
            impl _TRIGGERED0W { }
            pub struct TRIGGERED1W;
            impl TRIGGERED1W { }
            pub struct _TRIGGERED1W { w: &'static mut W, }
            impl _TRIGGERED1W { }
            pub struct TRIGGERED2W;
            impl TRIGGERED2W { }
            pub struct _TRIGGERED2W { w: &'static mut W, }
            impl _TRIGGERED2W { }
            pub struct TRIGGERED3W;
            impl TRIGGERED3W { }
            pub struct _TRIGGERED3W { w: &'static mut W, }
            impl _TRIGGERED3W { }
            pub struct TRIGGERED4W;
            impl TRIGGERED4W { }
            pub struct _TRIGGERED4W { w: &'static mut W, }
            impl _TRIGGERED4W { }
            pub struct TRIGGERED5W;
            impl TRIGGERED5W { }
            pub struct _TRIGGERED5W { w: &'static mut W, }
            impl _TRIGGERED5W { }
            pub struct TRIGGERED6W;
            impl TRIGGERED6W { }
            pub struct _TRIGGERED6W { w: &'static mut W, }
            impl _TRIGGERED6W { }
            pub struct TRIGGERED7W;
            impl TRIGGERED7W { }
            pub struct _TRIGGERED7W { w: &'static mut W, }
            impl _TRIGGERED7W { }
            pub struct TRIGGERED8W;
            impl TRIGGERED8W { }
            pub struct _TRIGGERED8W { w: &'static mut W, }
            impl _TRIGGERED8W { }
            pub struct TRIGGERED9W;
            impl TRIGGERED9W { }
            pub struct _TRIGGERED9W { w: &'static mut W, }
            impl _TRIGGERED9W { }
            pub struct TRIGGERED10W;
            impl TRIGGERED10W { }
            pub struct _TRIGGERED10W { w: &'static mut W, }
            impl _TRIGGERED10W { }
            pub struct TRIGGERED11W;
            impl TRIGGERED11W { }
            pub struct _TRIGGERED11W { w: &'static mut W, }
            impl _TRIGGERED11W { }
            pub struct TRIGGERED12W;
            impl TRIGGERED12W { }
            pub struct _TRIGGERED12W { w: &'static mut W, }
            impl _TRIGGERED12W { }
            pub struct TRIGGERED13W;
            impl TRIGGERED13W { }
            pub struct _TRIGGERED13W { w: &'static mut W, }
            impl _TRIGGERED13W { }
            pub struct TRIGGERED14W;
            impl TRIGGERED14W { }
            pub struct _TRIGGERED14W { w: &'static mut W, }
            impl _TRIGGERED14W { }
            pub struct TRIGGERED15W;
            impl TRIGGERED15W { }
            pub struct _TRIGGERED15W { w: &'static mut W, }
            impl _TRIGGERED15W { }
            impl R { }
            impl W { }
        }
        pub struct INTENCLR;
        pub mod intenclr {
            pub struct R;
            pub struct W;
            impl super::INTENCLR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED0R ;
            impl TRIGGERED0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED1R ;
            impl TRIGGERED1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED2R ;
            impl TRIGGERED2R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED3R ;
            impl TRIGGERED3R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED4R ;
            impl TRIGGERED4R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED5R ;
            impl TRIGGERED5R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED6R ;
            impl TRIGGERED6R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED7R ;
            impl TRIGGERED7R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED8R ;
            impl TRIGGERED8R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED9R ;
            impl TRIGGERED9R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED10R ;
            impl TRIGGERED10R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED11R ;
            impl TRIGGERED11R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED12R ;
            impl TRIGGERED12R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED13R ;
            impl TRIGGERED13R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED14R ;
            impl TRIGGERED14R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct TRIGGERED15R ;
            impl TRIGGERED15R { }
            pub struct TRIGGERED0W ;
            impl TRIGGERED0W { }
            pub struct _TRIGGERED0W { w: &'static mut W, }
            impl _TRIGGERED0W { }
            pub struct TRIGGERED1W ;
            impl TRIGGERED1W { }
            pub struct _TRIGGERED1W { w: &'static mut W, }
            impl _TRIGGERED1W { }
            pub struct TRIGGERED2W ;
            impl TRIGGERED2W { }
            pub struct _TRIGGERED2W { w: &'static mut W, }
            impl _TRIGGERED2W { }
            pub struct TRIGGERED3W ;
            impl TRIGGERED3W { }
            pub struct _TRIGGERED3W { w: &'static mut W, }
            impl _TRIGGERED3W { }
            pub struct TRIGGERED4W ;
            impl TRIGGERED4W { }
            pub struct _TRIGGERED4W { w: &'static mut W, }
            impl _TRIGGERED4W { }
            pub struct TRIGGERED5W ;
            impl TRIGGERED5W { }
            pub struct _TRIGGERED5W { w: &'static mut W, }
            impl _TRIGGERED5W { }
            pub struct TRIGGERED6W ;
            impl TRIGGERED6W { }
            pub struct _TRIGGERED6W { w: &'static mut W, }
            impl _TRIGGERED6W { }
            pub struct TRIGGERED7W ;
            impl TRIGGERED7W { }
            pub struct _TRIGGERED7W { w: &'static mut W, }
            impl _TRIGGERED7W { }
            pub struct TRIGGERED8W ;
            impl TRIGGERED8W { }
            pub struct _TRIGGERED8W { w: &'static mut W, }
            impl _TRIGGERED8W { }
            pub struct TRIGGERED9W ;
            impl TRIGGERED9W { }
            pub struct _TRIGGERED9W { w: &'static mut W, }
            impl _TRIGGERED9W { }
            pub struct TRIGGERED10W ;
            impl TRIGGERED10W { }
            pub struct _TRIGGERED10W { w: &'static mut W, }
            impl _TRIGGERED10W { }
            pub struct TRIGGERED11W ;
            impl TRIGGERED11W { }
            pub struct _TRIGGERED11W { w: &'static mut W, }
            impl _TRIGGERED11W { }
            pub struct TRIGGERED12W ;
            impl TRIGGERED12W { }
            pub struct _TRIGGERED12W { w: &'static mut W, }
            impl _TRIGGERED12W { }
            pub struct TRIGGERED13W ;
            impl TRIGGERED13W { }
            pub struct _TRIGGERED13W { w: &'static mut W, }
            impl _TRIGGERED13W { }
            pub struct TRIGGERED14W ;
            impl TRIGGERED14W { }
            pub struct _TRIGGERED14W { w: &'static mut W, }
            impl _TRIGGERED14W { }
            pub struct TRIGGERED15W ;
            impl TRIGGERED15W { }
            pub struct _TRIGGERED15W { w: &'static mut W, }
            impl _TRIGGERED15W { }
            impl R { }
            impl W { }
        }
    }
    pub struct PWM0;pub mod pwm0 {
        #[repr(C)]
        pub struct PSEL;
        pub mod psel {
            pub struct OUT;
            pub mod out {
                pub struct R;
                pub struct W;
                impl super::OUT { }
                pub struct PINR;
                impl PINR { }
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct CONNECTR;
                impl CONNECTR { }
                pub struct _PINW { w: &'static mut W, }
                impl _PINW { }
                pub struct CONNECTW ;
                impl CONNECTW { }
                pub struct _CONNECTW { w: &'static mut W, }
                impl _CONNECTW { }
                impl R { }
                impl W { }

            }
        }
        pub struct TASKS_SEQSTART;
        pub mod tasks_seqstart {
            pub struct W;
            impl super::TASKS_SEQSTART { }
            pub struct TASKS_SEQSTARTW;
            impl TASKS_SEQSTARTW { }
            pub struct _TASKS_SEQSTARTW { w: &'static mut W, }
            impl _TASKS_SEQSTARTW { }
            impl W { }
        }
        pub struct TASKS_NEXTSTEP;
        pub mod tasks_nextstep {
            pub struct W;
            impl super::TASKS_NEXTSTEP { }
            pub struct TASKS_NEXTSTEPW;
            impl TASKS_NEXTSTEPW { }
            pub struct _TASKS_NEXTSTEPW { w: &'static mut W, }
            impl _TASKS_NEXTSTEPW { }
            impl W { }
        }
        pub struct EVENTS_STOPPED;
        pub mod events_stopped {
            pub struct R;
            pub struct W;
            impl super::EVENTS_STOPPED { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_STOPPEDR;
            impl EVENTS_STOPPEDR { }
            pub struct EVENTS_STOPPEDW;
            impl EVENTS_STOPPEDW { }
            pub struct _EVENTS_STOPPEDW { w: &'static mut W, }
            impl _EVENTS_STOPPEDW { }
            impl R { }
            impl W { }
        }
        pub struct EVENTS_SEQSTARTED;
        pub mod events_seqstarted {
            pub struct R;
            pub struct W;
            impl super::EVENTS_SEQSTARTED { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_SEQSTARTEDR;
            impl EVENTS_SEQSTARTEDR { }
            pub struct EVENTS_SEQSTARTEDW;
            impl EVENTS_SEQSTARTEDW { }
            pub struct _EVENTS_SEQSTARTEDW { w: &'static mut W, }
            impl _EVENTS_SEQSTARTEDW { }
            impl R { }
            impl W { }
        }
        pub struct EVENTS_SEQEND;
        pub mod events_seqend {
            pub struct R;
            pub struct W;
            impl super::EVENTS_SEQEND { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_SEQENDR;
            impl EVENTS_SEQENDR { }
            pub struct EVENTS_SEQENDW;
            impl EVENTS_SEQENDW { }
            pub struct _EVENTS_SEQENDW { w: &'static mut W, }
            impl _EVENTS_SEQENDW { }
            impl R { }
            impl W { }
        }
        pub struct EVENTS_LOOPSDONE;
        pub mod events_loopsdone {
            pub struct R;
            pub struct W;
            impl super::EVENTS_LOOPSDONE { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct EVENTS_LOOPSDONER;
            impl EVENTS_LOOPSDONER { }
            pub struct EVENTS_LOOPSDONEW;
            impl EVENTS_LOOPSDONEW { }
            pub struct _EVENTS_LOOPSDONEW { w: &'static mut W, }
            impl _EVENTS_LOOPSDONEW { }
            impl R { }
            impl W { }
        }
        pub struct INTEN;
        pub mod inten {
            pub struct R;
            pub struct W;
            impl super::INTEN { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SEQSTARTED0R;
            impl SEQSTARTED0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SEQSTARTED1R;
            impl SEQSTARTED1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SEQEND0R;
            impl SEQEND0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SEQEND1R;
            impl SEQEND1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct PWMPERIODENDR;
            impl PWMPERIODENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LOOPSDONER;
            impl LOOPSDONER { }
            pub struct STOPPEDW;
            impl STOPPEDW { }
            pub struct _STOPPEDW { w: &'static mut W, }
            impl _STOPPEDW { }
            pub struct SEQSTARTED0W;
            impl SEQSTARTED0W { }
            pub struct _SEQSTARTED0W { w: &'static mut W, }
            impl _SEQSTARTED0W { }
            pub struct SEQSTARTED1W;
            impl SEQSTARTED1W { }
            pub struct _SEQSTARTED1W { w: &'static mut W, }
            impl _SEQSTARTED1W { }
            pub struct SEQEND0W;
            impl SEQEND0W { }
            pub struct _SEQEND0W { w: &'static mut W, }
            impl _SEQEND0W { }
            pub struct SEQEND1W;
            impl SEQEND1W { }
            pub struct _SEQEND1W { w: &'static mut W, }
            impl _SEQEND1W { }
            pub struct PWMPERIODENDW;
            impl PWMPERIODENDW { }
            pub struct _PWMPERIODENDW { w: &'static mut W, }
            impl _PWMPERIODENDW { }
            pub struct LOOPSDONEW;
            impl LOOPSDONEW { }
            pub struct _LOOPSDONEW { w: &'static mut W, }
            impl _LOOPSDONEW { }
            impl R { }
            impl W { }
        }
        pub struct INTENCLR;
        pub mod intenclr {
            pub struct R;
            pub struct W;
            impl super::INTENCLR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct STOPPEDR;
            impl STOPPEDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SEQSTARTED0R;
            impl SEQSTARTED0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SEQSTARTED1R;
            impl SEQSTARTED1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SEQEND0R;
            impl SEQEND0R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct SEQEND1R;
            impl SEQEND1R { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct PWMPERIODENDR;
            impl PWMPERIODENDR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct LOOPSDONER;
            impl LOOPSDONER { }
            pub struct STOPPEDW;
            impl STOPPEDW { }
            pub struct _STOPPEDW { w: &'static mut W, }
            impl _STOPPEDW { }
            pub struct SEQSTARTED0W;
            impl SEQSTARTED0W { }
            pub struct _SEQSTARTED0W { w: &'static mut W, }
            impl _SEQSTARTED0W { }
            pub struct SEQSTARTED1W;
            impl SEQSTARTED1W { }
            pub struct _SEQSTARTED1W { w: &'static mut W, }
            impl _SEQSTARTED1W { }
            pub struct SEQEND0W;
            impl SEQEND0W { }
            pub struct _SEQEND0W { w: &'static mut W, }
            impl _SEQEND0W { }
            pub struct SEQEND1W;
            impl SEQEND1W { }
            pub struct _SEQEND1W { w: &'static mut W, }
            impl _SEQEND1W { }
            pub struct PWMPERIODENDW;
            impl PWMPERIODENDW { }
            pub struct _PWMPERIODENDW { w: &'static mut W, }
            impl _PWMPERIODENDW { }
            pub struct LOOPSDONEW;
            impl LOOPSDONEW { }
            pub struct _LOOPSDONEW { w: &'static mut W, }
            impl _LOOPSDONEW { }
            impl R { }
            impl W { }
        }
        pub struct MODE;
        pub mod mode {
            pub struct R;
            pub struct W;
            impl super::MODE { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct UPDOWNR;
            impl UPDOWNR { }
            pub struct UPDOWNW;
            impl UPDOWNW { }
            pub struct _UPDOWNW { w: &'static mut W, }
            impl _UPDOWNW { }
            impl R { }
            impl W { }
        }
        pub struct PRESCALER;
        pub mod prescaler {
            pub struct R;
            pub struct W;
            impl super::PRESCALER { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct PRESCALERR;
            impl PRESCALERR { }
            pub struct PRESCALERW;
            impl PRESCALERW { }
            pub struct _PRESCALERW { w: &'static mut W, }
            impl _PRESCALERW { }
            impl R { }
            impl W { }
        }
        pub struct LOOP;
        pub mod loop_ {
            pub struct R;
            pub struct W;
            impl super::LOOP { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct CNTR;
            impl CNTR { }
            pub struct CNTW;
            impl CNTW { }
            pub struct _CNTW { w: &'static mut W, }
            impl _CNTW { }
            impl R { }
            impl W { }
        }
    }
    pub struct NVMC;pub mod nvmc {
        pub struct READY;
        pub mod ready {
            pub struct R;
            impl super::READY { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct READYR;
            impl READYR { }
            impl R { }
        }
        pub struct ERASEUICR;
        pub mod eraseuicr {
            pub struct R;
            pub struct W;
            impl super::ERASEUICR { }
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct ERASEUICRR;
            impl ERASEUICRR { }
            pub struct ERASEUICRW;
            impl ERASEUICRW { }
            pub struct _ERASEUICRW { w: &'static mut W, }
            impl _ERASEUICRW { }
            impl R { }
            impl W { }
        }
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

}
