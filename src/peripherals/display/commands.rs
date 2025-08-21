#![allow(dead_code)]

#[rustfmt::skip]
macro_rules! commands {
    () => {
        pub const RDDID: u8 = 0x04;   // Read Display ID
        pub const RDDST: u8 = 0x09;   // Read Display Status
        pub const SLPIN: u8 = 0x10;   // Sleep In
        pub const SLPOUT: u8 = 0x11;  // Sleep Out
        pub const PTLON: u8 = 0x12;   // Partial Mode On
        pub const NORON: u8 = 0x13;   // Normal Display Mode
        pub const INVOFF: u8 = 0x20;  // Display Inversion
        pub const INVON: u8 = 0x21;   // Display Inversion
        pub const DISPOFF: u8 = 0x28; // Display Off
        pub const DISPON: u8 = 0x29;  // Display On
        pub const CASET: u8 = 0x2A;   // Column Address Set
        pub const RASET: u8 = 0x2B;   // Row
        pub const RAMWR: u8 = 0x2C;   // Memory Write
        pub const RAMRD: u8 = 0x2E;   // Memory Read
        pub const PTLAR: u8 = 0x30;   // Partial Area
        pub const VSCDEF: u8 = 0x33;  // Vertical Scroll Definition
        pub const TEOFF: u8 = 0x34;   // Tearing Effect Line Off
        pub const TEON: u8 = 0x35;    // Tearing
        pub const MADCTL: u8 = 0x36;  // Memory Access Control
        pub const VSCSAD: u8 = 0x37;  // Vertical Scrolling Start Address
        pub const IDMOFF: u8 = 0x38;  // Idle Mode Off
        pub const IDMON: u8 = 0x39;   // Idle Mode On
        pub const COLMOD: u8 = 0x3A;  // Interface Pixel Format
        pub const WRDISBV: u8 = 0x3C; // Write Display Brightness
    };
}

commands!();
