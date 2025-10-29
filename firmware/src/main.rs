#![no_std]
#![no_main]

use embassy_executor::Spawner;
use esp_alloc as _;
use esp_backtrace as _;
use log::{error, info, warn};
use smartknob::App;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    esp_alloc::heap_allocator!(256 * 1024);

    esp_println::logger::init_logger_from_env();

    let mut app = match App::new().await {
        Ok(app) => {
            info!("Smartknob application initialized successfully");
            app
        },
        Err(e) => {
            error!("Failed to initialize Smartknob: {}", e);
            return;
        },
    };

    if let Err(e) = app.run().await {
        error!("Application error: {}", e);
    }

    warn!("Application has exited unexpectedly. Please check the logs for details");
}
