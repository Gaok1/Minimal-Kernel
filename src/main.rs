#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(unused, dead_code)]

use core::panic::PanicInfo;
pub mod gdt;

use crate::vga::WRITER;

mod vga;

pub mod interrupt_handler;

#[panic_handler]
fn in_panic(_info: &PanicInfo) -> ! {
    eprintln!("PANIC!\n{}", _info);
    x86_64::instructions::hlt(); //halts the kernel to avoid "burning" CPU time
    hlt_loop();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    hlt_loop();
}

pub fn init() {
    interrupt_handler::interrupts::init_idt();
    gdt::init();
    unsafe { interrupt_handler::interrupts::PICS.lock().initialize() }; // new
    x86_64::instructions::interrupts::enable(); // new
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    let x = 0;
    stack_overflow(); // for each recursion, the return address is pushed
    unsafe {
        core::ptr::read_volatile(&x);
    } // prevent tail recursion optimizations
}
