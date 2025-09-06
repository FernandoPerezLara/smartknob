use crate::error::SmartknobError;
use crate::hardware::Hardware;
use crate::peripherals::display::Display;
use crate::peripherals::display::graphics::{Circle, Line};
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

        match self.display.set_background(0xF800).await {
            Ok(_) => info!("Screen filled successfully"),
            Err(e) => error!("Failed to fill screen: {}", e),
        }
        let circle = Circle {
            x: 119,
            y: 119,
            radius: 50,
            color: 0x001F,
        };
        self.display.draw(&circle);
        let line = Line {
            x1: 0,
            y1: 0,
            x2: 239,
            y2: 239,
            color: 0x07E0,
        };
        self.display.draw(&line);

        self.display.render().await?;

        info!("Starting main loop");
        loop {
            Timer::after(Duration::from_millis(1000)).await;
        }
    }
}
