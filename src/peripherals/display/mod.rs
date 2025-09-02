mod commands;
mod config;
pub mod error;
pub mod graphics;

use self::config::CONFIG;
use self::error::DisplayError;
use crate::hardware::spi::SpiInterface;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;
use log::{debug, info};

const DISPLAY_WIDTH: u16 = 240;
const DISPLAY_HEIGHT: u16 = 240;
const BUFFER_SIZE: usize = 480;

enum Operation {
    Command(u8),
    Data(&'static [u8]),
    Delay(u64),
}

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
                Operation::Data(data) => self.write_data(data).await?,
                Operation::Delay(delay) => Timer::after(Duration::from_millis(*delay)).await,
            }
        }

        Ok(())
    }

    async fn write_command(&mut self, command: u8) -> Result<(), DisplayError> {
        self.dc.set_low();
        self.spi.write(&[command]).await?;

        Ok(())
    }

    pub async fn write_data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        self.dc.set_high();
        self.spi.write(data).await?;

        Ok(())
    }

    pub async fn sleep(&mut self) -> Result<(), DisplayError> {
        debug!("Putting display to sleep");

        self.write_command(commands::DISPOFF).await?;
        self.write_command(commands::SLPIN).await?;

        Ok(())
    }

    pub async fn wake(&mut self) -> Result<(), DisplayError> {
        debug!("Waking display");

        self.write_command(commands::SLPOUT).await?;
        self.write_command(commands::DISPON).await?;

        Ok(())
    }

    pub async fn set_frame(
        &mut self,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
    ) -> Result<(), DisplayError> {
        debug!(
            "Setting frame: x1: {}, y1: {}, x2: {}, y2: {}",
            x1, y1, x2, y2
        );

        if x1 > x2 || y1 > y2 {
            return Err(DisplayError::OutOfBounds { x1, x2, y1, y2 });
        }

        if x1 >= DISPLAY_WIDTH || y1 >= DISPLAY_HEIGHT {
            return Err(DisplayError::OutOfBounds {
                x1,
                y1,
                x2: DISPLAY_WIDTH,
                y2: DISPLAY_HEIGHT,
            });
        }

        if x2 >= DISPLAY_WIDTH || y2 >= DISPLAY_HEIGHT {
            return Err(DisplayError::OutOfBounds {
                x1: x2,
                y1: y2,
                x2: DISPLAY_WIDTH,
                y2: DISPLAY_HEIGHT,
            });
        }

        self.write_command(commands::CASET).await?;
        self.write_data(&[
            (x1 >> 8) as u8,
            (x1 & 0xFF) as u8,
            (x2 >> 8) as u8,
            (x2 & 0xFF) as u8,
        ])
        .await?;

        self.write_command(commands::RASET).await?;
        self.write_data(&[
            (y1 >> 8) as u8,
            (y1 & 0xFF) as u8,
            (y2 >> 8) as u8,
            (y2 & 0xFF) as u8,
        ])
        .await?;

        self.write_command(commands::RAMWR).await?;

        Ok(())
    }
}
