#![no_std]

#[repr(C)]
pub struct ExceptionFrame { }

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "Rust" { fn main() -> !; }
    main()
}

#[link_section = ".HardFault.default"]
#[no_mangle]
pub unsafe extern "C" fn HardFault_(_: &ExceptionFrame) -> ! {
    loop {     }
}

#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    loop {    }
}

#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

#[allow(dead_code)]
pub union Vector {
    reserved: usize,
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 14] = [
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
];

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 32] = [{
    extern "C" { fn DefaultHandler(); }
    DefaultHandler
}; 32];
