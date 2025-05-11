#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ehkzi::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ehkzi::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to the world, ekhzi");
    ehkzi::init(); // new

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    ehkzi::hlt_loop();            // new
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ehkzi::hlt_loop();            // new
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ehkzi::test_panic_handler(info)
}