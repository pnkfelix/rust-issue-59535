#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock;
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL;
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Start TWI receive sequence"]
pub struct TASKS_STARTRX;
#[doc = "Start TWI receive sequence"]
pub mod tasks_startrx;
#[doc = "Start TWI transmit sequence"]
pub struct TASKS_STARTTX;
#[doc = "Start TWI transmit sequence"]
pub mod tasks_starttx;
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
#[doc = "TWI stopped"]
pub struct EVENTS_STOPPED;
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "TWI RXD byte received"]
pub struct EVENTS_RXDREADY;
#[doc = "TWI RXD byte received"]
pub mod events_rxdready;
#[doc = "TWI TXD byte sent"]
pub struct EVENTS_TXDSENT;
#[doc = "TWI TXD byte sent"]
pub mod events_txdsent;
#[doc = "TWI error"]
pub struct EVENTS_ERROR;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "TWI byte boundary, generated before each byte that is sent or received"]
pub struct EVENTS_BB;
#[doc = "TWI byte boundary, generated before each byte that is sent or received"]
pub mod events_bb;
#[doc = "TWI entered the suspended state"]
pub struct EVENTS_SUSPENDED;
#[doc = "TWI entered the suspended state"]
pub mod events_suspended;
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
#[doc = "Error source"]
pub struct ERRORSRC;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "Enable TWI"]
pub struct ENABLE;
#[doc = "Enable TWI"]
pub mod enable;
#[doc = "RXD register"]
pub struct RXD;
#[doc = "RXD register"]
pub mod rxd;
#[doc = "TXD register"]
pub struct TXD;
#[doc = "TXD register"]
pub mod txd;
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub struct FREQUENCY;
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "Address used in the TWI transfer"]
pub struct ADDRESS;
#[doc = "Address used in the TWI transfer"]
pub mod address;
