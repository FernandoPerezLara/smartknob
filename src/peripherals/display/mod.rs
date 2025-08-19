pub mod error;

use crate::hardware::spi::SpiInterface;
use embassy_time::{Duration, Timer};
use error::DisplayError;
use esp_hal::gpio::Output;
use log::{debug, info};

const DISPLAY_WIDTH: u16 = 240;
const DISPLAY_HEIGHT: u16 = 240;
const BUFFER_SIZE: usize = 480;

#[allow(dead_code)]
mod commands {
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
}

enum Operation {
    Command(u8),
    Data(u8),
    Delay(u64),
}

const CONFIG: &[Operation] = &[
    Operation::Command(0xEF),
    Operation::Command(0xEB),
    Operation::Data(0x14),
    Operation::Command(0xFE),
    Operation::Command(0xEF),
    Operation::Command(0xEB),
    Operation::Data(0x14),
    Operation::Command(0x84),
    Operation::Data(0x40),
    Operation::Command(0x85),
    Operation::Data(0xFF),
    Operation::Command(0x86),
    Operation::Data(0xFF),
    Operation::Command(0x87),
    Operation::Data(0xFF),
    Operation::Command(0x88),
    Operation::Data(0x0A),
    Operation::Command(0x89),
    Operation::Data(0x21),
    Operation::Command(0x8A),
    Operation::Data(0x00),
    Operation::Command(0x8B),
    Operation::Data(0x80),
    Operation::Command(0x8C),
    Operation::Data(0x01),
    Operation::Command(0x8D),
    Operation::Data(0x01),
    Operation::Command(0x8E),
    Operation::Data(0xFF),
    Operation::Command(0x8F),
    Operation::Data(0xFF),
    Operation::Command(0xB6),
    Operation::Data(0x00),
    Operation::Data(0x20),
    Operation::Command(commands::MADCTL),
    Operation::Data(0x08),
    Operation::Command(commands::COLMOD),
    Operation::Data(0x05),
    Operation::Command(0x90),
    Operation::Data(0x08),
    Operation::Data(0x08),
    Operation::Data(0x08),
    Operation::Data(0x08),
    Operation::Command(0xBD),
    Operation::Data(0x06),
    Operation::Command(0xBC),
    Operation::Data(0x00),
    Operation::Command(0xFF),
    Operation::Data(0x60),
    Operation::Data(0x01),
    Operation::Data(0x04),
    Operation::Command(0xC3),
    Operation::Data(0x13),
    Operation::Command(0xC4),
    Operation::Data(0x13),
    Operation::Command(0xC9),
    Operation::Data(0x22),
    Operation::Command(0xBE),
    Operation::Data(0x11),
    Operation::Command(0xE1),
    Operation::Data(0x10),
    Operation::Data(0x0E),
    Operation::Command(0xDF),
    Operation::Data(0x21),
    Operation::Data(0x0C),
    Operation::Data(0x02),
    Operation::Command(0xF0),
    Operation::Data(0x45),
    Operation::Data(0x09),
    Operation::Data(0x08),
    Operation::Data(0x08),
    Operation::Data(0x26),
    Operation::Data(0x2A),
    Operation::Command(0xF1),
    Operation::Data(0x43),
    Operation::Data(0x70),
    Operation::Data(0x72),
    Operation::Data(0x36),
    Operation::Data(0x37),
    Operation::Data(0x6F),
    Operation::Command(0xF2),
    Operation::Data(0x45),
    Operation::Data(0x09),
    Operation::Data(0x08),
    Operation::Data(0x08),
    Operation::Data(0x26),
    Operation::Data(0x2A),
    Operation::Command(0xF3),
    Operation::Data(0x43),
    Operation::Data(0x70),
    Operation::Data(0x72),
    Operation::Data(0x36),
    Operation::Data(0x37),
    Operation::Data(0x6F),
    Operation::Command(0xED),
    Operation::Data(0x1B),
    Operation::Data(0x0B),
    Operation::Command(0xAE),
    Operation::Data(0x77),
    Operation::Command(0xCD),
    Operation::Data(0x63),
    Operation::Command(0x70),
    Operation::Data(0x07),
    Operation::Data(0x07),
    Operation::Data(0x04),
    Operation::Data(0x0E),
    Operation::Data(0x0F),
    Operation::Data(0x09),
    Operation::Data(0x07),
    Operation::Data(0x08),
    Operation::Data(0x03),
    Operation::Command(0xE8),
    Operation::Data(0x34),
    Operation::Command(0x62),
    Operation::Data(0x18),
    Operation::Data(0x0D),
    Operation::Data(0x71),
    Operation::Data(0xED),
    Operation::Data(0x70),
    Operation::Data(0x70),
    Operation::Data(0x18),
    Operation::Data(0x0F),
    Operation::Data(0x71),
    Operation::Data(0xEF),
    Operation::Data(0x70),
    Operation::Data(0x70),
    Operation::Command(0x63),
    Operation::Data(0x18),
    Operation::Data(0x11),
    Operation::Data(0x71),
    Operation::Data(0xF1),
    Operation::Data(0x70),
    Operation::Data(0x70),
    Operation::Data(0x18),
    Operation::Data(0x13),
    Operation::Data(0x71),
    Operation::Data(0xF3),
    Operation::Data(0x70),
    Operation::Data(0x70),
    Operation::Command(0x64),
    Operation::Data(0x28),
    Operation::Data(0x29),
    Operation::Data(0xF1),
    Operation::Data(0x01),
    Operation::Data(0xF1),
    Operation::Data(0x00),
    Operation::Data(0x07),
    Operation::Command(0x66),
    Operation::Data(0x3C),
    Operation::Data(0x00),
    Operation::Data(0xCD),
    Operation::Data(0x67),
    Operation::Data(0x45),
    Operation::Data(0x45),
    Operation::Data(0x10),
    Operation::Data(0x00),
    Operation::Data(0x00),
    Operation::Data(0x00),
    Operation::Command(0x67),
    Operation::Data(0x00),
    Operation::Data(0x3C),
    Operation::Data(0x00),
    Operation::Data(0x00),
    Operation::Data(0x00),
    Operation::Data(0x01),
    Operation::Data(0x54),
    Operation::Data(0x10),
    Operation::Data(0x32),
    Operation::Data(0x98),
    Operation::Command(0x74),
    Operation::Data(0x10),
    Operation::Data(0x85),
    Operation::Data(0x80),
    Operation::Data(0x00),
    Operation::Data(0x00),
    Operation::Data(0x4E),
    Operation::Data(0x00),
    Operation::Command(0x98),
    Operation::Data(0x3E),
    Operation::Data(0x07),
    Operation::Command(commands::TEON),
    Operation::Command(commands::INVON),
    Operation::Command(commands::SLPOUT),
    Operation::Delay(120),
    Operation::Command(commands::DISPON),
    Operation::Delay(20),
];

