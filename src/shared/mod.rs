use embassy_rp::{
    bind_interrupts,
    peripherals::{PIO0, USB},
};

pub mod led;
pub mod logger;

bind_interrupts!(pub struct Irqs {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});
