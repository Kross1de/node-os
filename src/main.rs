#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(NodeOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use NodeOS::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[unsafe(no_mangle)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use NodeOS::memory::active_level_4_table;
    use x86_64::VirtAddr;
    
    println!("Hello World{}", "!");
    NodeOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }
    
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
