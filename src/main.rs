
use esp_idf_svc::{hal::{delay::FreeRtos, gpio::PinDriver, prelude::Peripherals}, nvs::EspDefaultNvsPartition};
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let mut led = PinDriver::output(peripherals.pins.gpio8).unwrap();
    
    let nvs = EspDefaultNvsPartition::take()?;

    loop {
        led.set_high().unwrap();
        FreeRtos::delay_ms(1000);
        led.set_low().unwrap();
        FreeRtos::delay_ms(1000);
        log::info!("Hello, world!");
    }
}
