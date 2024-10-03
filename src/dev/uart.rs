const UART: u32 = 0x1000_0000;

pub struct Writer;
impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print_str(s);
        Ok(())
    }
}

pub fn print_str(s: &str) {
    for c in s.chars() {
        print_char(c);
    }
}
pub fn print_char(c: char) {
    unsafe {
        core::ptr::write_volatile(UART as *mut u8, c as u8);
    }
}