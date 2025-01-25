use super::Irqs;
use embassy_rp::{
    peripherals::{DMA_CH0, PIN_16, PIO0},
    pio::Pio,
    pio_programs::ws2812::{PioWs2812, PioWs2812Program},
};
use smart_leds::RGB8;

const NUM_LEDS: usize = 1;

pub struct NeoPixel<'d> {
    pub ws2812: PioWs2812<'d, PIO0, 0, 1>,
}

impl<'d> NeoPixel<'d> {
    pub fn new(pio_0: PIO0, dma: DMA_CH0, pin: PIN_16) -> Self {
        let Pio {
            mut common, sm0, ..
        } = Pio::new(pio_0, Irqs);

        let program = PioWs2812Program::new(&mut common);
        let ws2812 = PioWs2812::new(&mut common, sm0, dma, pin, &program);
        Self { ws2812 }
    }

    pub async fn set_light(&mut self, color: RGB8) {
        let data = [color; NUM_LEDS];
        self.ws2812.write(&data).await;
    }

    pub async fn set_light_with_brightness(&mut self, color: RGB8, brightness: u8) {
        self.set_light(self.get_color_brightness(color, brightness))
            .await;
    }

    pub async fn off(&mut self) {
        self.set_light(RGB8::default()).await;
    }

    fn get_color_brightness(&self, color: RGB8, brightness: u8) -> RGB8 {
        RGB8 {
            r: (color.r as u16 * (brightness as u16 + 1) / 256) as u8,
            g: (color.g as u16 * (brightness as u16 + 1) / 256) as u8,
            b: (color.b as u16 * (brightness as u16 + 1) / 256) as u8,
        }
    }
}
