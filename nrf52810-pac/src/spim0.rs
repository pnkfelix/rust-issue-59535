#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock;
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL;
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r" Register block"]
#[repr(C)]
pub struct RXD;
#[doc = r" Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD;
#[doc = r" Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "Start SPI transaction"]
pub struct TASKS_START;
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "Stop SPI transaction"]
pub struct TASKS_STOP;
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "Suspend SPI transaction"]
pub struct TASKS_SUSPEND;
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "Resume SPI transaction"]
pub struct TASKS_RESUME;
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "SPI transaction has stopped"]
pub struct EVENTS_STOPPED;
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "End of RXD buffer reached"]
pub struct EVENTS_ENDRX;
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "End of RXD buffer and TXD buffer reached"]
pub struct EVENTS_END;
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "End of TXD buffer reached"]
pub struct EVENTS_ENDTX;
#[doc = "End of TXD buffer reached"]
pub mod events_endtx;
#[doc = "Transaction started"]
pub struct EVENTS_STARTED;
#[doc = "Transaction started"]
pub mod events_started;
#[doc = "Shortcuts between local events and tasks"]
pub struct SHORTS;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable interrupt"]
pub struct INTENSET;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Enable SPIM"]
pub struct ENABLE;
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub struct FREQUENCY;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "Configuration register"]
pub struct CONFIG;
#[doc = "Configuration register"]
pub mod config;
#[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
pub struct ORC;
#[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
pub mod orc;
