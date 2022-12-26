// MIT License

// Copyright (c) 2021 AnonmousDapper

#![deny(rust_2018_idioms)]
#![no_std]
#![no_main]
#![feature(asm, panic_info_message)]
//#![feature(custom_test_frameworks)]
//#![test_runner(risky::test_runner)]

use core::panic::PanicInfo;

use risky::sprintln;

#[no_mangle]
extern "C" fn kmain() {
    sprintln!("* Hello, World! *");
    sprintln!("  from risc-v :) ");

    unsafe {
        sprintln!(
            "Heap: {} MiB ({} KiB)",
            risky::memory::HEAP_SIZE / 1024 / 1024,
            risky::memory::HEAP_SIZE / 1024
        );
        sprintln!("Heap address: {:#x}", risky::memory::HEAP_START);
    }

    risky::halt();
}

//#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    sprintln!(" !  Kernel panic  ! ");

    if let Some(args) = info.message() {
        sprintln!("\t{}", args);
    } else {
        sprintln!("\tCause unknown\n");
    }

    if let Some(loc) = info.location() {
        sprintln!("\tFrom: {} L{}", loc.file(), loc.line());
    } else {
        sprintln!("\tOrigin unknown\n");
    }

    risky::halt();
}