pub struct Display {
    spi: SpiInterface,
    dc: Output<'static>,
    rst: Output<'static>,
}

impl Display {
    pub fn new(spi: SpiInterface, dc: Output<'static>, rst: Output<'static>) -> Self {
        Self { spi, dc, rst }
    }

    pub async fn begin(&mut self) -> Result<(), DisplayError> {
        info!("Initializing display");

        self.hardware_reset().await;
        self.initialize_display().await?;

        info!("Display initialized successfully");
        Ok(())
    }

    async fn hardware_reset(&mut self) {
        debug!("Resetting display");

        self.rst.set_high();
        Timer::after(Duration::from_millis(10)).await;
        self.rst.set_low();
        Timer::after(Duration::from_millis(120)).await;
        self.rst.set_high();
        Timer::after(Duration::from_millis(120)).await;
    }

    async fn initialize_display(&mut self) -> Result<(), DisplayError> {
        debug!("Executing display initialization sequence");

        for operation in CONFIG.iter() {
            match operation {
                Operation::Command(command) => self.write_command(*command).await?,
                Operation::Data(data) => self.write_data(*data).await?,
                Operation::Delay(delay) => Timer::after(Duration::from_millis(*delay)).await,
            }
        }

        Ok(())
    }

    async fn write_command(&mut self, command: u8) -> Result<(), DisplayError> {
        self.dc.set_low();
        self.spi.write(&mut [command]).await?;

        Ok(())
    }

    async fn write_data(&mut self, data: u8) -> Result<(), DisplayError> {
        self.dc.set_high();
        self.spi.write(&mut [data]).await?;

        Ok(())
    }

    async fn set_frame(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) -> Result<(), DisplayError> {
        debug!(
            "Setting frame: x1: {}, y1: {}, x2: {}, y2: {}",
            x1, y1, x2, y2
        );

        if x1 >= DISPLAY_WIDTH
            || x2 >= DISPLAY_WIDTH
            || y1 >= DISPLAY_HEIGHT
            || y2 >= DISPLAY_HEIGHT
        {
            return Err(DisplayError::OutOfBounds { x1, y1, x2, y2 });
        }

        self.write_command(commands::CASET).await?;
        self.write_data((x1 >> 8) as u8).await?;
        self.write_data((x1 & 0xFF) as u8).await?;
        self.write_data((x2 >> 8) as u8).await?;
        self.write_data((x2 & 0xFF) as u8).await?;

        self.write_command(commands::RASET).await?;
        self.write_data((y1 >> 8) as u8).await?;
        self.write_data((y1 & 0xFF) as u8).await?;
        self.write_data((y2 >> 8) as u8).await?;
        self.write_data((y2 & 0xFF) as u8).await?;

        self.write_command(commands::RAMWR).await?;

        Ok(())
    }

    pub async fn set_background(&mut self, color: u16) -> Result<(), DisplayError> {
        debug!("Setting background color: 0x{:04X}", color);

        self.sleep().await?;

        self.set_frame(0, 0, DISPLAY_WIDTH - 1, DISPLAY_HEIGHT - 1)
            .await?;

        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;

        let mut buffer = [0u8; BUFFER_SIZE];
        for i in (0..BUFFER_SIZE).step_by(2) {
            buffer[i] = hi;
            buffer[i + 1] = lo;
        }

        self.dc.set_high();

        for _row in 0..DISPLAY_HEIGHT {
            self.spi.write(&mut buffer).await?;
        }

        self.wake().await?;

        Ok(())
    }

    async fn sleep(&mut self) -> Result<(), DisplayError> {
        debug!("Putting display to sleep");

        self.write_command(commands::DISPOFF).await?;
        self.write_command(commands::SLPIN).await?;

        Ok(())
    }

    async fn wake(&mut self) -> Result<(), DisplayError> {
        debug!("Waking display");

        self.write_command(commands::SLPOUT).await?;
        self.write_command(commands::DISPON).await?;

        Ok(())
    }
}
