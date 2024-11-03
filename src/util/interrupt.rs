use crate::print;
use crate::println;
use core::arch::asm;

#[allow(dead_code)]
fn placeholder_handler() {}

#[repr(C)]
struct HardwareInterruptTable {
    int0: u32,
    int1: fn(),
    int2: fn(),
    int3: fn(),
    int4: fn(),
    int5: fn(),
    int6: fn(),
    int7: fn(), // handler0: fn(),
                // ssip: fn(),
                // handler2: fn(),
                // msip: fn(),
                // handler4: fn(),
                // stip: fn(),
                // handler6: fn(),
                // mtip: fn(),
                // handler8: fn(),
                // seip: fn(),
                // handler10: fn(),
                // meip: fn(),
                // handler12: fn(),
                // lcofip: fn()
}

enum MachineInterruptRegister {
    SSIP = 1,
    MSIP = 3,
    STIP = 5,
    MTIP = 7,
    SEIP = 9,
    MEIP = 11,
    LCOFIP = 13,
}

static RISCV_MACHINE_INTERRUPT_HANDLERS: HardwareInterruptTable = HardwareInterruptTable {
    // handler0: placeholder_handler,
    // ssip: placeholder_handler,
    // handler2: placeholder_handler,
    // msip: placeholder_handler,
    // handler4: placeholder_handler,
    // stip: placeholder_handler,
    // handler6: placeholder_handler,
    // mtip: timer_handler,
    // handler8: placeholder_handler,
    // seip: placeholder_handler,
    // handler10: placeholder_handler,
    // meip: placeholder_handler,
    // handler12: placeholder_handler,
    // lcofip: placeholder_handler,
    int0: 0,
    int1: timer_handler,
    int2: timer_handler,
    int3: timer_handler,
    int4: timer_handler,
    int5: timer_handler,
    int6: timer_handler,
    int7: timer_handler,
};

fn enable_interrupt(register: MachineInterruptRegister) {
    let read_value: usize;
    unsafe {
        asm!(
            "csrr {0}, mie",
            out(reg) read_value
        );
    }
    println!("BEFORE: {}", read_value);

    let shifted_value: usize = 1 << (register as u8);
    unsafe {
        asm!(
            "csrr t1, mie",
            "or t1, t1, {0}",
            "csrw mie, t1",
            in(reg) shifted_value
        );
    }

    let read_value: usize;
    unsafe {
        asm!(
            "csrr {0}, mie",
            out(reg) read_value
        );
    }
    println!("AFTER: {}", read_value);
}

fn software_handler() {}

fn timer_handler() {
    println!("TIMER HERE TIMER HERE");
}

fn get_vec_base() -> u64 {
    let vec_base_addr: u64;
    unsafe {
        asm!("csrr {0}, mtvec", out(reg) vec_base_addr);
    }

    return vec_base_addr;
}

fn write_vec_base(addr: usize) {
    let addr_vectored = addr | 1;
    unsafe {
        asm!(
            "csrw mtvec, {0}",
            in(reg) addr_vectored
        );
    }
}

pub fn init() {
    //initialize timer
    write_vec_base(&RISCV_MACHINE_INTERRUPT_HANDLERS as *const HardwareInterruptTable as usize);

    let read_value = get_vec_base();
    println!("MTEC VALUE: {:X}", read_value);
    enable_interrupt(MachineInterruptRegister::MTIP);
}
