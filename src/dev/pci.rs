use crate::println;
use crate::print;

const PCI_BASE: u32 = 0x3000_0000;
const PCI_CONFIG_ADDRESS: u32 = 0xCF8;
const PCI_CONFIG_DATA: u32 = 0xCFC;
pub struct PCI {
    
}

//static PCI function wall read word
pub fn pci_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let address;
    let lbus = bus as u32;
    let lslot = slot as u32;
    let lfunc = func as u32;
    let tmp: u16;
    address = (PCI_BASE) | (lbus << 16) | 
              (lslot << 11  ) | (lfunc << 8) | (offset & 0xfc) as u32;
    println!("PCI Read Word: {:x}", address);

    let ptr_write = PCI_CONFIG_ADDRESS as *mut u32;
    let ptr_read = PCI_CONFIG_DATA as *mut u32;
    unsafe {
        ptr_write.write_volatile(address);
        println!("PCI Read Word: {:x}", ptr_write.read_volatile());
        tmp = ((ptr_read.read_volatile() >> ((offset & 2) * 8)) & 0xFFFF) as u16;
    }
    tmp
}
pub fn pci_check_vendor(bus: u8, slot: u8) -> u16 {
    let vendor_id = pci_read_word(bus, slot, 0, 0);
    if vendor_id != 0xFFFF {
        println!("No device found");
    }
    vendor_id
}