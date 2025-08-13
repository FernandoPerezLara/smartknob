mod error;
mod spi;

pub use error::HardwareError;
use esp_hal::spi::Mode;
use esp_hal::timer::systimer::SystemTimer;
use log::info;
pub use spi::SpiInterface;

pub struct Hardware {
    pub display_spi: SpiInterface,
}

impl Hardware {
    pub async fn init() -> Result<Self, HardwareError> {
        info!("Initializing components");

        // let peripherals = Self::init_peripherals()?;
        let peripherals = esp_hal::init(esp_hal::Config::default());

        // Self::init_embassy(peripherals)?;
        let timer = SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(timer.alarm0);

        // TODO: Probar a usar &peripherals.SPI2...
        let display_spi = SpiInterface::new(
            25,
            Mode::_0,
            peripherals.SPI2,
            peripherals.GPIO19,
            peripherals.GPIO18,
            peripherals.GPIO20,
            peripherals.GPIO21,
        )?;

        info!("Components initialized successfully");

        Ok(Self { display_spi })
    }

    // fn init_peripherals() -> Result<Peripherals, HardwareError> {
    //     debug!("Initializing ESP32 peripherals");

    //     match esp_hal::init(esp_hal::Config::default()) {
    //         peripherals => {
    //             debug!("ESP32 peripherals initialized successfully");
    //             Ok(peripherals)
    //         }
    //     }
    // }

    // fn init_embassy(peripherals: Peripherals) -> Result<(), HardwareError> {
    //     debug!("Initializing Embassy timer");

    //     let timer = SystemTimer::new(peripherals.SYSTIMER);

    //     esp_hal_embassy::init(timer.alarm0);

    //     debug!("Embassy timer initialized successfully");
    //     Ok(())
    // }
}
