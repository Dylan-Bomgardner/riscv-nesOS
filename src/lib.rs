// Dylaedin Operating System
// Caedin and Dylan
// 10/1/2025
#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]
#![cfg(not(test))]
/*
    Mods
*/
mod dev;
mod emu;
mod srv;
mod util;
/*
   Idk Stuff Here ;)
*/
use core::{arch::asm, assert, panic::PanicInfo};
use dev::uart::Uart;
use dev::{pci, vga::*};
use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
use srv::console::Console;
use util::{alloc::Alloc, interrupt, thread::Thread};
/*
    Globals
*/
const TEST_STRING: &str = "TEST";

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
		use core::fmt::Write;
		let _ = write!(crate::dev::uart::Uart::new(0x1000_0000 as *mut u8), $($args)+);
	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

// ///////////////////////////////////
// / LANGUAGE STRUCTURES / FUNCTIONS
// ///////////////////////////////////
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(_p) = info.location() {
        println!("line {}, file {}: {}", _p.line(), _p.file(), info.message());
    } else {
        println!("no information available.");
    }
    loop {
        unsafe {
            asm!("wfi", options(nomem, nostack, preserves_flags));
        }
    }
}

#[no_mangle]
fn get_dts() -> u64 {
    let value: u64;
    unsafe {
        asm!(
            "mv {0}, a2",
            out(reg) value
        );
    }
    return value;
}

// Put all inits here.
#[no_mangle]
fn k_init() {
    Alloc::init();
    interrupt::init();
    kmain();
}

#[no_mangle]
fn kmain() {
    // Getting the device tree from a register
    let device_tree_addr: u64 = get_dts();
    let kernel_uart: Uart = Uart::new(0x1000_0000 as *mut u8);
    // let kconsole: Console = Console::new(kernel_uart);
    println!("Hello, World!");
    //printsizeof PCIHeader0
    println!(
        "Size of PCIHeader0: {}",
        core::mem::size_of::<pci::PCIDevice>()
    );
    let pci = pci::PCIDevice::get(0, 1);
    println!("Vendor ID: {:#X}", pci.header.vendor_id);
    println!("Device ID: {:#X}", pci.header.device_id);
    println!("Class Code: {:#X}", pci.header.class_code);
    println!("Subclass: {:#X}", pci.header.subclass);
    //check the first outside the address range
    // pci.header.command().set_memory_space(true);
    unsafe {
        println!("Address {:#X}", pci.read(0x10));
    }

    //read the value back
    // println!("Vendor ID: {:#X}", result);
    // kconsole.listen();

    // Testing allocator
    println!("[TESTING]: Testing Allocator and Free");
    let page = Alloc::get(2).expect("Page not allocated");
    let page2 = Alloc::get(2).expect("Page not allocated");
    let page3 = Alloc::get(2).expect("err");
    let page4 = Alloc::get(2).expect("err");
    let page5 = Alloc::get(2).expect("");
    Alloc::free(page2);
    Alloc::free(page4);
    let page6 = Alloc::get(1).expect("err");
    let page7 = Alloc::get(1).expect("err");

    let page8 = Alloc::get(1).expect("err");
    let page9 = Alloc::get(1).expect("err");
    let page10 = Alloc::get(1).expect("err");

    assert!(
        ((page5 as *const usize as usize) - (page7 as *const usize as usize)) / 4098 == 5,
        "[FAIL]"
    );
    assert!(
        ((page5 as *const usize as usize) - (page7 as *const usize as usize)) % 4098 == 0,
        "FAIL"
    );
    assert!(
        ((page9 as *const usize as usize) - (page7 as *const usize as usize)) / 4098 == 4,
        "FAIL"
    );
    assert!(
        ((page9 as *const usize as usize) - (page7 as *const usize as usize)) % 4098 == 0,
        "FAIL"
    );
    assert!(
        ((page10 as *const usize as usize) - (page as *const usize as usize)) / 4098 == 10,
        "FAIL"
    );
    assert!(
        ((page10 as *const usize as usize) - (page as *const usize as usize)) % 4098 == 0,
        "FAIL"
    );
    println!("[PASS]");

    Alloc::free(page);
    Alloc::free(page3);
    Alloc::free(page5);
    Alloc::free(page6);
    Alloc::free(page7);
    Alloc::free(page8);
    Alloc::free(page9);
    Alloc::free(page10);

    //print out address
    for i in 0..6 {
        println!("BAR{}: {:#X}", i, pci.bar_read(i));
    }
    //try to find bochs version

    let mut vga = dev::vga::VGA::new(0, 1, 0x4000_0000).unwrap();
    println!("Bochs version: {:#X}", vga.get_bochs_version());
    let mut display = ModeXDisplay::new(vga, 640, 480); //unsafe { Mode13Display::new(vga.fb) };
    display.rectangle(0, 0, 256, 240, Rgb888::BLUE);
    // //write NES in the middle of the rectanlge
    display.print_pos("Nes", 128, 120, Rgb888::WHITE);
    // display.clear();

    // display.rectangle(256, 0, 320-256, 240, Rgb888::WHITE);

    // display.print_pos("DEBUG", 256, 10, Rgb888::WHITE);
    // display.print_pos("H", 128, 220, Rgb888::WHITE);
    // display.print_pos("H", 0, 0, Rgb888::WHITE);
    // display.clear();
    // // display.switch_buffer();
    // //read the value back
    // // println!("Vendor ID: {:#X}", result);/

    // // kconsole.listen();
    let max = 250000000 / 60;
    let mut i = 0;
    loop {
        if (i == 0) {
            display.rectangle(0, 0, display.width, display.height, Rgb888::WHITE);
        } else if i == max / 2 {
            display.rectangle(0, 0, display.width, display.height, Rgb888::BLUE);
        }
        i += 1;
        // display.swap_buffer();
        // println!("Swap")
        if (i == max) {
            i = 0;
        }
    }
    loop {}
    //get bochs version
}
