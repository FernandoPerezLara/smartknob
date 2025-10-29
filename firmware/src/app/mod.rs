mod state;

use alloc::boxed::Box;

use embassy_time::{Duration, Timer};
use log::{debug, error, info};

pub use self::state::AppState;
use crate::{
    error::SmartknobError,
    hardware::Hardware,
    peripherals::{
        display::{Display, graphics::Color},
        encoder::{ANGLE_TO_DEGREES, Encoder},
    },
    ui::{LightView, View, ViewManager},
};

pub struct App {
    display: Display,
    encoder: Encoder,
    view: ViewManager,
    state: AppState,
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

        let encoder = Encoder::new(hardware.encoder_spi);
        debug!("Encoder interface created successfully");

        let mut view = ViewManager::new();
        view.add(Box::new(LightView::new("Light 1")));

        let state = AppState::new();

        Ok(Self {
            display,
            encoder,
            view,
            state,
        })
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
        self.view.select(0, &self.state, &mut self.display)?;
        self.display.render().await?;

        info!("Starting main loop");
        loop {
            let position = self.encoder.read().await?;

            self.state.angle = position.angle as f32 * ANGLE_TO_DEGREES;

            self.display.clear(Color::BLACK);
            self.display.set_pixel(120, 120, 0xF800);
            self.view.select(0, &self.state, &mut self.display)?;
            self.display.render().await?;

            Timer::after(Duration::from_millis(16)).await;
        }
    }
}
