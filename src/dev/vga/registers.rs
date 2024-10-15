pub(super) struct VGARegister {
    pub reg: u16,
    pub index: u8,
}

pub(super) const CRTC: u16 = 0x3D4;

pub(super) const HORIZONTAL_TOTAL: VGARegister = VGARegister { reg: CRTC, index: 0x00 };
pub(super) const HORIZONTAL_DISPLAY_END: VGARegister = VGARegister { reg: CRTC, index: 0x01 };
pub(super) const HORIZONTAL_BLANK_START: VGARegister = VGARegister { reg: CRTC, index: 0x02 };
pub(super) const HORIZONTAL_BLANK_END: VGARegister = VGARegister { reg: CRTC, index: 0x03 };
pub(super) const HORIZONTAL_RETRACE_START: VGARegister = VGARegister { reg: CRTC, index: 0x04 };
pub(super) const HORIZONTAL_RETRACE_END: VGARegister = VGARegister { reg: CRTC, index: 0x05 };
pub(super) const VERTICAL_TOTAL: VGARegister = VGARegister { reg: CRTC, index: 0x06 };
pub(super) const OVERFLOW_REGISTER: VGARegister = VGARegister { reg: CRTC, index: 0x07 };
pub(super) const PRESET_ROW_SCAN: VGARegister = VGARegister { reg: CRTC, index: 0x08 };
pub(super) const MAXIMUM_SCAN_LINE: VGARegister = VGARegister { reg: CRTC, index: 0x09 };
pub(super) const VERTICAL_RETRACE_START: VGARegister = VGARegister { reg: CRTC, index: 0x10 };
pub(super) const VERTICAL_RETRACE_END: VGARegister = VGARegister { reg: CRTC, index: 0x11 };
pub(super) const VERTICAL_DISPLAY_END: VGARegister = VGARegister { reg: CRTC, index: 0x12 };
pub(super) const LOGICAL_WIDTH: VGARegister = VGARegister { reg: CRTC, index: 0x13 };
pub(super) const UNDERLINE_LOCATION: VGARegister = VGARegister { reg: CRTC, index: 0x14 };
pub(super) const VERTICAL_BLANK_START: VGARegister = VGARegister { reg: CRTC, index: 0x15 };
pub(super) const VERTICAL_BLANK_END: VGARegister = VGARegister { reg: CRTC, index: 0x16 };
pub(super) const MODE_CONTROL: VGARegister = VGARegister { reg: CRTC, index: 0x17 };

pub(super) const SETRESET: VGARegister = VGARegister { reg: 0x3CE, index: 0x00 };
pub(super) const GRAPHICS_MODE: VGARegister = VGARegister { reg: 0x3CE, index: 0x05 };
pub(super) const MISCELLANEOUS_GRAPHICS: VGARegister = VGARegister { reg: 0x3CE, index: 0x06 };

pub(super) const SEQUENCER_MEMORY_MODE: VGARegister = VGARegister { reg: 0x3C4, index: 0x04 };
pub(super) const MAP_MASK: VGARegister = VGARegister { reg: 0x3C4, index: 0x02 };
pub(super) const CLOCK_MODE_98_DOT_MODE: VGARegister = VGARegister { reg: 0x3C4, index: 0x01 };

pub(super) const MISCELLANEOUS_OUTPUT_REGISTER: VGARegister = VGARegister { reg: 0x3C2, index: 0x00 };

pub(super) const PALETTE_ADDRESS_SOURCE: u8 = 0b10_0000;
pub(super) const ATTRIBUTE_P54S: u8 =  0b1000_0000;
pub(super) const ATTRIBUTE_8BIT: u8 =  0b0100_0000;
pub(super) const ATTRIBUTE_PPM: u8 =   0b0010_0000;
pub(super) const ATTRIBUTE_BLINK: u8 = 0b0000_1000;
pub(super) const ATTRIBUTE_LGE: u8 =   0b0000_0100;
pub(super) const ATTRIBUTE_MONO: u8 =  0b0000_0010;
pub(super) const ATTRIBUTE_ATGE: u8 =  0b0000_0001;

