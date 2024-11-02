pub fn memset(addr: u64, data: u8) {
    unsafe {
        let ptr = addr as *mut u8;
        ptr.write_volatile(data);
    }
}
pub fn memsetn(addr: u64, data: u64, size: u64) {
    for i in 0..size {
        unsafe {
            let ptr = addr as *mut u64;
            ptr.add(i as usize).write_volatile(data);
        }
    }
}
pub fn memread(addr: u64) -> u64 {
    unsafe {
        let ptr = addr as *mut u64;
        ptr.read_volatile()
    }
}

#[macro_export]
macro_rules! read_csr {
    ($csr:expr) => {{
        let value: usize;
        unsafe {
            asm!(
                "csrr {0}, {1}",     // Read the CSR register
                out(reg) value,      // Store the result in 'value'
                const $csr           // CSR constant to read from
            );
        }
        value
    }};
}

#[macro_export]
macro_rules! write_csr {
    ($csr:expr, $val:expr) => {{
        unsafe {
            asm!(
                "csrw {0}, {1}",     // Write to the CSR register
                const $csr,          // CSR constant to write to
                in(reg) $val         // Value to write
            );
        }
    }};
}
