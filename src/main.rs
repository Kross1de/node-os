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

	use x86_64::registers::control::Cr3;
	
	let (level_4_page_table, _) = Cr3::read();
	println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    
    #[cfg(test)]
    test_main();

    println!("Not crashed{},", "idk");
    NodeOS::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    NodeOS::hlt_loop();
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
