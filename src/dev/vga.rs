// use super::pci::{PCI, PCIError};
// pub struct VGA {
//     pub pci: PCI,
//     pub framebuffer: *mut u8,
//     pub framebuffer_size: usize,

// }

// impl VGA {
//     pub fn new(bus: u8, slot: u8) -> Result<VGA, PCIError> {
//         let pci = PCI::get(bus, slot);
//         if pci.class_code() != 0x03 {
//             return Err(PCIError::InvalidDevice);
//         }
//         Ok(VGA {
//             pci,
//             framebuffer: 
//         })
//     }
// }

//0x400 - 0xC0