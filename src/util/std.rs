pub fn memset(addr: u64, data: u8) {
    unsafe {
        let ptr = addr as *mut u8;
        ptr.write_volatile(data);
    }
}
pub fn memsetn(addr: u64 , data: u64, size: u64) {
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
