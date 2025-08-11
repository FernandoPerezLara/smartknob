#![no_std]
#![no_main]

use core::fmt;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config, ConfigError, Spi};
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
// use esp_println::logger::init_logger_from_env;
// use log::{debug, error, info};
use log::{error, info};

esp_bootloader_esp_idf::esp_app_desc!();

enum HardwareError {
    SpiConfig(ConfigError),
}

impl From<ConfigError> for HardwareError {
    fn from(err: ConfigError) -> Self {
        HardwareError::SpiConfig(err)
    }
}

impl fmt::Display for HardwareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HardwareError::SpiConfig(e) => write!(f, "Failed to configure SPI: {e:?}"),
        }
    }
}

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // init_logger_from_env();
    esp_println::logger::init_logger(log::LevelFilter::Debug);

    // let hardware = Hardware::init().await?;
    // let mut app = App::new(hardware);

    // info!("SmartKnob initialized successfully");

    // app.run().await;

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let spi = peripherals.SPI2;
    let sclk = peripherals.GPIO19;
    let mosi = peripherals.GPIO18;
    let miso = peripherals.GPIO20;
    let cs = peripherals.GPIO21;

    let mut spi = match init_spi(spi, sclk, mosi, miso, cs) {
        Ok(spi) => spi,
        Err(e) => {
            error!("{e}");
            return;
        },
    };

    info!("SmartKnob initialized successfully");

    let send_buffer = [0, 1, 2, 3, 4, 5, 6, 7];
    loop {
        let mut buffer = [0; 8];
        info!("Sending bytes");

        if let Err(e) =
            embedded_hal_async::spi::SpiBus::transfer(&mut spi, &mut buffer, &send_buffer).await
        {
            error!("SPI transfer failed: {:?}", e);
            Timer::after(Duration::from_millis(1000)).await;
            continue;
        }

        info!("Bytes received: {:?}", buffer);
        Timer::after(Duration::from_millis(5_000)).await;
    }
}

fn init_spi<SPI, SCLK, MOSI, MISO, CS>(
    spi: SPI,
    sclk: SCLK,
    mosi: MOSI,
    miso: MISO,
    cs: CS,
) -> Result<esp_hal::spi::master::Spi<'static, esp_hal::Async>, HardwareError>
where
    SPI: esp_hal::spi::master::Instance + 'static,
    SCLK: esp_hal::gpio::OutputPin + 'static,
    MOSI: esp_hal::gpio::OutputPin + 'static,
    MISO: esp_hal::gpio::InputPin + 'static,
    CS: esp_hal::gpio::OutputPin + 'static,
{
    let spi = Spi::new(
        spi,
        Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(Mode::_0),
    )?
    .with_sck(sclk)
    .with_mosi(mosi)
    .with_miso(miso)
    .with_cs(cs)
    .into_async();

    Ok(spi)
}
