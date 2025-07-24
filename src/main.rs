#![no_std]
#![no_main]

use avr_device::entry;
use panic_halt as _;

mod drivers;

#[entry]
fn main() -> ! {
    // your init & loop
    loop {}
}
