const UART: u32 = 0x1000_0000;

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