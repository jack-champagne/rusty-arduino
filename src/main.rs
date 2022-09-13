#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut led = pins.d13.into_output();
    let sense = pins.d12.into_pull_up_input();
    led.set_high();

    loop {
        if sense.is_high() {
            led.toggle();
            arduino_hal::delay_ms(1000);
        }
        arduino_hal::delay_us(1);
    }
}
