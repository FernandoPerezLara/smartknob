use crate::error::SmartknobError;
use crate::hardware::Hardware;
use crate::peripherals::display::Display;
use embassy_time::{Duration, Timer};
use log::{debug, error, info};

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
        );
        debug!("Display interface created successfully");

        Ok(Self { display })
    }

    pub async fn run(&mut self) -> Result<(), SmartknobError> {
        match self.display.begin().await {
            Ok(_) => info!("Display initialized successfully"),
            Err(e) => {
                error!("Failed to initialize display: {}", e);
                return Err(e.into());
            },
        }

        info!("Starting main loop");

        match self.display.set_background(0xF800).await {
            Ok(_) => info!("Screen filled successfully"),
            Err(e) => error!("Failed to fill screen: {}", e),
        }

        let mut x = 0;
        let mut y = 0;

        loop {
            self.display.set_pixel(x, y, 0x07E0).await?;

            x += 1;
            if x >= 240 {
                x = 0;
            }
            y += 1;
            if y >= 240 {
                y = 0;
            }

            Timer::after(Duration::from_millis(10)).await;
        }
    }
}
