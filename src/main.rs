#![no_std]
#![no_main]

use avr_device::entry;
use gps::drivers;
use gps::firmware;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut buzzer = drivers::buzzer::Buzzer::new(
        firmware::buzzer_pwm::BuzzerPwm::new(),
        firmware::shared::delay::BusyDelay::new(),
    );
    loop {
        for i in (0u32..8).cycle() {
            buzzer.tone(400 + i * 120, 50, 100);
        }
    }
}
