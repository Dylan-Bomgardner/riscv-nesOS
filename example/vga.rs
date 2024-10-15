//! QEMU https://wiki.osdev.org/VGA_Hardware
//! http://www.osdever.net/FreeVGA/vga/vga.htm

use core::convert::Infallible;

use crate::println;
use crate::print;

use super::pci::{PCIDevice, PCIError};
use bitfield_struct::bitfield;
use embedded_graphics::{
    mono_font::{ascii::{FONT_6X12, FONT_7X14}, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
    text::Text,
};
pub struct VGA {
    pub pci: PCIDevice,
    pub fb: *mut u8,
    pub fb_size: usize,

    pub io: *mut u8,
    pub io_size: usize,

    pub base_port: *mut u8,

}

impl VGA {
    pub fn new(bus: u8, slot: u8, mem_start: usize) -> Result<VGA, PCIError> {
        let pci = PCIDevice::get(bus, slot);
        if pci.header.class_code != 0x03 {
            return Err(PCIError::InvalidDevice);
        }
        if 0x4000_0000 > mem_start && mem_start > 0x8000_0000 {
            return Err(PCIError::InvalidAddress);
        }
        unsafe 
        {
            let io = (mem_start as usize + pci.get_bar_address_size(0) as usize) as *mut u8;
            let vga = VGA {              
                fb: mem_start as *mut u8,
                fb_size: pci.get_bar_address_size(0) as usize,
                io_size: pci.get_bar_address_size(2) as usize,
                pci: pci,
                base_port: io.add(0x100 - 0xC0),
                io: io,
            };

            //tell the vga which address to use
            (*vga.pci.header.command).set_memory_space(true);
            vga.pci.bar_write(0, vga.fb as u32 | 8);
            //bar2
            vga.pci.bar_write(2, vga.io as u32 | 8);
            Ok(vga)
        }
    }
    pub fn set_modeX(&self) -> ModeXDisplay {
        unsafe {
            for (register, data) in MODE_X_REGS_BASIC {
                self.set_register(register.reg, register.index, *data);
            }

            //print out a couple of exampels and see if they match the values in the array
    
            // self.io.add(0x406).write_volatile(0xFF);
            // self.io.add(0x408).write_volatile(0);
            let p = self.io.add(0x409);
            for rgb in PALETTE.into_iter() {
                let b = rgb as u8;
                let g = (rgb >> 8) as u8;
                let r = (rgb >> 16) as u8;
    
                p.write_volatile(r);
                p.write_volatile(g);
                p.write_volatile(b);
            }
            ModeXDisplay::new(self.fb)
        }
    }
    unsafe fn set_register(&self, port: u16, index: u8, value: u8) {
        let port_addr = self.base_port.add(port as usize);

        match port {
            0x3C0 => {
                self.base_port.add(0x3DA).read_volatile();
                port_addr.write_volatile(index);
                port_addr.write_volatile(value);
            }
            0x3C2 => {
                // This is the miscellaneous output register. 
                // It uses port 0x3C2 for writing, and 0x3CC for reading. 
                // Bit 0 of this register controls the location of several other registers: 
                // if cleared, port 0x3D4 is mapped to 0x3B4, and port 0x3DA is mapped to 0x3BA. 
                // For readability, only the first port is listed and bit 0 is assumed to be set.
                port_addr.write_volatile(value);
            }
            _ => {
                port_addr.write_volatile(index);
                port_addr.add(1).write_volatile(value);
            }
        }
    }
    unsafe fn read_register(&self, port: u16, index: u8) -> u8 {
        let port_addr = self.base_port.add(port as usize);

        match port {
            0x3C0 => {
                self.base_port.add(0x3DA).read_volatile();
                port_addr.write_volatile(index);
                port_addr.add(1).read_volatile()

            }
            0x3C2 => {
                // This is the miscellaneous output register. 
                // It uses port 0x3C2 for writing, and 0x3CC for reading. 
                // Bit 0 of this register controls the location of several other registers: 
                // if cleared, port 0x3D4 is mapped to 0x3B4, and port 0x3DA is mapped to 0x3BA. 
                // For readability, only the first port is listed and bit 0 is assumed to be set.
                self.base_port.add(0x3CC).read_volatile()
            }
            _ => {
                port_addr.write_volatile(index);
                port_addr.add(1).read_volatile()
            }
        }
    }
    // fn pixels_to_characters(&self, x: i32, y: i32) -> (i32, i32) {
    //     (x / 8, y / 16)
    // }
}
struct VGARegister {
    reg: u16,
    index: u8,
}

// 0x400 - const D = 0xC0;
const HORIZONTAL_TOTAL: VGARegister = VGARegister { reg: 0x3D4, index: 0x00 };
const HORIZONTAL_DISPLAY_END: VGARegister = VGARegister { reg: 0x3D4, index: 0x01 };
const HORIZONTAL_BLANK_START: VGARegister = VGARegister { reg: 0x3D4, index: 0x02 };
const HORIZONTAL_BLANK_END: VGARegister = VGARegister { reg: 0x3D4, index: 0x03 };
const HORIZONTAL_RETRACE_START: VGARegister = VGARegister { reg: 0x3D4, index: 0x04 };
const HORIZONTAL_RETRACE_END: VGARegister = VGARegister { reg: 0x3D4, index: 0x05 };
const VERTICAL_TOTAL: VGARegister = VGARegister { reg: 0x3D4, index: 0x06 };
const OVERFLOW_REGISTER: VGARegister = VGARegister { reg: 0x3D4, index: 0x07 };
const PRESET_ROW_SCAN: VGARegister = VGARegister { reg: 0x3D4, index: 0x08 };
const MAXIMUM_SCAN_LINE: VGARegister = VGARegister { reg: 0x3D4, index: 0x09 };
const VERTICAL_RETRACE_START: VGARegister = VGARegister { reg: 0x3D4, index: 0x10 };
const VERTICAL_RETRACE_END: VGARegister = VGARegister { reg: 0x3D4, index: 0x11 };
const VERTICAL_DISPLAY_END: VGARegister = VGARegister { reg: 0x3D4, index: 0x12 };
const LOGICAL_WIDTH: VGARegister = VGARegister { reg: 0x3D4, index: 0x13 };
const UNDERLINE_LOCATION: VGARegister = VGARegister { reg: 0x3D4, index: 0x14 };
const VERTICAL_BLANK_START: VGARegister = VGARegister { reg: 0x3D4, index: 0x15 };
const VERTICAL_BLANK_END: VGARegister = VGARegister { reg: 0x3D4, index: 0x16 };
const MODE_CONTROL: VGARegister = VGARegister { reg: 0x3D4, index: 0x17 };
// Register values to set mode13h
// (vport, index, data)

//0x3CE
const SETRESET: VGARegister = VGARegister { reg: 0x3CE, index: 0x00 };
const GRAPHICS_MODE: VGARegister = VGARegister { reg: 0x3CE, index: 0x05 };
/// http://www.osdever.net/FreeVGA/vga/graphreg.htm#03
/// ### 3-2 Memory Map Select
/// For bits 3-2 of the Miscellaneous Graphics Register, the values are:
/// This field specifies the range of host memory addresses that is decoded by the
/// VGA hardware and mapped into display memory accesses. The values of this field and their
/// corresponding host memory ranges are:
/// #### Range
///     00b -- A0000h-BFFFFh (128K region)
///     01b -- A0000h-AFFFFh (64K region)
///     10b -- B0000h-B7FFFh (32K region)
///     11b -- B8000h-BFFFFh (32K region)
/// ### 1 Chain Odd/Even
/// 
const MISCELLANEOUS_GRAPHICS: VGARegister = VGARegister { reg: 0x3CE, index: 0x06 };


//0x3C4
const SEQUENCER_MEMORY_MODE: VGARegister = VGARegister { reg: 0x3C4, index: 0x04 };
const MAP_MASK: VGARegister = VGARegister { reg: 0x3C4, index: 0x02 };
const CLOCK_MODE_98_DOT_MODE: VGARegister = VGARegister { reg: 0x3C4, index: 0x01 };

const MISCELLANEOUS_OUTPUT_REGISTER: VGARegister = VGARegister { reg: 0x3C2, index: 0x00 };

//"This bit is set to 0 to load color values to the registers in the internal palette. 
//It is set to 1 for normal operation of the attribute controller. 
//Note: Do not access the internal palette while this bit is set to 1. 
//While this bit is 1, the Type 1 video subsystem disables accesses to the palette; 
//however, the Type 2 does not, and the actual color value addressed cannot be ensured.
const PALETTE_ADDRESS_SOURCE: u8 = 0b10_0000;


///P54S -- Palette Bits 5-4 Select
/// This bit selects the source for the P5 and P4 video bits that act as inputs to the video DAC. 
/// When this bit is set to 0, P5 and P4 are the outputs of the Internal Palette registers. 
/// When this bit is set to 1, P5 and P4 are bits 1 and 0 of the Color Select register."
const ATTRIBUTE_P54S: u8 =  0b1000_0000;
///"When this bit is set to 1, 
/// the video data is sampled so that eight bits are available to select a color in the 256-color mode (0x13). 
/// This bit is set to 0 in all other modes."
const ATTRIBUTE_8BIT: u8 =  0b0100_0000;
///This field allows the upper half of the screen to pan independently of the lower screen. 
/// If this field is set to 0 then nothing special occurs during a successful line compare 
/// (see the Line Compare field.) If this field is set to 1, then upon a successful line compare, 
/// the bottom portion of the screen is displayed as if the Pixel Shift Count and Byte Panning fields are set to 0.
const ATTRIBUTE_PPM: u8 =   0b0010_0000;
///"When this bit is set to 0, 
/// the most-significant bit of the attribute selects the background intensity 
/// (allows 16 colors for background). 
/// When set to 1, this bit enables blinking."
const ATTRIBUTE_BLINK: u8 = 0b0000_1000;
///This field is used in 9 bit wide character modes to provide continuity for the horizontal 
/// line characters in the range C0h-DFh. If this field is set to 0, then the 9th column of 
/// these characters is replicated from the 8th column of the character. 
/// Otherwise, if it is set to 1 then the 9th column 
/// is set to the background like the rest of the characters.
const ATTRIBUTE_LGE: u8 =   0b0000_0100;
///This field is used to store your favorite bit. 
/// According to IBM, "When this bit is set to 1, monochrome emulation mode is selected. 
/// When this bit is set to 0, color |emulation mode is selected." 
/// It is present and programmable in all of the hardware but it apparently does nothing. 
/// The internal palette is used to provide monochrome emulation instead.
const ATTRIBUTE_MONO: u8 =  0b0000_0010;
///"When set to 1, this bit selects the graphics mode of operation."
const ATTRIBUTE_ATGE: u8 =  0b0000_0001;

///"When set to 0, this bit allows bit 5 to control the loading of the 
/// shift registers. When set to 1, this bit causes the shift registers to be 
/// loaded in a manner that supports the 256-color mode."
const GRPAHICS_SHIFT256: u8 = 0b0100_0000;
///When set to 1, this bit directs the shift registers in the graphics controller to
/// format the serial data stream with even-numbered bits from both maps on even-
/// numbered maps, and odd-numbered bits from both maps on the odd-numbered maps. 
/// This bit is used for modes 4 and 5.
const GRAPHICS_SHIFT_INTERLEAVE: u8 = 0b0010_0000;
///When set to 1, this bit selects the odd/even addressing mode used by the IBM Color/Graphics Monitor Adapter. Normally, the value here follows the value of Memory Mode register bit 2 in the sequencer.
const GRAPHICS_HOST_ODD_EVEN: u8 = 0b0001_0000;
///This field selects between two read modes, simply known as Read Mode 0, and Read Mode 1, 
/// based upon the value of this field:
///0b -- Read Mode 0: 
///     In this mode, a byte from one of the four planes is returned on read operations. 
///     The plane from which the data is returned is determined by the value of the Read Map Select field.
///1b -- Read Mode 1: 
///     In this mode, a comparison is made between display memory and a reference color defined by the 
///     Color Compare field. Bit planes not set in the Color Don't Care field then the corresponding color 
///     plane is not considered in the comparison. Each bit in the returned result represents one comparison 
///     between the reference color, with the bit being set if the comparison is true.
const GRAPHICS_READ_MODE: u8 = 0b0000_1000;


const GRAPHICS_WRITE_MODE_0: u8 = 0b0000_0000;
const GRAPHICS_WRITE_MODE_1: u8 = 0b0000_0001;
const GRAPHICS_WRITE_MODE_2: u8 = 0b0000_0010;
const GRAPHICS_WRITE_MODE_3: u8 = 0b0000_0011;

const GRAPHICS_MEM_MAP_128K: u8 = 0b0000;
const GRAPHICS_MEM_MAP_64K: u8 = 0b0100;
const GRAPHICS_MEM_MAP_32K_B0000_B7FFF: u8 = 0b1000;
const GRAPHICS_MEM_MAP_32K_B8000_BFFFF: u8 = 0b1100;

//This bit controls alphanumeric mode addressing. When set to 1, this bit selects graphics modes, which also disables the character generator latches.
const GRAPHICS_ALPHA_DISABLE: u8 = 0b0000_0001;


const ATTRIBUTE_MODE_CONTROL: VGARegister = VGARegister { reg: 0x3C0, index: PALETTE_ADDRESS_SOURCE | 0x10 };
const HORIZONTAL_PANNING: VGARegister = VGARegister { reg: 0x3C0, index: PALETTE_ADDRESS_SOURCE | 0x13 };

const MODE_X_REGS_BASIC: &[(VGARegister, u8)] = &[
    (MISCELLANEOUS_OUTPUT_REGISTER ,1),
    //enable 256 color
    (ATTRIBUTE_MODE_CONTROL, ATTRIBUTE_8BIT | ATTRIBUTE_ATGE),
    (GRAPHICS_MODE, GRPAHICS_SHIFT256),

    //disable alpha
    (MISCELLANEOUS_GRAPHICS, GRAPHICS_MEM_MAP_128K | GRAPHICS_ALPHA_DISABLE),

    //8 pixels per character
    (CLOCK_MODE_98_DOT_MODE, 0x01),
    
    //CONSTANTS
    (LOGICAL_WIDTH, 0x28),
    //Enable MAP13 and MAP14
    (MODE_CONTROL, 0b0000_0011),


    (OVERFLOW_REGISTER, 0b0000_0010),
    
    (HORIZONTAL_DISPLAY_END, 0x4F),
    (VERTICAL_DISPLAY_END, 0xDF),
    
    (MAXIMUM_SCAN_LINE, 0b0100_0001),

];

const MODE_X_REGS: &[(VGARegister, u8)] = &[
    //only needs to be one, but the spec says 0xE3
    //1 makes sure the mapping is correct
    //if 0 then 0x3D4 would be mapped to 0x3B4
    //also controls clock
    (MISCELLANEOUS_OUTPUT_REGISTER ,1),
    
    // (HORIZONTAL_PANNING, 0x0),
    //make it so that there are 8 bits per color
    //enalbe the attribute controller graphics
    (ATTRIBUTE_MODE_CONTROL, ATTRIBUTE_8BIT | ATTRIBUTE_ATGE),
    
    //did not affect output
    // (SETRESET, 0x00),
    //another step to enable 256 color
    (GRAPHICS_MODE, GRPAHICS_SHIFT256),
    
    //select the 
    (MISCELLANEOUS_GRAPHICS, GRAPHICS_MEM_MAP_128K | GRAPHICS_ALPHA_DISABLE),

    //1 = 8 pixels per character, 0 = 9 pixels per character
    (CLOCK_MODE_98_DOT_MODE, 0x01),
    //chain 4 TODO: COME BACK HERE FIRST IF BREAK
    // (SEQUENCER_MEMORY_MODE, 0b1000),

    //not required?
    // (HORIZONTAL_TOTAL, 0x5F),

    (HORIZONTAL_DISPLAY_END, 0x4F),

    // (HORIZONTAL_BLANK_START, 0x50),
    // (HORIZONTAL_BLANK_END, 0x82),
    // (HORIZONTAL_RETRACE_START, 0x54),
    // (HORIZONTAL_RETRACE_END, 0x80),

    // (VERTICAL_TOTAL, 0x0D),
    (OVERFLOW_REGISTER, 0x3E),
    // (PRESET_ROW_SCAN, 0x00),

    (MAXIMUM_SCAN_LINE, 0x41),

    // (VERTICAL_RETRACE_START, 0xEA),
    // (VERTICAL_RETRACE_END, 0xAC),
    
    (VERTICAL_DISPLAY_END, 0xDF),
    (LOGICAL_WIDTH, 0x28),
    // (UNDERLINE_LOCATION, 0x00),
    // (VERTICAL_BLANK_START, 0xE7),
    // (VERTICAL_BLANK_END, 0x06),
    (MODE_CONTROL, 0xE3),
];



static PALETTE: [u32; 256] = [
    0x000000, 0x0000AA, 0x00AA00, 0x00AAAA, 0xAA0000, 0xAA00AA, 0xAA5500, 0xAAAAAA, 0x555555,
    0x5555FF, 0x55FF55, 0x55FFFF, 0xFF5555, 0xFF55FF, 0xFFFF55, 0xFFFFFF, 0x000000, 0x330000,
    0x660000, 0x990000, 0xCC0000, 0xFF0000, 0x003300, 0x333300, 0x663300, 0x993300, 0xCC3300,
    0xFF3300, 0x006600, 0x336600, 0x666600, 0x996600, 0xCC6600, 0xFF6600, 0x009900, 0x339900,
    0x669900, 0x999900, 0xCC9900, 0xFF9900, 0x00CC00, 0x33CC00, 0x66CC00, 0x99CC00, 0xCCCC00,
    0xFFCC00, 0x00FF00, 0x33FF00, 0x66FF00, 0x99FF00, 0xCCFF00, 0xFFFF00, 0x000033, 0x330033,
    0x660033, 0x990033, 0xCC0033, 0xFF0033, 0x003333, 0x333333, 0x663333, 0x993333, 0xCC3333,
    0xFF3333, 0x006633, 0x336633, 0x666633, 0x996633, 0xCC6633, 0xFF6633, 0x009933, 0x339933,
    0x669933, 0x999933, 0xCC9933, 0xFF9933, 0x00CC33, 0x33CC33, 0x66CC33, 0x99CC33, 0xCCCC33,
    0xFFCC33, 0x00FF33, 0x33FF33, 0x66FF33, 0x99FF33, 0xCCFF33, 0xFFFF33, 0x000066, 0x330066,
    0x660066, 0x990066, 0xCC0066, 0xFF0066, 0x003366, 0x333366, 0x663366, 0x993366, 0xCC3366,
    0xFF3366, 0x006666, 0x336666, 0x666666, 0x996666, 0xCC6666, 0xFF6666, 0x009966, 0x339966,
    0x669966, 0x999966, 0xCC9966, 0xFF9966, 0x00CC66, 0x33CC66, 0x66CC66, 0x99CC66, 0xCCCC66,
    0xFFCC66, 0x00FF66, 0x33FF66, 0x66FF66, 0x99FF66, 0xCCFF66, 0xFFFF66, 0x000099, 0x330099,
    0x660099, 0x990099, 0xCC0099, 0xFF0099, 0x003399, 0x333399, 0x663399, 0x993399, 0xCC3399,
    0xFF3399, 0x006699, 0x336699, 0x666699, 0x996699, 0xCC6699, 0xFF6699, 0x009999, 0x339999,
    0x669999, 0x999999, 0xCC9999, 0xFF9999, 0x00CC99, 0x33CC99, 0x66CC99, 0x99CC99, 0xCCCC99,
    0xFFCC99, 0x00FF99, 0x33FF99, 0x66FF99, 0x99FF99, 0xCCFF99, 0xFFFF99, 0x0000CC, 0x3300CC,
    0x6600CC, 0x9900CC, 0xCC00CC, 0xFF00CC, 0x0033CC, 0x3333CC, 0x6633CC, 0x9933CC, 0xCC33CC,
    0xFF33CC, 0x0066CC, 0x3366CC, 0x6666CC, 0x9966CC, 0xCC66CC, 0xFF66CC, 0x0099CC, 0x3399CC,
    0x6699CC, 0x9999CC, 0xCC99CC, 0xFF99CC, 0x00CCCC, 0x33CCCC, 0x66CCCC, 0x99CCCC, 0xCCCCCC,
    0xFFCCCC, 0x00FFCC, 0x33FFCC, 0x66FFCC, 0x99FFCC, 0xCCFFCC, 0xFFFFCC, 0x0000FF, 0x3300FF,
    0x6600FF, 0x9900FF, 0xCC00FF, 0xFF00FF, 0x0033FF, 0x3333FF, 0x6633FF, 0x9933FF, 0xCC33FF,
    0xFF33FF, 0x0066FF, 0x3366FF, 0x6666FF, 0x9966FF, 0xCC66FF, 0xFF66FF, 0x0099FF, 0x3399FF,
    0x6699FF, 0x9999FF, 0xCC99FF, 0xFF99FF, 0x00CCFF, 0x33CCFF, 0x66CCFF, 0x99CCFF, 0xCCCCFF,
    0xFFCCFF, 0x00FFFF, 0x33FFFF, 0x66FFFF, 0x99FFFF, 0xCCFFFF, 0xFFFFFF, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
#[inline(always)]
fn _color_component_to_safe_color(c: u8) -> u8 {
    const TABLE: [u8; 256] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    ];
    TABLE[c as usize]
}
pub struct Mode13Display {
    base: *mut u8,
    last_x: i32,
    last_y: i32,
    last_length: i32,
}


pub struct ModeXDisplay {
    base: *mut u8,
    last_x: i32,
    last_y: i32,
    last_length: i32,
    width: u32,
    height: u32,
}

impl ModeXDisplay {
    #[inline]
    pub unsafe fn new(base: *mut u8) -> Self {
        Self { base, last_x: 0, last_y: 10, last_length: 0, width: 320, height: 240 }
    }



    pub fn set_pixel(&mut self, coord: Point, color: Rgb888) -> Option<()> {
        let x = coord.x;
        let y = coord.y;

        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = x as usize + y as usize * self.width as usize;

            let r = _color_component_to_safe_color(color.r());
            let g = _color_component_to_safe_color(color.g());
            let b = _color_component_to_safe_color(color.b());
            let color = 16 + r + g * 6 + b * 36;

            unsafe {
                self.base.add(index).write_volatile(color);
            }

            Some(())
        } else {
            None
        }
    }
    pub fn rectangle(&mut self, x: i32, y: i32, width: i32, height: i32, color: Rgb888) {
        for y in y..y + height {
            for x in x..x + width {
                self.set_pixel(Point::new(x, y), color);
            }
        }
    }
    
    pub fn print_pos(&mut self, text: &str, x: i32, y: i32, color: Rgb888) {
        Text::new(
            text,
            Point::new(x, y),
            MonoTextStyleBuilder::new()
                .font(&FONT_6X12)
                .text_color(color)
                .build(),
            )
            .draw(self)
            .unwrap();
        
        self.last_x = x;
        self.last_y = y;
        self.last_length = text.len() as i32 * 6;
    }
    pub fn print_with_color(&mut self, text: &str, color: Rgb888) {

        let x = self.last_x + self.last_length; 
        self.print_pos(text, x, self.last_y, color);
    }
    pub fn println_with_color(&mut self, text: &str, color: Rgb888) {
        self.print_pos(text, 0, self.last_y + 10, color);
    }

    pub fn println(&mut self, text: &str) {
        self.print_pos(text, 0, self.last_y + 10, Rgb888::WHITE);
    }

    pub fn print(&mut self, text: &str) {
        self.print_with_color(text, Rgb888::WHITE);
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(Point::new(x as i32, y as i32), Rgb888::BLACK);
            }
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

impl DrawTarget for ModeXDisplay {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            self.set_pixel(coord, color);
        }
        Ok(())
    }
}

impl OriginDimensions for ModeXDisplay {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

