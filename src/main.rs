#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(NodeOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use NodeOS::println;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    NodeOS::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // uncomment line below to trigger a stack overflow
    // stack_overflow();
    
    #[cfg(test)]
    test_main();

    println!("Not crashed{},", "idk");

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    NodeOS::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
