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
#[doc = "Start TWI receive sequence"]
pub struct TASKS_STARTRX;
#[doc = "Start TWI receive sequence"]
pub mod tasks_startrx;
#[doc = "Start TWI transmit sequence"]
pub struct TASKS_STARTTX;
#[doc = "Start TWI transmit sequence"]
pub mod tasks_starttx;
#[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
pub struct TASKS_STOP;
#[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
pub mod tasks_stop;
#[doc = "Suspend TWI transaction"]
pub struct TASKS_SUSPEND;
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "Resume TWI transaction"]
pub struct TASKS_RESUME;
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "TWI stopped"]
pub struct EVENTS_STOPPED;
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "TWI error"]
pub struct EVENTS_ERROR;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
pub struct EVENTS_SUSPENDED;
#[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
pub mod events_suspended;
#[doc = "Receive sequence started"]
pub struct EVENTS_RXSTARTED;
#[doc = "Receive sequence started"]
pub mod events_rxstarted;
#[doc = "Transmit sequence started"]
pub struct EVENTS_TXSTARTED;
#[doc = "Transmit sequence started"]
pub mod events_txstarted;
#[doc = "Byte boundary, starting to receive the last byte"]
pub struct EVENTS_LASTRX;
#[doc = "Byte boundary, starting to receive the last byte"]
pub mod events_lastrx;
#[doc = "Byte boundary, starting to transmit the last byte"]
pub struct EVENTS_LASTTX;
#[doc = "Byte boundary, starting to transmit the last byte"]
pub mod events_lasttx;
#[doc = "Shortcuts between local events and tasks"]
pub struct SHORTS;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable or disable interrupt"]
pub struct INTEN;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt"]
pub struct INTENSET;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Error source"]
pub struct ERRORSRC;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "Enable TWIM"]
pub struct ENABLE;
#[doc = "Enable TWIM"]
pub mod enable;
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub struct FREQUENCY;
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "Address used in the TWI transfer"]
pub struct ADDRESS;
#[doc = "Address used in the TWI transfer"]
pub mod address;
