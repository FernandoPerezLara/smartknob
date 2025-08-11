#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::systimer::SystemTimer;
use esp_println::logger::init_logger_from_env;
use log::{debug, info};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let mut led = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());

    info!("SmartKnob initialized successfully");

    loop {
        debug!("LED ON");
        led.set_high();
        Timer::after(Duration::from_millis(2000)).await;

        debug!("LED OFF");
        led.set_low();
        Timer::after(Duration::from_millis(2000)).await;
    }
}
