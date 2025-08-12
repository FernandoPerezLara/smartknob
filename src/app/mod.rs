use crate::HardwareError;
use crate::hardware::Hardware;
use embassy_time::{Duration, Timer};
use log::{error, info};

pub struct App {
    hardware: Hardware,
}

impl App {
    pub async fn new() -> Result<Self, HardwareError> {
        let hardware = Hardware::init().await?;

        Ok(Self { hardware })
    }

    pub async fn run(&mut self) -> Result<(), HardwareError> {
        log::info!("Starting Smartknob application");

        let send_buffer = [0, 1, 2, 3, 4, 5, 6, 7];
        loop {
            let mut read_buffer = [0; 8];

            if let Err(e) = self
                .hardware
                .display_spi
                .transfer(&mut read_buffer, &send_buffer)
                .await
            {
                error!("SPI transfer failed: {:?}", e);
                Timer::after(Duration::from_millis(1000)).await;
                continue;
            }

            info!("Bytes sent: {:?}", send_buffer);
            info!("Bytes received: {:?}", read_buffer);

            Timer::after(Duration::from_millis(5_000)).await;
        }
    }
}
