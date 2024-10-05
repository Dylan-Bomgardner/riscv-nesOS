use crate::dev::uart::Uart;
use crate::print;
use crate::println;

pub struct Console {
    uart: Uart
}

impl Console {
    pub fn new(uart: Uart) -> Self {
        uart.enable_fifo();
        uart.enable_interrupts();
        Self {
            uart
        }
    }

    pub fn listen(&self) -> ! {
        loop {
            let c: u8 = self.uart.read_char();
            match c {
                8 => {
                    // Backspace
                    print!("{}{}{}", 8 as char, ' ', 8 as char);
                },
                10 | 13 => {
                    println!();
                }
                _ => {
                    print!("{}", c as char);
                }
            }
        }
    }
}