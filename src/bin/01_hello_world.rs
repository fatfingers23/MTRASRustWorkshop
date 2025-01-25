//! Hello world example

#![no_std]
#![no_main]
#![allow(non_local_definitions)]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use mtras_workshop::shared::logger::logger_task;
use rp2040_panic_usb_boot as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    //This spawns a task. Tasks can run asynchronously of each other
    spawner.spawn(logger_task(p.USB)).unwrap();
    //Logging takes a bit to get going and for elf2uf2-rs to connect so we wait a bit. Not needed
    Timer::after(Duration::from_secs(2)).await;

    log::info!("I am running on the Pico!");
    log::info!("Can press ctrl + c to stop and the Pico will reboot to BOOTSEL mode");
    let mut counter = 0;
    //TODO find a way to show them that this is wrong and what the compiler error looks like
    // let counter = 0;
    loop {
        log::info!("Counter: {}", counter);
        counter += 1;
        Timer::after(Duration::from_secs(1)).await;
    }
}
