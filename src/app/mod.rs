use crate::error::SmartknobError;
use crate::hardware::Hardware;
use crate::peripherals::display::Display;
use crate::peripherals::display::graphics::{Circle, Figure, Line};
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
        Timer::after(Duration::from_millis(1000)).await;

        let line = Line::new(0, 0, 120, 120, 0x07E0);
        match line.draw(&mut self.display).await {
            Ok(_) => info!("Line drawn successfully"),
            Err(e) => error!("Failed to draw line: {}", e),
        }
        Timer::after(Duration::from_millis(1000)).await;

        let circle = Circle::new(119, 119, 119, 0x001F);
        match circle.draw(&mut self.display).await {
            Ok(_) => info!("Circle drawn successfully"),
            Err(e) => error!("Failed to draw circle: {}", e),
        }
        Timer::after(Duration::from_millis(500)).await;

        loop {
            Timer::after(Duration::from_millis(1000)).await;
        }
    }
}
