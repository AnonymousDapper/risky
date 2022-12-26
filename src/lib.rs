// MIT License

// Copyright (c) 2021 AnonymousDapper

#![deny(rust_2018_idioms)]
#![no_std]
#![feature(asm, global_asm, panic_info_message)]
//#![feature(custom_test_frameworks)]
//#![test_runner(crate::test_runner)]

//use core::panic::PanicInfo;

pub mod assembly;

pub mod uart;

pub mod memory;

/*pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        sprint!("  {:<48}\t....\t", core::any::type_name::<T>());
        self();
        sprintln!("\x1b[32;1m[ok]\x1b[0m");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    sprintln!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    // TODO: exit qemu

    halt();
}

#[cfg(test)]
fn test_panic_handler(info: &PanicInfo<'_>) -> ! {
    sprintln!(" !  Kernel panic  ! ");
    sprintln!(" .    [ test ]    . ");

    if let Some(args) = info.message() {
        sprintln!("{}", args);
    } else {
        sprintln!("Cause unknown\n");
    }

    if let Some(loc) = info.location() {
        sprintln!("From: {} L{}", loc.file(), loc.line());
    } else {
        sprintln!("Origin unknown\n");
    }

    halt()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    test_panic_handler(info)
}*/

pub fn halt() -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}
