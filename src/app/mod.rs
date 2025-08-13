use crate::error::{HardwareError, SmartknobError};
use crate::hardware::Hardware;
use embassy_time::{Duration, Timer};
use log::{debug, info};

pub struct App {
    hardware: Hardware,
}

impl App {
    pub async fn new() -> Result<Self, SmartknobError> {
        info!("Starting application");

        // let hardware = match Hardware::init().await {
        //     Ok(hardware) => {
        //         debug!("Hardware initialized successfully");
        //         hardware
        //     },
        //     Err(e) => {
        //         error!("Failed to initialize hardware: {:?}", e);
        //         return Err(SmartknobError::Hardware(e));
        //     },
        // };

        let hardware = Hardware::init().await?;
        debug!("Hardware initialized successfully");

        Ok(Self { hardware })
    }

    pub async fn run(&mut self) -> Result<(), SmartknobError> {
        log::info!("Starting main loop");

        let send_buffer = [0, 1, 2, 3, 4, 5, 6, 7];
        loop {
            let mut read_buffer = [0; 8];

            self.hardware
                .display_spi
                .transfer(&mut read_buffer, &send_buffer)
                .await
                .map_err(|e| SmartknobError::Hardware(HardwareError::Spi(e)))?;

            info!("Bytes sent: {:?}", send_buffer);
            info!("Bytes received: {:?}", read_buffer);

            Timer::after(Duration::from_millis(5_000)).await;
        }
    }
}
