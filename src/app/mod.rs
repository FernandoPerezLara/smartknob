use crate::error::SmartknobError;
use crate::hardware::Hardware;
use crate::peripherals::display::Display;
use embassy_time::{Duration, Timer};
use log::{debug, info};

pub struct App {
    display: Display,
}

impl App {
    pub async fn new() -> Result<Self, SmartknobError> {
        info!("Starting application");

        let hardware = Hardware::init().await?;
        debug!("Components initialized successfully");

        let display = Display::new(
            hardware.display_spi,
            hardware.pins.display_dc,
            hardware.pins.display_rst,
            hardware.pins.display_cs,
        );

        // display.begin().await?;

        Ok(Self { display })
    }

    pub async fn run(&mut self) -> Result<(), SmartknobError> {
        match self.display.begin().await {
            Ok(_) => info!("Display initialized successfully"),
            Err(e) => {
                log::error!("Failed to initialize display: {:?}", e);
                return Err(e.into());
            },
        }

        log::info!("Starting main loop");

        loop {
            // Rojo
            match self.display.set_background(0xF800).await {
                Ok(_) => info!("Screen filled successfully"),
                Err(e) => log::error!("Failed to fill screen: {:?}", e),
            }

            Timer::after(Duration::from_millis(3000)).await;

            // Verde
            match self.display.set_background(0x07E0).await {
                Ok(_) => info!("Screen filled successfully"),
                Err(e) => log::error!("Failed to fill screen: {:?}", e),
            }

            Timer::after(Duration::from_millis(3000)).await;

            // Azul
            match self.display.set_background(0x001F).await {
                Ok(_) => info!("Screen filled successfully"),
                Err(e) => log::error!("Failed to fill screen: {:?}", e),
            }

            Timer::after(Duration::from_millis(3000)).await;
        }
    }
}
