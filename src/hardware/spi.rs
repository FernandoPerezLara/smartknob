use crate::HardwareError;
use esp_hal::gpio::{InputPin, OutputPin};
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config, Instance, Spi};
use esp_hal::time::Rate;

pub struct SpiInterface {
    spi: Spi<'static, esp_hal::Async>,
}

impl SpiInterface {
    pub fn new<SPI, SCLK, MOSI, MISO, CS>(
        frequency: u32,
        mode: Mode,
        spi_instance: SPI,
        sclk: SCLK,
        mosi: MOSI,
        miso: MISO,
        cs: CS,
    ) -> Result<Self, HardwareError>
    where
        SPI: Instance + 'static,
        SCLK: OutputPin + 'static,
        MOSI: OutputPin + 'static,
        MISO: InputPin + 'static,
        CS: OutputPin + 'static,
    {
        let spi_config = Config::default()
            .with_frequency(Rate::from_khz(frequency))
            .with_mode(mode);

        let spi = Spi::new(spi_instance, spi_config)
            .map_err(HardwareError::SpiConfig)?
            .with_sck(sclk)
            .with_mosi(mosi)
            .with_miso(miso)
            .with_cs(cs)
            .into_async();

        Ok(Self { spi })
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<(), HardwareError> {
        embedded_hal_async::spi::SpiBus::write(&mut self.spi, data)
            .await
            .map_err(|_| HardwareError::SpiWrite)
    }

    pub async fn read(&mut self, data: &mut [u8]) -> Result<(), HardwareError> {
        embedded_hal_async::spi::SpiBus::read(&mut self.spi, data)
            .await
            .map_err(|_| HardwareError::SpiRead)
    }

    pub async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), HardwareError> {
        embedded_hal_async::spi::SpiBus::transfer(&mut self.spi, read, write)
            .await
            .map_err(|_| HardwareError::SpiTransfer)
    }
}
