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
#[doc = "Unspecified"]
pub mod rxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD;
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = "Acquire SPI semaphore"]
pub struct TASKS_ACQUIRE;
#[doc = "Acquire SPI semaphore"]
pub mod tasks_acquire;
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub struct TASKS_RELEASE;
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub mod tasks_release;
#[doc = "Granted transaction completed"]
pub struct EVENTS_END;
#[doc = "Granted transaction completed"]
pub mod events_end;
#[doc = "End of RXD buffer reached"]
pub struct EVENTS_ENDRX;
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "Semaphore acquired"]
pub struct EVENTS_ACQUIRED;
#[doc = "Semaphore acquired"]
pub mod events_acquired;
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
#[doc = "Semaphore status register"]
pub struct SEMSTAT;
#[doc = "Semaphore status register"]
pub mod semstat;
#[doc = "Status from last transaction"]
pub struct STATUS;
#[doc = "Status from last transaction"]
pub mod status;
#[doc = "Enable SPI slave"]
pub struct ENABLE;
#[doc = "Enable SPI slave"]
pub mod enable;
#[doc = "Configuration register"]
pub struct CONFIG;
#[doc = "Configuration register"]
pub mod config;
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub struct DEF;
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub mod def;
#[doc = "Over-read character"]
pub struct ORC;
#[doc = "Over-read character"]
pub mod orc;
