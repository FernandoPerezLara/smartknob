use super::{Operation, commands};

#[rustfmt::skip]
pub const CONFIG: &[Operation] = &[
    Operation::Command(0xEF),
    Operation::Command(0xEB),
    Operation::Data(&[0x14]),

    Operation::Command(0xFE),
    Operation::Command(0xEF),

    Operation::Command(0xEB),
    Operation::Data(&[0x14]),

    Operation::Command(0x84),
    Operation::Data(&[0x40]),

    Operation::Command(0x85),
    Operation::Data(&[0xFF]),

    Operation::Command(0x86),
    Operation::Data(&[0xFF]),

    Operation::Command(0x87),
    Operation::Data(&[0xFF]),

    Operation::Command(0x88),
    Operation::Data(&[0x0A]),

    Operation::Command(0x89),
    Operation::Data(&[0x21]),

    Operation::Command(0x8A),
    Operation::Data(&[0x00]),

    Operation::Command(0x8B),
    Operation::Data(&[0x80]),

    Operation::Command(0x8C),
    Operation::Data(&[0x01]),

    Operation::Command(0x8D),
    Operation::Data(&[0x01]),

    Operation::Command(0x8E),
    Operation::Data(&[0xFF]),

    Operation::Command(0x8F),
    Operation::Data(&[0xFF]),

    Operation::Command(0xB6),
    Operation::Data(&[0x00, 0x20]),

    Operation::Command(commands::MADCTL),
    Operation::Data(&[0x08]),

    Operation::Command(commands::COLMOD),
    Operation::Data(&[0x05]),

    Operation::Command(0x90),
    Operation::Data(&[0x08, 0x08, 0x08, 0x08]),

    Operation::Command(0xBD),
    Operation::Data(&[0x06]),

    Operation::Command(0xBC),
    Operation::Data(&[0x00]),

    Operation::Command(0xFF),
    Operation::Data(&[0x60, 0x01, 0x04]),

    Operation::Command(0xC3),
    Operation::Data(&[0x13]),
    Operation::Command(0xC4),
    Operation::Data(&[0x13]),

    Operation::Command(0xC9),
    Operation::Data(&[0x22]),

    Operation::Command(0xBE),
    Operation::Data(&[0x11]),

    Operation::Command(0xE1),
    Operation::Data(&[0x10, 0x0E]),

    Operation::Command(0xDF),
    Operation::Data(&[0x21, 0x0C, 0x02]),

    Operation::Command(0xF0),
    Operation::Data(&[0x45, 0x09, 0x08, 0x08, 0x26, 0x2A]),

    Operation::Command(0xF1),
    Operation::Data(&[0x43, 0x70, 0x72, 0x36, 0x37, 0x6F]),

    Operation::Command(0xF2),
    Operation::Data(&[0x45, 0x09, 0x08, 0x08, 0x26, 0x2A]),

    Operation::Command(0xF3),
    Operation::Data(&[0x43, 0x70, 0x72, 0x36, 0x37, 0x6F]),

    Operation::Command(0xED),
    Operation::Data(&[0x1B, 0x0B]),

    Operation::Command(0xAE),
    Operation::Data(&[0x77]),

    Operation::Command(0xCD),
    Operation::Data(&[0x63]),

    Operation::Command(0x70),
    Operation::Data(&[0x07, 0x07, 0x04, 0x0E, 0x0F, 0x09, 0x07, 0x08, 0x03]),

    Operation::Command(0xE8),
    Operation::Data(&[0x34]),

    Operation::Command(0x62),
    Operation::Data(&[0x18, 0x0D, 0x71, 0xED, 0x70, 0x70, 0x18, 0x0F, 0x71, 0xEF, 0x70, 0x70]),

    Operation::Command(0x63),
    Operation::Data(&[0x18, 0x11, 0x71, 0xF1, 0x70, 0x70, 0x18, 0x13, 0x71, 0xF3, 0x70, 0x70]),

    Operation::Command(0x64),
    Operation::Data(&[0x28, 0x29, 0xF1, 0x01, 0xF1, 0x00, 0x07]),

    Operation::Command(0x66),
    Operation::Data(&[0x3C, 0x00, 0xCD, 0x67, 0x45, 0x45, 0x10, 0x00, 0x00, 0x00]),

    Operation::Command(0x67),
    Operation::Data(&[0x00, 0x3C, 0x00, 0x00, 0x00, 0x01, 0x54, 0x10, 0x32, 0x98]),

    Operation::Command(0x74),
    Operation::Data(&[0x10, 0x85, 0x80, 0x00, 0x00, 0x4E, 0x00]),

    Operation::Command(0x98),
    Operation::Data(&[0x3E, 0x07]),

    Operation::Command(commands::TEON),
    Operation::Command(commands::INVON),

    Operation::Command(commands::SLPOUT),
    Operation::Delay(120),
    Operation::Command(commands::DISPON),
    Operation::Delay(20),
];
