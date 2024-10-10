use crate::println;
use crate::print;
use core::fmt;
use core::result::Result;
use core::fmt::Display;
use bitfield_struct::{bitfield};
const PCI_BASE: u32 = 0x3000_0000;
#[derive(Debug)]
pub enum PCIError
{
    InvalidRegister,
    InvalidDevice,
}




#[bitfield(u16)]
pub struct PCICommandReg {
    ///I/O Space - 
    /// If set to 1 the device can respond to I/O Space accesses; 
    /// otherwise, the device's response is disabled.
    #[bits(1, access = RW)]
    pub io_space: bool,
    ///Memory Space - 
    /// If set to 1 the device can respond to Memory Space accesses; 
    /// otherwise, the device's response is disabled.
    #[bits(1, access = RW)]
    pub memory_space: bool,
    ///Bus Master - 
    /// If set to 1 the device can behave as a bus master; 
    /// otherwise, the device can not generate PCI accesses.
    #[bits(1, access = RW)]
    pub bus_master: bool,
    ///Special Cycles - 
    /// If set to 1 the device can monitor Special Cycle operations; 
    /// otherwise, the device will ignore them.
    #[bits(1, access = RO)]
    pub special_cycles: bool,
    ///Memory Write and Invalidate Enable - 
    /// If set to 1 the device can generate the Memory Write and Invalidate command; 
    /// otherwise, the Memory Write command must be used.
    #[bits(1, access = RO)]
    pub memory_space_enable: bool,
    ///VGA Palette Snoop - 
    /// If set to 1 the device does not respond to palette register writes and will snoop the data; 
    /// otherwise, the device will trate palette write accesses like all other accesses.
    #[bits(1, access = RO)]
    pub vga_palette_snoop: bool,
    ///Parity Error Response - 
    /// If set to 1 the device will take its normal action when a parity error is detected; 
    /// otherwise, when an error is detected, the device will set bit 15 of the Status register (Detected Parity Error Status Bit), but will not assert the PERR# (Parity Error) pin and will continue operation as normal.
    #[bits(1, access = RW)]
    pub parity_error_response: bool,
    ///as of PCI 3.0 bit is hardwired to 0
    #[bits(1, access = RO)]
    bit_7: bool,
    ///SERR# Enable - 
    /// If set to 1 the SERR# driver is enabled; 
    /// otherwise, the driver is disabled.
    #[bits(1, access = RW)]
    pub seer_enable: bool,
    ///Fast Back-Back Enable - 
    /// If set to 1 indicates a device is allowed to generate fast back-to-back transactions; 
    /// otherwise, fast back-to-back transactions are only allowed to the same agent.
    #[bits(1, access = RO)]
    pub fast_back_to_back_enable: bool,
    ///Interrupt Disable - 
    /// If set to 1 the assertion of the devices INTx# signal is disabled; 
    /// otherwise, assertion of the signal is enabled.
    #[bits(1, access = RW)]
    pub interrupt_disable: bool,
    #[bits(5)]
    unused: usize,
}
#[bitfield(u16)]
pub struct PCIStatusReg
{
    #[bits(3, access = None)]
    reserved: u8,
    ///Interrupt Status - 
    /// Represents the state of the device's INTx# signal. If set to 1 and bit 10 of the Command register (Interrupt Disable bit) is set to 0 the signal will be asserted; otherwise, the signal will be ignored.
    #[bits(1, access = RO)]
    pub interrupt_status: bool,
    ///Capabilities List - 
    /// If set to 1 the device implements the pointer for a New Capabilities Linked list at offset 0x34; otherwise, the linked list is not available.
    #[bits(1, access = RO)]
    pub capabilities_list: bool,
    ///66 MHz Capable - 
    /// If set to 1 the device is capable of running at 66 MHz; otherwise, the device runs at 33 MHz.
    #[bits(1, access = RO)]
    pub mhz_66_capable: bool,
    ///Bit 6 - As of revision 3.0 of the PCI Local Bus specification this bit is reserved. 
    /// In revision 2.1 of the specification this bit was used to indicate whether or not a device supported User Definable Features.
    #[bits(1, access = RO)]
    pub bit_6: bool,
    ///Fast Back-to-Back Capable - 
    /// If set to 1 the device can accept fast back-to-back transactions that are not from the same agent; 
    /// otherwise, transactions can only be accepted from the same agent.
    #[bits(1, access = RO)]
    pub fast_back_to_back_capable: bool,
    ///Master Data Parity Error - 
    /// This bit is only set when the following conditions are met. 
    /// The bus agent asserted PERR# on a read or observed an assertion of PERR# on a write, 
    /// the agent setting the bit acted as the bus master for the operation in which the error occurred, 
    /// and bit 6 of the Command register (Parity Error Response bit) is set to 1.
    #[bits(1, access = RW)]
    pub master_data_parity_error: bool,
    ///DEVSEL Timing - 
    /// Read only bits that represent the slowest time that a device will assert DEVSEL# for any bus command except Configuration Space read and writes. 
    /// Where a value of 0x0 represents fast timing, a value of 0x1 represents medium timing, and a value of 0x2 represents slow timing.
    #[bits(2, access = RO)]
    pub devsel_timing: usize,
    ///Signalled Target Abort - 
    /// This bit will be set to 1 whenever a target device terminates a transaction with Target-Abort.
    #[bits(1, access = RW)]
    pub signaled_target_abort: bool,
    ///Received Target Abort - This bit will be set to 1, by a master device, whenever its transaction is terminated with Target-Abort.
    #[bits(1, access = RW)]
    pub received_target_abort: bool,
    ///Received Master Abort - 
    /// This bit will be set to 1, by a master device, whenever its transaction 
    /// (except for Special Cycle transactions) is terminated with Master-Abort.
    #[bits(1, access = RW)]
    pub received_master_abort: bool,
    ///Signalled System Error - 
    /// This bit will be set to 1 whenever the device asserts SERR#.
    #[bits(1, access = RW)]
    pub signalled_system_error: bool,
    ///Detected Parity Error - 
    /// This bit will be set to 1 whenever the device detects a parity error, 
    /// even if parity error handling is disabled.
    #[bits(1, access = RW)]
    pub detected_parity_error: bool,

}
//use enum with offset instead of bitfield
#[bitfield(u8)]
pub struct PCIBISTReg {
    #[bits(4)]
    pub completion_code: u8,
    #[bits(2)]
    pub reserved: u8,
    #[bits(1)]
    pub start_bist: bool,
    #[bits(1)]
    pub bist_capable: bool,
}

