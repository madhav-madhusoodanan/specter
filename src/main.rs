/* required because there can be no os-level abstractions for an os :) */
/* println macro fails, panic handler unavailable, and many others */
/* the main function is pointless as there is no runtime that could call it :( */
#![no_std]
#![no_main]

/* overwriting the os entry point with this _start function. It's special! */
/* no_mangle would tell the compiler not to uniquefy the function and really output a function called _start */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}

use core::panic::PanicInfo;

/* the never type means that function should never return */
/* coz bruhh, to whom would an os return a value?? */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
