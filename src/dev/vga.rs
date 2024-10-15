//! QEMU https://wiki.osdev.org/VGA_Hardware
//! http://www.osdever.net/FreeVGA/vga/vga.htm
pub mod registers;
use core::convert::Infallible;

use crate::println;
use crate::print;

use super::pci::{PCIDevice, PCIError};
use super::vga::registers::{*};
use bitfield_struct::bitfield;
use embedded_graphics::{
    mono_font::{ascii::{FONT_6X12, FONT_7X14}, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};
pub struct VGA {
    pub pci: PCIDevice,
    pub fb: *mut u8,
    pub fb_size: usize,
    pub fb2: *mut u8,
    pub fb2_size: usize,

    pub io: *mut u8,
    pub io_size: usize,

    pub vga_port: *mut u8,
    bochs_base: *mut u16,

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
            let fb_total_size = pci.get_bar_address_size(0) as usize;
            let fb = mem_start as *mut u8;
            let io = (mem_start as usize + fb_total_size) as *mut u8;
            let fb_size = fb_total_size;
            let vga = VGA {              
                fb: fb,
                fb_size: fb_size/2,
                fb2: fb.add(fb_total_size/2),
                fb2_size: fb_size/2,
                io_size: pci.get_bar_address_size(2) as usize,
                pci: pci,
                //offset of ports because qmeu maps then to 0x400, which corresponds to 0x3C0
                //0x400 - 0x3C0 = 0x40
                vga_port: io.add(0x400 - 0x3C0),
                bochs_base: io.add(0x500) as *mut u16,
                io: io,
            };

            //tell the vga which address to use
            (*vga.pci.header.command).set_memory_space(true);
            vga.pci.bar_write(0, vga.fb2 as u32 | 8);
            //bar2
            vga.pci.bar_write(2, vga.io as u32 | 8);
            
            vga.io.add(0x406).write_volatile(0xFF);
            vga.io.add(0x408).write_volatile(0);

            //dac controller
            let p = vga.vga_port.add(0x3C9);
            for rgb in PALETTE.into_iter() {
                let b = rgb as u8;
                let g = (rgb >> 8) as u8;
                let r = (rgb >> 16) as u8;

                p.write_volatile(r);
                p.write_volatile(g);
                p.write_volatile(b);
            }

            Ok(vga)
        }
    }
    unsafe fn set_register(&self, port: u16, index: u8, value: u8) {
        let port_addr = self.vga_port.add(port as usize);

        match port {
            0x3C0 => {
                self.vga_port.add(0x3DA).read_volatile();
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
        let port_addr = self.vga_port.add(port as usize);

        match port {
            0x3C0 => {
                self.vga_port.add(0x3DA).read_volatile();
                port_addr.write_volatile(index);
                port_addr.add(1).read_volatile()

            }
            0x3C2 => {
                // This is the miscellaneous output register. 
                // It uses port 0x3C2 for writing, and 0x3CC for reading. 
                // Bit 0 of this register controls the location of several other registers: 
                // if cleared, port 0x3D4 is mapped to 0x3B4, and port 0x3DA is mapped to 0x3BA. 
                // For readability, only the first port is listed and bit 0 is assumed to be set.
                self.vga_port.add(0x3CC).read_volatile()
            }
            _ => {
                port_addr.write_volatile(index);
                port_addr.add(1).read_volatile()
            }
        }
    }

    fn read_bochs_reg(&self, index: u8) -> u16 {
        unsafe {
            self.bochs_base.add(index as usize).read_volatile()
        }
    }
    fn write_bochs_reg(&self, index: u8, value: u16) {
        unsafe {
            self.bochs_base.add(index as usize).write_volatile(value);
        }
    }

    pub fn get_bochs_version(&self) -> u16 {
        self.read_bochs_reg(0)
    }

    fn set_resolution(&self, width: u32, height: u32) {
        //in qemu width must be divisible by 8
        if width % 8 != 0 {
            panic!("Width must be divisible by 8");
        }
        //setup bochs
        unsafe {
                let regs: &[(VGARegister, u8)] = MODE_X_REGS;
                for (register, data) in regs {
                        self.set_register(register.reg, register.index, *data);
                    }    
                }
        self.write_bochs_reg(VBE_DISPI_INDEX_ENABLE, VBE_DISPI_DISABLED);

        self.write_bochs_reg(VBE_DISPI_INDEX_XRES, width as u16);
        self.write_bochs_reg(VBE_DISPI_INDEX_YRES, height as u16);
        self.write_bochs_reg(VBE_DISPI_INDEX_BPP, VBE_DISPI_BPP_8);

        self.write_bochs_reg(VBE_DISPI_INDEX_ENABLE, VBE_DISPI_ENABLED | VBE_DISPI_NOCLEARMEM | VBE_DISPI_LFB_ENABLED);
    }
    // fn pixels_to_characters(&self, x: i32, y: i32) -> (i32, i32) {
    //     (x / 8, y / 16)
    // }
}


const VBE_DISPI_INDEX_ID: u8 = 0x0;
const VBE_DISPI_INDEX_XRES: u8 = 0x1;
const VBE_DISPI_INDEX_YRES: u8 = 0x2;
const VBE_DISPI_INDEX_BPP: u8 = 0x3;
const VBE_DISPI_INDEX_ENABLE: u8 = 0x4;
const VBE_DISPI_INDEX_BANK: u8 = 0x5;
const VBE_DISPI_INDEX_VIRT_WIDTH: u8 = 0x6;
const VBE_DISPI_INDEX_VIRT_HEIGHT: u8 = 0x7;
const VBE_DISPI_INDEX_X_OFFSET: u8 = 0x8;
const VBE_DISPI_INDEX_Y_OFFSET: u8 = 0x9;

const VBE_DISPI_LFB_ENABLED: u16 = 0x40;

const VBE_DISPI_DISABLED: u16 = 0x00;
const VBE_DISPI_ENABLED: u16 = 0x01;
const VBE_DISPI_VBE_ENABLED: u16 = 0x40;
const VBE_DISPI_NOCLEARMEM: u16 = 0x80;

const VBE_DISPI_BPP_4: u16 = 0x04;
const VBE_DISPI_BPP_8: u16 = 0x08;
const VBE_DISPI_BPP_15: u16 = 0x0F;
const VBE_DISPI_BPP_16: u16 = 0x10;
const VBE_DISPI_BPP_24: u16 = 0x18;
const VBE_DISPI_BPP_32: u16 = 0x20;







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
fn _color_component_to_safe_color(c: Rgb888) -> u8 {
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
    let r = TABLE[c.r() as usize];
    let g = TABLE[c.g() as usize];
    let b = TABLE[c.b() as usize];
    16 + r + g * 6 + b * 36
}
pub struct ModeXDisplay<> {
    vga: VGA,
    last_x: i32,
    last_y: i32,
    last_length: i32,
    pub width: u32,
    pub height: u32,
}

impl<'a> ModeXDisplay<> {
    #[inline]
    pub fn new(vga: VGA, width: u32, height: u32) -> Self {
        vga.set_resolution(width, height);
        Self { vga, last_x: 0, last_y: 10, last_length: 0, width: width, height: height }
    }
    pub fn swap_buffer(&mut self) {
        unsafe {

            self.vga.pci.bar_write(0, self.vga.fb2 as u32 | 8);
        }
    }
    pub fn set_pixel(&mut self, coord: Point, color: Rgb888) -> Option<()> {
        let x = coord.x;
        let y = coord.y;

        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = x as usize + y as usize * self.width as usize;
            let color = _color_component_to_safe_color(color);

            unsafe {
                self.vga.fb.add(index).write_volatile(color);
            }

            Some(())
        } else {
            None
        }
    }
    pub fn set_region(&mut self, coord: Point, size: Size, color: Rgb888) -> Option<()> {
        //make sure size is divisible by 8
        if size.width % 8 != 0 {
            return None;
        }
        if size.height % 8 != 0 {
            return None;
        }
        
        let x = coord.x;
        let y = coord.y;
        let color = _color_component_to_safe_color(color);
        let color_u64 = (color as u64) << 56 | (color as u64) << 48 | (color as u64) << 40 | (color as u64) << 32 | (color as u64) << 24 | (color as u64) << 16 | (color as u64) << 8 | (color as u64);

        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = x as usize + y as usize * self.width as usize;
            unsafe {
                for i in 0..size.height {
                    for j in (0..size.width).step_by(8) {
                        let index = (x + j as i32) as usize + (y + i as i32) as usize * self.width as usize;
                        (self.vga.fb.add(index) as *mut u64).write_volatile(color_u64);
                    }
                }
            }

            Some(())
        } else {
            None
        }
    }
    pub fn rectangle(&mut self, x: i32, y: i32, width: u32, height: u32, color: Rgb888) {
        self.set_region(Point::new(x,y), Size::new(width, height), color);
    }
    
    pub fn print_pos(&mut self, text: &str, x: i32, y: i32, color: Rgb888) {
        //make sure that if x is out of bound: 0 to width, it is set to 0
        let x = x % self.width as i32;
        let y = y%self.height as i32;
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
                let index = x as usize + y as usize * self.width as usize;
                unsafe {

                    
                }
            }
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.vga.set_resolution(width, height);
        self.width = width;
        self.height = height;
    }
}

impl<> DrawTarget for ModeXDisplay<> {
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

impl<> OriginDimensions for ModeXDisplay<> {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