pub struct PCICommonHeader {
    //32
    pub vendor_id: u16,
    pub device_id: u16,

    //32
    pub command: *mut PCICommandReg,
    pub status:  *mut PCIStatusReg,

    //32
    pub revision_id: u8,
    pub prog_if: u8,
    pub subclass: u8,
    pub class_code: u8,
    //32

    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub multi_function: bool,
    pub bist: *mut PCIBISTReg,
}




impl PCICommonHeader
{
    pub fn get(bus: u8, slot: u8) -> PCICommonHeader {
        let pci_address = pci_device_address(bus, slot) as *mut u32;
        
        unsafe 
        {
            let pci_address_16 = pci_address as *mut u16;
            let command_address = pci_address_16.add(2) as *mut u16;
            let status_address = pci_address_16.add(3) as *mut u16;

            let bist_address = (pci_address as *mut u8).add(0xC) as *mut u8;

            let bist = bist_address as *mut PCIBISTReg;

            PCICommonHeader {
                vendor_id: pci_address.read_volatile() as u16,
                device_id: (pci_address.read_volatile() >> 16) as u16,
                command: command_address as *mut PCICommandReg,
                status: status_address as *mut PCIStatusReg,
                revision_id: (pci_address.add(2).read_volatile()) as u8,
                prog_if: (pci_address.add(2).read_volatile() >> 8) as u8,
                subclass: (pci_address.add(2).read_volatile() >> 16) as u8,
                class_code: (pci_address.add(2).read_volatile() >> 24) as u8,

                cache_line_size: pci_address.add(3).read_volatile() as u8,
                latency_timer: (pci_address.add(3).read_volatile() >> 8) as u8,
                header_type: (pci_address.add(3).read_volatile() >> 16) as u8,  
                multi_function: ((pci_address.add(3).read_volatile() >> 23) & 1) != 0,
                bist: bist,
                
    
            }
        }
    }
}
#[bitfield(u16)]
pub struct PCICapabilityControlRegister{
    pub enable: bool,
    #[bits(3)]
    pub multiple_message_capable: u8,
    #[bits(3)]
    pub multiple_message_enable: u8,
    pub _64bit: bool,
    #[bits(8)]
    __: u8,
}
#[bitfield(u128)]
pub struct PCICapabilitiesList {
    pub capability_id: u8,
    pub next: u8,
    #[bits(16)]
    pub control: PCICapabilityControlRegister,
    pub address: u64,
    pub data: u16,
    pub __: u16,


}

