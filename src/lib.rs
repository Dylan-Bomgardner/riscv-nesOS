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
use core::{arch::{asm}, panic::PanicInfo, assert};
use dev::uart::Uart;
use srv::console::Console;
use dev::{pci, vga};
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
	//printsizeof PCIHeader0
	println!("Size of PCIHeader0: {}", core::mem::size_of::<pci::PCIDevice>());
	let pci = pci::PCIDevice::get(0, 1);
	println!("Vendor ID: {:#X}", pci.header.vendor_id);
	println!("Device ID: {:#X}", pci.header.device_id);
	println!("Class Code: {:#X}", pci.header.class_code);
	println!("Subclass: {:#X}", pci.header.subclass);
	println!("Address Range: {:#X}", pci.address_range);
	//check the first outside the address range
	// pci.header.command().set_memory_space(true);
	unsafe {
	println!("Address {:#X}", pci.read(0x10));
	}

	//read the value back
	// println!("Vendor ID: {:#X}", result);/
	// kconsole.listen();

	let mut vga = VGA::new(0, 1);

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

	Alloc::free(page);
	Alloc::free(page3);
	Alloc::free(page5);
	Alloc::free(page6);
	Alloc::free(page7);
	Alloc::free(page8);
	Alloc::free(page9);
	Alloc::free(page10);

	loop {}
}

