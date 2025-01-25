//! Hello world example final

#![no_std]
#![no_main]
#![allow(unused_assignments)]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use mtras_workshop::shared::logger::logger_task;
use rp2040_panic_usb_boot as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    //This is how we gain access to the peripherals on the Pico. like GPIO, USB, i2c, spi etc.
    let p = embassy_rp::init(Default::default());

    //This spawns a task. Tasks can run asynchronously of each other. We will not focus on the code in the shared folder but just know
    //This runs a spawned task I wrote in the shared folder. for logging via USB serial.
    //If you ctrl + c from cargo run it will reboot the Pico into BOOTSEL mode
    //Also can be done by sending q to the serial monitor
    spawner.spawn(logger_task(p.USB)).unwrap();

    //Logging takes a bit to get going and for elf2uf2-rs to connect so we wait a bit.
    //Also a good time to show that with embassy we have async. So when you call the Timer it needs to be awaited
    //Any thing you see with async in front of it needs to be awaited, there is also a warning during build
    Timer::after(Duration::from_secs(2)).await;

    log::info!("I am running on the Pico!");

    log::info!("Can press ctrl + c to stop and the Pico will reboot to BOOTSEL mode");

    //Rust has an ownership system that helps prevent memory leaks. It's a bit difficult at first but we will go over some basics here

    //You can set a variable to a value like this
    let one = 1;
    //Then can write out the value of that variable to the logger like this
    log::info!("One: {}", one);

    //But if you want to change the value of that variable you need to use the mut keyword
    let mut maybe_two = 1;
    //We increment the value of maybe_two by 1 to get it to 2
    maybe_two += 1;
    log::info!("Two: {}", maybe_two);
    //This type of ownership helps ensure memory safety and a lower memory footprint
    let mut counter = 0;
    loop {
        log::info!("Counter: {}", counter);
        counter += 1;
        Timer::after(Duration::from_secs(1)).await;
    }
}
