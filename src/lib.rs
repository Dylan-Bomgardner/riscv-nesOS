// Dylaedin Operating System
// Caedin and Dylan
// 10/1/2025
#![no_std]
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
use core::{arch::asm, panic::PanicInfo};
use dev::uart::Uart;
use srv::console::Console;
use dev::pci;
use util::{alloc::Alloc};
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
extern "C"
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
	
	loop {}
}

