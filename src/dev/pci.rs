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
    address = ((PCI_BASE) | (lbus << 20) | 
              (lslot << 15) | (lfunc << 12)) as u32;
    println!("PCI Read Word: {:x}", address);

    let pci_ptr = address as *mut u32;
    unsafe {
        let id = pci_ptr.read_volatile() as u32;
        println!("PCI Read Word: {:x}", id);
        tmp = ((id >> 16) & 0xFFFF) as u16;
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