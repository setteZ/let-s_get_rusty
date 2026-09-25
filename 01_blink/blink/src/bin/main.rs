#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::main;
use esp_hal::spi::master::{Config as SpiConfig, Spi};
use esp_hal::time::{Duration, Instant, Rate};

use smart_leds::{RGB8, SmartLedsWrite};
use ws2812_spi::Ws2812;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.4.0
    // generator parameters: -o esp32c6 -o helix

    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    let spi = Spi::new(
        peripherals.SPI2,
        SpiConfig::default().with_frequency(Rate::from_khz(2400)),
    )
    .unwrap()
    .with_mosi(peripherals.GPIO8);

    let mut turn_led_on = true;
    let mut ws = Ws2812::new(spi);
    let led_on = [RGB8::new(30, 30, 30)];
    let led_off = [RGB8::new(0, 0, 0)];

    loop {
        if turn_led_on {
            ws.write(led_on.into_iter()).unwrap();
        } else {
            ws.write(led_off.into_iter()).unwrap();
        }
        turn_led_on = !turn_led_on;

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.2.2/examples
}
