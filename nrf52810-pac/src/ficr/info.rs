#[doc = "Part code"]
pub struct PART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Part code"]
pub mod part;
#[doc = "Part variant, hardware version and production configuration"]
pub struct VARIANT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Part variant, hardware version and production configuration"]
pub mod variant;
#[doc = "Package option"]
pub struct PACKAGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Package option"]
pub mod package;
#[doc = "RAM variant"]
pub struct RAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RAM variant"]
pub mod ram;
#[doc = "Flash variant"]
pub struct FLASH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash variant"]
pub mod flash;
