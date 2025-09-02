use super::error::SpiError;
use embedded_hal_async::spi::SpiBus;
use esp_hal::dma::{AnyGdmaChannel, DmaChannelConvert, DmaChannelFor, DmaRxBuf, DmaTxBuf};
use esp_hal::gpio::{InputPin, Output, OutputPin};
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config, Instance, Spi, SpiDmaBus};
use esp_hal::time::Rate;
use esp_hal::{Async, dma_buffers};
use log::debug;

const DMA_BUFFER_SIZE: usize = 32000;

pub struct SpiInterface {
    spi: SpiDmaBus<'static, Async>,
    cs: Output<'static>,
}

impl SpiInterface {
    #[allow(clippy::too_many_arguments)]
    pub fn new<SPI, DMA, SCLK, MOSI, MISO>(
        frequency: u32,
        mode: Mode,
        spi_instance: SPI,
        dma_channel: DMA,
        sclk: SCLK,
        mosi: MOSI,
        miso: MISO,
        cs: Output<'static>,
    ) -> Result<Self, SpiError>
    where
        SPI: Instance + 'static,
        DMA: DmaChannelConvert<AnyGdmaChannel<'static>> + DmaChannelFor<SPI> + 'static,
        SCLK: OutputPin + 'static,
        MOSI: OutputPin + 'static,
        MISO: InputPin + 'static,
    {
        if !(0..=80).contains(&frequency) {
            return Err(SpiError::invalid_parameters(
                "Frequency must be between 0Mhz and 80Mhz",
            ));
        }

        debug!("Initializing SPI interface");

        let spi_config = Config::default()
            .with_frequency(Rate::from_mhz(frequency))
            .with_mode(mode);

        #[allow(clippy::manual_div_ceil)]
        let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(DMA_BUFFER_SIZE);
        let dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer)?;
        let dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer)?;

        let spi = Spi::new(spi_instance, spi_config)
            .map_err(SpiError::from)?
            .with_sck(sclk)
            .with_mosi(mosi)
            .with_miso(miso)
            .with_dma(dma_channel)
            .with_buffers(dma_rx_buf, dma_tx_buf)
            .into_async();

        debug!("SPI interface initialized successfully");

        Ok(Self { spi, cs })
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<(), SpiError> {
        if data.is_empty() {
            return Err(SpiError::invalid_parameters(
                "Write data buffer cannot be empty",
            ));
        }

        self.cs.set_low();

        let result = SpiBus::write(&mut self.spi, data).await;

        if result.is_ok() {
            let _ = SpiBus::flush(&mut self.spi).await;
        }

        self.cs.set_high();

        result.map_err(|_| SpiError::write_failed("Failed to write data to SPI bus"))
    }

    pub async fn read(&mut self, data: &mut [u8]) -> Result<(), SpiError> {
        if data.is_empty() {
            return Err(SpiError::invalid_parameters(
                "Read data buffer cannot be empty",
            ));
        }

        self.cs.set_low();

        let result = SpiBus::read(&mut self.spi, data).await;

        self.cs.set_high();

        result.map_err(|_| SpiError::read_failed("Failed to read data from SPI bus"))
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

        self.cs.set_low();

        let result = SpiBus::transfer(&mut self.spi, read, write).await;

        self.cs.set_high();

        result.map_err(|_| SpiError::read_failed("Failed to read data from SPI bus"))
    }
}
