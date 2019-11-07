#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock;
#[doc = "Ready flag"]
pub struct READY;
#[doc = "Ready flag"]
pub mod ready;
#[doc = "Configuration register"]
pub struct CONFIG;
#[doc = "Configuration register"]
pub mod config;
#[doc = "Register for erasing a page in code area"]
pub struct ERASEPAGE;
#[doc = "Register for erasing a page in code area"]
pub mod erasepage;
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub struct ERASEPCR1;
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub mod erasepcr1;
#[doc = "Register for erasing all non-volatile user memory"]
pub struct ERASEALL;
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub struct ERASEPCR0;
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub mod erasepcr0;
#[doc = "Register for erasing user information configuration registers"]
pub struct ERASEUICR;
#[doc = "Register for erasing user information configuration registers"]
pub mod eraseuicr;
#[doc = "Register for partial erase of a page in code area"]
pub struct ERASEPAGEPARTIAL;
#[doc = "Register for partial erase of a page in code area"]
pub mod erasepagepartial;
#[doc = "Register for partial erase configuration"]
pub struct ERASEPAGEPARTIALCFG;
#[doc = "Register for partial erase configuration"]
pub mod erasepagepartialcfg;
