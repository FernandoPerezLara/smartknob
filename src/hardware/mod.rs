mod spi;

use crate::HardwareError;
use esp_hal::spi::Mode;
use esp_hal::timer::systimer::SystemTimer;
pub use spi::SpiInterface;

pub struct Hardware {
    pub display_spi: SpiInterface,
}

impl Hardware {
    pub async fn init() -> Result<Self, HardwareError> {
        let peripherals = esp_hal::init(esp_hal::Config::default());

        let timer = SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(timer.alarm0);

        let display_spi = SpiInterface::new(
            100,
            Mode::_0,
            peripherals.SPI2,
            peripherals.GPIO19,
            peripherals.GPIO18,
            peripherals.GPIO20,
            peripherals.GPIO21,
        )?;

        Ok(Self { display_spi })
    }
}
