// Dylaedin Operating System
// Caedin and Dylan
// 10/1/2025
#![no_std]
#![allow(unused_variables)]
#![allow(dead_code)]
/*
	Mods
*/
mod srv;
mod dev;
mod emu;
mod util;
/*
	Idk Stuff Here ;)
 */
use core::{arch::asm, panic::PanicInfo, assert};
use dev::uart::Uart;
use srv::console::Console;
use dev::pci;
use util::{alloc::Alloc, thread::Thread};
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
		println!(
				 "line {}, file {}: {}",
				 _p.line(),
				 _p.file(),
				 info.message()
		);
	}
	else {
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
	kmain();
}

#[no_mangle]
fn kmain() {
	// Getting the device tree from a register
	let device_tree_addr: u64 =  get_dts();
	let kernel_uart: Uart = Uart::new(0x1000_0000 as *mut u8);
	// let kconsole: Console = Console::new(kernel_uart);
	println!("Hello, World!");

	let pci = pci::PCI::get(0, 1);
	println!("Vendor ID: {:#X}", pci.vendor_id());
	println!("Device ID: {:#X}", pci.device_id());
	// println!("Vendor ID: {:#X}", result);/
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

	assert!(((page5 as *const usize as usize) - (page7 as *const usize as usize)) / 4098 == 5, "[FAIL]");
	assert!(((page5 as *const usize as usize) - (page7 as *const usize as usize)) % 4098 == 0, "FAIL");
	assert!(((page9 as *const usize as usize) - (page7 as *const usize as usize)) / 4098 == 4, "FAIL");
	assert!(((page9 as *const usize as usize) - (page7 as *const usize as usize)) % 4098 == 0, "FAIL");
	assert!(((page10 as *const usize as usize) - (page as *const usize as usize)) / 4098 == 10, "FAIL");
	assert!(((page10 as *const usize as usize) - (page as *const usize as usize)) % 4098 == 0, "FAIL");
	println!("[PASS]");

	loop {}
}

