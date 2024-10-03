// Dylaedin Operating System
// Caedin and Dylan
// 10/1/2025
#![no_std]
mod util;

use core::arch::asm;

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({

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
#[no_mangle]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(_p) = info.location() {
		println!(
				 "line {}, file {}: {}",
				 _p.line(),
				 _p.file(),
				 info.message().unwrap()
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
extern "C"
fn kmain() {
	
	print_str("Hello, World! Check out the Dylaedin Operating System!\n");
	loop {}
}

fn print_char(c: char) {
	util::std::memset(0x10000000, c as u8);
}

fn print_str(s: &str) {
	for c in s.chars() {
		print_char(c);
	}
}

