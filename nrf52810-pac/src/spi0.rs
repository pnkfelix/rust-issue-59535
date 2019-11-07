#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock;
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL;
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "TXD byte sent and RXD byte received"]
pub struct EVENTS_READY;
#[doc = "TXD byte sent and RXD byte received"]
pub mod events_ready;
#[doc = "Enable interrupt"]
pub struct INTENSET;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Enable SPI"]
pub struct ENABLE;
#[doc = "Enable SPI"]
pub mod enable;
#[doc = "RXD register"]
pub struct RXD;
#[doc = "RXD register"]
pub mod rxd;
#[doc = "TXD register"]
pub struct TXD;
#[doc = "TXD register"]
pub mod txd;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub struct FREQUENCY;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "Configuration register"]
pub struct CONFIG;
#[doc = "Configuration register"]
pub mod config;
