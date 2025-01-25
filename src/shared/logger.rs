use core::str;
use embassy_rp::{peripherals::USB, rom_data::reset_to_usb_boot, usb::Driver};
use embassy_usb_logger::ReceiverHandler;

use crate::shared::Irqs;

struct USBSerialHandler;

impl ReceiverHandler for USBSerialHandler {
    async fn handle_data(&self, data: &[u8]) {
        if let Ok(data) = str::from_utf8(data) {
            let data = data.trim();
            // If you are using elf2uf2-term with the '-t' flag, then when closing the serial monitor,
            // this will automatically put the pico into boot mode
            if data == "q" || data == "elf2uf2-term" {
                reset_to_usb_boot(0, 0); // Restart the chip
            }
        }
    }

    fn new() -> Self {
        Self
    }
}

// #[embassy_executor::task]
// pub async fn logger_task(driver: Driver<'static, USB>) {
//     //Starts a task that logs and listens for a reboot to bootsel
//     embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver, USBSerialHandler);
// }

#[embassy_executor::task]
pub async fn logger_task(usb: USB) {
    let driver = Driver::new(usb, Irqs);
    //Starts a task that logs and listens for a reboot to bootsel
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver, USBSerialHandler);
}
