// Dylaedin Operating System
// Caedin and Dylan
// 10/1/2025
#![no_std]
/*
	Mods
*/
mod services;
mod dev;
/*
	Idk Stuff Here ;)
 */
use core::{arch::asm, panic::PanicInfo};
use dev::uart::Uart;
use services::console::Console;

/*
	Globals
*/

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
extern "C"
fn kmain() {
	let kernel_uart: Uart = Uart::new(0x1000_0000 as *mut u8);
	let kconsole: Console = Console::new(kernel_uart);
	kconsole.listen();
	loop {}
}

