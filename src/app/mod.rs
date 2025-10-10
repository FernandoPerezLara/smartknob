use alloc::boxed::Box;

use embassy_time::{Duration, Timer};
use log::{debug, error, info};

use crate::{
    error::SmartknobError,
    hardware::Hardware,
    peripherals::display::{Display, graphics::Color},
    ui::{LightView, View, ViewManager},
};

pub struct App {
    display: Display,
    view: ViewManager,
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

        let mut view = ViewManager::new();
        view.add(Box::new(LightView::new("Bedroom")));

        Ok(Self { display, view })
    }

    pub async fn run(&mut self) -> Result<(), SmartknobError> {
        match self.display.begin().await {
            Ok(_) => info!("Display initialized successfully"),
            Err(e) => {
                error!("Failed to initialize display: {}", e);
                return Err(e.into());
            },
        }

        self.display.clear(Color::BLACK);
        self.view.select(0, &mut self.display)?;
        self.display.render().await?;

        info!("Starting main loop");
        loop {
            Timer::after(Duration::from_millis(1000)).await;
        }
    }
}
