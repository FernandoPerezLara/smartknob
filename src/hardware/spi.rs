use crate::hardware::error::SpiError;
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
    ) -> Result<Self, SpiError>
    where
        SPI: Instance + 'static,
        SCLK: OutputPin + 'static,
        MOSI: OutputPin + 'static,
        MISO: InputPin + 'static,
        CS: OutputPin + 'static,
    {
        if !(0..=80).contains(&frequency) {
            return Err(SpiError::invalid_parameters(
                "Frequency must be between 0Mhz and 80Mhz",
            ));
        }

        let spi_config = Config::default()
            .with_frequency(Rate::from_mhz(frequency))
            .with_mode(mode);

        let spi = Spi::new(spi_instance, spi_config)
            .map_err(SpiError::from)?
            .with_sck(sclk)
            .with_mosi(mosi)
            .with_miso(miso)
            .with_cs(cs)
            .into_async();

        Ok(Self { spi })
    }

    pub async fn _write(&mut self, data: &[u8]) -> Result<(), SpiError> {
        if data.is_empty() {
            return Err(SpiError::invalid_parameters(
                "Write data buffer cannot be empty",
            ));
        }

        embedded_hal_async::spi::SpiBus::write(&mut self.spi, data)
            .await
            .map_err(|_| SpiError::write_failed("Failed to write data to SPI bus"))
    }

    pub async fn _read(&mut self, data: &mut [u8]) -> Result<(), SpiError> {
        if data.is_empty() {
            return Err(SpiError::invalid_parameters(
                "Read data buffer cannot be empty",
            ));
        }

        embedded_hal_async::spi::SpiBus::read(&mut self.spi, data)
            .await
            .map_err(|_| SpiError::read_failed("Failed to read data from SPI bus"))
    }

    pub async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), SpiError> {
        if read.is_empty() && write.is_empty() {
            return Err(SpiError::invalid_parameters(
                "Read and write data buffers cannot be empty",
            ));
        }

        if read.len() != write.len() {
            return Err(SpiError::invalid_parameters(
                "Read and write data buffers must have the same length",
            ));
        }

        embedded_hal_async::spi::SpiBus::transfer(&mut self.spi, read, write)
            .await
            .map_err(|_| SpiError::transfer_failed("Failed to transfer data on SPI bus"))
    }
}
