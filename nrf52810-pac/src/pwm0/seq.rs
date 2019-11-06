#[doc = "Description cluster: Beginning address in RAM of this sequence"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Beginning address in RAM of this sequence"]
pub mod ptr;
#[doc = "Description cluster: Number of values (duty cycles) in this sequence"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Number of values (duty cycles) in this sequence"]
pub mod cnt;
#[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register"]
pub struct REFRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register"]
pub mod refresh;
#[doc = "Description cluster: Time added after the sequence"]
pub struct ENDDELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Time added after the sequence"]
pub mod enddelay;
