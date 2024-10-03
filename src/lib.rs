// Dylaedin Operating System
// Caedin and Dylan
// 10/1/2025
#![no_std]
mod util;
mod dev;
use core::{arch::asm, fmt::Write, panic::PanicInfo};

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
		let _ = write!(dev::uart::Writer, $($args)+);
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
extern "C"
fn kmain() {
	
	dev::uart::print_str("Hello, World! Check out the Dylaedin Operating System!\n");
	loop {}
}