pub(super) const GRPAHICS_SHIFT256: u8 = 0b0100_0000;
pub(super) const GRAPHICS_SHIFT_INTERLEAVE: u8 = 0b0010_0000;
pub(super) const GRAPHICS_HOST_ODD_EVEN: u8 = 0b0001_0000;
pub(super) const GRAPHICS_READ_MODE: u8 = 0b0000_1000;

pub(super) enum GraphicsWriteMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

pub(super) const GRAPHICS_MEM_MAP_128K: u8 = 0b0000;
pub(super) const GRAPHICS_MEM_MAP_64K: u8 = 0b0100;
pub(super) const GRAPHICS_MEM_MAP_32K_B0000_B7FFF: u8 = 0b1000;
pub(super) const GRAPHICS_MEM_MAP_32K_B8000_BFFFF: u8 = 0b1100;

pub(super) const GRAPHICS_ALPHA_DISABLE: u8 = 0b0000_0001;

pub(super) const ATTRIBUTE_MODE_CONTROL: VGARegister = VGARegister { reg: 0x3C0, index: PALETTE_ADDRESS_SOURCE | 0x10 };
pub(super) const HORIZONTAL_PANNING: VGARegister = VGARegister { reg: 0x3C0, index: PALETTE_ADDRESS_SOURCE | 0x13 };

pub(super) const MODE_X_REGS_BASIC: &[(VGARegister, u8)] = &[
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

pub(super) const MODE_X_REGS: &[(VGARegister, u8)] = &[
    //only needs to be one, but the spec says 0xE3
    //1 makes sure the mapping is correct
    //if 0 then 0x3D4 would be mapped to 0x3B4
    //also controls clock
    (MISCELLANEOUS_OUTPUT_REGISTER ,1),
    
    // (HORIZONTAL_PANNING, 0x0),
    // //make it so that there are 8 bits per color
    // //enalbe the attribute controller graphics
    (ATTRIBUTE_MODE_CONTROL, ATTRIBUTE_ATGE),
    
    // //did not affect output
    // (SETRESET, 0x00),
    // //another step to enable 256 color
    (GRAPHICS_MODE, GRPAHICS_SHIFT256),
    
    // //select the 
    (MISCELLANEOUS_GRAPHICS, GRAPHICS_MEM_MAP_128K | GRAPHICS_ALPHA_DISABLE),

    //1 = 8 pixels per character, 0 = 9 pixels per character
    (CLOCK_MODE_98_DOT_MODE, 0x01),
    //chain 4 TODO: COME BACK HERE FIRST IF BREAK
    (SEQUENCER_MEMORY_MODE, 0b1000),

    //not required?
    (HORIZONTAL_TOTAL, 0x5F),

    (HORIZONTAL_DISPLAY_END, 0x4F),

    (HORIZONTAL_BLANK_START, 0x50),
    (HORIZONTAL_BLANK_END, 0x82),
    (HORIZONTAL_RETRACE_START, 0x54),
    (HORIZONTAL_RETRACE_END, 0x80),

    (VERTICAL_TOTAL, 0x0D),
    (OVERFLOW_REGISTER, 0x3E),
    (PRESET_ROW_SCAN, 0x00),

    (MAXIMUM_SCAN_LINE, 0x41),

    (VERTICAL_RETRACE_START, 0xEA),
    (VERTICAL_RETRACE_END, 0xAC),
    
    (VERTICAL_DISPLAY_END, 0xDF),
    (LOGICAL_WIDTH, 0x28),
    (UNDERLINE_LOCATION, 0x00),
    (VERTICAL_BLANK_START, 0xE7),
    (VERTICAL_BLANK_END, 0x06),
    (MODE_CONTROL, 0xE3),
];