pub struct PCIDevice
{
    pub base_address: *mut u32,
    pub header: PCICommonHeader,
    pub cardbus_cis_pointer: *mut u32,
    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,
    pub expansion_rom_base_address: *mut u32,
    pub capabilities_head: *const PCICapabilitiesList,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub min_grant: u8,
    pub max_latency: u8,
    pub address_range: u32,
}



impl PCIDevice
{
    pub fn get(bus: u8, slot: u8) -> PCIDevice {
        let addr = pci_device_address(bus, slot) as *mut u32;
        let header = PCICommonHeader::get(bus, slot);
        unsafe
        {
            let bar0 = addr.add(4);
            let original = bar0.read_volatile();
            bar0.write_volatile(0xFFFF_FFFF);
            let result = bar0.read_volatile();
            let range = !(result & 0xFFFF_FFF0) + 1;
            bar0.write_volatile(original);
            PCIDevice
            {
                base_address: addr,
                header,
                cardbus_cis_pointer: addr.add(0xA),
                subsystem_vendor_id: addr.add(0xB).read_volatile() as u16,
                subsystem_id: (addr.add(0xB).read_volatile() >> 16) as u16,
                expansion_rom_base_address: addr.add(0xC).read_volatile() as *mut u32,
                capabilities_head: addr.add(addr.add(0xD).read_volatile() as usize) as *const PCICapabilitiesList,
                interrupt_line: addr.add(0xF).read_volatile() as u8,
                interrupt_pin: (addr.add(0xF).read_volatile() >> 8) as u8,
                min_grant: (addr.add(0xF).read_volatile() >> 16) as u8,
                max_latency: (addr.add(0xF).read_volatile() >> 24) as u8,
                address_range: range,
            }
        }
    }
    pub fn bar_read(&self, index: usize) -> u32 {
        //require that index is less than 6       
        unsafe { self.get_bar_address(index).read_volatile() }
    }
    
    pub fn bar_write(&self, index: usize, value: u32) {
            unsafe { self.get_bar_address(index).write_volatile(value); }
    }
    pub unsafe fn read(&self, offset: usize) -> u32 {
        self.base_address.add(offset).read_volatile()

    }
    pub unsafe fn write<T>(&self, offset: usize, value: T) {
        core::ptr::write_volatile(self.base_address.add(offset) as *mut T, value);
    }
    pub unsafe fn get_bar_address(&self, index: usize) -> *mut u32 {
        self.base_address.add(4+index)
    }
    pub fn get_bar_size(&self, index: usize) -> u32 {
        let original = self.bar_read(index);
        self.bar_write(index, 0xFFFF_FFFF);
        let result = self.bar_read(index);
        let size = !(result & 0xFFFF_FFF0) + 1;
        self.bar_write(index, original);
        size
    }
}

//static PCI function wall read word
pub fn pci_device_address(bus: u8, slot: u8) -> u32 {
    let lbus = bus as u32;
    let lslot = slot as u32;
    ((PCI_BASE) | 
    (lbus << 20) | 
    (lslot << 15)) as u32
}

pub fn pci_function_address(bus: u8, slot: u8, func: u8) -> u32 {
    let lbus = bus as u32;
    let lslot = slot as u32;
    let lfunc = func as u32;
    ((PCI_BASE) | 
    (lbus << 20) | 
    (lslot << 15) | 
    (lfunc << 12)) as u32
}

