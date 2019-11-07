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
#[doc = "Stop TWI transaction"]
pub struct TASKS_STOP;
#[doc = "Stop TWI transaction"]
pub mod tasks_stop;
#[doc = "Suspend TWI transaction"]
pub struct TASKS_SUSPEND;
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "Resume TWI transaction"]
pub struct TASKS_RESUME;
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "Prepare the TWI slave to respond to a write command"]
pub struct TASKS_PREPARERX;
#[doc = "Prepare the TWI slave to respond to a write command"]
pub mod tasks_preparerx;
#[doc = "Prepare the TWI slave to respond to a read command"]
pub struct TASKS_PREPARETX;
#[doc = "Prepare the TWI slave to respond to a read command"]
pub mod tasks_preparetx;
#[doc = "TWI stopped"]
pub struct EVENTS_STOPPED;
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "TWI error"]
pub struct EVENTS_ERROR;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "Receive sequence started"]
pub struct EVENTS_RXSTARTED;
#[doc = "Receive sequence started"]
pub mod events_rxstarted;
#[doc = "Transmit sequence started"]
pub struct EVENTS_TXSTARTED;
#[doc = "Transmit sequence started"]
pub mod events_txstarted;
#[doc = "Write command received"]
pub struct EVENTS_WRITE;
#[doc = "Write command received"]
pub mod events_write;
#[doc = "Read command received"]
pub struct EVENTS_READ;
#[doc = "Read command received"]
pub mod events_read;
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
#[doc = "Status register indicating which address had a match"]
pub struct MATCH;
#[doc = "Status register indicating which address had a match"]
pub mod match_;
#[doc = "Enable TWIS"]
pub struct ENABLE;
#[doc = "Enable TWIS"]
pub mod enable;
#[doc = "Description collection: TWI slave address n"]
pub struct ADDRESS;
#[doc = "Description collection: TWI slave address n"]
pub mod address;
#[doc = "Configuration register for the address match mechanism"]
pub struct CONFIG;
#[doc = "Configuration register for the address match mechanism"]
pub mod config;
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub struct ORC;
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub mod orc;
