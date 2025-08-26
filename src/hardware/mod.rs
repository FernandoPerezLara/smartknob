pub mod error;
pub mod spi;

use self::error::HardwareError;
use self::spi::SpiInterface;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::peripherals::Peripherals;
use esp_hal::spi::Mode;
use esp_hal::timer::systimer::SystemTimer;
use log::{debug, info};

pub struct Pins {
    pub display_dc: Output<'static>,
    pub display_rst: Output<'static>,
}

pub struct Hardware {
    pub display_spi: SpiInterface,
    pub pins: Pins,
}

impl Hardware {
    pub async fn init() -> Result<Self, HardwareError> {
        info!("Initializing components");

        let peripherals = Self::init_peripherals()?;

        let timer = SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(timer.alarm0);

        let display_spi = SpiInterface::new(
            40,
            Mode::_0,
            peripherals.SPI2,
            peripherals.GPIO19,
            peripherals.GPIO18,
            peripherals.GPIO20,
            Output::new(peripherals.GPIO0, Level::High, OutputConfig::default()),
        )?;

        let pins = Pins {
            display_dc: Output::new(peripherals.GPIO1, Level::High, OutputConfig::default()),
            display_rst: Output::new(peripherals.GPIO2, Level::High, OutputConfig::default()),
        };

        info!("Components initialized successfully");

        Ok(Self { display_spi, pins })
    }

    fn init_peripherals() -> Result<Peripherals, HardwareError> {
        debug!("Initializing ESP32 peripherals");

        let peripherals = esp_hal::init(esp_hal::Config::default());

        debug!("ESP32 peripherals initialized successfully");
        Ok(peripherals)
    }
}
