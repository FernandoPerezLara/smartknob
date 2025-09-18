use crate::error::SmartknobError;
use crate::hardware::Hardware;
use crate::peripherals::display::Display;
use crate::peripherals::display::graphics::Color;
use crate::ui::{LightView, View, ViewManager};
use alloc::boxed::Box;
use embassy_time::{Duration, Timer};
use log::{debug, error, info};

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
        view.add(Box::new(LightView::new("Luz Dormitorio")));
        view.add(Box::new(LightView::new("Luz Salon")));

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

        const BLACK: Color = Color::BLACK;

        self.display.clear(BLACK);
        self.view.select(0, &mut self.display);
        self.display.render().await?;

        let mut index = 0;

        info!("Starting main loop");
        loop {
            self.display.clear(BLACK);
            self.view.select(index, &mut self.display);
            self.display.render().await?;
            index += 1;
            if index > self.view.len() {
                index = 0;
            }
            Timer::after(Duration::from_millis(1000)).await;
        }
    }
}
