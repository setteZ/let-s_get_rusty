#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::cell::RefCell;
use critical_section::Mutex;

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::usb::usb_serial_jtag::UsbSerialJtag;

use heapless::Vec;

const RX_CAPACITY: usize = 256;

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

static USB_SERIAL: Mutex<RefCell<Option<UsbSerialJtag<'static, esp_hal::Blocking>>>> =
    Mutex::new(RefCell::new(None));

// Bytes collected by the interrupt handler, waiting to be parsed.
static RX_BUFFER: Mutex<RefCell<Vec<u8, RX_CAPACITY>>> = Mutex::new(RefCell::new(Vec::new()));

#[main]
fn main() -> ! {
    // generator version: 1.4.0
    // generator parameters: -o esp32c6 -o helix

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let mut usb_serial = UsbSerialJtag::new(_peripherals.USB_DEVICE);
    usb_serial.set_interrupt_handler(usb_ih);
    usb_serial.listen_rx_packet_recv_interrupt();
    usb_serial.write(b"ready\n\r");
    critical_section::with(|cs| USB_SERIAL.borrow_ref_mut(cs).replace(usb_serial));

    loop {
        process_rx();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.2.2/examples
}

fn process_rx() {
    critical_section::with(|cs| {
        let mut buf = RX_BUFFER.borrow_ref_mut(cs);

        if let Some(newline_pos) = buf.iter().position(|&b| b == b'\n') {
            let line_len = newline_pos + 1; // include the '\n' itself

            let mut usb_serial = USB_SERIAL.borrow_ref_mut(cs);
            if let Some(usb_serial) = usb_serial.as_mut() {
                usb_serial.write(&buf[..line_len]).ok();
            }

            // Shift any bytes after this line down to the front, then
            // shrink the buffer to just what's left.
            let remaining_len = buf.len() - line_len;
            buf.copy_within(line_len.., 0);
            buf.truncate(remaining_len);
        }
    });
}

#[esp_hal::handler]
fn usb_ih() {
    critical_section::with(|cs| {
        let mut usb_serial = USB_SERIAL.borrow_ref_mut(cs);
        if let Some(usb_serial) = usb_serial.as_mut() {
            let mut buf = RX_BUFFER.borrow_ref_mut(cs);
            while let nb::Result::Ok(c) = usb_serial.read_byte() {
                // If the buffer's full, incoming bytes are dropped until
                // process_rx() catches up and frees some space.
                let _ = buf.push(c);
            }

            usb_serial.reset_rx_packet_recv_interrupt();
        }
    });
}
