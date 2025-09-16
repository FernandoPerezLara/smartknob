use crate::error::SmartknobError;
use crate::hardware::Hardware;
use crate::peripherals::display::Display;
use crate::peripherals::display::graphics::{Alignment, Color, FilledCircle, Text};
use alloc::string::ToString;
use core::f32;
use libm::{cosf, sinf};
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

        const BLACK: Color = Color::BLACK;
        const WHITE: Color = Color::WHITE;

        self.display.clear(BLACK);
        self.display.render().await?;

        let mut position = 0;
        let radius = 120 - 5 - 10;

        info!("Starting main loop");
        loop {
            self.display.clear(BLACK);

            let angle = (position * 2) as f32 * f32::consts::PI / 180.0 - f32::consts::PI / 2.0;
            let x = 120.0 + (radius as f32) * cosf(angle);
            let y = 120.0 + (radius as f32) * sinf(angle);

            self.display.draw(&FilledCircle {
                x: x as u16,
                y: y as u16,
                diameter: 12,
                color: WHITE,
            });

            self.display.draw(&Text {
                content: position.to_string(),
                x: 120,
                y: 120,
                alignment: Alignment::Center,
                color: WHITE,
            });

            position += 1;

            if position > 180 {
                position = 0;
            }

            self.display.render().await?;
        }
    }
}
