use core::fmt::{Error, Write};

pub struct Uart {
    addr: *mut u8,
}

impl Uart {
    // Constructor
    pub fn new(addr: *mut u8) -> Self {
        Self {
            addr
        }
    }

    pub fn print_str(&self, s: &str) {
        for c in s.chars() {
            self.print_char(c);
        }
    }
    
    pub fn print_char(&self, c: char) {
        unsafe {
            self.addr.write_volatile(c as u8);
        }
    }

    pub fn read_char(&self) -> u8 {
        unsafe {
            return self.addr.read_volatile();
        }
    }

    pub fn enable_fifo(&self) {
        // Enable FIFO
        unsafe {
            self.addr.add(2).write_volatile(1 << 0);
        }
    }

    pub fn enable_interrupts(&self) {
        unsafe {
            self.addr.add(1).write_volatile(1 << 0);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print_str(s);
        Ok(())
    }
}

