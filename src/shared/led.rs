use embassy_rp::peripherals::PIO0;
use smart_leds::RGB8;

// /// Input a value 0 to 255 to get a color value
// /// The colours are a transition r - g - b - back to r.
// pub fn wheel(mut wheel_pos: u8) -> RGB8 {
//     wheel_pos = 255 - wheel_pos;
//     if wheel_pos < 85 {
//         return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
//     }
//     if wheel_pos < 170 {
//         wheel_pos -= 85;
//         return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
//     }
//     wheel_pos -= 170;
//     (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
// }

pub struct NeoPixel {}

impl NeoPixel {
    pub fn new(pio_0: PIO0) -> Self {
        NeoPixel {}
    }
}
