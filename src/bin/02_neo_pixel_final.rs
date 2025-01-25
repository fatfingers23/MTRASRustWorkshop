//! Hello world example final

#![no_std]
#![no_main]
#![allow(unused_assignments)]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker, Timer};
use mtras_workshop::shared::{led::NeoPixel, logger::logger_task};
use rp2040_panic_usb_boot as _;
use smart_leds::{colors, RGB8};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    spawner.spawn(logger_task(p.USB)).unwrap();

    //Logging takes a bit to get going and for elf2uf2-rs to connect so we wait a bit.
    Timer::after(Duration::from_secs(2)).await;

    //This is something else I wrote in shared. It's a struct that controls the NeoPixel on the Pico via PIO that is doing PWM
    let mut neo_pixel = NeoPixel::new(p.PIO0, p.DMA_CH0, p.PIN_16);

    //You can set the color via rgb 0-255
    let bright_white = RGB8::new(255, 255, 255);
    neo_pixel.set_light(bright_white).await;
    Timer::after(Duration::from_secs(2)).await;

    //Or we can use the smart_Leds crate and set the color with friendly names
    //with the set_light_with_brightness function you can set the brightness as well 0-255
    neo_pixel.set_light_with_brightness(colors::BLUE, 25).await;

    Timer::after(Duration::from_secs(2)).await;
    //Can also turn off the neopixel, to turn back on just set a color again
    neo_pixel.off().await;

    let mut brightness = 0;
    let mut count = 0;

    //**BONUS: set the brightness to go up with each iteration **/
    let mut ticker = Ticker::every(Duration::from_millis(10));
    loop {
        for j in 0..(256 * 5) {
            let new_color = wheel(((256 as u16 / 1 as u16 + j as u16) & 255) as u8);
            log::info!(
                "New color: R: {} G: {} B: {}",
                new_color.r,
                new_color.g,
                new_color.b
            );
            neo_pixel.set_light(new_color).await;
            ticker.next().await;
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
