// //! This example test the RP Pico W on board LED.
// //!
// //! It does not work with the RP Pico board.

// #![no_std]
// #![no_main]
// #![allow(non_local_definitions)]

// // use defmt::*;
// use core::str;
// use defmt_rtt as _;
// use embassy_executor::Spawner;
// use embassy_rp::rom_data::reset_to_usb_boot;
// use embassy_rp::usb::Driver;
// use embassy_rp::{
//     bind_interrupts,
//     peripherals::{PIO0, USB},
//     pio::Pio,
//     pio_programs::ws2812::{PioWs2812, PioWs2812Program},
// };
// use embassy_time::{Duration, Ticker, Timer};
// use embassy_usb_logger::ReceiverHandler;
// use mtras_workshop::shared::led;
// use rp2040_panic_usb_boot as _;
// use smart_leds::RGB8;

// bind_interrupts!(struct Irqs {
//     PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
//     USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
// });

// struct USBSerialHandler;

// impl ReceiverHandler for USBSerialHandler {
//     async fn handle_data(&self, data: &[u8]) {
//         if let Ok(data) = str::from_utf8(data) {
//             let data = data.trim();
//             // If you are using elf2uf2-term with the '-t' flag, then when closing the serial monitor,
//             // this will automatically put the pico into boot mode
//             if data == "q" || data == "elf2uf2-term" {
//                 reset_to_usb_boot(0, 0); // Restart the chip
//             }
//         }
//     }

//     fn new() -> Self {
//         Self
//     }
// }

// #[embassy_executor::task]
// async fn logger_task(driver: Driver<'static, USB>) {
//     //TODO latest has an example of this. See if we can get to build without git and clean up the cargo.toml
//     // embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
//     embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver, USBSerialHandler);
// }

// #[embassy_executor::main]
// async fn main(spawner: Spawner) {
//     let p = embassy_rp::init(Default::default());

//     let driver = Driver::new(p.USB, Irqs);
//     spawner.spawn(logger_task(driver)).unwrap();
//     //Waits for logging to be set up
//     Timer::after(Duration::from_secs(2)).await;

//     log::info!("Start");

//     let Pio {
//         mut common, sm0, ..
//     } = Pio::new(p.PIO0, Irqs);

//     // This is the number of leds in the string. Helpfully, the sparkfun thing plus and adafruit
//     // feather boards for the 2040 both have one built in.
//     const NUM_LEDS: usize = 1;
//     let mut data = [RGB8::default(); NUM_LEDS];

//     // Common neopixel pins:
//     // Thing plus: 8
//     // Adafruit Feather: 16;  Adafruit Feather+RFM95: 4
//     let program = PioWs2812Program::new(&mut common);
//     let mut ws2812 = PioWs2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_16, &program);

//     // Loop forever making RGB values and pushing them out to the WS2812.
//     let mut ticker = Ticker::every(Duration::from_millis(10));
//     let mut counter = 0;
//     loop {
//         for j in 0..(256 * 5) {
//             log::debug!("New Colors:");
//             for i in 0..NUM_LEDS {
//                 data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
//                 log::debug!("R: {} G: {} B: {}", data[i].r, data[i].g, data[i].b);
//             }
//             ws2812.write(&data).await;

//             ticker.next().await;
//         }
//         counter += 1;
//         log::info!("Tick {}", counter);
//         // panic!("This is a panic");
//     }
// }
